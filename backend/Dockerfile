# Build stage
FROM rust:1.92.0 as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
# Cargo.lock will be generated during build if it doesn't exist

# Create a dummy source file to cache dependencies
RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    cargo generate-lockfile && \
    cargo build --release --bin backend && \
    rm -rf backend/src

# Copy actual source code
COPY backend ./backend

# Build the actual application
RUN touch backend/src/main.rs && \
    cargo build --release --bin backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/backend /app/backend

# Copy migrations directory (needed for sqlx::migrate!)
COPY --from=builder /app/backend/migrations ./migrations

# Expose port
EXPOSE 3000

# Run the application
CMD ["./backend"]

