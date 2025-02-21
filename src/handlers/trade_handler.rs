use axum::{http::StatusCode, Extension, Json};

use crate::{
    biance::biance_trade::get_biance_risk,
    database::strategy_db::db_update_strategy,
    error::error_code,
    models::{
        trade_model::{
            ClosePositionRequest, CreatePositionRequest, GetRiskResponse, GetStategyResponse,
            UpdateStrategy,
        },
        CommonError, CommonResponse, IntoCommonResponse,
    },
    static_items::{
        percision::get_symbol_percision,
        position::{
            inser_user_positon, remove_user_symbol_direction_position, Direction, Position,
        },
        price::get_symbol_price,
        secret_key::get_secret_key,
        strategy::{get_user_spec_strategy, get_user_strategy, update_user_strategy},
    },
    utils::{calculate_quantity, create_position_order, get_symbol_direction_quantity},
};

#[utoipa::path(
    get,
    path = "/get_risk",
    responses(
        (status = 200, description = "Succeed", body = GetRiskResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户币安当前持仓情况"
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
    request_body = CreatePositionRequest,
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

    let percision = get_symbol_percision(&payload.symbol).await.ok_or_else(|| {
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

    let (side, position_side, price) = match payload.direction {
        Direction::Long => ("BUY", "LONG", &price.buy),
        Direction::Short => ("SELL", "SHORT", &price.sell),
    };
    let price_f64: f64 = price.parse().unwrap();
    let quantity = calculate_quantity(&payload, price_f64, percision);

    let secret_key = get_secret_key(&user_id).await.ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let order = create_position_order(
        &payload.symbol,
        side,
        position_side,
        &quantity,
        &secret_key.key,
        &secret_key.secret,
    )
    .await
    .map_err(|e| {
        eprintln!("Create position error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let price_f64: f64 = order.avg_price.parse().unwrap();
    let position = Position::new(
        order.order_id,
        user_id.clone(),
        payload.symbol,
        price_f64,
        payload.direction,
        quantity,
        payload.leverage,
        payload.stop_loss_percent,
        strategy,
        secret_key.key,
        secret_key.secret,
    )
    .await;

    inser_user_positon(position).await.map_err(|e| {
        eprintln!("inser_user_positon: {:?}", e);
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
    path = "/close_position",
    request_body = ClosePositionRequest,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "平仓"
)]
pub async fn close_position(
    Extension(user_id): Extension<String>,
    Json(payload): Json<ClosePositionRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let (side, position_side) = match payload.direction {
        Direction::Long => ("SELL", "LONG"),
        Direction::Short => ("BUY", "SHORT"),
    };

    let secret_key = get_secret_key(&user_id).await.ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let quantity = get_symbol_direction_quantity(
        &payload.symbol,
        position_side,
        &secret_key.key,
        &secret_key.secret,
    )
    .await
    .map_err(|e| {
        eprintln!("get_symbol_direction_quantity: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let _order = create_position_order(
        &payload.symbol,
        side,
        position_side,
        &quantity,
        &secret_key.key,
        &secret_key.secret,
    )
    .await
    .map_err(|e| {
        eprintln!("Create position error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    remove_user_symbol_direction_position(&payload.symbol, &user_id, &payload.direction).await;

    let res = CommonResponse::default();
    Ok(Json(res))
}
