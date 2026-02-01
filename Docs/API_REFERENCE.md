# API Reference (Linear Local Test Flow)

Base URL: `http://localhost:3000`

This guide is a single, copy/paste-friendly flow that takes you from registering users through the full ticket lifecycle: `unverified → verifying → verified → reserved → paid`, including bot claim/verify and Stripe webhook capture.

## Prerequisites
- Tools: `curl`, `jq`, `stripe` CLI.
- Services running locally on `:3000` (backend) and Postgres via `docker-compose up -d`.
- Environment values (from `.env`):
```bash
export BASE_URL=http://localhost:3000
export ADMIN_API_KEY=change-me-admin
export BOT_API_KEY=change-me-bot
export STRIPE_SECRET_KEY=sk_test_xxx
export STRIPE_WEBHOOK_SECRET=whsec_xxx
```

## 1) Health Check
```bash
curl $BASE_URL/health
```

## 2) Register + Verify + Login (Seller)
```bash
# Register seller
SELLER_CODE=$(curl -s -X POST $BASE_URL/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"seller@msu.edu","password":"password123"}' | jq -r '.verification_code')

# Verify seller email
curl -s -X POST $BASE_URL/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{"email":"seller@msu.edu","code":"'"$SELLER_CODE"'"}'

# Login seller
SELLER_LOGIN=$(curl -s -X POST $BASE_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"seller@msu.edu","password":"password123"}')
SELLER_TOKEN=$(echo "$SELLER_LOGIN" | jq -r '.token')
SELLER_ID=$(echo "$SELLER_LOGIN" | jq -r '.user.id')
```

## 3) Admin Creates a Game
```bash
GAME=$(curl -s -X POST $BASE_URL/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: $ADMIN_API_KEY" \
  -d '{
    "sport_type": "football",
    "name": "MSU vs Michigan",
    "game_time": "2026-12-01T20:00:00Z"
  }')
GAME_ID=$(echo "$GAME" | jq -r '.id')
GAME_NAME=$(echo "$GAME" | jq -r '.name')
```

## 4) Seller Creates Ticket (unverified)
```bash
TICKET=$(curl -s -X POST $BASE_URL/api/tickets \
  -H "Content-Type: application/json" \
  -H "Authorization: $SELLER_TOKEN" \
  -d '{
    "game_id": "'"$GAME_ID"'",
    "level": "STUD",
    "seat_section": "GEN",
    "seat_row": "128",
    "seat_number": "28",
    "price": 15000
  }')
TICKET_ID=$(echo "$TICKET" | jq -r '.id')
EVENT_NAME=$(echo "$TICKET" | jq -r '.event_name')
SEAT_SECTION=$(echo "$TICKET" | jq -r '.seat_section')
SEAT_ROW=$(echo "$TICKET" | jq -r '.seat_row')
SEAT_NUMBER=$(echo "$TICKET" | jq -r '.seat_number')
```

## 5) Bot Claims and Verifies (unverified → verifying → verified)
```bash
# Bot claim (unverified → verifying)
CLAIM=$(curl -s -X POST $BASE_URL/api/tickets/claim \
  -H "Content-Type: application/json" \
  -H "Authorization: $BOT_API_KEY" \
  -d '{
    "event_name": "'"$EVENT_NAME"'",
    "seat_section": "'"$SEAT_SECTION"'",
    "seat_row": "'"$SEAT_ROW"'",
    "seat_number": "'"$SEAT_NUMBER"'"
  }')
echo "$CLAIM" | jq

# Bot verify (verifying → verified)
VERIFY=$(curl -s -X PATCH $BASE_URL/api/tickets/$TICKET_ID/verify \
  -H "Authorization: $BOT_API_KEY")
echo "$VERIFY" | jq

# Optional: Bot unclaim (verifying → unverified) if you need to roll back
# curl -s -X DELETE $BASE_URL/api/tickets/$TICKET_ID/unclaim \
#   -H "Authorization: $BOT_API_KEY"
```

## 6) Register + Verify + Login (Buyer)
```bash
BUYER_CODE=$(curl -s -X POST $BASE_URL/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"buyer@msu.edu","password":"password123"}' | jq -r '.verification_code')

curl -s -X POST $BASE_URL/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{"email":"buyer@msu.edu","code":"'"$BUYER_CODE"'"}'

BUYER_LOGIN=$(curl -s -X POST $BASE_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"buyer@msu.edu","password":"password123"}')
BUYER_TOKEN=$(echo "$BUYER_LOGIN" | jq -r '.token')
BUYER_ID=$(echo "$BUYER_LOGIN" | jq -r '.user.id')
```

## 7) Buyer Reserves Ticket (verified → reserved)
```bash
RESERVE=$(curl -s -X POST $BASE_URL/api/tickets/$TICKET_ID/reserve \
  -H "Authorization: $BUYER_TOKEN")
echo "$RESERVE" | jq
PRICE_AT_RES=$(echo "$RESERVE" | jq -r '.price_at_reservation')
RESERVED_AT=$(echo "$RESERVE" | jq -r '.reserved_at')
```

## 8) Stripe: Listen, Create PI, Confirm (reserved → paid via webhook)
Run listener in a separate terminal:
```bash
stripe listen --forward-to localhost:3000/api/webhooks/stripe --api-key $STRIPE_SECRET_KEY
# Ensure STRIPE_WEBHOOK_SECRET in env matches the secret printed by stripe listen (or set it explicitly).
```

Create Payment Intent with metadata:
```bash
PI_ID=$(stripe payment_intents create \
  --amount=$PRICE_AT_RES \
  --currency=usd \
  --capture-method=manual \
  --metadata[ticket_id]=$TICKET_ID \
  --metadata[buyer_id]=$BUYER_ID \
  --metadata[reserved_at]=$RESERVED_AT \
  --api-key $STRIPE_SECRET_KEY | jq -r '.id')
echo "PI_ID=$PI_ID"
```

Confirm (triggers webhook):
```bash
stripe payment_intents confirm $PI_ID \
  --payment-method=pm_card_visa \
  --api-key $STRIPE_SECRET_KEY
```

The webhook (`/api/webhooks/stripe`) will:
- Insert the payment_intent row (idempotent)
- Gatekeep reservation window and buyer match
- Set ticket to `paid` and capture, or cancel the PI if expired

## 9) Verify Final Ticket State
```bash
curl -s $BASE_URL/api/tickets/my-listings \
  -H "Authorization: $SELLER_TOKEN" \
  | jq '.tickets[] | select(.id == "'"$TICKET_ID"'")'
```

Expected: `"status": "Paid"`.

## Reference: Key Endpoints
- Health: `GET /health`
- Auth: `POST /api/auth/register`, `POST /api/auth/verify-email`, `POST /api/auth/login`
- Games (admin): `GET /api/games`, `POST /api/games`, `DELETE /api/games/:id`
- Tickets (seller/buyer): `GET /api/tickets`, `POST /api/tickets`, `GET /api/tickets/my-listings`, `POST /api/tickets/:id/reserve`
- Bot: `POST /api/tickets/claim`, `PATCH /api/tickets/:id/verify`, `DELETE /api/tickets/:id/unclaim`
- Stripe Webhook: `POST /api/webhooks/stripe`

## Error Format
```json
{ "error": "message" }
```
Common codes: 400 (bad request), 401 (unauthorized), 404 (not found), 409 (conflict), 500 (server error).
# API Reference

Base URL: `http://localhost:3000`

---

## Health

### GET /health
Check server health (no authentication required).

**CLI Command:**
```bash
curl http://localhost:3000/health
```

**Response (200 OK):**
```
OK
```

---

## Authentication

### POST /api/auth/register
Register a new user account.

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

**Request:**
```json
{
  "email": "student@msu.edu",
  "password": "password123"
}
```

**Response (201 Created):**
```json
{
  "message": "Registration successful. Please check your email for verification code.",
  "user_id": "uuid-here"
}
```

---

### POST /api/auth/verify-email
Verify email with verification code.

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "code": "123456"
  }'
```

**Request:**
```json
{
  "email": "student@msu.edu",
  "code": "123456"
}
```

**Response (200 OK):**
```json
{
  "message": "Email verified successfully"
}
```

---

### POST /api/auth/login
Login with email and password.

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

**Request:**
```json
{
  "email": "student@msu.edu",
  "password": "password123"
}
```

**Response (200 OK):**
```json
{
  "token": "jwt-token-here",
  "user": {
    "id": "uuid-here",
    "email": "student@msu.edu",
    "email_verified": true
  }
}
```

---

## Games

### GET /api/games
List all upcoming games (public, no authentication required).

**CLI Command:**
```bash
curl http://localhost:3000/api/games
```

**Response (200 OK):**
```json
{
  "games": [
    {
      "id": "uuid-here",
      "sport_type": "Football",
      "name": "Richmond @ Spartan Football",
      "game_time": "2026-09-09T15:30:00Z",
      "cutoff_time": "2026-09-09T14:30:00Z"
    }
  ]
}
```

---

### POST /api/games
Create a new game (admin only).

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: your-admin-api-key-here" \
  -d '{
    "sport_type": "football",
    "name": "Richmond @ Spartan Football",
    "game_time": "2026-09-09T15:30:00Z"
  }'
```

**Headers:**
```
Authorization: <ADMIN_API_KEY>
```

**Request:**
```json
{
  "sport_type": "football",
  "name": "Richmond @ Spartan Football",
  "game_time": "2026-09-09T15:30:00Z"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid-here",
  "sport_type": "Football",
  "name": "Richmond @ Spartan Football",
  "game_time": "2026-09-09T15:30:00Z",
  "cutoff_time": "2026-09-09T14:30:00Z"
}
```

**Sport Types:** `football`, `basketball`, `hockey`

---

### DELETE /api/games/:id
Delete a game by ID (admin only).

**CLI Command:**
```bash
curl -X DELETE http://localhost:3000/api/games/<game-id> \
  -H "Authorization: your-admin-api-key-here"
```

**Headers:**
```
Authorization: <ADMIN_API_KEY>
```

**Response (204 No Content)**

---

## Tickets

### GET /api/tickets
List all verified tickets available for sale (public, no authentication required).

**CLI Command:**
```bash
curl http://localhost:3000/api/tickets
```

**Response (200 OK):**
```json
{
  "tickets": [
    {
      "id": "uuid-here",
      "seller_id": "uuid-here",
      "game_id": "uuid-here",
      "event_name": "Richmond @ Spartan Football",
      "event_date": "2026-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 5000,
      "status": "Verified",
      "created_at": "2026-01-03T12:00:00Z"
    }
  ]
}
```

**Note:** Only tickets with status `"Verified"` are returned. Tickets are ordered by event date (earliest first), then by creation date.

---

### POST /api/tickets
Create a new ticket listing (authenticated).

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -H "Authorization: your-jwt-token-here" \
  -d '{
    "game_id": "uuid-here",
    "level": "STUD",
    "seat_section": "GEN",
    "seat_row": "128",
    "seat_number": "28",
    "price": 5000
  }'
```

**Headers:**
```
Authorization: <JWT_TOKEN>
```

**Request:**
```json
{
  "game_id": "uuid-here",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000
}
```

**Response (201 Created):**
```json
{
  "id": "uuid-here",
  "seller_id": "uuid-here",
  "game_id": "uuid-here",
  "event_name": "Richmond @ Spartan Football",
  "event_date": "2026-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000,
  "status": "Unverified",
  "created_at": "2026-01-03T12:00:00Z"
}
```

**Note:** The backend automatically populates `event_name` and `event_date` from the games table, and sets `status` to `"Unverified"`. The ticket must be verified (by the custodian/bot) before it becomes available for sale.

---

### GET /api/tickets/my-listings
List user's own tickets (authenticated).

**CLI Command:**
```bash
# List all tickets
curl http://localhost:3000/api/tickets/my-listings \
  -H "Authorization: your-jwt-token-here"

# Filter by status (optional)
curl "http://localhost:3000/api/tickets/my-listings?status=verified" \
  -H "Authorization: your-jwt-token-here"
```

**Headers:**
```
Authorization: <JWT_TOKEN>
```

**Query Parameters:**
- `status` (optional): Filter by status. Valid values: `unverified`, `verified`, `reserved`, `paid`, `sold`, `refunding`, `cancelled`

**Response (200 OK):**
```json
{
  "tickets": [
    {
      "id": "uuid-here",
      "seller_id": "uuid-here",
      "game_id": "uuid-here",
      "event_name": "Richmond @ Spartan Football",
      "event_date": "2026-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 5000,
      "status": "Verified",
      "created_at": "2026-01-03T12:00:00Z"
    }
  ]
}
```

**Note:** Returns all tickets where `seller_id` matches the authenticated user's ID. Tickets are ordered by creation date (newest first). Use the `status` query parameter to filter by ticket status.

---

### PATCH /api/tickets/:id
Update a ticket. Supports both user operations (cancel, update price) and admin operations (verify).

**Authentication:**
- **User operations**: Requires JWT token (owner-only)
- **Admin operations**: Requires ADMIN_API_KEY

**User Operations (JWT Auth):**
```bash
# Cancel a ticket
curl -X PATCH http://localhost:3000/api/tickets/<ticket-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: your-jwt-token-here" \
  -d '{
    "status": "cancelled"
  }'

# Update price
curl -X PATCH http://localhost:3000/api/tickets/<ticket-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: your-jwt-token-here" \
  -d '{
    "price": 4500
  }'

# Cancel and update price in one request
curl -X PATCH http://localhost:3000/api/tickets/<ticket-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: your-jwt-token-here" \
  -d '{
    "status": "cancelled",
    "price": 4500
  }'
```

**Admin Operations (ADMIN_API_KEY):**
```bash
# Verify a ticket (admin/bot only)
curl -X PATCH http://localhost:3000/api/tickets/<ticket-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: your-admin-api-key-here" \
  -d '{
    "status": "verified"
  }'
```

**Headers:**
```
Authorization: <JWT_TOKEN> (for user operations)
Authorization: <ADMIN_API_KEY> (for admin operations)
```

**Request (User):**
```json
{
  "status": "cancelled",
  "price": 4500
}
```

**Request (Admin):**
```json
{
  "status": "verified"
}
```

Both `status` and `price` are optional for user operations, but at least one must be provided. For admin operations, only `status: "verified"` is allowed.

**Response (200 OK):**
```json
{
  "id": "uuid-here",
  "seller_id": "uuid-here",
  "game_id": "uuid-here",
  "event_name": "Richmond @ Spartan Football",
  "event_date": "2026-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000,
  "status": "Verified",
  "created_at": "2026-01-03T12:00:00Z"
}
```

**Response (400 Bad Request):**
```json
{
  "error": "Cannot update ticket. Only unverified or verified tickets can be updated."
}
```
or
```json
{
  "error": "Ticket must be in unverified state to be verified"
}
```

**Response (403 Forbidden):**
```json
{
  "error": "You can only update your own tickets"
}
```

**Note:**
- **User operations**: Only tickets with status `unverified` or `verified` can be updated. Only the ticket owner (seller) can update their tickets. To cancel, set `status` to `"cancelled"`. To update price, provide a `price` value >= 0.
- **Admin operations**: Only tickets with status `unverified` can be verified. The admin endpoint is used by the Selenium bot after it receives a ticket transfer and accepts it in Paciolan. Admin cannot update ticket price.

---

### POST /api/tickets/:id/reserve
Reserve a ticket for purchase (verified → reserved). This creates a temporary lock on the ticket while the buyer completes checkout.

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/tickets/<ticket-id>/reserve \
  -H "Authorization: your-jwt-token-here"
```

**Headers:**
```
Authorization: <JWT_TOKEN>
```

**Request:**
No request body required.

**Response (200 OK):**
```json
{
  "ticket_id": "uuid-here",
  "status": "Reserved",
  "price_at_reservation": 15000,
  "reserved_at": "2025-01-03T12:00:00Z"
}
```

**Response (409 Conflict):**
```json
{
  "error": "Ticket is no longer available"
}
```

**Note:**
- Requires authentication (JWT token)
- Only tickets with status `verified` can be reserved (or `reserved` tickets with expired reservations)
- The reservation locks the price at the time of reservation (`price_at_reservation`)
- Reservations expire after `${TOTAL_RESERVATION_WINDOW_MINUTES}` minutes (default: 7 minutes)
- After reservation, the frontend should create a Stripe Payment Intent and redirect to Stripe Checkout

---

## Webhooks

### POST /api/webhooks/stripe
Handle Stripe webhook events for payment processing. This endpoint processes `payment_intent.amount_capturable_updated` events to transition tickets from `reserved` to `paid` status.

**Note:** This endpoint is called by Stripe, not by clients. For testing, use Stripe CLI (see Testing Stripe Webhooks section below).

**Headers:**
```
Stripe-Signature: <stripe_signature>
```

**Request:**
Stripe webhook payload (JSON format).

**Response (200 OK):**
```json
{
  "received": true,
  "payment_intent_id": "pi_xxx"
}
```

**Response (200 OK - Duplicate):**
```json
{
  "received": true,
  "duplicate": true
}
```

**Note:**
- Webhook signature verification is required (handled automatically by Stripe CLI)
- Only processes `payment_intent.amount_capturable_updated` events
- Idempotent: duplicate webhooks are safely ignored
- Performs gatekeeper check to validate reservation is still valid
- If reservation is valid: captures payment and updates ticket status to `paid`
- If reservation expired: cancels payment intent and releases authorization hold

---

## Testing Stripe Webhooks

To test the Stripe webhook integration locally, use Stripe CLI to forward webhook events to your local server.

### Prerequisites

1. **Install Stripe CLI:**
   ```bash
   # macOS (using Homebrew)
   brew install stripe/stripe-cli/stripe
   
   # Linux/Windows: Download from https://stripe.com/docs/stripe-cli
   ```

2. **Login to Stripe CLI:**
   ```bash
   stripe login
   ```

3. **Start your local server:**
   ```bash
   # Ensure your backend is running on http://localhost:3000
   ```

### Testing with Stripe CLI

#### 1. Forward Webhooks to Local Server

In a separate terminal, start the Stripe CLI listener to forward webhooks to your local endpoint:

```bash
stripe listen --forward-to localhost:3000/api/webhooks/stripe
```

This will:
- Display a webhook signing secret (e.g., `whsec_xxx`)
- Forward all webhook events from your Stripe account to your local endpoint
- Sign webhooks with the provided secret (configure this in your `.env` as `STRIPE_WEBHOOK_SECRET`)

**Important:** Use the webhook signing secret shown by `stripe listen` in your `.env` file for local testing.

#### 2. Trigger Payment Intent Events

In another terminal, trigger the `payment_intent.amount_capturable_updated` event:

```bash
# Create a test payment intent with metadata matching your ticket structure
stripe payment_intents create \
  --amount=15000 \
  --currency=usd \
  --metadata[ticket_id]=<ticket-uuid> \
  --metadata[buyer_id]=<buyer-uuid> \
  --metadata[reserved_at]=2025-01-03T12:00:00Z \
  --capture-method=manual
```

Then confirm it with a test card to trigger the webhook:

```bash
# Confirm the payment intent (replace pi_xxx with the ID from previous command)
stripe payment_intents confirm <payment-intent-id> \
  --payment-method=pm_card_visa
```

#### 3. Alternative: Use Stripe CLI Trigger (Simplified Testing)

For simpler testing, you can directly trigger the webhook event:

```bash
# Trigger the payment_intent.amount_capturable_updated event
stripe trigger payment_intent.amount_capturable_updated
```

**Note:** You may need to customize the event payload to include your metadata (ticket_id, buyer_id, reserved_at). Check Stripe CLI documentation for advanced triggering options.

### Testing Complete Flow

See the "Testing Ticket State Transitions" section below for a complete end-to-end testing flow.

---

## Testing Ticket State Transitions

This section provides a complete testing workflow for the ticket state transition flow: `unverified → verified → reserved → paid`.

### Prerequisites

1. Backend server running on `http://localhost:3000`
2. Database with at least one game created
3. Two user accounts (seller and buyer)
4. Stripe CLI installed and configured (for webhook testing)
5. Environment variables configured (especially `ADMIN_API_KEY` and `STRIPE_WEBHOOK_SECRET`)

### Step 1: Create Ticket (unverified)

```bash
# Register seller account
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "seller@msu.edu",
    "password": "password123"
  }'

# Verify email (use code from email)
curl -X POST http://localhost:3000/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "seller@msu.edu",
    "code": "123456"
  }'

# Login to get JWT token
SELLER_TOKEN=$(curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "seller@msu.edu",
    "password": "password123"
  }' | jq -r '.token')

# Create ticket (status: unverified)
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -H "Authorization: $SELLER_TOKEN" \
  -d '{
    "game_id": "<game-uuid>",
    "level": "STUD",
    "seat_section": "GEN",
    "seat_row": "128",
    "seat_number": "28",
    "price": 15000
  }'
```

**Expected Response:** Ticket with `status: "Unverified"`

### Step 2: Verify Ticket (unverified → verified)

```bash
# Verify ticket using admin API key
curl -X PATCH http://localhost:3000/api/tickets/<ticket-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: <ADMIN_API_KEY>" \
  -d '{
    "status": "verified"
  }'
```

**Expected Response:** Ticket with `status: "Verified"`

### Step 3: Reserve Ticket (verified → reserved)

```bash
# Register buyer account
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "buyer@msu.edu",
    "password": "password123"
  }'

# Verify email and login
BUYER_TOKEN=$(curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "buyer@msu.edu",
    "password": "password123"
  }' | jq -r '.token')

# Reserve ticket
curl -X POST http://localhost:3000/api/tickets/<ticket-id>/reserve \
  -H "Authorization: $BUYER_TOKEN"
```

**Expected Response:**
```json
{
  "ticket_id": "uuid-here",
  "status": "Reserved",
  "price_at_reservation": 15000,
  "reserved_at": "2025-01-03T12:00:00Z"
}
```

### Step 4: Create Stripe Payment Intent

After reserving the ticket, create a Stripe Payment Intent (this would normally be done by the frontend):

```bash
# Using Stripe CLI
stripe payment_intents create \
  --amount=15000 \
  --currency=usd \
  --metadata[ticket_id]=<ticket-uuid> \
  --metadata[buyer_id]=<buyer-uuid> \
  --metadata[reserved_at]=2025-01-03T12:00:00Z \
  --capture-method=manual
```

**Note:** In production, the frontend creates the Payment Intent using Stripe.js after receiving the reservation response.

### Step 5: Start Stripe CLI Listener

In a separate terminal, start the Stripe CLI listener:

```bash
stripe listen --forward-to localhost:3000/api/webhooks/stripe
```

**Important:** Copy the webhook signing secret (e.g., `whsec_xxx`) and ensure it matches your `STRIPE_WEBHOOK_SECRET` environment variable.

### Step 6: Confirm Payment Intent (reserved → paid)

```bash
# Confirm the payment intent with a test card
stripe payment_intents confirm <payment-intent-id> \
  --payment-method=pm_card_visa
```

This will trigger the `payment_intent.amount_capturable_updated` webhook event, which your backend will receive and process.

**Expected Result:**
- Webhook is received by backend
- Ticket status changes from `reserved` to `paid`
- Payment is captured from buyer
- Payment intent status is updated to `captured`

### Verify Final State

```bash
# Check ticket status
curl http://localhost:3000/api/tickets/my-listings \
  -H "Authorization: $SELLER_TOKEN" | jq '.tickets[] | select(.id == "<ticket-id>")'
```

**Expected Response:** Ticket with `status: "Paid"`

---

## Error Responses

All error responses follow this format:
```json
{
  "error": "Error message description"
}
```

**Common Status Codes:**
- `200 OK` - Successful request
- `201 Created` - Resource created
- `204 No Content` - Successful deletion
- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Missing or invalid authentication
- `404 Not Found` - Resource doesn't exist
- `409 Conflict` - Resource conflict (e.g., ticket no longer available)
- `500 Internal Server Error` - Server error

