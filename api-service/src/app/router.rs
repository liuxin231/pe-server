use crate::app::state::AppState;
use axum::Router;

pub fn get_all_routers(app_state: AppState) -> Router {
    let router = Router::new()
        .merge(crate::test::get_routers(app_state.clone()))
        .merge(crate::knowledge::get_routers(app_state.clone()))
        .merge(crate::file::get_routers(app_state.clone()))
        .merge(crate::pe::get_routers(app_state.clone()))
        .merge(crate::tools::routers(app_state.clone()));
    log::info!("Successfully obtained all routing information");
    router
}
