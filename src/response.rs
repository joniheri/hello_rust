use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiSuccessResponse<T> {
    pub status: &'static str,
    pub data: T,
}

pub fn success<T>(data: T) -> Json<ApiSuccessResponse<T>>
where
    T: Serialize,
{
    Json(ApiSuccessResponse {
        status: "success",
        data,
    })
}
