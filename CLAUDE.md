# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MSU student sports ticket marketplace with custodial Paciolan account integration. Rust/Axum backend with PostgreSQL, Stripe payments, and bot-based ticket verification.

## Build & Run Commands

All development is done via Docker (no local Rust installation required).

```bash
docker-compose up -d              # Start backend + PostgreSQL
docker-compose up -d --build      # Rebuild and start
docker-compose logs -f backend    # View logs
docker-compose down               # Stop services
./reset-db.sh                     # Reset database
```

## Architecture

### Tech Stack
- **Framework**: Axum (async Rust web framework)
- **Database**: PostgreSQL 16 with SQLx (compile-time checked queries)
- **Auth**: JWT tokens (users) + API keys (bot/admin)
- **Payments**: Stripe with manual capture flow
- **Rate Limiting**: Governor (token bucket)

### Ticket State Machine
Core business logic - all transitions are atomic with race condition protection:

```
unverified → verifying → verified → reserved → paid → sold
     ↓           ↓
  (deleted)  (timeout reset)
```

| Transition | Endpoint | Auth | Trigger |
|------------|----------|------|---------|
| unverified → verifying | `POST /api/tickets/claim` | BOT_API_KEY | Bot detects incoming transfer |
| verifying → verified | `PATCH /api/tickets/:id/verify` | BOT_API_KEY | Bot accepts transfer in Paciolan |
| verifying → unverified | `DELETE /api/tickets/:id/unclaim` | BOT_API_KEY | Bot rollback if accept fails |
| verified → reserved | `POST /api/tickets/:id/reserve` | JWT | Buyer clicks buy |
| reserved → paid | `POST /api/webhooks/stripe` | Stripe signature | Stripe payment captured |
| paid → sold | `PATCH /api/tickets/:id/sold` | BOT_API_KEY | Bot transfers ticket to buyer in Paciolan |

### Key Files
- `backend/src/handlers/tickets.rs` - Core ticket operations (state transitions)
- `backend/src/handlers/webhooks.rs` - Stripe webhook handler
- `backend/src/utils/cleanup.rs` - Background cleanup tasks
- `backend/src/routes.rs` - Router setup with middleware
- `backend/src/models/ticket.rs` - Ticket state definitions

### Background Cleanup Tasks
Spawned on startup as Tokio tasks:
- **Expired unverified**: Deletes tickets past `transfer_deadline` (hourly)
- **Stuck verifying**: Resets to unverified after timeout (every 60s)
- **Expired reservations**: Resets to verified after window expires (hourly)

### Race Condition Protections
- `FOR UPDATE SKIP LOCKED` in claim endpoint
- Atomic `UPDATE...WHERE` with status checks
- `payment_intents` table for webhook idempotency
- Unique constraint: `(game_id, level, seat_section, seat_row, seat_number)`

## Environment Variables

Key configuration (see `.env` for full list):

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | PostgreSQL connection string |
| `JWT_SECRET` | JWT signing key |
| `BOT_API_KEY` | Bot authentication |
| `ADMIN_API_KEY` | Admin operations |
| `STRIPE_SECRET_KEY` | Stripe API key |
| `STRIPE_WEBHOOK_SECRET` | Webhook signature verification |
| `TRANSFER_DEADLINE_HOURS` | Time for seller to transfer (default: 24) |
| `TOTAL_RESERVATION_WINDOW_MINUTES` | Buyer checkout window (default: 7) |

## Testing

### Stripe Webhook Testing
```bash
# Terminal 1: Start webhook listener
stripe listen --forward-to localhost:3000/api/webhooks/stripe

# Terminal 2: Create and confirm payment intent
stripe payment_intents create --amount=15000 --currency=usd --capture-method=manual \
  --metadata[ticket_id]=<uuid> --metadata[buyer_id]=<uuid> --metadata[reserved_at]=<timestamp>
stripe payment_intents confirm <pi_id> --payment-method=pm_card_visa
```

See `Docs/API_REFERENCE.md` for complete testing flow and `Docs/TICKET_STATE_TRANSITIONS.md` for state machine details.
