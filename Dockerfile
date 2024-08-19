FROM rust:1.70 as builder
   WORKDIR /usr/src/app
   COPY Cargo.toml Cargo.lock ./
   RUN mkdir src && echo "fn main() {}" > src/main.rs
   RUN cargo build --release
   COPY src ./src
   RUN touch src/main.rs && cargo build --release

   FROM debian:buster-slim
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   FROM ubuntu:20.04
   COPY --from=builder /usr/src/app/target/release/duke /usr/local/bin/duke
   EXPOSE 8080
   CMD ["duke"]
