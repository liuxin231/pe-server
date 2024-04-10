use axum::{routing::get, Router};

use crate::app::state::AppState;

pub mod hex;
pub mod param_convert;
pub mod pe_read;
pub mod pe_tools;

pub fn routers(state: AppState) -> Router {
    Router::new().nest(
        "/tools",
        Router::new()
            .route("/encode/:input_string", get(hex::encode))
            .route("/decode/:hex_string", get(hex::decode))
            .with_state(state),
    )
}
