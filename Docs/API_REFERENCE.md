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
- `status` (optional): Filter by status. Valid values: `unverified`, `verified`, `reserved`, `paid`, `sold`, `cancelled`

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
- `500 Internal Server Error` - Server error

