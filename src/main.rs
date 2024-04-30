mod handlers;
mod models;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

/// Web server backend for the Girl Scout Starlight Service unit cookie scheduling site
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1912)]
    port: u16,
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    // Connect to the database
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("scouts").use_db("scouts").await?;

    let app = Router::new()
        .route("/status", get(handlers::status::handler))
        .route("/api/location", get(handlers::location::handler_get))
        .route("/api/location", post(handlers::location::handler_post))
        .route("/api/seed_data", get(handlers::seed_data::handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
