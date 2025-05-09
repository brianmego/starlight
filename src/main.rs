mod error;
mod handlers;
mod models;
mod queries;
use chrono::{TimeZone, prelude::*};
use chrono_tz::America::Chicago;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use axum::Router;
use axum::http::{Method, header::AUTHORIZATION};
use axum::routing::{get, post};
use clap::Parser;
use surrealdb::Surreal;
use surrealdb::{
    engine::remote::ws::{Client as DbClient, Ws},
    opt::auth::Root,
};

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

#[derive(Clone)]
pub struct AppState {
    time_offset: i64,
}

impl AppState {
    pub fn new() -> Self {
        let now = std::env::var("NOW");
        let time_offset = match now {
            Ok(d) => {
                let desired_now = Chicago
                    .from_local_datetime(
                        &NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
                    .single()
                    .unwrap();
                (desired_now - Utc::now().with_timezone(&Chicago)).num_seconds()
            }
            Err(_) => 0,
        };
        Self { time_offset }
    }
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let db_host = std::env::var("DB_HOST").unwrap_or("localhost:8000".into());
    let db_namespace = std::env::var("DB_NAMESPACE").unwrap_or("scouts".into());
    let db_database = std::env::var("DB_DATABASE").unwrap_or("scouts".into());
    let db_user = std::env::var("DB_USER").unwrap_or("root".into());
    let db_pwd = std::env::var("DB_PASS").unwrap_or("root".into());

    // Connect to the database
    DB.connect::<Ws>(db_host).await?;
    DB.use_ns(db_namespace).use_db(db_database).await?;
    DB.signin(Root {
        username: &db_user,
        password: &db_pwd,
    })
    .await?;

    let shared_state = AppState::new();
    let app = Router::new()
        .route("/status", get(handlers::status::handler))
        .route("/login", post(handlers::login::handler_post))
        .route("/api/location", get(handlers::location::handler_get))
        .route("/api/reservation", get(handlers::reservation::handler_get))
        .route(
            "/api/reservation/{id}",
            get(handlers::reservation::handler_get_user_reservations)
                .delete(handlers::reservation::handler_delete_reservation)
                .post(handlers::reservation::handler_post),
        )
        .route(
            "/api/reservation/swap/{old_id}/{new_id}",
            post(handlers::reservation::handler_swap_reservations),
        )
        .route(
            "/api/reservation/reserveswap/{id}",
            post(handlers::reservation::handler_reserve_swap),
        )
        .route("/api/user/{id}", get(handlers::user::handler_get))
        .route("/api/history", get(handlers::history::handler_get))
        .with_state(shared_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
                .allow_headers([AUTHORIZATION]),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
