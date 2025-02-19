use axum::{http::StatusCode, Extension, Json};

use crate::{
    biance::biance_trade::get_biance_risk,
    database::strategy_db::db_update_strategy,
    error::error_code,
    models::{
        trade_model::{CreatePositionRequest, GetRiskResponse, GetStategyResponse, UpdateStrategy},
        CommonError, CommonResponse, IntoCommonResponse,
    },
    static_items::{
        price::get_symbol_price,
        secret_key::get_secret_key,
        strategy::{get_user_spec_strategy, get_user_strategy, update_user_strategy},
    },
};

#[utoipa::path(
    get,
    path = "/get_risk",
    responses(
        (status = 200, description = "Succeed", body = GetRiskResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户当前持仓情况"
)]
pub async fn get_risk(
    Extension(user_id): Extension<String>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let secret_key = get_secret_key(&user_id).await.ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    let data = get_biance_risk(&secret_key.key, &secret_key.secret)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_code::SERVER_ERROR.into()),
            )
        })?;

    let res = data.into_common_response_data();
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/get_strategy",
    responses(
        (status = 200, description = "Succeed", body = GetStategyResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户当前策略"
)]
pub async fn get_strategy(
    Extension(user_id): Extension<String>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let strategy = get_user_strategy(&user_id).await.ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let res = strategy.into_common_response_data();
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/update_strategy",
    request_body = UpdateStrategy,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "更新用户当前策略"
)]
pub async fn update_strategy(
    Extension(user_id): Extension<String>,
    Json(payload): Json<UpdateStrategy>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    update_user_strategy(&user_id, payload.cfg.clone()).await;
    db_update_strategy(&user_id, payload).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let res = CommonResponse::default();
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/create_position",
    request_body = UpdateStrategy,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "开仓"
)]
pub async fn create_position(
    Extension(user_id): Extension<String>,
    Json(payload): Json<CreatePositionRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let price = get_symbol_price(&payload.symbol).await.map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::INVALIAD_SYMBOLE.into()),
        )
    })?;

    let strategy = get_user_spec_strategy(&user_id, payload.strategy_id)
        .await
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_code::SERVER_ERROR.into()),
            )
        })?;

    let res = CommonResponse::default();
    Ok(Json(res))
}
