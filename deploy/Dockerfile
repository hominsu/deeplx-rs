FROM rust:latest AS base

FROM base AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libclang-dev cmake &&  \
    rm -rf /var/lib/apt/lists/ && \
    apt-get autoremove -y &&  \
    apt-get autoclean -y

COPY . .

RUN RUSTFLAGS='-C target-feature=+crt-static' \
    CARGO_BUILD_TARGET=$(rustc -vV | grep host | awk '{print $2}')  \
    cargo install --path . --features=impersonate,mimalloc

FROM scratch

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/deeplx /app/deeplx

EXPOSE 3000/tcp
VOLUME /data/conf

CMD ["./deeplx", "run", "--conf", "/data/conf"]
