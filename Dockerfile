# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the Rust application
RUN cargo build --release

# Make port 8080 available to the world outside this container
EXPOSE 8080

# Run the executable
CMD ["./target/release/app"]

