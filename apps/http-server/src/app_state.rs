use sea_orm::DatabaseConnection;

pub struct AppState {
    pub connection: DatabaseConnection,
}
