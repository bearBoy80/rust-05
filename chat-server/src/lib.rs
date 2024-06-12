mod config;
mod error;
mod handler;
mod models;
use anyhow::Context;
use chat_core::{DecodingKey, EncodingKey};
pub use config::*;
use error::AppError;
use sqlx::PgPool;
use std::{fmt::Debug, ops::Deref, sync::Arc};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}
pub struct AppStateInner {
    pub config: AppCfg,
    pub dk: DecodingKey,
    pub ek: EncodingKey,
    pub pool: PgPool,
}
impl AppState {
    pub async fn try_new(config: AppCfg) -> anyhow::Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir).await?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let pool = PgPool::connect(&config.server.db_url).await?;
        let inner = AppStateInner {
            config,
            dk,
            ek,
            pool,
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }
}
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
