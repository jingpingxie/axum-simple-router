use axum::http::StatusCode;
use axum::{
    Json,
    Router,
    routing::{
        post,
    },
};

use crate::database::user;
use crate::handler::auth::{Claims, generate_jwt};
use crate::handler::idl::*;
use crate::route::router_base;

pub struct User {}

impl router_base::RouterBase for User {
    fn get_path() -> &'static str {
        "/user"
    }
    fn get_routers() -> Router
    {
        Router::new()
            .route("/register", post(self::User::register))
            .route("/login", post(self::User::login))
    }
}

impl User {
    pub async fn register(
        Json(req): Json<RegisterRequest>,
    ) -> Result<Json<RegisterResponse>, StatusCode> {
        user::create(&req.username, &req.password)
            .await
            .map(|_| Json(RegisterResponse { success: true }))
            .map_err(|err| {
                tracing::error!("failed to create user: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }

    pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
        let user = user::find(&req.username)
            .await
            .map_err(|err| {
                tracing::error!("failed to find user: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or_else(|| {
                tracing::info!("user not found");
                StatusCode::UNAUTHORIZED
            })?;

        if user.password != req.password {
            tracing::info!("password mismatch");
            return Err(StatusCode::UNAUTHORIZED);
        }

        let jwt_token = generate_jwt(&Claims {
            username: user.username,
        })
            .map_err(|err| {
                tracing::error!("failed to generate jwt token: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(LoginResponse {
            success: true,
            token: jwt_token,
        }))
    }
}