# First Stage: Build the Rust application (generic)
FROM rust:1.78 as builder
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to build dependencies first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to allow caching of dependenciesa
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
    curl gnupg apt-transport-https \
    && curl -1sLf 'https://dl.cloudsmith.io/public/infisical/infisical-cli/setup.deb.sh' | bash \
    && apt-get update && apt-get install -y infisical \
    && rm -rf /var/lib/apt/lists/*

# Clean up any existing files or directories with the same name
RUN rm -rf /usr/local/bin/duke

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/duke

# Expose the port the application will run on
EXPOSE 8080

# Run the application with Infisical CLI to inject secrets
CMD ["infisical", "run", "--projectId", "${INFISICAL_PROJECT_ID}", "--", "${SERVICE_EXECUTABLE}"]

# Health check to ensure the container is healthy
HEALTHCHECK --interval=1m --timeout=10s --start-period=1m --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Script to manage Docker Swarm tokens and CA Fingerprint
RUN echo '#!/bin/bash' > /usr/local/bin/manage-swarm.sh && \
    echo 'INFISICAL_TOKEN=$(infisical login --method=universal-auth --client-id="$INFISICAL_CLIENT_ID" --client-secret="$INFISICAL_CLIENT_SECRET" --plain --silent)' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$INFISICAL_TOKEN" ] && { echo "Infisical login failed"; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo 'MANAGER_TOKEN=$(docker swarm join-token -q manager)' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$MANAGER_TOKEN" ] && { echo "Failed to retrieve Docker Swarm manager join token."; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo 'infisical secrets set DOCKER_SWARM_MANAGER_TOKEN="$MANAGER_TOKEN" --projectId "$INFISICAL_PROJECT_ID" --method=universal-auth --client-id="$INFISICAL_CLIENT_ID" --client-secret="$INFISICAL_CLIENT_SECRET"' >> /usr/local/bin/manage-swarm.sh && \
    echo 'CA_FINGERPRINT=$(docker info --format "{{.Swarm.Cluster.TLSInfo.TrustRoot}}" | openssl x509 -noout -fingerprint -sha256 | sed "s/.*=//")' >> /usr/local/bin/manage-swarm.sh && \
    echo '[ -z "$CA_FINGERPRINT" ] && { echo "Failed to retrieve Docker Swarm CA Fingerprint."; exit 1; }' >> /usr/local/bin/manage-swarm.sh && \
    echo 'infisical secrets set DOCKER_SWARM_CA_FINGERPRINT="$CA_FINGERPRINT" --projectId "$INFISICAL_PROJECT_ID" --method=universal-auth --client-id="$INFISICAL_CLIENT_ID" --client-secret="$INFISICAL_CLIENT_SECRET"' >> /usr/local/bin/manage-swarm.sh && \
    chmod +x /usr/local/bin/manage-swarm.sh

# Entry point
ENTRYPOINT ["/usr/local/bin/manage-swarm.sh"]
