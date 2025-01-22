mod biz;
mod conf;
mod data;
mod pkgs;
mod routes;

use crate::{Bootstrap, Result};

use deeplx::{Config, DeepLX};
use pkgs::exit::shutdown_signal;
use std::{future::IntoFuture, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "rpmalloc")]
#[global_allocator]
static GLOBAL: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

#[cfg(feature = "snmalloc")]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[cfg(feature = "tikv-jemallocator")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn run(args: Bootstrap) -> Result<()> {
    let manager = conf::manager(args.conf.as_str()).with_watcher(shutdown_signal());
    let config = manager.config();
    let conf::Config {
        debug,
        bind,
        concurrent,
        ..
    } = config.read().unwrap().clone();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}={}",
                    env!("CARGO_CRATE_NAME"),
                    if debug { "debug" } else { "info" }
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(concurrent)
        .build()?;

    runtime.block_on(async move {
        let manager_fut = manager.into_future();

        let translator = Arc::new(DeepLX::new(Config {
            proxy: config.read().unwrap().proxy.clone(),
            ..Config::default()
        }));
        let translate_repo = Arc::new(data::translate::TranslateRepo::new(translator.clone()));
        let translate_usecase = Arc::new(biz::translate::TranslateUsecase::new(
            translate_repo.clone(),
        ));
        let state = routes::AppState {
            translate_uc: translate_usecase,
            config: config.clone(),
        };

        let app = routes::router(state).layer(TraceLayer::new_for_http());
        let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

        tracing::debug!("listening on {}", listener.local_addr().unwrap());

        let serve_fut = axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .into_future();

        let _ = tokio::join!(serve_fut, manager_fut);
    });

    Ok(())
}
