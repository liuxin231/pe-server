use axum::extract::{Multipart, Path, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::Json;
use byte_unit::{Byte, Unit, UnitType};
use entity::model::{t_file, t_knowledge};
use migration::sea_orm::ColumnTrait;
use migration::sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};
use migration::{Expr, Value};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

use crate::app::response::{DataResponse, DefaultResponse};
use crate::app::state::AppState;
use crate::pe::UPLOAD_FILE_INFO;
use crate::tools::{self, pe_tools};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadFileInfo {
    pub file_id: String,
    pub file_name: String,
    pub file_size: String,
    pub address_group_information: Vec<AddressGroupInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressGroupInformation {
    pub address: String,
    pub bytes: Vec<ByteInformation>,
    pub translation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByteInformation {
    pub index: u64,
    pub bytes: String,
}

pub async fn upload(app_state: State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let filed = match multipart.next_field().await {
        Ok(data) => match data {
            None => {
                return DefaultResponse::error()
                    .msg("文件不能为空!".to_string())
                    .into_response()
            }
            Some(data) => data,
        },
        Err(err) => {
            return DefaultResponse::error()
                .msg(err.to_string())
                .into_response()
        }
    };
    let file_name = match filed.file_name() {
        None => {
            return DefaultResponse::error()
                .msg("文件名不能为空!".to_string())
                .into_response()
        }
        Some(data) => data.to_string(),
    };
    let file_bytes = match filed.bytes().await {
        Ok(data) => {
            if data.is_empty() {
                return DefaultResponse::error()
                    .msg("文件不合法, 请检查文件后重试!".to_string())
                    .into_response();
            }
            data
        }
        Err(err) => {
            return DefaultResponse::error()
                .msg(err.to_string())
                .into_response()
        }
    };
    let file_id = snowflake_rs::SnowFlakeId::new(1, snowflake_rs::STANDARD_EPOCH)
        .generate_id()
        .unwrap()
        .to_string();
    let file_md5 = format!("{:x}", md5::compute(&file_bytes));
    let file_active_model = entity::model::t_file::ActiveModel {
        id: Set(file_id.clone()),
        file_name: Set(file_name.clone()),
        file_md5: Set(file_md5),
        file_buf: Set(file_bytes.to_vec()),
        file_report: Default::default(),
        create_time: Set(chrono::Local::now().naive_local()),
        modify_time: Set(chrono::Local::now().naive_local()),
    };
    match file_active_model.insert(app_state.db_conn.as_ref()).await {
        Ok(_) => {
            let mut upload_file_info = UPLOAD_FILE_INFO.lock().await;
            *upload_file_info = Some(UploadFileInfo {
                file_id,
                file_name,
                file_size: Byte::from_f64_with_unit(file_bytes.len() as f64, Unit::B)
                    .unwrap()
                    .get_appropriate_unit(UnitType::Decimal)
                    .to_string(),
                address_group_information: get_address_group_information_from_file_buf(
                    file_bytes.to_vec(),
                ),
            });
            DefaultResponse::success().into_response()
        }
        Err(err) => {
            log::error!("save file error: {}", err.to_string());
            DefaultResponse::error()
                .msg("保存文件失败，请检查后重试！".to_string())
                .into_response()
        }
    }
}

fn get_address_group_information_from_file_buf(file_buf: Vec<u8>) -> Vec<AddressGroupInformation> {
    let address_group_num = (file_buf.len() as f64 / 16f64).ceil() as u64;
    let mut information_list = (1..address_group_num)
        .into_par_iter()
        .map(|index| {
            let end_byte_index = index * 16 - 1;
            let start_byte_index = end_byte_index - 15;
            let bytes_seq = file_buf
                .iter()
                .enumerate()
                .filter(|(index, _item)| {
                    index <= &(end_byte_index as usize) && index >= &(start_byte_index as usize)
                })
                .collect::<Vec<_>>();
            let byte_information = bytes_seq
                .iter()
                .map(|(index, item)| ByteInformation {
                    index: *index as u64,
                    bytes: format!("{:02X}", item),
                })
                .collect::<Vec<_>>();
            let translation = bytes_seq
                .iter()
                .map(|(_index, item)| {
                    if **item >= 33u8 && **item <= 126u8 {
                        String::from_utf8(vec![**item]).unwrap_or_else(|_| String::from("."))
                    } else {
                        String::from(".")
                    }
                })
                .collect::<Vec<_>>();
            AddressGroupInformation {
                address: format!("{:08X}", end_byte_index - 15),
                bytes: byte_information,
                translation: translation.join(""),
            }
        })
        .collect::<Vec<_>>();
    information_list.sort_by(|a, b| a.address.cmp(&b.address));
    information_list
}

pub async fn analysis(
    app_state: State<AppState>,
    Path(file_id): Path<String>,
) -> impl IntoResponse {
    let select = t_file::Entity::find().filter(t_file::Column::Id.eq(file_id));
    // log::info!("{}", select.build(DbBackend::Postgres).to_string());
    let file = select.one(app_state.db_conn.as_ref()).await.unwrap();
    let mut file_buf = Vec::new();
    let mut file_name = "".to_string();
    let mut id = "".to_string();
    if let Some(model) = file {
        file_buf = model.file_buf;
        file_name = model.file_name;
        id = model.id;
    }
    let file_size = Byte::from_f64_with_unit(file_buf.len() as f64, Unit::B)
        .unwrap()
        .get_appropriate_unit(UnitType::Decimal)
        .to_string();
    let pe_study = match tools::pe_read::read_exe_file(hex::encode(file_buf), file_name, file_size)
    {
        Ok(data) => data,
        Err(err) => {
            return DefaultResponse::error()
                .msg(err.to_string())
                .into_response()
        }
    };
    let table_byname = &pe_study.byname_information;
    let knowledge = t_knowledge::Entity::find()
        .all(app_state.db_conn.as_ref())
        .await
        .unwrap_or_else(|err| {
            log::error!("get knowledge error: {}", err.to_string());
            Vec::new()
        });
    let knowledge_map: HashMap<String, String> = knowledge
        .iter()
        .map(|entity| {
            (
                pe_tools::vec_to_string(entity.func_name.clone().as_bytes()),
                format!(
                    "{}:{}",
                    entity.func_name.clone(),
                    entity
                        .func_desc
                        .as_ref()
                        .map_or_else(String::new, |desc| desc.clone()),
                ),
            )
        })
        .collect();
    let mut error_message: Vec<String> = Vec::new();
    for element in table_byname {
        'map_for: for (key, value) in &knowledge_map {
            if pe_tools::fuzzy_search(element, key) && element.len() >= 6 {
                error_message.push(value.to_string());
            } else {
                continue 'map_for;
            }
        }
    }
    if !error_message.is_empty() {
        let msg = format!(
            "该可执行程序运行可能会尝试调用{}个系统函数，可能会对计算机造成损害。分别为：{:?}",
            error_message.len(),
            &error_message
        );
        let res = t_file::Entity::update_many()
            .col_expr(
                t_file::Column::FileReport,
                Expr::value(Value::Bytes(Some(Box::new(
                    pe_study.generate_report(msg.clone()).as_bytes().to_vec(),
                )))),
            )
            .filter(t_file::Column::Id.eq(id))
            .exec(app_state.db_conn.as_ref())
            .await;
        if res.is_err() {
            log::error!("{:?}", res);
        };
        DefaultResponse::success().msg(msg).into_response()
    } else {
        let msg = "未检测到异常".to_string();
        let res = t_file::Entity::update_many()
            .col_expr(
                t_file::Column::FileReport,
                Expr::value(Value::Bytes(Some(Box::new(
                    pe_study.generate_report(msg.clone()).as_bytes().to_vec(),
                )))),
            )
            .filter(t_file::Column::Id.eq(id))
            .exec(app_state.db_conn.as_ref())
            .await;
        if res.is_err() {
            log::error!("{:?}", res);
        };
        DefaultResponse::success().msg(msg).into_response()
    }
}

pub async fn get_upload_file_info() -> impl IntoResponse {
    let upload_file_info = UPLOAD_FILE_INFO.lock().await;
    let upload_file_info = upload_file_info.clone();
    DataResponse::success(upload_file_info).into_response()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateByteParam {
    pub index: String,
    pub byte: String,
}
pub async fn update_file_byte(Json(param): Json<UpdateByteParam>) -> impl IntoResponse {
    let mut file_info = UPLOAD_FILE_INFO.lock().await;
    if file_info.is_none() {
        return DefaultResponse::error()
            .msg("索引信息为空，请重新上传文件!".to_string())
            .into_response();
    }
    let mut file_info_clone = file_info.clone().unwrap();
    file_info_clone
        .address_group_information
        .iter_mut()
        .for_each(|item| {
            item.bytes.iter_mut().for_each(|item| {
                if item.index == u64::from_str_radix(&param.index, 16).unwrap() {
                    item.bytes = param.byte.clone();
                }
            })
        });
    *file_info = Some(file_info_clone);
    DefaultResponse::success().into_response()
}

pub async fn init_upload_file(app_state: State<AppState>) -> impl IntoResponse {
    let mut upload_file_info = UPLOAD_FILE_INFO.lock().await;
    let upload_file_info_clone = upload_file_info.clone();
    match upload_file_info_clone {
        None => {}
        Some(file_info) => {
            let file_id = file_info.file_id;
            let file_name = file_info.file_name;
            let file_info = entity::model::t_file::Entity::find_by_id(&file_id)
                .one(app_state.db_conn.as_ref())
                .await;
            if let Ok(Some(file_model)) = file_info {
                let file_buf = file_model.file_buf;
                *upload_file_info = Some(UploadFileInfo {
                    file_id,
                    file_name,
                    file_size: Byte::from_f64_with_unit(file_buf.len() as f64, Unit::B)
                        .unwrap()
                        .get_appropriate_unit(UnitType::Decimal)
                        .to_string(),
                    address_group_information: get_address_group_information_from_file_buf(
                        file_buf,
                    ),
                });
            }
        }
    }
    DefaultResponse::success().into_response()
}

pub async fn download_current_file() -> impl IntoResponse {
    let file = UPLOAD_FILE_INFO.lock().await;
    let file_clone = file.clone();
    drop(file);
    let (file_name, file_buf) = match file_clone {
        None => (String::from("unknown"), vec![]),
        Some(file) => {
            let file_name = file.file_name;
            let mut file_buf = Vec::new();
            file.address_group_information.iter().for_each(|item| {
                item.bytes.iter().for_each(|item| {
                    file_buf.push(u8::from_str_radix(&item.bytes, 16).unwrap_or(0));
                })
            });
            (file_name, file_buf)
        }
    };
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
