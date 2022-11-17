use std::time::{Duration, SystemTime};
use axum::{
    extract::Path,
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server, TypedHeader,
};
use headers::{ContentType, Expires};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


use crate::config::Config;
use heos_axum::http;
use heos_axum::config;
use heos_axum::axum_ructe::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "heos_axum=debug,heos_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::debug!("listening ");
    let diver = heos_api::HeosDriver::new("192.168.178.34:1255").await?;
    println!("Got Driver");
    http::serve(Config, diver).await?;
    Ok(())
}
