use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    AppState,
    middlewares::auth::AuthenticatedUser,
    models::team::CreateTeam,
};
use super::ValidatedJson;

pub async fn all_team(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let teams = state.team_repository
        .all_by_user(user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(teams))
}

pub async fn create_team(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    ValidatedJson(mut payload): ValidatedJson<CreateTeam>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub.clone())
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if let Some(ref email) = user.email {
        if !payload.user_emails.contains(email) {
            payload.user_emails.push(email.clone());
        }
    }

    let team = state.team_repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(team)))
}
