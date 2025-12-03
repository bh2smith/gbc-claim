# Build stage
FROM rust:1-alpine AS builder

# Install build deps
RUN apk add --no-cache musl-dev openssl-dev pkgconfig ca-certificates

WORKDIR /usr/src/app
COPY . .

# Build for musl (static)
RUN rustup target add x86_64-unknown-linux-musl \
 && cargo build --release --target x86_64-unknown-linux-musl \
 && strip target/x86_64-unknown-linux-musl/release/gbc-claim

# Runtime stage
FROM scratch

# CA certs for HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Your binary + abis
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/gbc-claim /gbc-claim
COPY --from=builder /usr/src/app/abis /abis

WORKDIR /
ENTRYPOINT ["/gbc-claim"]
