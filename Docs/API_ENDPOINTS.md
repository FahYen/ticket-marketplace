# API Endpoints Specification

## Phase 2: Core API Endpoints

### Authentication Endpoints (`/api/auth`)

#### 1. User Registration
```
POST /api/auth/register
Content-Type: application/json

Request Body:
{
  "email": "user@example.com",
  "password": "securepassword123"
}

Response (201 Created):
{
  "message": "Registration successful. Please check your email for verification.",
  "user_id": "uuid-here"
}

Response (400 Bad Request):
{
  "error": "Email already exists" | "Invalid email format" | "Password too weak"
}
```

#### 2. User Login
```
POST /api/auth/login
Content-Type: application/json

Request Body:
{
  "email": "user@example.com",
  "password": "securepassword123"
}

Response (200 OK):
{
  "token": "jwt-token-here",
  "user": {
    "id": "uuid-here",
    "email": "user@example.com",
    "email_verified": true
  }
}

Response (401 Unauthorized):
{
  "error": "Invalid email or password"
}
```

#### 3. Verify Email
```
POST /api/auth/verify-email
Content-Type: application/json

Request Body:
{
  "token": "verification-token-uuid"
}

Response (200 OK):
{
  "message": "Email verified successfully"
}

Response (400 Bad Request):
{
  "error": "Invalid or expired verification token"
}
```

#### 4. Get Current User (Optional for Phase 2)
```
GET /api/auth/me
Authorization: Bearer <jwt-token>

Response (200 OK):
{
  "id": "uuid-here",
  "email": "user@example.com",
  "email_verified": true,
  "created_at": "2023-09-09T15:30:00Z"
}
```

---

### Ticket Endpoints (`/api/tickets`)

#### 5. List Available Tickets (Public)
```
GET /api/tickets
Query Parameters:
  - status (optional): listed | sold | cancelled (default: listed)
  - event_date_from (optional): ISO 8601 date
  - event_date_to (optional): ISO 8601 date
  - limit (optional): number (default: 50, max: 100)
  - offset (optional): number (default: 0)

Response (200 OK):
{
  "tickets": [
    {
      "id": "uuid-here",
      "seller_id": "uuid-here",
      "event_name": "Richmond @ Spartan Football",
      "event_date": "2023-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 5000,
      "status": "listed",
      "created_at": "2023-09-01T10:00:00Z"
    }
  ],
  "total": 42,
  "limit": 50,
  "offset": 0
}
```

#### 6. Get Ticket Details (Public)
```
GET /api/tickets/:id

Response (200 OK):
{
  "id": "uuid-here",
  "seller_id": "uuid-here",
  "event_name": "Richmond @ Spartan Football",
  "event_date": "2023-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000,
  "status": "listed",
  "created_at": "2023-09-01T10:00:00Z",
  "updated_at": "2023-09-01T10:00:00Z"
}

Response (404 Not Found):
{
  "error": "Ticket not found"
}
```

#### 7. Create Ticket Listing (Authenticated)
```
POST /api/tickets
Authorization: Bearer <jwt-token>
Content-Type: application/json

Request Body:
{
  "event_name": "Richmond @ Spartan Football",
  "event_date": "2023-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000
}

Response (201 Created):
{
  "id": "uuid-here",
  "seller_id": "uuid-here",
  "event_name": "Richmond @ Spartan Football",
  "event_date": "2023-09-09T15:30:00Z",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000,
  "status": "listed",
  "created_at": "2023-09-01T10:00:00Z"
}

Response (400 Bad Request):
{
  "error": "Invalid input" | "Price must be >= 0" | "Event date must be in the future"
}
```

#### 8. Get User's Own Tickets (Authenticated)
```
GET /api/tickets/my-listings
Authorization: Bearer <jwt-token>
Query Parameters:
  - status (optional): listed | pending_sale | sold | cancelled
  - limit (optional): number
  - offset (optional): number

Response (200 OK):
{
  "tickets": [
    {
      "id": "uuid-here",
      "event_name": "Richmond @ Spartan Football",
      "event_date": "2023-09-09T15:30:00Z",
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 5000,
      "status": "listed",
      "created_at": "2023-09-01T10:00:00Z"
    }
  ],
  "total": 5
}
```

#### 9. Update Ticket (Authenticated, Owner Only)
```
PATCH /api/tickets/:id
Authorization: Bearer <jwt-token>
Content-Type: application/json

Request Body (all fields optional):
{
  "price": 4500,
  "status": "cancelled"
}

Response (200 OK):
{
  "id": "uuid-here",
  "price": 4500,
  "status": "cancelled",
  "updated_at": "2023-09-02T10:00:00Z"
}

Response (403 Forbidden):
{
  "error": "You can only update your own tickets"
}

Response (404 Not Found):
{
  "error": "Ticket not found"
}
```

#### 10. Delete Ticket (Authenticated, Owner Only)
```
DELETE /api/tickets/:id
Authorization: Bearer <jwt-token>

Response (204 No Content)

Response (403 Forbidden):
{
  "error": "You can only delete your own tickets"
}

Response (400 Bad Request):
{
  "error": "Cannot delete ticket with pending transactions"
}

Response (404 Not Found):
{
  "error": "Ticket not found"
}
```

---

## Phase 3: Transaction Endpoints (Preview)

These will be implemented in later phases but are listed here for reference:

### Transaction Endpoints (`/api/transactions`)

#### 11. Create Transaction (Purchase Ticket)
```
POST /api/transactions
Authorization: Bearer <jwt-token>
Content-Type: application/json

Request Body:
{
  "ticket_id": "uuid-here"
}

Response (201 Created):
{
  "id": "uuid-here",
  "ticket_id": "uuid-here",
  "buyer_id": "uuid-here",
  "seller_id": "uuid-here",
  "amount": 5000,
  "status": "pending_payment",
  "payment_intent_id": "stripe-payment-intent-id",
  "created_at": "2023-09-09T10:00:00Z"
}
```

#### 12. Get Transaction Details
```
GET /api/transactions/:id
Authorization: Bearer <jwt-token>

Response (200 OK):
{
  "id": "uuid-here",
  "ticket": { ... },
  "buyer_id": "uuid-here",
  "seller_id": "uuid-here",
  "amount": 5000,
  "status": "payment_received",
  "created_at": "2023-09-09T10:00:00Z"
}
```

#### 13. List User's Transactions
```
GET /api/transactions
Authorization: Bearer <jwt-token>
Query Parameters:
  - role (optional): buyer | seller
  - status (optional): pending_payment | payment_received | ...

Response (200 OK):
{
  "transactions": [ ... ],
  "total": 10
}
```

---

## Common Response Patterns

### Error Responses

All error responses follow this format:
```json
{
  "error": "Error message description"
}
```

### HTTP Status Codes

- `200 OK` - Successful GET/PATCH request
- `201 Created` - Successful POST request (resource created)
- `204 No Content` - Successful DELETE request
- `400 Bad Request` - Invalid input/request
- `401 Unauthorized` - Missing or invalid authentication
- `403 Forbidden` - Authenticated but not authorized
- `404 Not Found` - Resource doesn't exist
- `500 Internal Server Error` - Server error

### Authentication

Most endpoints require JWT authentication via the `Authorization` header:
```
Authorization: Bearer <jwt-token>
```

Public endpoints (no auth required):
- `POST /api/auth/register`
- `POST /api/auth/login`
- `POST /api/auth/verify-email`
- `GET /api/tickets` (list)
- `GET /api/tickets/:id` (get details)

---

## Implementation Priority for Phase 2

### Must Have (MVP):
1. ✅ POST /api/auth/register
2. ✅ POST /api/auth/login
3. ✅ POST /api/auth/verify-email
4. ✅ GET /api/tickets (list available)
5. ✅ GET /api/tickets/:id (get details)
6. ✅ POST /api/tickets (create listing)

### Nice to Have (Phase 2+):
7. GET /api/tickets/my-listings
8. PATCH /api/tickets/:id
9. DELETE /api/tickets/:id
10. GET /api/auth/me

### Phase 3+:
- Transaction endpoints
- Ticket transfer endpoints
- Payment webhook endpoints

