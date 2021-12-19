# Build Stage
FROM rust:latest AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new mailboar
WORKDIR /usr/src/mailboar
COPY Cargo.toml Cargo.lock ./
COPY static ./static
COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/mailboar .
COPY --from=builder /usr/src/mailboar/static ./static
ENV RUST_LOG=mailboar=info
EXPOSE 1025 1080 8025
CMD ["./mailboar", "--ip=0.0.0.0", "--smtp-port=1025", "--api-port=1080", "--http-port=8025"]
