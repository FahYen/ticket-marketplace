# Frontend API Reference

Base URL: `http://localhost:3000`

## Authentication

All authenticated endpoints require the JWT token in the `Authorization` header:
```
Authorization: <jwt_token>
```

---

## Auth Endpoints

### POST /api/auth/register

Register a new user account. Email must be an `@msu.edu` address.

**Request:**
```json
{
  "email": "student@msu.edu",
  "password": "password123"
}
```

**Response (201):**
```json
{
  "message": "Registration successful. Please check your email for verification code.",
  "verification_code": "123456"
}
```

> Note: `verification_code` is only returned in development mode. In production, it will be sent via email.

**Errors:**
- `400` - Invalid email (not @msu.edu) or weak password
- `409` - Email already exists

---

### POST /api/auth/verify-email

Verify email address with the 6-digit code.

**Request:**
```json
{
  "email": "student@msu.edu",
  "code": "123456"
}
```

**Response (200):**
```json
{
  "message": "Email verified successfully. Your account is now active.",
  "user_id": "uuid"
}
```

**Errors:**
- `400` - Invalid verification code

---

### POST /api/auth/login

Login and receive JWT token.

**Request:**
```json
{
  "email": "student@msu.edu",
  "password": "password123"
}
```

**Response (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "uuid",
    "email": "student@msu.edu",
    "email_verified": true
  }
}
```

**Errors:**
- `401` - Invalid credentials
- `403` - Email not verified

---

## Games Endpoints

### GET /api/games

List all upcoming games available for ticket trading. Returns only games where trading is still open (before cutoff time).

**Auth:** None required

**Response (200):**
```json
{
  "games": [
    {
      "id": "uuid",
      "sport_type": "Football",
      "name": "MSU vs Michigan",
      "game_time": "2026-09-09T15:30:00Z",
      "cutoff_time": "2026-09-09T14:30:00Z"
    }
  ]
}
```

**Sport Types:** `Football`, `Basketball`, `Hockey`

---

## Tickets Endpoints

### GET /api/tickets

List all verified tickets available for purchase.

**Auth:** None required

**Response (200):**
```json
{
  "tickets": [
    {
      "id": "uuid",
      "seller_id": "uuid",
      "game_id": "uuid",
      "event_name": "MSU vs Michigan",
      "event_date": "2026-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 15000,
      "status": "Verified",
      "transfer_deadline": "2026-09-08T12:00:00Z",
      "created_at": "2026-09-07T12:00:00Z"
    }
  ]
}
```

> Note: `price` is in cents (15000 = $150.00)

---

### POST /api/tickets

Create a new ticket listing. Ticket starts in `Unverified` status until bot verifies it.

**Auth:** Required (JWT)

**Request:**
```json
{
  "game_id": "uuid",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 15000
}
```

**Response (201):**
```json
{
  "id": "uuid",
  "seller_id": "uuid",
  "game_id": "uuid",
  "event_name": "MSU vs Michigan",
  "event_date": "2026-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 15000,
  "status": "Unverified",
  "transfer_deadline": "2026-09-08T12:00:00Z",
  "created_at": "2026-09-07T12:00:00Z"
}
```

**Errors:**
- `400` - Invalid input (negative price, empty seat details)
- `401` - Not authenticated
- `404` - Game not found
- `409` - Duplicate seat listing for this game

---

### GET /api/tickets/my-listings

Get current user's ticket listings.

**Auth:** Required (JWT)

**Query Parameters:**
- `status` (optional): Filter by status. Values: `unverified`, `verifying`, `verified`, `reserved`, `paid`, `sold`, `cancelled`

**Examples:**
```
GET /api/tickets/my-listings
GET /api/tickets/my-listings?status=verified
```

**Response (200):**
```json
{
  "tickets": [
    {
      "id": "uuid",
      "seller_id": "uuid",
      "game_id": "uuid",
      "event_name": "MSU vs Michigan",
      "event_date": "2026-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 15000,
      "status": "Verified",
      "transfer_deadline": "2026-09-08T12:00:00Z",
      "price_at_reservation": null,
      "created_at": "2026-09-07T12:00:00Z"
    }
  ]
}
```

---

### POST /api/tickets/:id/reserve

Reserve a ticket for purchase. Creates a temporary lock while buyer completes checkout.

**Auth:** Required (JWT)

**Rate Limited:** Yes (10 requests per 60 seconds)

**Response (200):**
```json
{
  "ticket_id": "uuid",
  "status": "Reserved",
  "price_at_reservation": 15000,
  "reserved_at": "2026-09-07T12:00:00Z"
}
```

**Errors:**
- `401` - Not authenticated
- `409` - Ticket not available (already reserved, not verified, or concurrent reservation limit reached)
- `429` - Rate limit exceeded

**Important Notes:**
- Reservation expires after 7 minutes (configurable)
- Maximum 3 concurrent reservations per user
- Price is locked at `price_at_reservation` for payment

---

## Ticket Status Flow

```
Unverified → Verifying → Verified → Reserved → Paid → Sold
```

| Status | Description | Visible in GET /api/tickets |
|--------|-------------|----------------------------|
| `Unverified` | Listed, awaiting bot verification | No |
| `Verifying` | Bot is processing | No |
| `Verified` | Available for purchase | Yes |
| `Reserved` | Locked for buyer checkout | No |
| `Paid` | Payment captured | No |
| `Sold` | Transferred to buyer | No |
| `Cancelled` | Cancelled by seller | No |

---

## Stripe Integration

After reserving a ticket, create a Stripe Payment Intent on the frontend:

```javascript
const paymentIntent = await stripe.paymentIntents.create({
  amount: reservation.price_at_reservation,
  currency: 'usd',
  capture_method: 'manual',
  metadata: {
    ticket_id: reservation.ticket_id,
    buyer_id: currentUser.id,
    reserved_at: reservation.reserved_at
  }
});
```

The backend webhook handler will:
1. Verify reservation is still valid (within 7-minute window)
2. Capture the payment if valid
3. Cancel the authorization if reservation expired

---

## Error Response Format

All errors return JSON with an `error` field:

```json
{
  "error": "Error message description"
}
```

**Common Status Codes:**
| Code | Meaning |
|------|---------|
| `200` | Success |
| `201` | Created |
| `400` | Bad request (invalid input) |
| `401` | Unauthorized (missing/invalid token) |
| `403` | Forbidden (email not verified) |
| `404` | Not found |
| `409` | Conflict (resource unavailable) |
| `429` | Rate limit exceeded |
| `500` | Server error |

---

## TypeScript Types

```typescript
interface User {
  id: string;
  email: string;
  email_verified: boolean;
}

interface Game {
  id: string;
  sport_type: 'Football' | 'Basketball' | 'Hockey';
  name: string;
  game_time: string; // ISO 8601
  cutoff_time: string; // ISO 8601
}

interface Ticket {
  id: string;
  seller_id: string;
  game_id: string;
  event_name: string;
  event_date: string; // ISO 8601
  level: string;
  seat_section: string;
  seat_row: string;
  seat_number: string;
  price: number; // cents
  status: 'Unverified' | 'Verifying' | 'Verified' | 'Reserved' | 'Paid' | 'Sold' | 'Cancelled';
  transfer_deadline: string; // ISO 8601
  price_at_reservation?: number; // cents, only when reserved
  created_at: string; // ISO 8601
}

interface ReservationResponse {
  ticket_id: string;
  status: 'Reserved';
  price_at_reservation: number; // cents
  reserved_at: string; // ISO 8601
}

interface LoginResponse {
  token: string;
  user: User;
}
```

---

## Example: Complete Purchase Flow

```typescript
// 1. Login
const { token, user } = await api.post('/api/auth/login', {
  email: 'buyer@msu.edu',
  password: 'password123'
});

// 2. Browse available tickets
const { tickets } = await api.get('/api/tickets');

// 3. Reserve a ticket
const reservation = await api.post(`/api/tickets/${ticketId}/reserve`, null, {
  headers: { Authorization: token }
});

// 4. Create Stripe Payment Intent (use price_at_reservation)
const paymentIntent = await stripe.paymentIntents.create({
  amount: reservation.price_at_reservation,
  currency: 'usd',
  capture_method: 'manual',
  metadata: {
    ticket_id: reservation.ticket_id,
    buyer_id: user.id,
    reserved_at: reservation.reserved_at
  }
});

// 5. Confirm payment with Stripe Elements
// (Stripe webhook handles capture automatically)
```

---

## Example: Seller Flow

```typescript
// 1. Login as seller
const { token } = await api.post('/api/auth/login', { ... });

// 2. Get available games
const { games } = await api.get('/api/games');

// 3. Create ticket listing
const ticket = await api.post('/api/tickets', {
  game_id: games[0].id,
  level: 'STUD',
  seat_section: 'GEN',
  seat_row: '128',
  seat_number: '28',
  price: 15000 // $150.00
}, {
  headers: { Authorization: token }
});

// 4. Seller transfers ticket to custodial Paciolan account
// (Manual step - seller does this in Paciolan)

// 5. Bot verifies and ticket becomes available
// (Automatic - no frontend action needed)

// 6. Check listing status
const { tickets } = await api.get('/api/tickets/my-listings', {
  headers: { Authorization: token }
});
```
