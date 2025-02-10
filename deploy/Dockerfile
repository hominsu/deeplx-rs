FROM rust:latest AS base

RUN cargo install sccache && \
    cargo install cargo-chef


FROM base AS planner

WORKDIR /app
COPY . .

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccach

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=${SCCACHE_DIR},sharing=locked \
    cargo chef prepare --recipe-path recipe.json


FROM base AS builder

WORKDIR /app

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccach

RUN apt-get update && \
    apt-get install -y --no-install-recommends libclang-dev cmake &&  \
    rm -rf /var/lib/apt/lists/ && \
    apt-get autoremove -y &&  \
    apt-get autoclean -y

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=${SCCACHE_DIR},sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=${SCCACHE_DIR},sharing=locked \
    RUSTFLAGS='-C target-feature=+crt-static' \
    CARGO_BUILD_TARGET=$(rustc -vV | grep host | awk '{print $2}')  \
    cargo install --path . --features=impersonate

FROM scratch

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/deeplx /app/deeplx

EXPOSE 3000/tcp
VOLUME /data/conf

CMD ["./deeplx", "run", "--conf", "/data/conf"]
