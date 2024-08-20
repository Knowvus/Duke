# First Stage: Build the Rust application
FROM rust:1.70 as builder
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
FROM ubuntu:20.04
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/duke

# Debugging steps to verify the binary
RUN ls -la /usr/local/bin/duke
RUN /usr/local/bin/duke --help

# Expose the port the application will run on
EXPOSE 8080

# Set the default command to run your application
CMD ["duke"]

# Health check to ensure the container is healthy
HEALTHCHECK --interval=1m --timeout=10s --start-period=1m --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
