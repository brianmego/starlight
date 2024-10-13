mod handlers;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
mod error;
mod models;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use axum::extract::ws::Message;
use axum::routing::{delete, get, post};
use axum::Router;
use clap::Parser;
use surrealdb::engine::remote::ws::{Client as DbClient, Ws};
use surrealdb::Surreal;

static DB: Lazy<Surreal<DbClient>> = Lazy::new(Surreal::init);

type Error = error::Error;
type Result<T> = error::Result<T>;

/// Web server backend for the Girl Scout Starlight Service unit cookie scheduling site
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1912)]
    port: u16,
}

#[derive(Serialize)]
pub struct WsResponse {}

#[derive(Debug, Clone)]
pub struct Client {
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, axum::Error>>>,
    pub responses: Arc<RwLock<HashMap<String, mpsc::Sender<WsResponse>>>>,
}
impl Client {
    pub fn new(
        sender: Option<mpsc::UnboundedSender<std::result::Result<Message, axum::Error>>>,
        responses: Arc<RwLock<HashMap<String, mpsc::Sender<WsResponse>>>>,
    ) -> Self {
        Self { sender, responses }
    }
}

type Clients = Arc<RwLock<HashMap<String, Client>>>;

pub struct AppState {
    clients: Clients,
}

impl AppState {
    fn new(clients: Clients) -> Self {
        Self { clients }
    }
}


#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    // Connect to the database
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("scouts").use_db("scouts").await?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/ws", handlers::ws::on_connect);

    let clients: Clients = Clients::default();
    let shared_state = Arc::new(AppState::new(clients));
    let app = Router::new()
        .route("/status", get(handlers::status::handler))
        .route("/login", post(handlers::login::handler_post))
        .route("/api/location", get(handlers::location::handler_get))
        .route("/api/location", post(handlers::location::handler_post))
        .route("/api/location", delete(handlers::location::handler_delete))
        .route("/api/dayofweek", get(handlers::dayofweek::handler_get))
        .route("/api/timeslot", get(handlers::timeslot::handler_get))
        .route("/api/seed_data", get(handlers::seed_data::handler))
        .layer(layer)
        .with_state(shared_state)
        .layer(CorsLayer::new().allow_origin(Any));
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
