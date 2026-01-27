# ---- build stage ----
FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app

# 1) Copy manifests first (cache deps)
COPY Cargo.toml Cargo.lock ./

# 2) Copy source
COPY src ./src

# Copy migrations directory
COPY migrations ./migrations


# 3) Build
RUN cargo build --release

# ---- runtime stage ----
FROM debian:bookworm-slim
RUN useradd -m appuser
WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
USER appuser

EXPOSE 8080
CMD ["/app/backend"]