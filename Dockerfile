# Use an official Rust image as the base image
FROM rust:1.60 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a slim image as the final image
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/duke

# Set the entry point
ENTRYPOINT ["duke"]

# Expose the port the app runs on
EXPOSE 3030
