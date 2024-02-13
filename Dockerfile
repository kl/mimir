# Build stage
FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
# This copy invalidates the layer cache so chef prepare is always run
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies only - only run when recipe.json changes
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/mimir mimir
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./mimir"]