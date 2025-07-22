FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy Rust project files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY proto/ ./proto/
COPY src/ ./src/

# Build the WASM binary
RUN cargo build --target wasm32-unknown-unknown --release

FROM ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Substreams CLI
RUN curl -sSL https://github.com/streamingfast/substreams/releases/download/v1.7.0/substreams_linux_x86_64.tar.gz | tar -xz -C /usr/local/bin/

# Install MongoDB sink
RUN curl -sSL https://github.com/streamingfast/substreams-sink-mongodb/releases/download/v2.0.1/substreams-sink-mongodb_linux_x86_64.tar.gz | tar -xz -C /usr/local/bin/

WORKDIR /app

# Copy built artifacts and configuration
COPY --from=builder /app/target ./target
COPY substreams.yaml ./
COPY proto/ ./proto/

# Create startup script
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
echo "Waiting for MongoDB to be ready..."\n\
until curl -s mongodb:27017 > /dev/null 2>&1; do\n\
  sleep 2\n\
done\n\
\n\
echo "Starting Substreams processing..."\n\
exec "$@"' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]