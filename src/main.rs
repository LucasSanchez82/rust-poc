use anyhow::Error;
use anyhow::Result;
use axum::routing::MethodRouter;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::instrument::WithSubscriber;

use crate::modules::states::ConnectionState;
use crate::modules::user::route::user_router;

pub mod modules;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // build our application with a single routew-*+
    use migration::{Migrator, MigratorTrait};

    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global subscriber for this application.
        .init();

    let database_url = "postgres://postgres:password@localhost:5432/database".to_owned();
    info!("Trying to connect to database...");
    let connection = sea_orm::Database::connect(database_url).await?;
    Migrator::up(&connection, None).await?;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/users", user_router())
        .with_state(connection)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let target = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(target).await.unwrap();

    info!("The server is running on http://{}", target);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
