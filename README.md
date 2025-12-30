# Ticket Marketplace

An MSU student sports ticket exchange marketplace with custodial Paciolan account integration.

### Using Docker Compose

1. **Start all services:**
   ```bash
   docker-compose up --build
   ```

2. **Stop all services:**
   ```bash
   docker-compose down
   ```

### Local Development (Optional)

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Run the backend:**
   ```bash
   cd backend
   cargo run
   ```

   The server will start on `http://localhost:3000`

3. **Test the health endpoint:**
   ```bash
   curl http://localhost:3000/health
   ```

## Project Structure

```
ticket-marketplace/
├── backend/              # Rust backend application
│   ├── src/
│   │   └── main.rs      # Main application entry point
│   ├── Cargo.toml       # Backend dependencies
│   └── Dockerfile       # Backend container definition
├── docker-compose.yml   # Local development environment
├── Cargo.toml          # Workspace configuration
└── README.md           # This file
```

## Development Workflow

1. Make changes to the code
2. Rebuild and restart:
   ```bash
   docker-compose up --build
   ```
3. Test your changes
4. Iterate!

## Next Steps (Phase 2)

- Set up database schema
- Create basic CRUD API endpoints
- Implement user registration and authentication

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string (default set in docker-compose.yml)
- `RUST_LOG`: Logging level (default: `backend=debug,tower_http=debug`)

## API Endpoints

### Health Check
- **GET** `/health`
- Returns service health status and version

## Troubleshooting

- **Port already in use:** Change ports in `docker-compose.yml`
- **Build fails:** Ensure Docker has enough resources allocated
- **Database connection issues:** Wait for PostgreSQL to be healthy before accessing
