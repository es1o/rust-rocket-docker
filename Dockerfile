FROM rust:1.62.0 as builder

RUN mkdir /build

WORKDIR /build

COPY Cargo.lock /build
COPY Cargo.toml /build

COPY src/ /build/src/

RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /app

COPY Rocket.toml /app

COPY --from=builder --chown=nonroot:nonroot /build/target/release/rocket-test /app

USER nonroot

EXPOSE 8000

CMD ["/app/rocket-test"]