use axum::{routing::get, Router};
use chat_server::AppCfg;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = AppCfg::load()?;
    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    info!("Starting server on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
