# Ticket State Transition Blueprint
## unverified → verified → reserved → paid

**Version:** 1.0  
**Last Updated:** 2025-01-XX  
**Status:** Draft - Pending Review

---

## Overview

This document defines the state transition flow for tickets from initial listing (`unverified`) through verification, reservation, and payment capture (`paid`). The design emphasizes atomic database operations, race condition prevention, and financial integrity.

### Key Principles

1. **Atomic Operations**: All state transitions use single-query atomic updates
2. **Passive Cleanup**: Expired reservations are automatically considered available
3. **Price Locking**: Price is locked at reservation time to prevent seller manipulation
4. **Grey Period**: `${GREY_PERIOD_MINUTES}`-minute buffer after `${RESERVATION_WINDOW_MINUTES}`-minute reservation window for webhook processing
5. **Idempotency**: All operations must be safe to retry

### Environment Variables

All time durations are configurable via environment variables:

- `TRANSFER_DEADLINE_HOURS` - Time seller has to send transfer request (default: 24)
- `RESERVATION_WINDOW_MINUTES` - Time buyer has to complete checkout (default: 5)
- `GREY_PERIOD_MINUTES` - Buffer time for webhook processing delays (default: 2)
- `TOTAL_RESERVATION_WINDOW_MINUTES` - Total reservation validity (RESERVATION_WINDOW + GREY_PERIOD, default: 7)
- `BOT_POLLING_INTERVAL_SECONDS` - How often bot polls for incoming transfers (default: 20)
- `TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS` - How often to check for expired transfer deadlines (default: 1)
- `RESERVATION_CLEANUP_INTERVAL_SECONDS` - How often to check for expired reservations (default: 60)

---

## Database Schema Changes

### Important: Ticket Uniqueness

**Tickets are unique** - for the same seat number, row, level (seat_section), and game, there is only one ticket. This constraint should be enforced either:
- At the database level with a unique constraint: `UNIQUE (game_id, level, seat_section, seat_row, seat_number)`
- Or at the application level during ticket creation

This uniqueness simplifies edge case handling and prevents duplicate listings for the same physical ticket.

**False Listing Handling:**
When an unverified ticket's transfer deadline expires (indicating a false listing), the ticket is **deleted** from the database rather than marked as `cancelled`. This approach:
- Keeps the database clean (no stale false listing records)
- Simplifies the state machine (no need for `cancelled` → `unverified` transitions)
- Allows the real owner to list the same ticket later without conflicts
- Reduces complexity in ticket creation logic

Note: The `cancelled` status is still used for legitimate tickets that are cancelled after verification (e.g., seller cancels a verified listing, or refund scenarios).

### Required Migrations

#### 1. Add Transfer Deadline Tracking

```sql
-- Track when the transfer deadline expires (TRANSFER_DEADLINE_HOURS after listing)
-- Note: Application code will set this using TRANSFER_DEADLINE_HOURS env var (default: 24 hours)
ALTER TABLE tickets 
ADD COLUMN transfer_deadline TIMESTAMPTZ NOT NULL;

-- Add index for bot polling and deadline checking
CREATE INDEX idx_tickets_unverified_for_verification 
ON tickets(status, transfer_deadline) 
WHERE status = 'unverified';

-- Add index for deadline expiration cleanup
CREATE INDEX idx_tickets_unverified_deadline 
ON tickets(transfer_deadline) 
WHERE status = 'unverified';
```

#### 2. Add Price Locking Column

```sql
-- Store price at reservation time
ALTER TABLE tickets 
ADD COLUMN price_at_reservation INTEGER;

-- Add comment for clarity
COMMENT ON COLUMN tickets.price_at_reservation IS 
'Price locked when ticket was reserved. Used for payment processing.';
```

#### 3. Add Payment Intent Tracking

```sql
-- Track Stripe payment intents for idempotency
CREATE TABLE payment_intents (
    id VARCHAR(255) PRIMARY KEY, -- Stripe payment_intent ID
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE RESTRICT,
    buyer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    amount INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL, -- 'created', 'capturable', 'captured', 'cancelled'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_intents_ticket_id ON payment_intents(ticket_id);
CREATE INDEX idx_payment_intents_status ON payment_intents(status);
```

---

## Stage 1: Verification (unverified → verified)

### Purpose
Ensure the seller transfers the ticket to the custodian Paciolan account within `${TRANSFER_DEADLINE_HOURS}` hours of listing. The bot waits for incoming transfer requests and accepts them if they match the ticket details.

### Process Overview

1. **Seller lists ticket** → Status: `unverified`, `transfer_deadline` = NOW() + `${TRANSFER_DEADLINE_HOURS}` hours
2. **Seller sends transfer request** → Seller manually initiates transfer in Paciolan to custodian account
3. **Bot polls for incoming transfers** → Bot checks Paciolan account for pending transfer requests
4. **Transfer matches ticket** → Bot accepts transfer, ticket becomes `verified`
5. **Transfer deadline expires** → If no transfer received within `${TRANSFER_DEADLINE_HOURS}` hours, ticket is `cancelled`

### Process Flow

#### 1.1 Ticket Creation (Seller Lists Ticket)

When a seller creates a ticket listing:

```sql
-- This happens in the create_ticket handler
-- Application code calculates: NOW() + INTERVAL '1 hour' * TRANSFER_DEADLINE_HOURS
INSERT INTO tickets (
    seller_id, game_id, event_name, event_date,
    level, seat_section, seat_row, seat_number, price, status,
    transfer_deadline
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'unverified', $10)  -- $10 = calculated deadline
RETURNING id, transfer_deadline;
```

**Notification to Seller**: Email sent with instructions:
- "You have ${TRANSFER_DEADLINE_HOURS} hours to transfer your ticket to [custodian account email]"
- "Transfer must match: [event_name], Section [seat_section], Row [seat_row], Seat [seat_number]"

#### 1.2 Bot Polling for Incoming Transfers

The bot continuously polls the Paciolan custodian account for incoming transfer requests:

**Bot Process:**
1. Logs into Paciolan custodian account
2. Checks for pending transfer requests
3. For each pending transfer, extracts ticket details (event, section, row, seat)
4. Matches against `unverified` tickets in database

#### 1.3 Success Path (Transfer Matches Ticket)

When bot finds a matching transfer request:

**Step 1: Find Matching Ticket**
```sql
-- Find unverified ticket matching transfer details
-- Note: Tickets are unique per (game_id, level, seat_section, seat_row, seat_number)
-- This query will return 0 or 1 row
-- FOR UPDATE SKIP LOCKED prevents conflicts with deadline cleanup job
SELECT id, seller_id, event_name, seat_section, seat_row, seat_number
FROM tickets
WHERE status = 'unverified'
  AND event_name = $event_name
  AND seat_section = $seat_section
  AND seat_row = $seat_row
  AND seat_number = $seat_number
  AND transfer_deadline > NOW()  -- Still within deadline
ORDER BY created_at ASC
LIMIT 1
FOR UPDATE SKIP LOCKED;
```

**Step 2: Accept Transfer in Paciolan**
- Bot clicks "Accept" on the transfer request in Paciolan UI
- Transfer completes in external system

**Step 3: Atomic Status Update**
```sql
UPDATE tickets
SET status = 'verified',
    updated_at = NOW()
WHERE id = $ticket_id
  AND status = 'unverified'
  AND transfer_deadline > NOW()  -- Double-check deadline
RETURNING id;
```

**Guardrail**: If 0 rows updated, either:
- Another process already verified it (race condition)
- Deadline expired between match and update

**Notification**: Email sent to seller: "Your ticket has been verified and is now available for sale!"

#### 1.4 Failure Path (Transfer Deadline Expires)

A background job (or bot) periodically checks for expired transfer deadlines:

**Cleanup Query:**
```sql
-- Find tickets that passed their transfer deadline
-- FOR UPDATE SKIP LOCKED prevents conflicts if bot is processing a transfer simultaneously
SELECT id, seller_id, event_name, created_at
FROM tickets
WHERE status = 'unverified'
  AND transfer_deadline <= NOW()
FOR UPDATE SKIP LOCKED;
```

**Atomic Deletion:**
```sql
-- Delete unverified tickets that failed verification (false listings)
-- This is safe because unverified tickets have no foreign key dependencies
-- (no payment_intents, no reservations, etc.)
DELETE FROM tickets
WHERE id = $ticket_id
  AND status = 'unverified'
  AND transfer_deadline <= NOW();
```

**Why Delete Instead of Cancel?**
- **Cleaner database**: No stale false listing records cluttering the database
- **Simpler logic**: Real owner can list the same ticket later without checking for existing `cancelled` records
- **No state transitions needed**: No need to handle `cancelled` → `unverified` transitions
- **Safe to delete**: Unverified tickets have no foreign key dependencies (no payment_intents, no active reservations)

**Notification**: Email sent to seller:
- "Your ticket listing has been removed because the transfer was not received within ${TRANSFER_DEADLINE_HOURS} hours."
- "Please try listing again and ensure you send the transfer request immediately after creating the listing."

#### 1.5 Edge Cases

**Note**: Tickets are unique - for the same seat number, row, level, and game, there is only one ticket. This simplifies edge case handling.

**Transfer Cannot Be Processed (Bot Rejects Transfer):**

From the backend's perspective, the bot rejects/ignores the transfer in the following scenarios (all result in the same action):

1. **No Matching Ticket Found**: Transfer details (event_name, seat_section, seat_row, seat_number) don't match any `unverified` ticket
   - Bot cannot find matching ticket in database
   - Action: Reject transfer, ticket remains `unverified` until deadline expires

2. **Ticket Already Verified**: Bot finds matching ticket, but atomic update returns 0 rows (ticket already `verified`)
   - Can happen if seller sent multiple transfer requests
   - Action: Reject transfer (ticket already processed)

3. **Transfer Arrives After Deadline**: Bot finds matching ticket, but `transfer_deadline <= NOW()` check fails
   - Ticket deadline expired
   - Action: Reject transfer, ticket will be deleted by cleanup job (false listing)

### API Endpoint

**Not Required**: This is an internal bot process. However, for monitoring/debugging:

```
PATCH /api/tickets/:id/verify
Authorization: Bearer <ADMIN_API_KEY>
```

**Response:**
```json
{
  "ticket_id": "uuid",
  "status": "verified",
  "transfer_deadline": "2025-01-XX T12:05:00Z"
}
```

### Timing Considerations

- **Transfer Window**: `${TRANSFER_DEADLINE_HOURS}` hours from ticket creation
- **Bot Polling Frequency**: Should poll every `${BOT_POLLING_INTERVAL_SECONDS}` seconds to catch transfers quickly
- **Deadline Cleanup Frequency**: Should run every `${TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS}` hours to cancel expired tickets promptly

---

## Stage 2: Reservation (verified → reserved)

### Purpose
Create a temporary lock for a specific buyer to prevent "double-buying" while they enter payment details.

### Trigger
Buyer clicks the "Buy" button on the frontend.

### Process Flow

#### 2.1 Availability Check Logic

A ticket is considered **available** if:
- Status is `verified`
- **OR** Status is `reserved` AND the `reserved_at` timestamp is older than `${TOTAL_RESERVATION_WINDOW_MINUTES}` minutes

Note: The total reservation window consists of `${RESERVATION_WINDOW_MINUTES}` minutes for the user timer plus `${GREY_PERIOD_MINUTES}` minutes for the Grey Period buffer.

#### 2.2 Atomic Reservation Query

```sql
-- Application code calculates: NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
-- $expiry_time is calculated in application code using TOTAL_RESERVATION_WINDOW_MINUTES env var
UPDATE tickets
SET status = 'reserved',
    reserved_at = NOW(),
    reserved_by = $buyer_id,
    price_at_reservation = price,  -- Lock the price
    updated_at = NOW()
WHERE id = $ticket_id
  AND (
    status = 'verified'
    OR (
      status = 'reserved' 
      AND reserved_at < $expiry_time  -- $expiry_time = NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
    )
  )
RETURNING id, price_at_reservation, reserved_at;
```

**Critical**: This is a single atomic operation. If 0 rows are returned, the ticket is no longer available.

#### 2.3 Response Handling

**Success (1 row updated):**
- Return ticket details with `status: 'reserved'`
- Frontend redirects to Stripe Checkout with `price_at_reservation`

**Failure (0 rows updated):**
- Return `409 Conflict` with message: "Ticket is no longer available"
- Frontend shows error and refreshes ticket list

### API Endpoint

```
POST /api/tickets/:id/reserve
Authorization: Bearer <JWT_TOKEN>
```

**Request:**
```json
{}
```

**Response (200 OK):**
```json
{
  "ticket_id": "uuid",
  "status": "reserved",
  "price_at_reservation": 15000,
  "reserved_at": "2025-01-XX T12:00:00Z",
  "expires_at": "2025-01-XX T12:00:00Z + ${TOTAL_RESERVATION_WINDOW_MINUTES} minutes"
}
```

**Response (409 Conflict):**
```json
{
  "error": "Ticket is no longer available"
}
```

### Security Considerations

1. **Authentication Required**: Buyer must be authenticated (JWT token)
2. **Self-Reservation Check**: Optionally prevent sellers from reserving their own tickets
3. **Rate Limiting**: Prevent abuse (e.g., max 5 reservations per user per hour)

#### 2.4 Expired Reservation Cleanup

A background job periodically checks for expired reservations and resets them to `verified` status:

**Cleanup Query:**
```sql
-- Find tickets with expired reservations
-- Application code calculates: NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
-- FOR UPDATE SKIP LOCKED prevents conflicts if reservation endpoint is processing simultaneously
SELECT id, reserved_by, reserved_at
FROM tickets
WHERE status = 'reserved'
  AND reserved_at < $expiry_time  -- $expiry_time = NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
FOR UPDATE SKIP LOCKED;
```

**Atomic Reset:**
```sql
-- Reset expired reservations back to verified status
UPDATE tickets
SET status = 'verified',
    reserved_at = NULL,
    reserved_by = NULL,
    price_at_reservation = NULL,
    updated_at = NOW()
WHERE id = $ticket_id
  AND status = 'reserved'
  AND reserved_at < $expiry_time;
```

**Why Active Cleanup?**
- **Clean database**: Prevents tickets from remaining in `reserved` status indefinitely
- **Better analytics**: `reserved` status accurately reflects active reservations
- **Consistent pattern**: Matches the transfer deadline cleanup approach (Section 1.4)
- **Safety net**: Reservation endpoint (Section 2.2) still handles expired reservations as a fallback

**Note**: The reservation endpoint query (Section 2.2) includes expiration logic as a safety net for race conditions between cleanup job and reservation attempts.

---

## Stage 3: Authorization (The "Freeze")

### Purpose
Verify the buyer has the funds and "freeze" them at the bank without taking them yet.

### Trigger
Buyer enters card details and clicks "Pay" on the Stripe-hosted checkout page.

### Process Flow

#### 3.1 Frontend Checkout Flow

1. Frontend calls `POST /api/tickets/:id/reserve` (Stage 2)
2. On success, frontend creates Stripe Payment Intent:
   ```javascript
   const paymentIntent = await stripe.paymentIntents.create({
     amount: ticket.price_at_reservation,
     currency: 'usd',
     metadata: {
       ticket_id: ticket.id,
       buyer_id: buyer.id,
       reserved_at: ticket.reserved_at
     }
   });
   ```
3. Frontend redirects buyer to Stripe Checkout
4. Buyer enters payment details
5. Stripe contacts bank and places authorization hold

#### 3.2 Stripe Webhook: `payment_intent.amount_capturable_updated`

**Event**: Stripe sends webhook when funds are frozen and ready to capture.

**Webhook Payload:**
```json
{
  "type": "payment_intent.amount_capturable_updated",
  "data": {
    "object": {
      "id": "pi_xxx",
      "amount": 15000,
      "status": "requires_capture",
      "metadata": {
        "ticket_id": "uuid",
        "buyer_id": "uuid",
        "reserved_at": "2025-01-XX T12:00:00Z"
      }
    }
  }
}
```

#### 3.3 Webhook Handler Processing

**Status**: Ticket remains `reserved` at this stage.

**Actions**:
1. Verify webhook signature (Stripe signature validation)
2. Extract `ticket_id`, `buyer_id`, `reserved_at` from metadata
3. Store payment intent record (for idempotency)
4. Trigger Stage 4 (Gatekeeper Check)

### Webhook Endpoint

```
POST /api/webhooks/stripe
```

**Security**: Must verify Stripe webhook signature using `Stripe-Signature` header.

**Idempotency Check:**

**Purpose**: Stripe webhooks can be delivered multiple times due to network issues, retries, or Stripe's delivery guarantees. Without idempotency, processing the same webhook twice could cause:
- Double-charging the buyer (capturing funds twice)
- Moving ticket status from `reserved` → `paid` multiple times
- Creating duplicate payment records
- Race conditions in the payment flow

**How It Works:**
```sql
-- Attempt to insert the payment intent record
-- The payment_intents.id column has a UNIQUE constraint (Stripe payment_intent ID)
INSERT INTO payment_intents (id, ticket_id, buyer_id, amount, status)
VALUES ($payment_intent_id, $ticket_id, $buyer_id, $amount, 'capturable')
ON CONFLICT (id) DO NOTHING
RETURNING id;
```

**Behavior:**
- **First webhook delivery**: INSERT succeeds, returns the new row ID → Continue processing (move to Stage 4)
- **Duplicate webhook delivery**: INSERT fails due to UNIQUE constraint, `ON CONFLICT DO NOTHING` prevents error → Returns empty result → Skip processing, return 200 OK to Stripe

**Why This Works:**
- Stripe payment intent IDs are globally unique
- The database UNIQUE constraint ensures we can only store each payment intent once
- `ON CONFLICT DO NOTHING` makes the operation idempotent (safe to retry)
- If the record already exists, we know we've already processed this webhook

**Alternative Approach (More Explicit):**
```sql
-- Check if we've already processed this payment intent
SELECT id FROM payment_intents WHERE id = $payment_intent_id;

-- If exists, return 200 OK (idempotent) - skip all processing
-- If not, insert and continue processing
INSERT INTO payment_intents (id, ticket_id, buyer_id, amount, status)
VALUES ($payment_intent_id, $ticket_id, $buyer_id, $amount, 'capturable')
ON CONFLICT (id) DO NOTHING
RETURNING id;
```

The `ON CONFLICT` approach is preferred because it's atomic and handles race conditions better (if two webhook deliveries arrive simultaneously).

---

## Stage 4: Gatekeeper Check (reserved → paid)

### Purpose
The final integrity check. Determines if the "freeze" becomes a "charge" or a "release."

### Trigger
Server receives `payment_intent.amount_capturable_updated` webhook from Stage 3.

### Process Flow

#### 4.1 Atomic Status Check

**Query:**
```sql
-- Application code calculates: NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
-- $expiry_time is calculated in application code using TOTAL_RESERVATION_WINDOW_MINUTES env var
UPDATE tickets
SET status = 'paid',
    updated_at = NOW()
WHERE id = $ticket_id
  AND status = 'reserved'
  AND reserved_by = $buyer_id
  AND reserved_at > $expiry_time  -- $expiry_time = NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES
RETURNING id, price_at_reservation;
```

#### 4.2 Branch A: Happy Path (Success)

**Condition**: Query returns 1 row (ticket is still reserved by this buyer within ${TOTAL_RESERVATION_WINDOW_MINUTES}-minute window)

**Actions**:
1. Update payment intent status:
   ```sql
   UPDATE payment_intents
   SET status = 'captured',
       updated_at = NOW()
   WHERE id = $payment_intent_id;
   ```

2. Call Stripe API to capture funds:
   ```rust
   stripe::PaymentIntent::capture(&payment_intent_id, None)
   ```

3. Log success event

**Result**: Ticket status is `paid`, funds are captured, buyer owns the ticket.

#### 4.3 Branch B: Late Path (Failure)

**Condition**: Query returns 0 rows (reservation expired)

**Actions**:
1. Update payment intent status:
   ```sql
   UPDATE payment_intents
   SET status = 'cancelled',
       updated_at = NOW()
   WHERE id = $payment_intent_id;
   ```

2. Call Stripe API to cancel authorization:
   ```rust
   stripe::PaymentIntent::cancel(&payment_intent_id)
   ```

3. Log cancellation event with reason

**Result**: 
- Ticket status remains unchanged (may be `verified` or `cancelled`)
- Bank hold is released for buyer
- No charge to buyer
- No cost to marketplace

### Error Handling

#### Webhook Processing Errors

**Database Error**:
- Log error
- Return `500` to Stripe (will retry)
- Do NOT capture funds

**Stripe API Error (Capture Fails)**:
- Ticket status remains `reserved`
- Log error for manual intervention
- Return `500` to Stripe (will retry)
- Implement retry mechanism with exponential backoff

**Stripe API Error (Cancel Fails)**:
- Log error for manual intervention
- Ticket status remains `reserved`
- May need manual intervention to release hold

---

## State Transition Summary

| From State | To State | Trigger | Atomic Check | Price Lock |
|------------|----------|---------|--------------|------------|
| `unverified` | `verified` | Bot accepts transfer | `status = 'unverified' AND transfer_deadline > NOW()` | N/A |
| `unverified` | *deleted* | Transfer deadline expires | `status = 'unverified' AND transfer_deadline <= NOW()` | N/A |
| `verified` | `reserved` | Buyer "Buy" click | `status = 'verified' OR (reserved AND expired)` | ✅ `price_at_reservation` |
| `reserved` | `paid` | Stripe webhook | `status = 'reserved' AND buyer matches AND within ${TOTAL_RESERVATION_WINDOW_MINUTES}min` | Uses `price_at_reservation` |

**Note**: Unverified tickets that fail verification are deleted (not transitioned to `cancelled`). The `cancelled` status is reserved for legitimate tickets cancelled after verification (e.g., seller cancels verified listing, refund scenarios).

---

## Timing Windows

| Window | Duration (Env Var) | Default | Purpose |
|--------|-------------------|---------|---------|
| **Transfer Deadline** | `${TRANSFER_DEADLINE_HOURS}` hours | 24 hours | Seller must send transfer request within this window |
| **Reservation Window** | `${RESERVATION_WINDOW_MINUTES}` minutes | 5 minutes | Buyer has time to complete checkout |
| **Grey Period** | `${GREY_PERIOD_MINUTES}` minutes | 2 minutes | Buffer for webhook processing delays |
| **Total Reservation Window** | `${TOTAL_RESERVATION_WINDOW_MINUTES}` minutes | 7 minutes | Total time reservation is valid (RESERVATION_WINDOW + GREY_PERIOD) |
| **Bot Polling Interval** | `${BOT_POLLING_INTERVAL_SECONDS}` seconds | 20 seconds | How often bot polls for incoming transfers |
| **Transfer Deadline Cleanup Interval** | `${TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS}` hours | 1 hour | How often to check for expired transfer deadlines |
| **Reservation Cleanup Interval** | `${RESERVATION_CLEANUP_INTERVAL_SECONDS}` seconds | 60 seconds | How often to check for expired reservations |

---

## Race Condition Protections

### 1. Double Reservation Prevention
- **Mechanism**: Atomic `UPDATE ... WHERE` with status check
- **Result**: Only first buyer succeeds, others get `409 Conflict`

### 2. Late Webhook Prevention
- **Mechanism**: `reserved_at > $expiry_time` check where `$expiry_time = NOW() - INTERVAL '1 minute' * TOTAL_RESERVATION_WINDOW_MINUTES`
- **Result**: Expired reservations cannot transition to `paid`

### 3. Process Conflict Prevention
- **Mechanism**: `FOR UPDATE SKIP LOCKED` in transfer matching, deadline cleanup, and reservation cleanup
- **Result**: Prevents conflicts between bot verification process, deadline cleanup job, and reservation cleanup job when processing the same ticket

### 4. Webhook Idempotency
- **Mechanism**: `payment_intents` table with unique constraint
- **Result**: Duplicate webhooks are safely ignored

---

## Error Recovery Scenarios

### Scenario 1: Reservation Expires Before Webhook
- **State**: Ticket is `reserved`, webhook arrives after ${TOTAL_RESERVATION_WINDOW_MINUTES} minutes
- **Action**: Cancel authorization, cleanup job resets ticket to `verified` status
- **Outcome**: Buyer not charged, ticket available for others

### Scenario 2: Webhook Arrives Before Authorization
- **State**: Webhook arrives but payment intent not yet `capturable`
- **Action**: Store webhook, wait for `amount_capturable_updated` event
- **Outcome**: Process continues when authorization completes

### Scenario 3: Stripe Capture Fails After Status Update
- **State**: Ticket is `paid` but Stripe capture API call fails
- **Action**: Implement retry mechanism, log for manual intervention
- **Outcome**: Ticket marked `paid` but funds not captured (requires manual fix)

---

## Monitoring & Observability

### Key Metrics to Track

1. **Transfer Success Rate**: `verified / (verified + deleted)` within transfer deadline
2. **Transfer Time**: Average time from listing to transfer acceptance
3. **Deadline Expiration Rate**: Tickets deleted due to missed transfer deadline (false listings)
4. **Reservation Success Rate**: Successful reservations / total attempts
5. **Payment Capture Rate**: `paid / reserved`
6. **Webhook Processing Time**: Time from webhook receipt to status update
7. **Expired Reservations**: Count of reservations that expired before payment

### Logging Requirements

All state transitions should log:
- Ticket ID
- From/To status
- User ID (buyer/seller/bot)
- Timestamp
- Success/Failure
- Error messages (if applicable)

---

## Security Checklist

- [ ] JWT authentication required for reservation endpoint
- [ ] Stripe webhook signature verification
- [ ] Rate limiting on reservation endpoint
- [ ] Admin API key for verification endpoint
- [ ] SQL injection prevention (use parameterized queries)
- [ ] Input validation on all endpoints
- [ ] CORS configuration
- [ ] Audit logging for all state transitions

---

## Next Steps (Post-Review)

1. Implement database migrations
2. Implement Stage 1: Verification endpoint
3. Implement Stage 2: Reservation endpoint
4. Implement Stage 3 & 4: Webhook handler
5. Add comprehensive error handling
6. Add monitoring and logging
7. Write integration tests for race conditions
8. Load testing for concurrent reservations

---

## Open Questions

1. **Seller Self-Reservation**: Should sellers be able to reserve their own tickets? (Recommendation: No)
2. **Reservation Limits**: Should there be a limit on concurrent reservations per user? (Recommendation: Yes, 3-5 max)
3. **Retry Logic**: How many times should we retry Stripe capture on failure? (Recommendation: 3 attempts with exponential backoff)
4. **Manual Intervention**: What's the process for tickets stuck in `paid` but not captured? (Recommendation: Admin dashboard with manual capture button)

---

## Environment Variables Reference

Add these to your `.env` file during implementation:

```bash
# Transfer Deadline Configuration
# Time (in hours) seller has to send transfer request after listing
TRANSFER_DEADLINE_HOURS=24

# Reservation Window Configuration
# Time (in minutes) buyer has to complete checkout
RESERVATION_WINDOW_MINUTES=5

# Grey Period Configuration
# Buffer time (in minutes) for webhook processing delays
GREY_PERIOD_MINUTES=2

# Total Reservation Window
# Total time (in minutes) reservation is valid (RESERVATION_WINDOW + GREY_PERIOD)
# This should equal RESERVATION_WINDOW_MINUTES + GREY_PERIOD_MINUTES
TOTAL_RESERVATION_WINDOW_MINUTES=7

# Bot Configuration
# How often (in seconds) bot polls for incoming transfers
BOT_POLLING_INTERVAL_SECONDS=20

# Cleanup Configuration
# How often (in hours) to check for expired transfer deadlines
TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS=1
# How often (in seconds) to check for expired reservations
RESERVATION_CLEANUP_INTERVAL_SECONDS=60
```

### Implementation Notes

- All time values are in **minutes** except `TRANSFER_DEADLINE_HOURS` and `TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS` which are in **hours** and `BOT_POLLING_INTERVAL_SECONDS` and `RESERVATION_CLEANUP_INTERVAL_SECONDS` which are in **seconds**
- `TOTAL_RESERVATION_WINDOW_MINUTES` should be calculated as `RESERVATION_WINDOW_MINUTES + GREY_PERIOD_MINUTES` in application code
- Application code should read these values at startup and use them in SQL queries via parameterized queries
- Consider validating that `TOTAL_RESERVATION_WINDOW_MINUTES >= RESERVATION_WINDOW_MINUTES + GREY_PERIOD_MINUTES` at startup

---

**End of Blueprint**

