use axum::{extract::Path, response::IntoResponse};

use crate::app::response;
use crate::app::response::DataResponse;

extern crate hex;

pub async fn encode(Path(input_string): Path<String>) -> impl IntoResponse {
    let hex_string = hex::encode(input_string);
    DataResponse::success(hex_string).into_response()
}
pub async fn decode(Path(hex_string): Path<String>) -> impl IntoResponse {
    let output = hex::decode(hex_string);
    match output {
        Ok(data) => match String::from_utf8(data) {
            Ok(s) => DataResponse::success(s).into_response(),
            Err(e) => response::err(e.to_string()).into_response(),
        },
        Err(err) => response::err(err.to_string()).into_response(),
    }
}
