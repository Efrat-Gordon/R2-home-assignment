FROM rust:1.88-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Copy manifest and migrations, then build a dummy binary to cache dependencies
COPY Cargo.toml ./
COPY migrations ./migrations
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build the real application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ---- runtime image ----
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/be-low-level /usr/local/bin/be-low-level

EXPOSE 4000

CMD ["be-low-level"]
