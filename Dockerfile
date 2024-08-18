# Use an official Rust image as the build environment
FROM rust:1.60 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and lock files first, and build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source code and build the application
COPY src ./src
RUN cargo install --path .

# Use a smaller base image for the final image
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/duke_rs .

CMD ["./duke_rs"]
