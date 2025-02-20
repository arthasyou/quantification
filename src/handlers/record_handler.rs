use axum::{http::StatusCode, Extension, Json};

use crate::{
    models::{record_model::GetPositionsRequest, CommonError, CommonResponse, IntoCommonResponse},
    static_items::position::get_symbol_positions,
};

pub async fn get_positons(
    Extension(user_id): Extension<String>,
    Json(payload): Json<GetPositionsRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let positons = get_symbol_positions(&payload.symbol).await;
    let res = positons.into_common_response_data();
    Ok(Json(res))
}
