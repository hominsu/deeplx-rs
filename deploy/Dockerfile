FROM rust:latest AS base

FROM base AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libclang-dev cmake &&  \
    rm -rf /var/lib/apt/lists/ && \
    apt-get autoremove -y &&  \
    apt-get autoclean -y

COPY . .

RUN RUSTFLAGS='--cfg reqwest_unstable -C target-feature=+crt-static' \
    CARGO_BUILD_TARGET=$(rustc -vV | grep host | awk '{print $2}')  \
    cargo install --path crates/dlx-svr --features=mimalloc

FROM scratch

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/dlx-svr /app/dlx-svr

EXPOSE 3000/tcp
VOLUME /data/conf

CMD ["./dlx-svr", "run", "--conf", "/data/conf"]
