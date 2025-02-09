# Compile
FROM    rust:alpine AS compiler
RUN     apk add -q --update-cache --no-cache build-base openssl-dev git perl
WORKDIR /mailboar
COPY    . .
RUN     set -eux; \
        cargo build --release --config net.git-fetch-with-cli=true

# Run
FROM    alpine:3.16
COPY    --from=compiler /mailboar/target/release/mailboar .
COPY    --from=compiler /mailboar/crates/frontend/static ./crates/frontend/static

EXPOSE  1025 1080 8025

CMD     ["./mailboar", "--ip=0.0.0.0", "--smtp-port=1025", "--api-port=1080", "--http-port=8025"]
