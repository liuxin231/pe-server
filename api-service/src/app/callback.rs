use axum::http::Uri;
use axum::response::{IntoResponse, Response};

use crate::app::response::DefaultResponse;

pub struct Callback;
impl Callback {
    pub async fn setup() {
        log::info!("Service started successfully");
    }
    pub async fn fallback(uri: Uri) -> Response {
        DefaultResponse::error()
            .code(404)
            .msg(format!("请求不存在: {uri}"))
            .into_response()
    }
}
