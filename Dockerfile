# Compile
FROM    rust:alpine3.16 AS compiler
RUN     apk add -q --update-cache --no-cache build-base openssl-dev git
WORKDIR /mailboar
COPY    . .
RUN     set -eux; \
        cargo build --release --config net.git-fetch-with-cli=true

# Run
FROM    alpine:3.16
ENV     RUST_LOG='mailboar=info'
COPY    --from=compiler /mailboar/target/release/mailboar .
COPY    --from=compiler /mailboar/static ./static

EXPOSE  1025 1080 8025

CMD     ["./mailboar", "--ip=0.0.0.0", "--smtp-port=1025", "--api-port=1080", "--http-port=8025"]
