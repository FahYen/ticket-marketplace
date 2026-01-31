# Ticket State Transition Blueprint
## unverified → verifying → verified → reserved → paid

## Overview

State transition flow for tickets from listing through payment. Design emphasizes atomic operations, race condition prevention, and financial integrity.

### API Endpoints

| Transition | Endpoint | HTTP Verb | Auth |
|------------|----------|-----------|------|
| unverified → verifying | `/api/tickets/claim` | `POST` | `BOT_API_KEY` |
| verifying → verified | `/api/tickets/:id/verify` | `PATCH` | `BOT_API_KEY` |
| verifying → unverified | `/api/tickets/:id/claim` | `DELETE` | `BOT_API_KEY` |
| verified → reserved | `/api/tickets/:id/reserve` | `POST` | JWT |
| reserved → paid | `/api/webhooks/stripe` | `POST` | Stripe signature |

### Environment Variables

```bash
BOT_API_KEY=<secure-random-string>           # Bot authentication
TRANSFER_DEADLINE_HOURS=24                    # Hours seller has to transfer ticket
VERIFYING_TIMEOUT_MINUTES=10                  # Max time in 'verifying' before reset
RESERVATION_WINDOW_MINUTES=5                  # Buyer checkout time
GREY_PERIOD_MINUTES=2                         # Webhook processing buffer
TOTAL_RESERVATION_WINDOW_MINUTES=7            # RESERVATION + GREY_PERIOD
BOT_POLLING_INTERVAL_SECONDS=20               # Bot poll frequency
TRANSFER_DEADLINE_CLEANUP_INTERVAL_HOURS=1    # Expired deadline check frequency
VERIFYING_CLEANUP_INTERVAL_SECONDS=60         # Stuck verifying check frequency
RESERVATION_CLEANUP_INTERVAL_MINUTES=60       # Expired reservation check frequency
```

---

## Database Schema

### Ticket Uniqueness

Enforce unique constraint: `UNIQUE (game_id, level, seat_section, seat_row, seat_number)`

**False listings** (unverified tickets past deadline) are **deleted**, not cancelled. This keeps the database clean and allows re-listing.

### Required Migrations

```sql
-- 1. Transfer deadline tracking
ALTER TABLE tickets ADD COLUMN transfer_deadline TIMESTAMPTZ NOT NULL;
CREATE INDEX idx_tickets_unverified_deadline ON tickets(transfer_deadline) WHERE status = 'unverified';

-- 2. Price locking
ALTER TABLE tickets ADD COLUMN price_at_reservation INTEGER;

-- 3. Payment intent tracking (for idempotency)
CREATE TABLE payment_intents (
    id VARCHAR(255) PRIMARY KEY,  -- Stripe payment_intent ID
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE RESTRICT,
    buyer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    amount INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL,  -- 'created', 'capturable', 'captured', 'cancelled'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_payment_intents_ticket_id ON payment_intents(ticket_id);
```

---

## Stage 1: Verification (unverified → verifying → verified)

Seller must transfer ticket to custodian Paciolan account within `TRANSFER_DEADLINE_HOURS`. Bot polls for transfers, claims via API, accepts in Paciolan, then confirms via API.

### 1.1 Ticket Creation

```sql
INSERT INTO tickets (
    seller_id, game_id, event_name, event_date,
    level, seat_section, seat_row, seat_number, price, status, transfer_deadline
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'unverified', 
        NOW() + INTERVAL '1 hour' * $TRANSFER_DEADLINE_HOURS)
RETURNING id, transfer_deadline;
```

### 1.2 Bot Claim API (unverified → verifying)

Bot detects incoming transfer in Paciolan, calls backend to claim:

```
POST /api/tickets/claim
Authorization: <BOT_API_KEY>

{"event_name": "...", "seat_section": "...", "seat_row": "...", "seat_number": "..."}
```

**Backend SQL:**
```sql
UPDATE tickets
SET status = 'verifying', updated_at = NOW()
WHERE id = (
    SELECT id FROM tickets
    WHERE status = 'unverified'
      AND event_name = $event_name
      AND seat_section = $seat_section
      AND seat_row = $seat_row
      AND seat_number = $seat_number
      AND transfer_deadline > NOW()
    ORDER BY created_at ASC
    LIMIT 1
    FOR UPDATE SKIP LOCKED
)
RETURNING id, seller_id, event_name, seat_section, seat_row, seat_number;
```

**Responses:**
- `200 OK` → Bot accepts transfer in Paciolan, then calls verify API
- `404 Not Found` → Bot rejects transfer in Paciolan (expired or no match)

### 1.3 Bot Verify API (verifying → verified)

After accepting transfer in Paciolan:

```
PATCH /api/tickets/:id/verify
Authorization: <BOT_API_KEY>
```

**Backend SQL:**
```sql
UPDATE tickets
SET status = 'verified', updated_at = NOW()
WHERE id = $ticket_id AND status = 'verifying'
RETURNING id;
```

**Responses:**
- `200 OK` → Verification complete
- `409 Conflict` → Ticket reset due to timeout

### 1.4 Bot Rollback API (verifying → unverified)

If Paciolan accept fails:

```
DELETE /api/tickets/:id/claim
Authorization: <BOT_API_KEY>
```

**Backend SQL:**
```sql
UPDATE tickets
SET status = 'unverified', updated_at = NOW()
WHERE id = $ticket_id AND status = 'verifying';
```

### 1.5 Cleanup: Expired Deadlines

Delete `unverified` tickets past deadline (NOT `verifying`):

```sql
DELETE FROM tickets
WHERE id IN (
    SELECT id FROM tickets
    WHERE status = 'unverified' AND transfer_deadline <= NOW()
    FOR UPDATE SKIP LOCKED
);
```

### 1.6 Cleanup: Stuck Verifying

Reset `verifying` tickets stuck longer than `VERIFYING_TIMEOUT_MINUTES`:

```sql
UPDATE tickets
SET status = 'unverified', updated_at = NOW()
WHERE status = 'verifying'
  AND updated_at < NOW() - INTERVAL '1 minute' * $VERIFYING_TIMEOUT_MINUTES;
```

### 1.7 Edge Cases

Bot rejects transfer when claim API returns 404:
- No matching `unverified` ticket
- Ticket already `verifying` (another bot claimed it)
- Ticket already `verified`
- Ticket deadline expired

If Paciolan accept fails after claim succeeds, bot calls rollback API.

---

## Stage 2: Reservation (verified → reserved)

Buyer clicks "Buy" → temporary lock prevents double-buying during checkout.

### 2.1 Atomic Reservation

```sql
UPDATE tickets
SET status = 'reserved',
    reserved_at = NOW(),
    reserved_by = $buyer_id,
    price_at_reservation = price,
    updated_at = NOW()
WHERE id = $ticket_id
  AND (
    status = 'verified'
    OR (status = 'reserved' AND reserved_at < NOW() - INTERVAL '1 minute' * $TOTAL_RESERVATION_WINDOW_MINUTES)
  )
RETURNING id, price_at_reservation, reserved_at;
```

**API:**
```
POST /api/tickets/:id/reserve
Authorization: <JWT>
```

**Responses:**
- `200 OK` with `{ticket_id, status, price_at_reservation, reserved_at}`
- `409 Conflict` → Ticket unavailable

### 2.2 Cleanup: Expired Reservations

```sql
UPDATE tickets
SET status = 'verified',
    reserved_at = NULL,
    reserved_by = NULL,
    price_at_reservation = NULL,
    updated_at = NOW()
WHERE status = 'reserved'
  AND reserved_at < NOW() - INTERVAL '1 minute' * $TOTAL_RESERVATION_WINDOW_MINUTES;
```

---

## Stage 3: Authorization (Stripe Freeze)

Frontend creates Stripe Payment Intent with `capture_method: 'manual'` using `price_at_reservation`. Stripe freezes funds and sends webhook.

### 3.1 Frontend Payment Intent

```javascript
const paymentIntent = await stripe.paymentIntents.create({
  amount: ticket.price_at_reservation,
  currency: 'usd',
  capture_method: 'manual',
  metadata: { ticket_id: ticket.id, buyer_id: buyer.id, reserved_at: ticket.reserved_at }
});
```

### 3.2 Webhook: `payment_intent.amount_capturable_updated`

```
POST /api/webhooks/stripe
```

**Idempotency via payment_intents table:**
```sql
INSERT INTO payment_intents (id, ticket_id, buyer_id, amount, status)
VALUES ($payment_intent_id, $ticket_id, $buyer_id, $amount, 'capturable')
ON CONFLICT (id) DO NOTHING
RETURNING id;
```

- Row returned → First delivery, proceed to Stage 4
- No row returned → Duplicate, return 200 OK

---

## Stage 4: Gatekeeper (reserved → paid)

Final check: capture funds only if reservation still valid.

### 4.1 Atomic Capture Check

```sql
UPDATE tickets
SET status = 'paid', updated_at = NOW()
WHERE id = $ticket_id
  AND status = 'reserved'
  AND reserved_by = $buyer_id
  AND reserved_at > NOW() - INTERVAL '1 minute' * $TOTAL_RESERVATION_WINDOW_MINUTES
RETURNING id, price_at_reservation;
```

### 4.2 Happy Path (1 row returned)

```sql
UPDATE payment_intents SET status = 'captured', updated_at = NOW() WHERE id = $payment_intent_id;
```
```rust
stripe::PaymentIntent::capture(&payment_intent_id, None)
```

### 4.3 Late Path (0 rows returned)

```sql
UPDATE payment_intents SET status = 'cancelled', updated_at = NOW() WHERE id = $payment_intent_id;
```
```rust
stripe::PaymentIntent::cancel(&payment_intent_id)
```

Buyer not charged, ticket returns to `verified` via cleanup.

---

## State Transition Summary

| From | To | Trigger | Atomic Check |
|------|-----|---------|--------------|
| `unverified` | `verifying` | Bot claim API | `status='unverified' AND deadline>NOW()` |
| `verifying` | `verified` | Bot verify API | `status='verifying'` |
| `verifying` | `unverified` | Bot rollback / timeout | `status='verifying'` |
| `unverified` | *deleted* | Deadline expires | `status='unverified' AND deadline<=NOW()` |
| `verified` | `reserved` | Buyer reserve | `status='verified' OR (reserved AND expired)` |
| `reserved` | `paid` | Stripe webhook | `status='reserved' AND buyer AND within window` |

---

## Race Condition Protections

| Protection | Mechanism |
|------------|-----------|
| Verification claim | Atomic `UPDATE...WHERE` with `FOR UPDATE SKIP LOCKED` |
| Verification vs cleanup | `verifying` status protects during Paciolan operation |
| Double reservation | Atomic `UPDATE...WHERE` with status check |
| Late webhook | `reserved_at > expiry_time` check |
| Process conflicts | `FOR UPDATE SKIP LOCKED` in all cleanup queries |
| Webhook idempotency | `payment_intents` table with unique constraint |

---

## Error Recovery

| Scenario | Action | Outcome |
|----------|--------|---------|
| Reservation expires before webhook | Cancel authorization, cleanup resets to `verified` | Buyer not charged |
| Stripe capture fails after status update | Retry with backoff, log for manual intervention | May need manual fix |
| Bot crashes mid-verification | Stuck cleanup resets to `unverified` | Transfer can retry |

---

## Security Checklist

- [ ] JWT auth for reservation endpoint
- [ ] Bot API key auth for claim/verify endpoints
- [ ] Stripe webhook signature verification
- [ ] Rate limiting on reservation endpoint
- [ ] Parameterized queries (SQL injection prevention)
- [ ] Input validation on all endpoints

---

## Monitoring Metrics

- Transfer success rate: `verified / (verified + deleted)`
- Claim success rate: successful claims / total attempts
- Verifying timeout rate: resets from `verifying` to `unverified`
- Reservation success rate: successful / total attempts
- Payment capture rate: `paid / reserved`
- Expired reservations count

---

## Open Questions

1. **Seller self-reservation**: Prevent? (Recommend: Yes)
2. **Reservation limits**: Max per user? (Recommend: 3-5)
3. **Capture retry**: How many attempts? (Recommend: 3 with exponential backoff)

---

**End of Blueprint**
