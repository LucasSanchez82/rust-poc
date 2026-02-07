use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub connection: Arc<DatabaseConnection>,
}

pub struct ConnectionState {
    pub connection: DatabaseConnection,
}
