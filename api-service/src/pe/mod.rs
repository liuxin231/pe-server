use axum::routing::{get, post};
use axum::Router;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::app::state::AppState;
use crate::pe::pe_service::UploadFileInfo;

pub mod pe_service;

pub static UPLOAD_FILE_INFO: Lazy<Mutex<Option<UploadFileInfo>>> = Lazy::new(|| Mutex::new(None));
pub fn get_routers(app_state: AppState) -> Router {
    Router::new().nest(
        "/pe",
        Router::new()
            .route("/upload", post(pe_service::upload))
            .route("/update_file_byte", post(pe_service::update_file_byte))
            .route("/init_upload_file", post(pe_service::init_upload_file))
            .route(
                "/download_current_file",
                get(pe_service::download_current_file),
            )
            .route(
                "/get_upload_file_info",
                get(pe_service::get_upload_file_info),
            )
            .route("/analysis/:file_id", get(pe_service::analysis))
            .with_state(app_state),
    )
}
