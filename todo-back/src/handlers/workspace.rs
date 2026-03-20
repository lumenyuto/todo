use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    AppState,
    middlewares::auth::AuthenticatedUser,
    models::workspace::CreateWorkspace,
};
use super::ValidatedJson;

pub async fn all_workspace(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let workspaces = state.workspace_repository
        .all_by_user(user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(workspaces))
}

pub async fn create_workspace(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateWorkspace>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub.clone())
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let workspace = state.workspace_repository
        .create(user.id, payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(workspace)))
}

