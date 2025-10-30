# Use Rust official image as base
FROM rust:1.85-slim as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy backend files only
COPY backend/Cargo.toml ./backend/
COPY backend/src ./backend/src

# Build the backend application
RUN cd backend && cargo build --release

# Production stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary
COPY --from=builder /app/backend/target/release/nft-marketplace-backend /app/backend

# Copy environment file template
COPY backend/.env.example /app/.env

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/ || exit 1

# Run the application
CMD ["./backend"]