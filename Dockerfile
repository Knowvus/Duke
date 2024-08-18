# Use an official Rust image as the build stage
FROM rust:1.64 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Cache dependencies first by copying the Cargo.toml and Cargo.lock files
# and running `cargo build` with an empty source directory.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Now copy the actual source code and build it
COPY src ./src
RUN cargo build --release

# Use a smaller base image for the final build
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app

# Set the entry point for the container
ENTRYPOINT ["/usr/local/bin/app"]

# Expose the port that your app runs on
EXPOSE 8080

# Set the default command for the container
CMD ["app"]
