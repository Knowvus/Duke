# First Stage: Build the Rust application (generic)
FROM rust:1.78 as builder
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to build dependencies first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to allow caching of dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code and build the project
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Second Stage: Create a minimal runtime environment
FROM ubuntu:22.04
WORKDIR /usr/local/bin

# Install necessary packages and Infisical CLI
RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    ca-certificates \
    curl \
    gnupg \
    apt-transport-https \
    jq \
    && curl -1sLf 'https://dl.cloudsmith.io/public/infisical/infisical-cli/setup.deb.sh' | bash \
    && apt-get update && apt-get install -y infisical \
    && rm -rf /var/lib/apt/lists/*

# Clean up any existing files or directories with the same name
RUN rm -rf /usr/local/bin/duke

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/duke

# Create a comprehensive manage-swarm.sh script
RUN echo '#!/bin/bash' > /usr/local/bin/manage-swarm.sh && \
    echo 'echo "Running manage-swarm.sh script..."' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "INFISICAL_CLIENT_ID: $INFISICAL_CLIENT_ID"' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "INFISICAL_CLIENT_SECRET: $INFISICAL_CLIENT_SECRET"' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "INFISICAL_PROJECT_ID: $INFISICAL_PROJECT_ID"' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$INFISICAL_CLIENT_ID" ] && { echo "INFISICAL_CLIENT_ID is not set"; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$INFISICAL_CLIENT_SECRET" ] && { echo "INFISICAL_CLIENT_SECRET is not set"; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$INFISICAL_PROJECT_ID" ] && { echo "INFISICAL_PROJECT_ID is not set"; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "Attempting Infisical login..."' >> /usr/local/bin/manage-swarm.sh && \
    echo 'curl -s -X POST "https://api.infisical.com/auth/login" -d "clientId=$INFISICAL_CLIENT_ID&clientSecret=$INFISICAL_CLIENT_SECRET"' >> /usr/local/bin/manage-swarm.sh && \
    echo 'LOGIN_RESPONSE=$(curl -s -X POST "https://api.infisical.com/auth/login" -d "clientId=$INFISICAL_CLIENT_ID&clientSecret=$INFISICAL_CLIENT_SECRET")' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "Login response: $LOGIN_RESPONSE"' >> /usr/local/bin/manage-swarm.sh && \
    echo 'INFISICAL_TOKEN=$(echo $LOGIN_RESPONSE | jq -r .token)' >> /usr/local/bin/manage-swarm.sh && \
    echo 'echo "INFISICAL_TOKEN: $INFISICAL_TOKEN"' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$INFISICAL_TOKEN" ] && { echo "Infisical login failed"; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    chmod +x /usr/local/bin/manage-swarm.sh

# Expose the port the application will run on
EXPOSE 8080

# Health check to ensure the container is healthy
HEALTHCHECK --interval=1m --timeout=10s --start-period=1m --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Set the entrypoint to the manage-swarm.sh script
ENTRYPOINT ["/usr/local/bin/manage-swarm.sh"]
