use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http_body_util::BodyExt;

use crate::app::response::DefaultResponse;

pub async fn handle_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    if parts.status != StatusCode::OK {
        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(err) => {
                return Ok(DefaultResponse::error()
                    .code(StatusCode::BAD_REQUEST.as_u16() as u32)
                    .msg(format!("{err}"))
                    .into_response());
            }
        };
        let response_body = match std::str::from_utf8(&bytes) {
            Ok(data) => data,
            Err(err) => {
                return Ok(DefaultResponse::error()
                    .code(StatusCode::BAD_REQUEST.as_u16() as u32)
                    .msg(err.to_string())
                    .into_response())
            }
        };
        Ok(DefaultResponse::error()
            .code(parts.status.as_u16() as u32)
            .msg(response_body.to_string())
            .into_response())
    } else {
        let response = Response::from_parts(parts, body);
        Ok(response)
    }
}
