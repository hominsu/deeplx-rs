use std::future::{Future, IntoFuture};
use std::io;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;

use config::{ConfigError, File};
use glob::glob;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub debug: bool,
    pub bind: SocketAddr,
    pub concurrent: u32,
    pub proxy: Option<String>,
    pub auth: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            bind: "0.0.0.0:3000".parse().unwrap(),
            concurrent: 1024,
            proxy: None,
            auth: None,
        }
    }
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let pattern = Path::new(path).join("*").to_string_lossy().into_owned();

        let glob_entries = glob(&pattern)
            .map_err(|e| ConfigError::Message(format!("Invalid glob pattern: {}", e)))?;

        config::Config::builder()
            .add_source(
                glob_entries
                    .filter_map(|entry| match entry {
                        Ok(path) => match path.extension() {
                            Some(ext) => {
                                let ext_str = ext.to_str()?;
                                if matches!(
                                    ext_str,
                                    "ini" | "json" | "yaml" | "toml" | "ron" | "json5"
                                ) {
                                    Some(File::from(path))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        },
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>(),
            )
            .build()?
            .try_deserialize()
    }
}

pub struct Manager {
    config: Arc<RwLock<Config>>,
    path: String,
}

impl Manager {
    pub fn new(path: &str) -> Self {
        let config = Config::new(path)
            .unwrap_or_else(|e| panic!("Failed to load config from '{}': {}", path, e));
        Manager {
            config: Arc::new(RwLock::new(config)),
            path: path.to_string(),
        }
    }

    pub fn with_watcher<F>(&self, signal: F) -> WithWatcher<F>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        WithWatcher {
            config: Arc::clone(&self.config),
            path: self.path.clone(),
            signal,
        }
    }

    #[allow(dead_code)]
    pub fn config(&self) -> Arc<RwLock<Config>> {
        Arc::clone(&self.config)
    }
}

impl IntoFuture for Manager {
    type Output = io::Result<()>;
    type IntoFuture = private::ManagerFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.with_watcher(std::future::pending()).into_future()
    }
}

pub struct WithWatcher<F> {
    config: Arc<RwLock<Config>>,
    path: String,
    signal: F,
}

impl<F> WithWatcher<F> {
    #[allow(dead_code)]
    pub fn config(&self) -> Arc<RwLock<Config>> {
        Arc::clone(&self.config)
    }
}

impl<F> IntoFuture for WithWatcher<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    type Output = io::Result<()>;
    type IntoFuture = private::ManagerFuture;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            config,
            path,
            signal,
        } = self;

        private::ManagerFuture(Box::pin(async move {
            let (tx, rx) = mpsc::channel();

            let mut watcher: RecommendedWatcher = Watcher::new(
                tx,
                notify::Config::default().with_poll_interval(Duration::from_secs(2)),
            )
            .expect("Failed to create file watcher");

            watcher
                .watch(Path::new(path.as_str()), RecursiveMode::NonRecursive)
                .unwrap_or_else(|e| panic!("Failed to watch config directory '{}': {}", path, e));

            let config_clone = Arc::clone(&config);
            let path_clone = path.clone();
            let task = tokio::task::spawn_blocking(move || {
                loop {
                    match rx.recv() {
                        Ok(Ok(Event {
                            kind: notify::EventKind::Modify(_),
                            ..
                        })) => match Config::new(path_clone.as_str()) {
                            Ok(new_config) => match config_clone.write() {
                                Ok(mut guard) => {
                                    *guard = new_config;
                                    tracing::info!("Config reloaded successfully");
                                }
                                Err(e) => {
                                    tracing::error!("Config lock poisoned, cannot reload: {}", e);
                                }
                            },
                            Err(e) => {
                                tracing::error!("Failed to reload config: {}", e);
                            }
                        },
                        Err(_) => break,
                        _ => {}
                    }
                }
            });

            tokio::pin!(signal, task);

            tokio::select! {
                _ = &mut signal => {
                    tracing::trace!("received graceful shutdown signal. Stopping watcher");
                    drop(watcher);
                    let _ = task.await;
                    Ok(())
                },
                res = &mut task => {
                    match res {
                        Ok(_) => Ok(()),
                        Err(err) => Err(io::Error::other(format!("watcher task failed: {err}"))),
                    }
                },
            }
        }))
    }
}

mod private {
    use std::{
        boxed::Box,
        future::Future,
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pub struct ManagerFuture(pub Pin<Box<dyn Future<Output = io::Result<()>> + Send>>);

    impl Future for ManagerFuture {
        type Output = io::Result<()>;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.0.as_mut().poll(cx)
        }
    }
}
