use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DefaultResponse {
    code: u32,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, ToSchema)]
pub struct PaginateInfo {
    pub total: u64,
    pub pages: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DataResponse<T>
where
    T: Clone + serde::Serialize,
{
    code: u32,
    message: Option<String>,
    data: T,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginateResponse<T>
where
    T: Clone + serde::Serialize,
{
    code: u32,
    message: Option<String>,
    paginate: PaginateInfo,
    data: Vec<T>,
}

impl DefaultResponse {
    pub fn success() -> Self {
        DefaultResponse {
            code: 0,
            message: None,
        }
    }
    pub fn error() -> Self {
        DefaultResponse {
            code: 500,
            message: Some("操作失败".to_string()),
        }
    }
    pub fn msg(mut self, msg: String) -> Self {
        self.message = Some(msg);
        self
    }
    pub fn code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }
}

#[allow(unused)]
impl<T> DataResponse<T>
where
    T: Clone + serde::Serialize,
{
    pub fn success(data: T) -> Self {
        DataResponse {
            code: 0,
            message: None,
            data,
        }
    }
    pub fn msg(mut self, msg: String) -> Self {
        self.message = Some(msg);
        self
    }
    pub fn code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }
}
#[allow(unused)]
impl<T> PaginateResponse<T>
where
    T: Clone + serde::Serialize,
{
    pub fn success(data: Vec<T>, paginate_info: PaginateInfo) -> Self {
        PaginateResponse {
            code: 0,
            message: None,
            paginate: paginate_info,
            data,
        }
    }
    pub fn msg(mut self, msg: String) -> Self {
        self.message = Some(msg);
        self
    }
    pub fn code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }
    pub fn paginate(mut self, paginate_info: PaginateInfo) -> Self {
        self.paginate = paginate_info;
        self
    }
    pub fn total(mut self, total: u64) -> Self {
        self.paginate.total = total;
        self
    }
    pub fn pages(mut self, pages: u64) -> Self {
        self.paginate.pages = pages;
        self
    }
    pub fn data(mut self, data: Vec<T>) -> Self {
        self.data = data;
        self
    }
}
impl IntoResponse for DefaultResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
impl<T> IntoResponse for DataResponse<T>
where
    T: Clone + serde::Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
impl<T> IntoResponse for PaginateResponse<T>
where
    T: Clone + serde::Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

impl<T> std::default::Default for PaginateResponse<T>
where
    T: Clone + serde::Serialize,
{
    fn default() -> Self {
        Self {
            code: 0,
            message: None,
            paginate: Default::default(),
            data: Vec::<T>::new(),
        }
    }
}
#[allow(dead_code)]
pub fn ok_msg(msg: &str) -> Response {
    (StatusCode::OK, [("code", "200"), ("msg", msg)]).into_response()
}

// 响应错误
pub fn err(errmsg: String) -> Response {
    (StatusCode::OK, [("code", "500"), ("msg", errmsg.as_str())]).into_response()
}

#[allow(dead_code)]
pub fn entity_ok(data: String) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("code", "200"), ("msg", &"ok")],
        Json(data),
    )
        .into_response()
}
