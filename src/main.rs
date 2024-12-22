mod server;

use clap::Parser;
use deeplx::{Config, DeepLX};
use mimalloc::MiMalloc;
use server::{biz, conf, data::translate::TranslateRepo, pkgs::exit::shutdown_signal, routes};
use std::{future::IntoFuture, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser)]
#[clap(name = "deeplx", version, about)]
struct Args {
    #[clap(short, long, default_value = "./configs")]
    conf: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = conf::manager(args.conf.as_str()).with_watcher(shutdown_signal());
    let config = manager.config();
    let manager_fut = manager.into_future();

    let translator = Arc::new(DeepLX::new(Config {
        proxy: config.read().unwrap().proxy.clone(),
        ..Config::default()
    }));
    let translate_repo = Arc::new(TranslateRepo::new(translator.clone()));
    let translate_usecase = Arc::new(biz::translate::TranslateUsecase::new(
        translate_repo.clone(),
    ));
    let state = routes::AppState {
        translate_uc: translate_usecase,
        config: config.clone(),
    };
    let app = routes::router(state).layer(TraceLayer::new_for_http());

    let addr = config.read().unwrap().addr.clone();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let serve_fut = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .into_future();

    let _ = tokio::join!(serve_fut, manager_fut);
}
