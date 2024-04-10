use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use migration::sea_orm::ActiveValue::Set;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder,
};

use crate::app::response::{DataResponse, DefaultResponse, PaginateInfo, PaginateResponse};
use crate::app::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveParam {
    pub id: Option<String>,
    pub name: String,
    pub desc: Option<String>,
    pub is_sensitive: bool,
}
pub async fn save(app_state: State<AppState>, Json(param): Json<SaveParam>) -> impl IntoResponse {
    if param.name.is_empty() {
        return DefaultResponse::error().msg("方法名称不能为空!".to_string());
    }
    let active_model = match param.id {
        None => {
            let new_id = uuid::Uuid::new_v4().simple().to_string();
            entity::model::t_knowledge::ActiveModel {
                id: Set(new_id),
                func_name: Set(param.name),
                func_desc: Set(param.desc),
                is_sensitive: Set(param.is_sensitive),
                create_time: Set(chrono::Local::now().naive_local()),
                modify_time: Set(chrono::Local::now().naive_local()),
            }
        }
        Some(ref id) => {
            match entity::model::t_knowledge::Entity::find_by_id(id)
                .one(app_state.db_conn.as_ref())
                .await
            {
                Ok(data) => match data {
                    None => {
                        return DefaultResponse::error()
                            .msg("数据不存在, 请确认后再试!".to_string())
                    }
                    Some(data) => {
                        let mut active_model = data.into_active_model();
                        active_model.modify_time = Set(chrono::Local::now().naive_local());
                        active_model.func_name = Set(param.name);
                        active_model.func_desc = Set(param.desc);
                        active_model.is_sensitive = Set(param.is_sensitive);
                        active_model
                    }
                },
                Err(err) => {
                    log::error!("find knowledge by id error: {}", err.to_string());
                    return DefaultResponse::error().msg("数据查询错误, 请稍后再试!".to_string());
                }
            }
        }
    };
    let result = match param.id {
        None => active_model.insert(app_state.db_conn.as_ref()).await,
        Some(_) => active_model.update(app_state.db_conn.as_ref()).await,
    };
    match result {
        Ok(_) => DefaultResponse::success(),
        Err(err) => {
            log::error!("保存数据失败, error: {}", err.to_string());
            DefaultResponse::error().msg("保存数据失败, 请确认数据后重试!".to_string())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteParam {
    ids: Vec<String>,
}
pub async fn delete(
    app_state: State<AppState>,
    Json(param): Json<DeleteParam>,
) -> impl IntoResponse {
    if param.ids.is_empty() {
        return DefaultResponse::success();
    }
    let result = entity::model::t_knowledge::Entity::delete_many()
        .filter(entity::model::t_knowledge::Column::Id.is_in(param.ids))
        .exec(app_state.db_conn.as_ref())
        .await;
    match result {
        Ok(_) => DefaultResponse::success(),
        Err(err) => {
            log::error!("delete knowledge error: {}", err.to_string());
            DefaultResponse::error().msg("删除失败，请重试!".to_string())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PageListParam {
    page: u64,
    size: u64,
    name: Option<String>,
}
pub async fn page_list(
    app_state: State<AppState>,
    Query(param): Query<PageListParam>,
) -> impl IntoResponse {
    let mut select = entity::model::t_knowledge::Entity::find();
    match param.name {
        None => {}
        Some(name) => {
            select = select
                .filter(entity::model::t_knowledge::Column::FuncName.like(format!("%{}%", &name)));
        }
    }

    select = select.order_by_desc(entity::model::t_knowledge::Column::ModifyTime);
    let paginate = select.paginate(app_state.db_conn.as_ref(), param.size);
    let total = paginate.num_items().await.unwrap_or_else(|err| {
        log::error!("get knowledge total num error: {}", err.to_string());
        0
    });
    let pages = paginate.num_pages().await.unwrap_or(0);
    if total == 0 {
        return PaginateResponse::success(Vec::new(), PaginateInfo::default());
    }
    let data = paginate.fetch_page(param.page).await.unwrap_or_else(|err| {
        log::error!("find knowledge page list error: {}", err.to_string());
        vec![]
    });
    PaginateResponse::success(data, PaginateInfo { total, pages })
}

pub async fn info(app_state: State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match entity::model::t_knowledge::Entity::find_by_id(id)
        .one(app_state.db_conn.as_ref())
        .await
    {
        Ok(data) => match data {
            None => DefaultResponse::error()
                .msg("数据不存在, 请检查后重试!".to_string())
                .into_response(),
            Some(data) => DataResponse::success(data).into_response(),
        },
        Err(err) => {
            log::error!("find knowledge by id error: {}", err.to_string());
            DefaultResponse::error().into_response()
        }
    }
}
