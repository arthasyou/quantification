use axum::{http::StatusCode, Extension, Json};
use rust_decimal::Decimal;

use crate::{
    database::{
        strategy_db::{db_create_strategy, db_get_strategy},
        user_db::{db_create_user, db_get_user_info},
    },
    error::error_code,
    models::{
        user_model::{CreateUserInput, CreateUserRequest, UserResponse},
        CommonError, CommonResponse, IntoCommonResponse,
    },
    static_items::{
        secret_key::{delete_secret_key, insert_secret_key, SecretKey},
        strategy::{delete_user_strategy, insert_user_strategy},
        user_info::{get_agent_id, insert_user_info, UserInfo},
    },
};

#[utoipa::path(
    post,
    path = "/create",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "Succeed", body = UserResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "创建用户"
)]
pub async fn create_user(
    Extension(user_id): Extension<String>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let input = CreateUserInput {
        user_id: user_id.clone(),
        agent_id: payload.agent_id,
        key: payload.key,
        secret: payload.secret,
    };

    let data = db_create_user(input).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    db_create_strategy(&user_id).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
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
    path = "/get_info",
    responses(
        (status = 200, description = "Succeed", body = UserResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "获取用户信息"
)]
pub async fn get_user_info(
    Extension(user_id): Extension<String>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let data = db_get_user_info(&user_id).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let key = SecretKey::new(data.user_id.clone(), data.key.clone(), data.secret.clone());
    insert_secret_key(key).await;

    let user_info = UserInfo::new(
        data.user_id.clone(),
        data.agent_id.clone(),
        data.balance.parse::<Decimal>().unwrap(),
    );
    insert_user_info(user_info).await;

    let strategy = db_get_strategy(&user_id).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    insert_user_strategy(strategy).await;

    let a = get_agent_id(&user_id).await;
    println!("agent_id: {:?}", a);

    let res = data.into_common_response_data();
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Succeed", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户退出"
)]
pub async fn logout(
    Extension(user_id): Extension<String>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    delete_secret_key(&user_id).await;
    delete_user_strategy(&user_id).await;
    let res = CommonResponse::default();
    Ok(Json(res))
}
