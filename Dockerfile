# Build stage
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/gbc-claim /usr/local/bin/gbc-claim
COPY --from=builder /usr/src/app/abis /usr/local/bin/abis
WORKDIR /usr/local/bin
ENTRYPOINT ["gbc-claim"]
