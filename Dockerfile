# Multi-stage Docker build for Dioxus fullstack application

# Stage 1: Build the application
FROM docker.io/library/rust:latest as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target for frontend builds
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies first (better caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code and assets
COPY src/ ./src/
COPY assets/ ./assets/

# Build the application
RUN cargo build --release

# Stage 2: Runtime image
FROM docker.io/library/debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/ox ./

# Copy assets
COPY --from=builder /app/assets/ ./assets/

# Change ownership to appuser
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the port (Dioxus default is 8080)
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV DIOXUS_PORT=8080
ENV DIOXUS_HOST=0.0.0.0

# Run the application
CMD ["./ox"]