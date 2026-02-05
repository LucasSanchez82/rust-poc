use sea_orm::DatabaseConnection;

pub struct ConnectionState {
    pub connection: DatabaseConnection,
}
