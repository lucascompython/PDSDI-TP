FROM rust:slim AS build-env
WORKDIR /app

COPY . /app
COPY backend/certs /app/backend/certs

RUN apt-get update && apt-get upgrade -y && apt-get install -y clang && rustup default nightly

ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo -C backend build --release --target x86_64-unknown-linux-gnu -Z unstable-options --features docker

FROM gcr.io/distroless/static-debian12
COPY --from=build-env /app/backend/target/x86_64-unknown-linux-gnu/release/backend /
CMD ["./backend"]