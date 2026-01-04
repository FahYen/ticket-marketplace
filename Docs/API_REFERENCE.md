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

### POST /api/tickets/:id/verify
Verify a ticket (admin/bot endpoint). Changes status from `unverified` to `verified`.

**CLI Command:**
```bash
curl -X POST http://localhost:3000/api/tickets/<ticket-id>/verify \
  -H "Authorization: your-admin-api-key-here"
```

**Headers:**
```
Authorization: <ADMIN_API_KEY>
```

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
  "error": "Ticket must be in unverified state to be verified"
}
```

**Note:** This endpoint is used by the Selenium bot after it receives a ticket transfer and accepts it in Paciolan. Only tickets with status `unverified` can be verified.

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

