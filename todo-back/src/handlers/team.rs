use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    AppState,
    auth::AuthenticatedUser,
    models::team::{CreateTeam, UpdateTeam},
    repositories::{
        label::LabelRepository,
        team::TeamRepository,
        todo::TodoRepository,
        user::UserRepository,
    },
};
use super::ValidatedJson;

pub async fn create_team<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    _auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    ValidatedJson(payload): ValidatedJson<CreateTeam>,
) -> Result<impl IntoResponse, StatusCode> {
    let team = state.team_repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(team)))
}