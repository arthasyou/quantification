use axum::{extract::Query, http::StatusCode, Extension, Json};

use crate::{
    models::{record_model::GetPositionsRequest, CommonError, CommonResponse, IntoCommonResponse},
    static_items::position::{get_user_symbol_direction_positions, Position},
};

#[utoipa::path(
    get,
    path = "/get_positions",
    params(("symbol" = String, Query, description = "货币符号比如:btcusdt"),),
    responses(
        (status = 200, description = "Succeed", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户某种货币本程序当前持仓情况"
)]
pub async fn get_positions(
    Extension(user_id): Extension<String>,
    Query(params): Query<GetPositionsRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let positions =
        get_user_symbol_direction_positions(&params.symbol, &params.dirction, &user_id).await;
    // 根据 user_id 过滤 positions
    let filtered_positions: Vec<Position> = positions
        .into_iter()
        .filter(|pos| pos.user_id == user_id) // 只保留 user_id 匹配的 Position
        .collect();

    let res = filtered_positions.into_common_response_data();
    Ok(Json(res))
}
