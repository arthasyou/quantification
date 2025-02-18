use axum::{http::StatusCode, Json};

use crate::{
    database::fee_db::db_create_fee,
    error::error_code,
    models::{fee_model::CreateFeeRequest, CommonError, CommonResponse},
};

pub async fn create_fee(
    Json(payload): Json<CreateFeeRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    db_create_fee(payload).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let res = CommonResponse::default();
    Ok(Json(res))
}
