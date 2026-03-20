use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    AppState,
    middlewares::auth::AuthenticatedUser,
    models::todo::{CreateTodo, RecommendedTodo, UpdateTodo},
    services::groq,
};
use super::ValidatedJson;

pub async fn create_todo(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(workspace_id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state.workspace_repository
        .is_member(workspace_id, user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !is_member {
        return Err(StatusCode::FORBIDDEN);
    }

    let todo = state.todo_repository
        .create(user.id, workspace_id, payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn all_todo(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(workspace_id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state.workspace_repository
        .is_member(workspace_id, user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !is_member {
        return Err(StatusCode::FORBIDDEN);
    }

    let todos = state.todo_repository
        .all_by_workspace(workspace_id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::OK, Json(todos)))
}

pub async fn update_todo(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, todo_id)): Path<(i32, i32)>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state.workspace_repository
        .is_member(workspace_id, user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let todo = state.todo_repository
        .find(todo_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_authorized = is_member && todo.workspace_id == workspace_id;
    if !is_authorized {
        return Err(StatusCode::FORBIDDEN);
    }

    let updated_todo = state.todo_repository
        .update(todo_id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(updated_todo)))
}

pub async fn delete_todo(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, todo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state.workspace_repository
        .is_member(workspace_id, user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let todo = state.todo_repository
        .find(todo_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_authorized = is_member && todo.workspace_id == workspace_id;
    if !is_authorized {
        return Err(StatusCode::FORBIDDEN);
    }

    state.todo_repository
        .delete(todo_id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn recommend_todos(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(workspace_id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state.workspace_repository
        .is_member(workspace_id, user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !is_member {
        return Err(StatusCode::FORBIDDEN);
    }

    let todos = state.todo_repository
        .all_by_workspace(workspace_id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let existing_texts: Vec<String> = todos.iter().map(|t| t.text.clone()).collect();

    let recommendations = groq::recommend_todos(&state.gemini_api_key, &existing_texts)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let result: Vec<RecommendedTodo> = recommendations
        .into_iter()
        .map(|text| RecommendedTodo { text })
        .collect();

    Ok((StatusCode::OK, Json(result)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        create_app,
        models::{
            label::Label,
            todo::{CreateTodo, TodoEntity},
            user::CreateUser,
        },
        repositories::{
            label::test_utils::LabelRepositoryForMemory,
            workspace::test_utils::WorkspaceRepositoryForMemory,
            todo::test_utils::TodoRepositoryForMemory,
            user::test_utils::UserRepositoryForMemory,
        },
    };
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use tower::ServiceExt;
    use crate::repositories::{todo::TodoRepository, user::UserRepository};

    const TEST_SUB: &str = "auth0|test_sub";

    fn build_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header("X-Test-Sub", TEST_SUB)
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header("X-Test-Sub", TEST_SUB)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> TodoEntity {
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: TodoEntity = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body: {}", body));
        todo
    }

    fn label_fixture() -> (Vec<Label>, Vec<i32>) {
        let id = 999;
        let user_id = 1;
        (
            vec![Label {
                id,
                name: String::from("test label"),
                user_id,
            }],
            vec![id],
        )
    }

    async fn seed_test_user(repo: &UserRepositoryForMemory) {
        repo.create(CreateUser::new(
            TEST_SUB.to_string(),
            "test_user".to_string(),
            "test@example.com".to_string(),
        ))
        .await
        .expect("failed to seed test user");
    }

    // Note: workspace-based tests require WorkspaceRepositoryForMemory to implement
    // is_member, which currently returns todo!(). Handler tests are simplified for now.
}
