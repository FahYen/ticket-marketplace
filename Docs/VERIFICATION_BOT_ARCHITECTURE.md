# Verification Bot Architecture Analysis

## Problem Statement

You need to automate the Paciolan ticket transfer acceptance process:
1. Monitor email for incoming transfer requests
2. Log into Paciolan
3. Click "Accept Transfer"
4. Update database when ticket is verified

## Architecture Options

### Option 1: Separate AWS Service/Container (RECOMMENDED ✅)

**Structure:**
- Backend API: AWS ECS/Fargate task (existing)
- Verification Bot: Separate AWS ECS/Fargate task (new)

```
┌─────────────────┐         ┌──────────────────┐
│   Backend API   │◄────────┤  Database (RDS)  │
│   (ECS/Fargate) │         └──────────────────┘
└─────────────────┘                  ▲
                                     │
                            ┌────────┴────────┐
                            │ Verification Bot│
                            │  (ECS/Fargate)  │
                            │  (Selenium)     │
                            └─────────────────┘
```

**Pros:**
- ✅ **Separation of concerns** - Bot crashes don't affect API
- ✅ **Independent scaling** - Run bot on schedule, API on-demand
- ✅ **Independent deployments** - Update bot without touching API
- ✅ **Resource isolation** - Selenium needs more CPU/memory
- ✅ **Easier debugging** - Separate logs, metrics
- ✅ **Cost optimization** - Bot can run only when needed (scheduled tasks)

**Cons:**
- ❌ More infrastructure to manage (2 services instead of 1)
- ❌ Slightly more complex deployment

**Implementation:**
- Docker container with Rust + Selenium WebDriver + Chrome
- Runs on schedule (e.g., every 5 minutes) or continuously
- Uses same DATABASE_URL to update tickets
- Can also use AWS EventBridge for scheduling

---

### Option 2: Separate Thread in Same Process

**Structure:**
```
┌─────────────────────────────┐
│   Backend API Process       │
│  ┌──────────┐  ┌─────────┐ │
│  │   API    │  │   Bot   │ │
│  │  Thread  │  │  Thread │ │
│  └──────────┘  └─────────┘ │
└─────────────────────────────┘
```

**Pros:**
- ✅ Simple deployment (single container)
- ✅ Shared database connection
- ✅ Lower infrastructure cost

**Cons:**
- ❌ **Tight coupling** - Bot crash affects API
- ❌ **Resource contention** - Selenium needs CPU/memory
- ❌ **Deployment complexity** - Can't update bot independently
- ❌ **Scaling issues** - Every API instance runs a bot (wasteful)
- ❌ **Mixed concerns** - Web server + browser automation in same process

**Verdict:** ❌ Not recommended for production

---

### Option 3: AWS Lambda + Chromium Layer

**Structure:**
- Backend API: ECS/Fargate (existing)
- Verification Bot: AWS Lambda (triggered by EventBridge schedule)

**Pros:**
- ✅ Serverless (no container management)
- ✅ Pay-per-execution
- ✅ Automatic scaling

**Cons:**
- ❌ **15-minute timeout limit** - Might be tight for browser automation
- ❌ **Complex setup** - Need Chromium Lambda layer
- ❌ **Cold starts** - Slower first execution
- ❌ **Memory limits** - Selenium + Chrome needs significant memory (512MB-3GB)
- ❌ **Not ideal for long-running browser sessions**

**Verdict:** ⚠️ Possible but not ideal for browser automation

---

### Option 4: Hybrid: Email Webhook + Manual Fallback

**Structure:**
- Email service (SendGrid/Mailgun) sends webhook on new email
- Webhook triggers Lambda/small service to check email
- If automated acceptance fails, flag for manual review

**Pros:**
- ✅ Real-time (no polling delay)
- ✅ More reliable than polling

**Cons:**
- ❌ Requires email service with webhook support
- ❌ Still need Selenium for Paciolan interaction
- ❌ More complex setup

**Verdict:** ⚠️ Good for future optimization, but start simpler

---

## Recommended Approach: Separate AWS Service

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        AWS Account                           │
│                                                              │
│  ┌──────────────────┐          ┌─────────────────────────┐ │
│  │   Backend API    │          │   Verification Bot      │ │
│  │   (ECS Fargate)  │          │   (ECS Fargate)         │ │
│  │                  │          │                         │ │
│  │  - REST API      │          │  - Selenium + Chrome    │ │
│  │  - Port 3000     │          │  - Email monitoring     │ │
│  │  - Always on     │          │  - Runs every 5 min     │ │
│  └────────┬─────────┘          │  - Updates DB           │ │
│           │                     └───────────┬─────────────┘ │
│           │                                 │               │
│           └──────────────┬──────────────────┘               │
│                          │                                  │
│                  ┌───────▼────────┐                         │
│                  │   RDS Postgres │                         │
│                  │   (Shared DB)  │                         │
│                  └────────────────┘                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Details

#### 1. Bot Container Structure

**Dockerfile:**
```dockerfile
FROM rust:latest as builder
# ... build Rust bot code ...

FROM mcr.microsoft.com/playwright:v1.40.0-focal
# Or use selenium/standalone-chrome image
# Install Rust runtime, copy binary, etc.
```

**Tech Stack Options:**
- **Option A:** Rust + `thirtyfour` (Selenium WebDriver) + `headless_chrome` or `chromium`
- **Option B:** Rust + `playwright-rust` (easier, more reliable than Selenium)
- **Option C:** Python + Selenium (simpler, but you wanted Rust)

**Recommendation:** Start with Python + Selenium (faster to prototype), migrate to Rust later if needed.

#### 2. Bot Workflow

```rust
// Pseudocode
loop {
    // 1. Check for unverified tickets
    let unverified_tickets = db.get_unverified_tickets().await?;
    
    // 2. Check email for transfer requests
    let transfer_emails = email_client.check_incoming_transfers().await?;
    
    // 3. Match emails to tickets (by user email, seat details, etc.)
    for transfer in transfer_emails {
        let ticket = find_matching_ticket(&transfer)?;
        
        // 4. Log into Paciolan
        let browser = launch_chrome().await?;
        paciolan_login(&browser, credentials).await?;
        
        // 5. Navigate to transfers, find pending transfer
        let transfer_page = navigate_to_transfers(&browser).await?;
        let pending = find_pending_transfer(&transfer_page, &transfer).await?;
        
        // 6. Accept transfer
        accept_transfer(&browser, &pending).await?;
        
        // 7. Update database
        db.mark_ticket_verified(ticket.id).await?;
        
        browser.close().await?;
    }
    
    // 8. Sleep for 5 minutes
    tokio::time::sleep(Duration::from_secs(300)).await;
}
```

#### 3. Email Monitoring

**Options:**
- **IMAP polling** - Connect to email via IMAP, check inbox
- **Email webhook** - Use SendGrid/Mailgun webhook (better, but adds dependency)
- **Gmail API** - If using Gmail, use Gmail API

**Recommendation:** Start with IMAP polling (simpler), move to webhook later.

#### 4. Database Schema Addition

```sql
-- Add to tickets table
ALTER TABLE tickets ADD COLUMN verification_requested_at TIMESTAMPTZ;
ALTER TABLE tickets ADD COLUMN verified_at TIMESTAMPTZ;
ALTER TABLE tickets ADD COLUMN verified BOOLEAN NOT NULL DEFAULT FALSE;

-- Bot can query:
SELECT * FROM tickets 
WHERE status = 'unverified' 
  AND verified = false
  AND created_at > NOW() - INTERVAL '24 hours';  -- Only check recent tickets
```

#### 5. Deployment Strategy

**Option A: Scheduled ECS Task (Recommended)**
- Run as AWS ECS Task on schedule (EventBridge)
- Runs for ~1-2 minutes every 5 minutes
- Costs: ~$0.01-0.05/month (very cheap)

**Option B: Continuous ECS Service**
- Run as long-running ECS service
- Always running, polls every 5 minutes
- Costs: ~$10-15/month (always-on Fargate)

**Recommendation:** Start with scheduled task (cheaper, simpler).

---

## Security Considerations

### 1. Credentials Storage

**Store in AWS Secrets Manager:**
- Paciolan username/password
- Email credentials (IMAP)
- Database connection string (can share with API)

```rust
use aws_sdk_secretsmanager::Client as SecretsClient;

let secret = secrets_client
    .get_secret_value()
    .secret_id("paciolan-credentials")
    .send()
    .await?;
```

### 2. Bot Authentication

- Bot should use separate database user with limited permissions
- Only allow: `SELECT` on tickets, `UPDATE` on tickets (verified fields only)

### 3. Error Handling

- Log all actions (don't log passwords!)
- Alert on failures (email/SNS notification)
- Retry logic for transient failures
- Manual override endpoint (admin can mark verified manually)

---

## Alternative: Start Simple (MVP Approach)

### Phase 1: Manual Verification (Start Here)
1. User creates ticket → status = `unverified`
2. Admin endpoint: `PATCH /api/admin/tickets/:id/verify`
3. You manually log in, accept transfer, then hit endpoint
4. **Time to implement:** 1 hour

### Phase 2: Email Monitoring Only
1. Bot checks email for transfers
2. Sends you notification: "Ticket X needs verification"
3. You manually verify via admin endpoint
4. **Time to implement:** 2-3 hours

### Phase 3: Full Automation (Selenium Bot)
1. Implement full Selenium automation
2. Run as separate service
3. **Time to implement:** 1-2 days

**Recommendation:** Start with Phase 1, add automation later when you understand the flow better.

---

## Questions for You

1. **How often do you expect transfers?**
   - If <10/day: Manual might be fine initially
   - If >50/day: Automation becomes necessary

2. **Paciolan login complexity?**
   - Simple username/password? → Easy to automate
   - 2FA/MFA? → More complex (need to handle)
   - CAPTCHA? → Harder to automate

3. **Language preference?**
   - Stick with Rust for consistency?
   - Use Python for faster Selenium prototyping?

4. **Budget/timeline?**
   - Need it automated immediately?
   - Can we start manual and automate later?

---

## Repository Structure Decision

### Option A: Same Repo, Separate Workspace Member (RECOMMENDED ✅)

**Structure:**
```
ticket-marketplace/
├── Cargo.toml              # Workspace with members: ["backend", "verification-bot"]
├── backend/
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
├── verification-bot/
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
└── docker-compose.yml
```

**Pros:**
- ✅ **Easy code sharing** - Both use same database schema, can share types
- ✅ **Unified versioning** - Same git history, easier to track changes
- ✅ **Shared dependencies** - Use workspace.dependencies (sqlx, chrono, etc.)
- ✅ **Single repo to clone** - Simpler development setup
- ✅ **Independent deployments** - Still separate Dockerfiles/containers
- ✅ **Can share models** - Create `shared/` crate later if needed

**Cons:**
- ❌ Slightly larger repo size
- ❌ Mixed concerns in one repo (but still separate packages)

**Implementation:**
```toml
# Cargo.toml (workspace root)
[workspace]
members = ["backend", "verification-bot"]

# verification-bot/Cargo.toml
[package]
name = "verification-bot"
version.workspace = true

[dependencies]
sqlx.workspace = true
tokio.workspace = true
# ... Selenium/browser automation deps
```

---

### Option B: Separate Repository

**Structure:**
```
ticket-marketplace/          # Main repo
├── backend/
└── ...

ticket-marketplace-bot/      # Separate repo
└── verification-bot/
```

**Pros:**
- ✅ **Complete separation** - Clear boundary between services
- ✅ **Independent versioning** - Different release cycles
- ✅ **Access control** - Can give different people access
- ✅ **Smaller repos** - Each repo is focused

**Cons:**
- ❌ **Code duplication** - Need to duplicate models/types or create shared crate
- ❌ **Sync issues** - Schema changes need to be synced across repos
- ❌ **More repos to manage** - Multiple clones, multiple CI/CD
- ❌ **Harder to share code** - Need to publish shared crate or use git dependencies

**When separate repo makes sense:**
- Different teams maintaining each service
- Want to open-source one but not the other
- Completely different deployment pipelines
- Bot is truly independent (doesn't share types/models)

---

## My Recommendation: Same Repo, Separate Workspace Member

**Why:**
1. **You're solo developer** - No need for access control separation
2. **Shared database schema** - Both services read/write same tables
3. **Rust workspace makes it easy** - Clean separation as packages
4. **Independent deployments** - Still separate Dockerfiles, separate ECS services
5. **Easier development** - One `git clone`, shared dependencies

**Structure:**
```toml
# Root Cargo.toml
[workspace]
members = ["backend", "verification-bot"]

# verification-bot/Cargo.toml
[package]
name = "verification-bot"
version.workspace = true
edition.workspace = true

[dependencies]
sqlx.workspace = true
tokio.workspace = true
chrono.workspace = true
uuid.workspace = true
# ... browser automation deps (selenium/playwright)
```

**Deployment:**
- `backend/Dockerfile` → Builds backend binary
- `verification-bot/Dockerfile` → Builds bot binary
- Separate ECS services, but same codebase

---

## When to Split Later

Consider separate repo if:
- Bot becomes complex (>5000 lines)
- You want to open-source backend but keep bot private
- Different people/teams maintain them
- Bot needs completely different CI/CD

For now, **same repo is simpler and more practical.**

## My Recommendation

**Start with:** Separate AWS ECS Fargate service (scheduled task)

**Repository:** Same repo, separate workspace member (`verification-bot/`)

**Why:**
- Best long-term architecture
- Minimal cost (scheduled tasks are cheap)
- Clean separation of concerns
- Easy to maintain and debug
- Easier code sharing and development

**But first:** Implement manual admin endpoint for Phase 1, then add automation once you understand the workflow.

What do you think? Should we start with the manual endpoint and add automation later, or build the full bot now?

