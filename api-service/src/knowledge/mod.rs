use axum::routing::{get, post};
use axum::Router;

use crate::app::state::AppState;

pub mod knowledge_service;

pub fn get_routers(app_state: AppState) -> Router {
    Router::new().nest(
        "/knowledge",
        Router::new()
            .route("/save", post(knowledge_service::save))
            .route("/delete", post(knowledge_service::delete))
            .route("/page_list", get(knowledge_service::page_list))
            .route("/info/:id", get(knowledge_service::info))
            .with_state(app_state),
    )
}
