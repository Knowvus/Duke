# Use an official Rust image as the build stage
FROM rust:1.70 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and lock files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty main file to cache dependencies
# RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build only the dependencies
RUN cargo build --release && rm -rf src

# Now copy the actual source code and build it
COPY src ./src
RUN cargo build --release

# Output the contents of the build directory to verify the binary is there
RUN ls -lh target/release/

# Use a smaller base image for the final build
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/app

# Output the contents of the /usr/local/bin/ directory to verify the binary is there
RUN ls -lh /usr/local/bin/

# Expose the port that your app runs on
EXPOSE 8080

# Ensure the container does not exit by adding a default command
CMD ["/usr/local/bin/app"]
