# Frontend Development Plan

## Tech Stack (Recommended)

- **Framework**: Next.js 14 (App Router)
- **Styling**: Tailwind CSS
- **State Management**: React Context + React Query
- **Forms**: React Hook Form + Zod validation
- **HTTP Client**: Axios or fetch
- **Payment**: Stripe.js + Stripe Elements (future)

---

## Design System

### Color Palette

| Name | Hex | Usage |
|------|-----|-------|
| Spartan Green | `#18453B` | Primary buttons, headers, accents, links |
| Black | `#191A23` | Text, dark backgrounds, footer |
| White | `#F3F3F3` | Backgrounds, cards, light text |

### Tailwind Config

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        'spartan-green': '#18453B',
        'spartan-black': '#191A23',
        'spartan-white': '#F3F3F3',
      },
    },
  },
}
```

### Typography

- **Headings**: Bold, Spartan Green or Black
- **Body**: Regular, Black on White background
- **Links**: Spartan Green with hover underline

---

## Pages & Routes

### Public Pages

| Route | Page | Description |
|-------|------|-------------|
| `/` | Landing | Hero with stadium background, CTA buttons |
| `/login` | Login | Email + password form |
| `/register` | Register | Registration form |
| `/tickets` | Browse Tickets | List of verified tickets for sale |
| `/games` | Games | List of upcoming games |

### Protected Pages (Requires Auth)

| Route | Page | Description |
|-------|------|-------------|
| `/dashboard` | Dashboard | User's ticket listings overview |
| `/sell` | Sell Ticket | Create new ticket listing form |
| `/my-listings` | My Listings | User's tickets with status filters |
| `/checkout/:ticketId` | Checkout | Reserve ticket + payment (future) |

---

## Page Designs

### 1. Landing Page (`/`)

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│  [Logo]                    [Login] [Register]           │  <- Navbar (transparent)
├─────────────────────────────────────────────────────────┤
│                                                         │
│         ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░          │
│         ░░  spartan-stadium.png background  ░░          │
│         ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░          │
│                                                         │
│              MSU TICKET MARKETPLACE                     │  <- White text, large
│           Buy and sell student tickets                  │  <- Subheading
│                                                         │
│         [Browse Tickets]    [Sell Your Ticket]          │  <- CTA buttons
│                                                         │
├─────────────────────────────────────────────────────────┤
│  How It Works:                                          │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │
│  │ 1. List │  │ 2.Verify│  │ 3. Sell │  │ 4. Done │    │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘    │
├─────────────────────────────────────────────────────────┤
│  Footer                                                 │
└─────────────────────────────────────────────────────────┘
```

**Styling:**
- Background: `spartan-stadium.png` with dark overlay (`rgba(25, 26, 35, 0.7)`)
- Text: White (`#F3F3F3`)
- CTA Buttons: Spartan Green background, white text

### 2. Login Page (`/login`)

```
┌─────────────────────────────────────────────────────────┐
│  [Logo]                              [Register]         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│              ┌─────────────────────┐                    │
│              │      Login          │                    │
│              │                     │                    │
│              │  Email              │                    │
│              │  [________________] │                    │
│              │                     │                    │
│              │  Password           │                    │
│              │  [________________] │                    │
│              │                     │                    │
│              │  [    Login     ]   │                    │
│              │                     │                    │
│              │  Don't have an      │                    │
│              │  account? Register  │                    │
│              └─────────────────────┘                    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 3. Register Page (`/register`)

```
┌─────────────────────────────────────────────────────────┐
│              ┌─────────────────────┐                    │
│              │    Create Account   │                    │
│              │                     │                    │
│              │  MSU Email          │                    │
│              │  [________@msu.edu] │                    │
│              │                     │                    │
│              │  Password           │                    │
│              │  [________________] │                    │
│              │                     │                    │
│              │  Confirm Password   │                    │
│              │  [________________] │                    │
│              │                     │                    │
│              │  [   Register   ]   │                    │
│              └─────────────────────┘                    │
└─────────────────────────────────────────────────────────┘
```

> **Note**: Email verification step is not implemented yet. After registration, users can login directly.

### 4. Browse Tickets (`/tickets`)

```
┌─────────────────────────────────────────────────────────┐
│  [Logo]    [Tickets] [Games]    [Sell] [Dashboard]      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Available Tickets                    [Filter ▼]        │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ MSU vs Michigan │ Sept 9 │ Section GEN Row 128  │   │
│  │ $150.00                          [Buy Ticket]   │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ MSU vs Ohio State │ Oct 5 │ Section 101 Row 12  │   │
│  │ $200.00                          [Buy Ticket]   │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- Filter by game/sport type
- Sort by price, date
- Show ticket details (seat, price, event)

### 5. Sell Ticket (`/sell`)

```
┌─────────────────────────────────────────────────────────┐
│              List Your Ticket                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Select Game                                            │
│  [MSU vs Michigan - Sept 9, 2026          ▼]           │
│                                                         │
│  Seat Details                                           │
│  Level        Section       Row         Seat            │
│  [STUD ▼]     [GEN    ]    [128  ]     [28   ]         │
│                                                         │
│  Price                                                  │
│  $ [150.00]                                             │
│                                                         │
│  [        List Ticket        ]                          │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ After listing, transfer your ticket to our      │   │
│  │ Paciolan account within 24 hours for            │   │
│  │ verification.                                    │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 6. My Listings (`/my-listings`)

```
┌─────────────────────────────────────────────────────────┐
│  My Listings                                            │
├─────────────────────────────────────────────────────────┤
│  [All] [Unverified] [Verified] [Sold]                   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ MSU vs Michigan    │ ● Unverified               │   │
│  │ Section GEN Row 128 Seat 28                     │   │
│  │ $150.00           Transfer by: Sept 8, 12:00pm  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ MSU vs Ohio State  │ ● Verified                 │   │
│  │ Section 101 Row 12 Seat 5                       │   │
│  │ $200.00                                         │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Status Badges:**
- `Unverified` - Yellow/Orange
- `Verifying` - Blue (processing)
- `Verified` - Green (available)
- `Reserved` - Purple (buyer checkout)
- `Paid` - Green (payment captured)
- `Sold` - Gray (completed)

### 7. Checkout (`/checkout/:ticketId`) - Future

```
┌─────────────────────────────────────────────────────────┐
│  Complete Purchase                                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────┐  ┌─────────────────────────┐  │
│  │ Ticket Details      │  │ Payment                 │  │
│  │                     │  │                         │  │
│  │ MSU vs Michigan     │  │ Card Number             │  │
│  │ Sept 9, 2026        │  │ [________________]      │  │
│  │ Section GEN         │  │                         │  │
│  │ Row 128, Seat 28    │  │ Exp      CVC            │  │
│  │                     │  │ [____]   [___]          │  │
│  │ Price: $150.00      │  │                         │  │
│  │                     │  │ [    Pay $150.00    ]   │  │
│  └─────────────────────┘  └─────────────────────────┘  │
│                                                         │
│  ⏱ Reservation expires in 6:32                         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

> **Note**: Stripe integration not yet added. For now, show placeholder or skip checkout flow.

---

## Component Structure

```
src/
├── app/
│   ├── layout.tsx              # Root layout with navbar
│   ├── page.tsx                # Landing page
│   ├── login/page.tsx
│   ├── register/page.tsx
│   ├── tickets/page.tsx
│   ├── games/page.tsx
│   ├── sell/page.tsx
│   ├── my-listings/page.tsx
│   ├── checkout/[ticketId]/page.tsx
│   └── dashboard/page.tsx
├── components/
│   ├── ui/
│   │   ├── Button.tsx
│   │   ├── Input.tsx
│   │   ├── Card.tsx
│   │   ├── Badge.tsx
│   │   └── Select.tsx
│   ├── layout/
│   │   ├── Navbar.tsx
│   │   ├── Footer.tsx
│   │   └── AuthGuard.tsx
│   ├── tickets/
│   │   ├── TicketCard.tsx
│   │   ├── TicketList.tsx
│   │   └── TicketStatusBadge.tsx
│   └── forms/
│       ├── LoginForm.tsx
│       ├── RegisterForm.tsx
│       └── SellTicketForm.tsx
├── lib/
│   ├── api.ts                  # API client
│   ├── auth.ts                 # Auth context/hooks
│   └── utils.ts                # Helpers (formatPrice, etc.)
├── hooks/
│   ├── useAuth.ts
│   ├── useTickets.ts
│   └── useGames.ts
└── types/
    └── index.ts                # TypeScript interfaces
```

---

## API Integration

### API Client (`lib/api.ts`)

```typescript
const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

class ApiClient {
  private token: string | null = null;

  setToken(token: string) {
    this.token = token;
    localStorage.setItem('token', token);
  }

  clearToken() {
    this.token = null;
    localStorage.removeItem('token');
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...(this.token && { Authorization: this.token }),
    };

    const res = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers: { ...headers, ...options?.headers },
    });

    if (!res.ok) {
      const error = await res.json();
      throw new Error(error.error || 'Request failed');
    }

    return res.json();
  }

  // Auth
  register(email: string, password: string) {
    return this.request('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  login(email: string, password: string) {
    return this.request<LoginResponse>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  // Games
  getGames() {
    return this.request<{ games: Game[] }>('/api/games');
  }

  // Tickets
  getTickets() {
    return this.request<{ tickets: Ticket[] }>('/api/tickets');
  }

  getMyListings(status?: string) {
    const query = status ? `?status=${status}` : '';
    return this.request<{ tickets: Ticket[] }>(`/api/tickets/my-listings${query}`);
  }

  createTicket(data: CreateTicketRequest) {
    return this.request<Ticket>('/api/tickets', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  reserveTicket(ticketId: string) {
    return this.request<ReservationResponse>(`/api/tickets/${ticketId}/reserve`, {
      method: 'POST',
    });
  }
}

export const api = new ApiClient();
```

---

## Implementation Phases

### Phase 1: Core Setup & Auth
- [ ] Next.js project setup with Tailwind
- [ ] Color theme configuration
- [ ] Layout components (Navbar, Footer)
- [ ] Landing page with stadium background
- [ ] Login page + API integration
- [ ] Register page + API integration
- [ ] Auth context + protected routes

### Phase 2: Ticket Browsing
- [ ] Games list page
- [ ] Tickets list page
- [ ] Ticket card component
- [ ] Filtering and sorting

### Phase 3: Seller Flow
- [ ] Sell ticket form
- [ ] My listings page
- [ ] Status badges
- [ ] Status filtering

### Phase 4: Buyer Flow (Partial)
- [ ] Reserve ticket functionality
- [ ] Checkout page placeholder
- [ ] Reservation timer display

### Phase 5: Polish
- [ ] Loading states
- [ ] Error handling
- [ ] Toast notifications
- [ ] Mobile responsive design

### Future: Payment Integration
- [ ] Stripe.js setup
- [ ] Payment form with Stripe Elements
- [ ] Webhook handling (already done in backend)

---

## Development Notes

### Currently Not Implemented (Backend)

1. **Email Verification** - Registration works, but verification code is returned in response (dev mode). Skip verify-email step for now, users can login directly after registration.

2. **Stripe Payments** - Backend webhook handler exists, but frontend Stripe integration not set up. For now, reserve ticket but skip actual payment.

### Environment Variables

```bash
# .env.local
NEXT_PUBLIC_API_URL=http://localhost:3000
NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_xxx  # Future
```

### Price Display

Backend stores prices in cents. Convert for display:

```typescript
function formatPrice(cents: number): string {
  return `$${(cents / 100).toFixed(2)}`;
}
```

### Status Colors

```typescript
const statusColors = {
  Unverified: 'bg-yellow-100 text-yellow-800',
  Verifying: 'bg-blue-100 text-blue-800',
  Verified: 'bg-green-100 text-green-800',
  Reserved: 'bg-purple-100 text-purple-800',
  Paid: 'bg-green-100 text-green-800',
  Sold: 'bg-gray-100 text-gray-800',
  Cancelled: 'bg-red-100 text-red-800',
};
```
