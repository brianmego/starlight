use surrealdb::opt::auth::Scope;

use crate::{
    models::{location::Location, user::User},
    DB,
};

pub async fn handler() -> &'static str {
    let _ = create_data().await;
    "Ok"
}

async fn create_data() -> surrealdb::Result<()> {
    // create_locations().await?;
    create_users().await?;
    Ok(())
}

// async fn create_locations() -> surrealdb::Result<()> {
//     let location: Result<Option<Location>, surrealdb::Error> = DB
//         .create(("location", "chuy's"))
//         .content(Location::new("Chuy's"))
//         .await;
//     match location {
//         Ok(l) => println!("Created {l:?}"),
//         Err(e) => println!("Could not create: {e}"),
//     };
//     Ok(())
// }

async fn create_users() -> surrealdb::Result<()> {
    let _: Result<surrealdb::opt::auth::Jwt, surrealdb::Error> = DB
        .signup(Scope {
            namespace: "scouts",
            database: "scouts",
            scope: "user",
            params: User::new("Brian", "abc123"),
        })
        .await;
    let _: Result<surrealdb::opt::auth::Jwt, surrealdb::Error> = DB
        .signup(Scope {
            namespace: "scouts",
            database: "scouts",
            scope: "loser",
            params: User::new("Strian", "abc123"),
        })
        .await;
    Ok(())
}
