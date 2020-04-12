# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:latest as cargo-build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/crab_mail

ADD --chown=rust:rust . ./

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 crab_mail

RUN adduser -D -s /bin/sh -u 1000 -G crab_mail crab_mail

WORKDIR /home/crab_mail/bin/

COPY --from=cargo-build /usr/src/crab_mail/target/x86_64-unknown-linux-musl/release/crab_mail .

RUN chown crab_mail:crab_mail crab_mail

USER crab_mail

CMD ["./crab_mail"]
