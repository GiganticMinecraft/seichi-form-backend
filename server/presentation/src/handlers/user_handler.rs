use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use domain::{
    repository::Repositories,
    user::models::{RoleQuery, User, UserSessionExpires},
};
use resource::repository::RealInfrastructureRepository;
use serde_json::json;
use usecase::user::UserUseCase;
use uuid::Uuid;

use crate::{handlers::error_handler::handle_error, schemas::user::DiscordOAuthToken};

pub async fn get_my_user_info(
    Extension(user): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case.fetch_user_information(&user, user.id).await {
        Ok(user_dto) => {
            let discord_user_id = user_dto.discord_user_id.map(|id| id.to_string());

            (
                StatusCode::OK,
                Json(json!({
                    "uuid": user_dto.user.id.to_string(),
                    "name": user_dto.user.name,
                    "role": user_dto.user.role.to_string(),
                    "discord_user_id": discord_user_id,
                })),
            )
                .into_response()
        }
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn get_user_info(
    Extension(actor): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case.fetch_user_information(&actor, uuid).await {
        Ok(user_dto) => {
            let discord_user_id = user_dto.discord_user_id.map(|id| id.to_string());

            (
                StatusCode::OK,
                Json(json!({
                    "uuid": user_dto.user.id.to_string(),
                    "name": user_dto.user.name,
                    "role": user_dto.user.role.to_string(),
                    "discord_user_id": discord_user_id,
                })),
            )
                .into_response()
        }
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn patch_user_role(
    Extension(actor): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
    Path(uuid): Path<Uuid>,
    Query(role): Query<RoleQuery>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case.patch_user_role(&actor, uuid, role.role).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn user_list(
    Extension(actor): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case.fetch_all_users(&actor).await {
        Ok(users) => (StatusCode::OK, Json(json!(users))).into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn start_session(
    State(repository): State<RealInfrastructureRepository>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(expires): Json<UserSessionExpires>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    let token = auth.token();

    match user_use_case
        .fetch_user_by_xbox_token(token.to_string())
        .await
    {
        Ok(Some(user)) => {
            let expires = expires.expires;

            match user_use_case
                .start_user_session(token.to_string(), &user, expires)
                .await
            {
                Ok(session_id) => (
                    StatusCode::OK,
                    [(
                        header::SET_COOKIE,
                        HeaderValue::from_str(
                            format!(
                                "SEICHI_PORTAL__SESSION_ID={session_id}; Max-Age={expires}; \
                                 Path=/; Secure; HttpOnly"
                            )
                            .as_str(),
                        )
                        .unwrap(),
                    )],
                )
                    .into_response(),
                Err(err) => handle_error(err).into_response(),
            }
        }
        Ok(None) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "reason": "invalid token" })),
            )
        }
        .into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn end_session(
    State(repository): State<RealInfrastructureRepository>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    let session_id = auth.token();

    match user_use_case.end_user_session(session_id.to_string()).await {
        Ok(_) => (
            StatusCode::OK,
            [(
                header::SET_COOKIE,
                HeaderValue::from_str(
                    format!(
                        "SEICHI_PORTAL__SESSION_ID={session_id}; Max-Age=0; Path=/; Secure; \
                         HttpOnly"
                    )
                    .as_str(),
                )
                .unwrap(),
            )],
        )
            .into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn link_discord(
    Extension(user): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
    Json(discord_token): Json<DiscordOAuthToken>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case
        .link_discord_user(discord_token.token, user)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn unlink_discord(
    Extension(user): Extension<User>,
    State(repository): State<RealInfrastructureRepository>,
) -> impl IntoResponse {
    let user_use_case = UserUseCase {
        repository: repository.user_repository(),
    };

    match user_use_case.unlink_discord_user(user).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}
