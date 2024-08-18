# Use an official Rust image as the build stage
FROM rust:1.70 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and lock files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty main file to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build only the dependencies
RUN cargo build --release && rm -rf src

# Now copy the actual source code and build it
COPY src ./src
RUN cargo build --release

# Use a smaller base image for the final build
FROM debian:bullseye-slim

# Set the working directory in the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/duke /usr/local/bin/duke

# Copy any necessary files for runtime (if any)
COPY ./config ./config

# Expose the port that your app runs on
EXPOSE 8080

# Run the Rust application
CMD ["/usr/local/bin/duke"]
