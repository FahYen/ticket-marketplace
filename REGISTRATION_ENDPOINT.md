# User Registration Endpoint Implementation

## Overview

The user registration endpoint has been implemented with MSU email validation and verification code system.

## Registration Flow

1. User submits email + password
2. System validates email ends with msu.edu
3. System validates password (minimum 8 characters)
4. System creates user account with `email_verified = false`
5. System generates 6-digit verification code
6. System stores verification code (TODO: send via email)
7. User submits email + verification code
8. System verifies code and activates account (`email_verified = true`)

## Password Requirements

- **Minimum 8 characters** (simple requirement, no complexity rules)
- No special character requirements
- No uppercase/lowercase requirements

## API Endpoints

### 1. Register User

```
POST /api/auth/register
Content-Type: application/json

Request Body:
{
  "email": "student@msu.edu",
  "password": "password123"
}

Response (201 Created):
{
  "message": "Registration successful. Please check your email for verification code.",
  "verification_code": "123456"  // TODO: Remove in production, send via email
}

Response (400 Bad Request):
{
  "error": "Email already exists" | "Invalid email format" | "Email must be an MSU email (must end with msu.edu)" | "Password must be at least 8 characters"
}
```

### 2. Verify Email

```
POST /api/auth/verify-email
Content-Type: application/json

Request Body:
{
  "email": "student@university.edu",
  "code": "123456"
}

Response (200 OK):
{
  "message": "Email verified successfully. Your account is now active.",
  "user_id": "uuid-here"
}

Response (400 Bad Request):
{
  "error": "Invalid verification code"
}
```

## Testing Commands

### 1. Test Registration (Valid School Email)

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

Expected: 201 Created with verification code in response

### 2. Test Registration (Invalid Email - Not MSU)

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@gmail.com",
    "password": "password123"
  }'
```

Expected: 400 Bad Request with "Email must be an MSU email (must end with msu.edu)"

### 3. Test Registration (Password Too Short)

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "1234567"
  }'
```

Expected: 400 Bad Request with "Password must be at least 8 characters"

### 4. Test Successful Email Verification (Complete Flow)

```bash
# Step 1: Register a new user
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

Expected Response (201 Created):
```json
{
  "message": "Registration successful. Please check your email for verification code.",
  "verification_code": "123456"
}
```

```bash
# Step 2: Verify email with the code from registration response
# Replace "123456" with the actual verification_code from Step 1
curl -X POST http://localhost:3000/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "code": "123456"
  }'
```

Expected Response (200 OK):
```json
{
  "message": "Email verified successfully. Your account is now active.",
  "user_id": "uuid-here"
}
```

### 5. Test Email Verification (Invalid Code)

```bash
curl -X POST http://localhost:3000/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "code": "000000"
  }'
```

Expected: 400 Bad Request with "Invalid verification code"

### 6. Test Duplicate Registration

```bash
# Register same email twice
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

Expected: 400 Bad Request with "Email already exists"

## Database Changes

### Migration: `002_create_users_table.sql`

The `verification_code` column is included in the users table creation:
- Type: VARCHAR(6)
- Nullable: Yes
- Indexed for faster lookups

## Implementation Details

### Files Created/Modified

1. **Error Handling** (`backend/src/error.rs`)
   - Custom error types with proper HTTP status codes
   - Error responses in JSON format

2. **Models** (`backend/src/models/user.rs`)
   - User struct matching database schema
   - Request/Response DTOs

3. **Utilities**
   - `backend/src/utils/email.rs` - Email validation and code generation
   - `backend/src/utils/password.rs` - Password hashing and validation

4. **Handlers** (`backend/src/handlers/auth.rs`)
   - `register()` - Registration endpoint handler
   - `verify_email()` - Email verification handler

5. **Routes** (`backend/src/routes/mod.rs`)
   - Route definitions for auth endpoints

## TODO for Production

1. **Email Service Integration**
   - Remove `verification_code` from registration response
   - Integrate email service (SendGrid, AWS SES, etc.)
   - Send verification code via email

2. **Code Expiration**
   - Add expiration time for verification codes (e.g., 15 minutes)
   - Clean up expired codes

3. **Rate Limiting**
   - Limit registration attempts per email/IP
   - Limit verification attempts

4. **Email Domain Whitelist** (Optional)
   - Currently only allows msu.edu domain
   - Can maintain a whitelist if other domains needed in future

## Notes

- Currently, verification codes are returned in the API response for testing
- In production, remove the code from the response and only send via email
- MSU email validation checks that email ends with `msu.edu`
- Password requirements are minimal (8+ characters) as requested

