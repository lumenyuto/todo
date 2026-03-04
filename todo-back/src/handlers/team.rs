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
    repositories::{
        label::LabelRepository,
        team::TeamRepository,
        todo::TodoRepository,
        user::UserRepository,
    },
};
use super::ValidatedJson;

pub async fn all_team<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
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

pub async fn create_team<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    ValidatedJson(mut payload): ValidatedJson<CreateTeam>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub.clone())
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !payload.user_ids.contains(&user.id) {
        payload.user_ids.push(user.id);
    }

    let team = state.team_repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(team)))
}