use anyhow::Error;
use anyhow::Result;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::modules::user::route::user_router;
use crate::utils::cfg::Config;

pub mod modules;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use migration::{Migrator, MigratorTrait};

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = Config::new();

    info!("Trying to connect to database... [{}]", config.database_url);
    let connection = sea_orm::Database::connect(config.database_url).await?;

    Migrator::up(&connection, None).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/users", user_router())
        .with_state(connection)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let target = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&target).await.unwrap();

    info!("The server is running on http://{}", target);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
