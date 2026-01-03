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
    "verification_code": "123456"
  }'
```

**Request:**
```json
{
  "email": "student@msu.edu",
  "verification_code": "123456"
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

