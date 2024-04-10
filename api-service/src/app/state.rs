use std::sync::Arc;

use migration::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_conn: Arc<DatabaseConnection>,
}
