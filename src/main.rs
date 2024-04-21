mod handlers;
use env_logger::Env;
use log::{error, info, warn};
use std::net::SocketAddr;

use axum::routing::get;
use clap::Parser;
use axum::Router;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;


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
    let app = Router::new()
        .route("/status", get(handlers::status::handler))
        .route("/api/location", get(handlers::location::handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
// #[tokio::main]
// async fn main() -> surrealdb::Result<()> {
//     // Connect to the server
//     let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

//     // Signin as a namespace, database, or root user
//     db.signin(Root {
//         username: "root",
//         password: "root",
//     })
//     .await?;

//     // Select a specific namespace / database
//     db.use_ns("test").use_db("test").await?;

//     // Create a new person with a random id
//     let created: Vec<Record> = db
//         .create("person")
//         .content(Person {
//             title: "Founder & CEO",
//             name: Name {
//                 first: "Tobie",
//                 last: "Morgan Hitchcock",
//             },
//             marketing: true,
//         })
//     .await?;
//     dbg!(created);

//     // Update a person record with a specific id
//     let updated: Option<Record> = db
//         .update(("person", "jaime"))
//         .merge(Responsibility { marketing: true })
//         .await?;
//     dbg!(updated);

//     // Select all people records
//     let people: Vec<Record> = db.select("person").await?;
//     dbg!(people);

//     // Perform a custom advanced query
//     let groups = db
//         .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
//         .bind(("table", "person"))
//         .await?;
//     dbg!(groups);

//     Ok(())
// }









#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
