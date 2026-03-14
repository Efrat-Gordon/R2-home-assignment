FROM rust:1.85-slim AS builder

WORKDIR /app

# Copy manifest and build a dummy binary to cache dependencies
COPY Cargo.toml ./
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
