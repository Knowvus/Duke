# Use an official Rust image as the build stage
FROM rust:1.70 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and lock files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty main file to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build only the dependencies
RUN cargo build --release && rm -rf src

# Now copy the actual source code and build it
COPY src ./src
RUN cargo build --release

# Use a more recent base image for the final build
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/app

# Set the entry point for the container
ENTRYPOINT ["/usr/local/bin/app"]

# Expose the port that your app runs on
EXPOSE 8080

# Set the default command for the container
CMD ["app"]
