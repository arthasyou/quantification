use std::sync::Arc;

use axum::Extension;
use axum::{http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use service_utils_rs::services::jwt::Jwt;

use crate::database::auth_db;
use crate::error::error_code;
use crate::models::auth_model::{
    Auth, AuthInput, Login, LoginRequest, LoginResponse, SignupRequest,
};
use crate::models::{CommonError, CommonResponse, IntoCommonResponse};

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignupRequest,
    responses(
        (status = 200, description = "User registered successfully", body = CommonResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    )
)]
pub async fn signup(
    Json(payload): Json<SignupRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    // 检查用户名是否已存在
    is_username_taken(&payload.username).await?;
    // 哈希密码
    let hashed_password = hash(&payload.password, DEFAULT_COST).map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let user = AuthInput {
        username: payload.username,
        password: hashed_password,
    };

    auth_db::db_singup(user).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let mut res = CommonResponse::default();
    res.message = "User registered successfully".to_string();
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User registered successfully", body = LoginResponse),
        (status = 500, description = "Internal server error", body = CommonError)
    )
)]
pub async fn login(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonError>)> {
    let db_user = get_current_user(&payload.username).await?;
    verify_password(&payload.password, db_user.password.as_ref())?;

    let user_id = db_user.uuid.clone().to_string();
    let (accece, refleash) = jwt.generate_token_pair(user_id.clone()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    let data = Login {
        access_token: accece,
        refresh: refleash,
    };
    let res = data.into_common_response_data();
    Ok(Json(res))
}

async fn get_current_user(username: &str) -> Result<Auth, (StatusCode, Json<CommonError>)> {
    let existing_user = auth_db::get_auth(username).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    match existing_user {
        Some(user) => Ok(user),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(error_code::USER_NOT_EXIST.into()),
        )),
    }
}

fn verify_password(password: &str, hash: &str) -> Result<bool, (StatusCode, Json<CommonError>)> {
    bcrypt::verify(password, hash).map_err(|_err| {
        (
            StatusCode::UNAUTHORIZED,
            Json(error_code::PASSWORD_ERROR.into()),
        )
    })
}

async fn is_username_taken(username: &str) -> Result<(), (StatusCode, Json<CommonError>)> {
    let existing_user = auth_db::get_auth(username).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    println!("existing: {:?}", existing_user);
    match existing_user {
        // Some(_) => Ok(()),
        Some(_) => Err((StatusCode::BAD_REQUEST, Json(error_code::USER_EXIST.into()))),
        None => Ok(()),
    }
}
