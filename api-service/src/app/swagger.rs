use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::test::test_service::page_list,
        crate::test::test_service::info,
    ),
    components(
        schemas(
            crate::app::response::PaginateResponse<entity::model::t_test::Model>,
            crate::app::response::PaginateInfo,
            crate::app::response::DataResponse<entity::model::t_test::Model>,
            crate::app::response::DefaultResponse,
        )
    ),
    tags(
        (name = "Pe", description = "Pe management API")
    )
)]
pub struct ApiDoc;
