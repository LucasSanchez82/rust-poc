use std::sync::Arc;

use anyhow::Error;
use anyhow::Result;
use axum::routing::post;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::modules::auth::route::handle_login;
use crate::modules::states::AppState;
use crate::modules::user::route::user_router;
use crate::utils::cfg::Config;

pub mod modules;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use migration::{Migrator, MigratorTrait};

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::new();

    info!("Trying to connect to database...");
    let connection = Arc::new(sea_orm::Database::connect(config.database_url).await?);
    Migrator::up(connection.as_ref(), None).await?;

    let app_state = AppState { connection };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(handle_login))
        .nest("/users", user_router())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let target = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&target).await.unwrap();

    info!("The server is running on http://{}", target);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
