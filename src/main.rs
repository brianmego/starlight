mod error;
mod handlers;
mod models;
mod queries;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use axum::http::{header::AUTHORIZATION, Method};
use axum::routing::{get, post};
use axum::Router;
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

    let app = Router::new()
        .route("/status", get(handlers::status::handler))
        .route("/login", post(handlers::login::handler_post))
        .route(
            "/api/location",
            get(handlers::location::handler_get)
                .post(handlers::location::handler_post)
                .delete(handlers::location::handler_delete),
        )
        .route("/api/reservation", get(handlers::reservation::handler_get))
        .route(
            "/api/reservation/:id",
            get(handlers::reservation::handler_get_user_reservations)
                .delete(handlers::reservation::handler_delete_reservation)
                .post(handlers::reservation::handler_post),
        )
        .route("/api/user/:id", get(handlers::user::handler_get))
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
