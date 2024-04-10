use crate::app::state::AppState;
use axum::routing::{get, post};
use axum::Router;

mod file_service;
pub fn get_routers(app_state: AppState) -> Router {
    Router::new().nest(
        "/file",
        Router::new()
            .route("/delete", post(file_service::delete))
            .route("/page_list", get(file_service::page_list))
            .route("/download_file/:file_id", get(file_service::download_file))
            .route(
                "/download_report/:file_id",
                get(file_service::download_report),
            )
            .with_state(app_state),
    )
}
