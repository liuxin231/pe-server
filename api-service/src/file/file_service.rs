use crate::app::response::{DefaultResponse, PaginateInfo, PaginateResponse};
use crate::app::state::AppState;
use axum::extract::{Path, Query, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::Json;
use migration::sea_orm;
use migration::sea_orm::prelude::DateTime;
use migration::sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use migration::Condition;
use serde::{Deserialize, Serialize};

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
    #[derive(Debug, Clone, FromQueryResult, Serialize, Deserialize)]
    struct FileInfo {
        id: String,
        file_name: String,
        file_md5: String,
        has_report: bool,
        create_time: DateTime,
        modify_time: DateTime,
    }
    let mut select = entity::model::t_file::Entity::find()
        .select_only()
        .column(entity::model::t_file::Column::Id)
        .column(entity::model::t_file::Column::FileName)
        .column(entity::model::t_file::Column::FileMd5)
        .column_as(
            entity::model::t_file::Column::FileReport.is_not_null(),
            "has_report",
        )
        .column(entity::model::t_file::Column::CreateTime)
        .column(entity::model::t_file::Column::ModifyTime);
    match param.name {
        None => {}
        Some(name) => {
            select = select.filter(
                Condition::any()
                    .add(entity::model::t_file::Column::FileName.like(format!("%{}%", &name)))
                    .add(entity::model::t_file::Column::FileMd5.like(format!("%{}%", &name))),
            );
        }
    }

    select = select.order_by_desc(entity::model::t_file::Column::ModifyTime);
    let paginate = select
        .into_model::<FileInfo>()
        .paginate(app_state.db_conn.as_ref(), param.size);
    let total = paginate.num_items().await.unwrap_or_else(|err| {
        log::error!("get file total num error: {}", err.to_string());
        0
    });
    let pages = paginate.num_pages().await.unwrap_or(0);
    if total == 0 {
        return PaginateResponse::success(Vec::new(), PaginateInfo::default());
    }
    let data = paginate.fetch_page(param.page).await.unwrap_or_else(|err| {
        log::error!("find file page list error: {}", err.to_string());
        vec![]
    });
    PaginateResponse::success(data, PaginateInfo { total, pages })
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
    let result = entity::model::t_file::Entity::delete_many()
        .filter(entity::model::t_file::Column::Id.is_in(param.ids))
        .exec(app_state.db_conn.as_ref())
        .await;
    match result {
        Ok(_) => DefaultResponse::success(),
        Err(err) => {
            log::error!("delete file error: {}", err.to_string());
            DefaultResponse::error().msg("删除失败，请重试!".to_string())
        }
    }
}

pub async fn download_file(
    app_state: State<AppState>,
    Path(file_id): Path<String>,
) -> impl IntoResponse {
    let file_model = match entity::model::t_file::Entity::find_by_id(&file_id)
        .one(app_state.db_conn.as_ref())
        .await
    {
        Ok(data) => match data {
            None => {
                return DefaultResponse::error()
                    .msg("文件为空，请重试!".to_string())
                    .into_response();
            }
            Some(data) => data,
        },
        Err(err) => {
            log::error!("find file by id error: {} [{}]", err.to_string(), file_id);
            return DefaultResponse::error()
                .msg("文件查找失败，请重试!".to_string())
                .into_response();
        }
    };
    let file_name = file_model.file_name;
    let file_buf = file_model.file_buf;
    let attachment = &format!("attachment; filename={}", file_name);
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(attachment).unwrap(),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/octet-stream").unwrap(),
    );

    (headers, file_buf).into_response()
}

pub async fn download_report(
    app_state: State<AppState>,
    Path(file_id): Path<String>,
) -> impl IntoResponse {
    let file_model = match entity::model::t_file::Entity::find_by_id(&file_id)
        .one(app_state.db_conn.as_ref())
        .await
    {
        Ok(data) => match data {
            None => {
                return DefaultResponse::error()
                    .msg("文件为空，请重试!".to_string())
                    .into_response();
            }
            Some(data) => data,
        },
        Err(err) => {
            log::error!("find file by id error: {} [{}]", err.to_string(), file_id);
            return DefaultResponse::error()
                .msg("文件查找失败，请重试!".to_string())
                .into_response();
        }
    };
    let file_name = "report.txt";
    let file_buf = file_model.file_report.unwrap_or(Vec::new());
    let attachment = &format!("attachment; filename={}", file_name);
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(attachment).unwrap(),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/octet-stream").unwrap(),
    );

    (headers, file_buf).into_response()
}
