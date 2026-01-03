# User Login Endpoint Implementation

## Overview

The user login endpoint has been implemented with JWT token-based authentication. Users must have verified their email before they can log in.

## Login Flow

1. User submits email + password
2. System validates email exists in database
3. System verifies password matches stored hash
4. System checks that email is verified (`email_verified = true`)
5. System generates JWT token
6. System returns JWT token and user information

## Requirements

- User must be registered
- User must have verified their email
- Password must be correct
- JWT token expires after 24 hours (configurable via `JWT_SECRET`)

## API Endpoint

### Login User

```
POST /api/auth/login
Content-Type: application/json

Request Body:
{
  "email": "student@msu.edu",
  "password": "password123"
}

Response (200 OK):
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid-here",
    "email": "student@msu.edu",
    "email_verified": true
  }
}

Response (401 Unauthorized):
{
  "error": "Invalid email or password"
}

Response (403 Forbidden):
{
  "error": "Email not verified"
}
```

## Testing Commands

### 1. Test Successful Login

**Prerequisites:**
- User must be registered (use registration endpoint first)
- User must have verified their email (use verify-email endpoint)

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "password123"
  }'
```

Expected Response (200 OK):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3OCIsImVtYWlsIjoic3R1ZGVudEBtc3UuZWR1IiwiZXhwIjoxNzA0MTAwMDAwfQ.example_signature",
  "user": {
    "id": "12345678-1234-1234-1234-123456789abc",
    "email": "student@msu.edu",
    "email_verified": true
  }
}
```

### 2. Test Login with Invalid Email

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "nonexistent@msu.edu",
    "password": "password123"
  }'
```

Expected Response (401 Unauthorized):
```json
{
  "error": "Invalid email or password"
}
```

### 3. Test Login with Invalid Password

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@msu.edu",
    "password": "wrongpassword"
  }'
```

Expected Response (401 Unauthorized):
```json
{
  "error": "Invalid email or password"
}
```

### 4. Test Login with Unverified Email

**Prerequisites:**
- Register a user but do NOT verify the email

```bash
# Step 1: Register user (but don't verify)
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "unverified@msu.edu",
    "password": "password123"
  }'

# Step 2: Try to login with unverified email
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "unverified@msu.edu",
    "password": "password123"
  }'
```

Expected Response (403 Forbidden):
```json
{
  "error": "Email not verified"
}
```

### 5. Test Complete Flow (Register → Verify → Login)

```bash
# Step 1: Register
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@msu.edu",
    "password": "password123"
  }'

# Step 2: Verify email (replace "123456" with the actual verification code from Step 1)
curl -X POST http://localhost:3000/api/auth/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@msu.edu",
    "code": "123456"
  }'

# Step 3: Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@msu.edu",
    "password": "password123"
  }'
```

Expected Response (200 OK):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid-here",
    "email": "newuser@msu.edu",
    "email_verified": true
  }
}
```

## Using the JWT Token

After successful login, include the JWT token in subsequent requests:

```bash
# Example: Using token in a protected endpoint (when implemented)
curl -X GET http://localhost:3000/api/tickets/my-listings \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## Environment Variables

- `JWT_SECRET`: Secret key for signing JWT tokens (required - must be set)
  - **Important:** Change this in production!
  - Should be a long, random string
  - Example: `openssl rand -base64 32`

## Implementation Details

### Files Created/Modified

1. **JWT Utilities** (`backend/src/utils/jwt.rs`)
   - `generate_token()` - Creates JWT token with user ID and email
   - `validate_token()` - Validates and decodes JWT token
   - Token expiration: 24 hours (default)

2. **Models** (`backend/src/models/user.rs`)
   - `LoginRequest` - Login request DTO
   - `LoginResponse` - Login response with token and user info
   - `UserInfo` - User information returned in login response

3. **Error Handling** (`backend/src/error.rs`)
   - `InvalidCredentials` - Invalid email or password
   - `EmailNotVerified` - Email not verified yet

4. **Handlers** (`backend/src/handlers/auth.rs`)
   - `login()` - Login endpoint handler

5. **Routes** (`backend/src/routes.rs`)
   - Added `/api/auth/login` route

## Security Notes

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours
- Invalid credentials return the same error message (prevents email enumeration)
- Email must be verified before login is allowed
- JWT_SECRET should be set as an environment variable in production

## TODO for Production

1. **JWT Secret Configuration**
   - Set `JWT_SECRET` environment variable
   - Use a secure random secret (e.g., `openssl rand -base64 32`)
   - Never commit secrets to version control

2. **Token Refresh** (Optional)
   - Implement refresh token mechanism
   - Allow users to refresh tokens without re-logging in

3. **Rate Limiting**
   - Limit login attempts per IP/email
   - Prevent brute force attacks

4. **Logging**
   - Log failed login attempts for security monitoring
   - Track successful logins

