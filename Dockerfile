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

# `--keystore <name>` looks in $HOME/.foundry/keystores. Point that at /keystores,
# so `-v ~/.foundry/keystores:/keystores:ro --keystore <name>` works.
RUN mkdir -p /out/.foundry && ln -s /keystores /out/.foundry/keystores

# Runtime stage
FROM scratch

# CA certs for HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Binary (the ABI is embedded at compile time)
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/gbc-claim /gbc-claim

WORKDIR /
COPY --from=builder /out/.foundry /.foundry
ENV HOME=/
ENTRYPOINT ["/gbc-claim"]
