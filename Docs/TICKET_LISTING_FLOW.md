# Ticket Listing Flow Design

## Overview

This document outlines the ticket listing flow and required database schema changes and API endpoints.

## User Flow Analysis

### Step 1: Click Sell Ticket
- User selects game from dropdown
- Games must be in database, classified by sport type
- User fills in exact seat details
- Trading stops before game starts (configurable cutoff time)

### Step 2: Transfer the Ticket
- Ticket starts in **unverified** state
- User transfers ticket to custodian MSU email via Paciolan
- Ticket becomes **verified** when custodian receives it
- Support relisting: `cancelled` → `listed`
- Support reselling: `sold` → `listed`

### Step 3: Get Paid!
- Buyer clicks "buy"
- Ticket goes into **reserved** state for 5 minutes
- During reservation, no other buyer can purchase
- Buyer completes payment within 5 minutes
- If payment not completed, ticket returns to `listed`
- When payment confirmed, ticket transfers to buyer

## Required Database Schema Changes

### 1. Games/Events Table

Need a new `games` table to populate the dropdown:

```sql
CREATE TYPE sport_type AS ENUM ('football', 'basketball', 'hockey', 'baseball', 'soccer', ...);

CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sport_type sport_type NOT NULL,
    name VARCHAR(255) NOT NULL,  -- e.g., "Richmond @ Spartan Football"
    game_date TIMESTAMPTZ NOT NULL,
    listing_cutoff_minutes INTEGER NOT NULL DEFAULT 60,  -- Stop trading X minutes before game
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 2. Update Ticket Status Enum

Current: `('listed', 'pending_sale', 'sold', 'cancelled')`

**Proposed:** `('unverified', 'listed', 'reserved', 'sold', 'cancelled')`

- `unverified` - Ticket created but not yet transferred to custodian
- `listed` - Verified and available for purchase
- `reserved` - Reserved for a buyer (5 minute window)
- `sold` - Successfully sold
- `cancelled` - Listing cancelled by seller

**Note:** Should we keep `pending_sale` or rename to `reserved`? I suggest `reserved` for clarity.

### 3. Update Tickets Table

Add fields for verification and reservation:

```sql
ALTER TABLE tickets ADD COLUMN game_id UUID REFERENCES games(id);
ALTER TABLE tickets ADD COLUMN verified BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE tickets ADD COLUMN verified_at TIMESTAMPTZ;
ALTER TABLE tickets ADD COLUMN reserved_at TIMESTAMPTZ;  -- When reservation started
ALTER TABLE tickets ADD COLUMN reserved_by UUID REFERENCES users(id);  -- Who reserved it
```

**Questions:**
- Should we keep `event_name` and `event_date` in tickets, or just reference `game_id`?
- I suggest keeping both: `game_id` for the dropdown relationship, but `event_name`/`event_date` for historical record (game details might change)

### 4. Ticket Status Transitions

Allowed transitions:
- `unverified` → `listed` (when verified)
- `listed` → `reserved` (when buyer clicks buy)
- `reserved` → `listed` (if payment not completed within 5 min)
- `reserved` → `sold` (when payment confirmed)
- `listed` → `cancelled` (seller cancels)
- `cancelled` → `listed` (seller relists)
- `sold` → `listed` (seller resells)

## Proposed API Endpoints

### Games Endpoints

#### 1. List Upcoming Games
```
GET /api/games
Query Parameters:
  - sport_type (optional): football | basketball | ...
  - upcoming_only (optional): true | false (default: true)
  - limit (optional): number (default: 50)

Response (200 OK):
{
  "games": [
    {
      "id": "uuid-here",
      "sport_type": "football",
      "name": "Richmond @ Spartan Football",
      "game_date": "2023-09-09T15:30:00Z",
      "listing_cutoff_minutes": 60,
      "can_list": true  // True if current time < (game_date - listing_cutoff_minutes)
    }
  ]
}
```

### Ticket Endpoints (Updated)

#### 2. Create Ticket Listing
```
POST /api/tickets
Authorization: Bearer <jwt-token>

Request Body:
{
  "game_id": "uuid-here",
  "level": "STUD",
  "seat_section": "GEN",
  "seat_row": "128",
  "seat_number": "28",
  "price": 5000
}

Response (201 Created):
{
  "id": "uuid-here",
  "game_id": "uuid-here",
  "seller_id": "uuid-here",
  "status": "unverified",
  "verified": false,
  "message": "Please transfer your ticket to [custodian email] to verify"
}
```

#### 3. List Available Tickets
```
GET /api/tickets
Query Parameters:
  - game_id (optional): filter by game
  - sport_type (optional): filter by sport
  - verified_only (optional): true | false (default: true)
  - status (optional): listed | all (default: listed)
  - min_price, max_price (optional): price range
  - limit, offset: pagination

Response (200 OK):
{
  "tickets": [
    {
      "id": "uuid-here",
      "game": {
        "id": "uuid-here",
        "name": "Richmond @ Spartan Football",
        "game_date": "2023-09-09T15:30:00Z",
        "sport_type": "football"
      },
      "level": "STUD",
      "seat_section": "GEN",
      "seat_row": "128",
      "seat_number": "28",
      "price": 5000,
      "status": "listed",
      "verified": true
    }
  ],
  "total": 42
}
```

#### 4. Verify Ticket (Admin/Custodian Endpoint)
```
PATCH /api/tickets/:id/verify
Authorization: Bearer <jwt-token>  // Admin/custodian only
Content-Type: application/json

Request Body:
{
  "verified": true
}

Response (200 OK):
{
  "id": "uuid-here",
  "status": "listed",  // Changes from unverified to listed
  "verified": true,
  "verified_at": "2023-09-01T12:00:00Z"
}
```

#### 5. Reserve Ticket (Buyer clicks "Buy")
```
POST /api/tickets/:id/reserve
Authorization: Bearer <jwt-token>

Response (200 OK):
{
  "id": "uuid-here",
  "status": "reserved",
  "reserved_at": "2023-09-01T12:00:00Z",
  "reserved_until": "2023-09-01T12:05:00Z",  // 5 minutes from now
  "message": "Please complete payment within 5 minutes"
}

Response (400 Bad Request):
{
  "error": "Ticket is not available" | "Ticket already reserved" | "Listing has closed"
}
```

#### 6. Cancel Reservation (if payment not completed)
```
DELETE /api/tickets/:id/reservation
Authorization: Bearer <jwt-token>

// Or automatic cleanup job that runs periodically
```

#### 7. Update Ticket Status (Cancel, Relist, Resell)
```
PATCH /api/tickets/:id
Authorization: Bearer <jwt-token>

Request Body:
{
  "status": "cancelled"  // or "listed" for relist/resell
}

Response (200 OK):
{
  "id": "uuid-here",
  "status": "cancelled"
}

Response (400 Bad Request):
{
  "error": "Invalid status transition" | "Only owner can update ticket"
}
```

## Questions & Decisions Needed

### 1. Games Table Design
- **Q:** How do you want to populate games? Manual admin entry or import from external source?
- **Q:** Should `listing_cutoff_minutes` be per-game or global setting?
- **Recommendation:** Start with global setting, make per-game later if needed

### 2. Ticket Schema
- **Q:** Keep `event_name`/`event_date` in tickets table, or just use `game_id`?
- **Recommendation:** Keep both - `game_id` for relationship, `event_name`/`event_date` for historical record

### 3. Verification Flow
- **Q:** How will you detect when custodian receives the ticket?
  - Option A: Manual admin endpoint to mark as verified
  - Option B: Paciolan webhook/API integration
  - Option C: Automated checking (polling?)
- **Recommendation:** Start with manual admin endpoint, add automation later

### 4. Reservation Expiry
- **Q:** How to handle expired reservations?
  - Option A: Background job that runs every minute
  - Option B: Check on each ticket query
  - Option C: Database trigger/scheduled task
- **Recommendation:** Background job or check on query (simpler to start)

### 5. Status Transitions
- **Q:** Should `pending_sale` be renamed to `reserved`?
- **Recommendation:** Yes, `reserved` is clearer

### 6. Pricing
- **Q:** Current schema uses INTEGER for price (cents). Is this correct?
- **Already using INTEGER:** ✅ Good

## Implementation Plan

### Phase 1: Database Schema Updates
1. Create `games` table migration
2. Create `sport_type` enum
3. Update `ticket_status` enum (add `unverified`, rename `pending_sale` to `reserved`)
4. Add columns to `tickets` table (game_id, verified, verified_at, reserved_at, reserved_by)

### Phase 2: Games API
1. CRUD endpoints for games (admin)
2. Public endpoint to list upcoming games

### Phase 3: Ticket Listing API
1. Create ticket endpoint (unverified state)
2. Update list tickets endpoint (filter by verified)
3. Verify ticket endpoint (admin)

### Phase 4: Reservation System
1. Reserve ticket endpoint (5 min window)
2. Cleanup expired reservations (background job or on-query)
3. Cancel reservation endpoint

### Phase 5: Ticket Management
1. Cancel listing endpoint
2. Relist endpoint (cancelled → listed)
3. Resell endpoint (sold → listed)

## Next Steps

1. **Confirm the decisions above** (games table, verification flow, etc.)
2. **Create migrations** for schema changes
3. **Implement games endpoints** first (needed for dropdown)
4. **Update ticket endpoints** with new flow

What would you like to tackle first? Should we start with the games table and endpoints, or discuss any of the questions above?

