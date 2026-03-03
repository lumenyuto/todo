use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    AppState,
    auth::AuthenticatedUser,
    models::todo::{CreateTodo, UpdateTodo},
    repositories::{
        label::LabelRepository,
        team::TeamRepository,
        todo::TodoRepository,
        user::UserRepository,
    },
};
use super::ValidatedJson;

pub async fn create_personal_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let todo = state.todo_repository
        .create(user.id, None, payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn create_team_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    Path(team_id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state
        .user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let is_member = state
        .team_repository
        .is_member(team_id, user.id)
        .await
        .or(Err(StatusCode::INSUFFICIENT_STORAGE))?;

    if !is_member {
        return Err(StatusCode::FORBIDDEN)
    }

    let todo = state
        .todo_repository
        .create(user.id, Some(team_id), payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let todo = state.todo_repository
        .find(id, user.id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let todo = state.todo_repository
        .all(user.id)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let todo = state.todo_repository
        .update(id, user.id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    state.todo_repository
        .delete(id, user.id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .or(Err(StatusCode::NOT_FOUND))
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
            team::test_utils::TeamRepositoryForMemory,
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

    #[tokio::test]
    async fn should_create_todo() {
        todo!();
        let (labels, _label_ids) = label_fixture();
        let user_id = 1;
        let team_id = 1;
        let expected = TodoEntity::new(1, "should_create_todo".to_string(), labels.clone(), user_id, Some(team_id));
        let label_repository = LabelRepositoryForMemory::new();
        let team_repository = TeamRepositoryForMemory::new();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let user_repository = UserRepositoryForMemory::new();
        seed_test_user(&user_repository).await;
        let req = build_req_with_json(
            "/todos",
            Method::POST,
            r#"{ "text": "should_create_todo", "label_ids": [999] }"#.to_string(),
        );
        let res = create_app(label_repository, team_repository, todo_repository, user_repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let (labels, label_ids) = label_fixture();
        let user_id = 1;
        let team_id = 1;
        let expected = TodoEntity::new(1, "should_find_todo".to_string(), labels.clone(), user_id, Some(team_id));
        let label_repository = LabelRepositoryForMemory::new();
        let team_repository = TeamRepositoryForMemory::new();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let user_repository = UserRepositoryForMemory::new();
        let _todo = todo_repository
            .create(user_id, Some(team_id), CreateTodo::new("should_find_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        seed_test_user(&user_repository).await;
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(label_repository, team_repository, todo_repository, user_repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let (labels, label_ids) = label_fixture();
        let user_id = 1;
        let team_id = 1;
        let expected = TodoEntity::new(1, "should_get_all_todos".to_string(), labels.clone(), user_id, Some(team_id));
        let label_repository = LabelRepositoryForMemory::new();
        let team_repository = TeamRepositoryForMemory::new();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let user_repository = UserRepositoryForMemory::new();
        let _todo = todo_repository
            .create(user_id, Some(team_id), CreateTodo::new("should_get_all_todos".to_string(), label_ids))
            .await
            .expect("failed create todo");
        seed_test_user(&user_repository).await;
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(label_repository, team_repository, todo_repository, user_repository).oneshot(req).await.unwrap();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todos: Vec<TodoEntity> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo list instance. body: {}", body));
        assert_eq!(vec![expected], todos);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let (labels, label_ids) = label_fixture();
        let user_id = 1;
        let team_id = 1;
        let expected = TodoEntity::new(1, "should_update_todo".to_string(), labels.clone(), user_id, Some(team_id));
        let label_repository = LabelRepositoryForMemory::new();
        let team_repository = TeamRepositoryForMemory::new();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let user_repository = UserRepositoryForMemory::new();
        let _todo = todo_repository
            .create(user_id, Some(team_id), CreateTodo::new("before_update_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        seed_test_user(&user_repository).await;
        let req = build_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{
                "text": "should_update_todo",
                "completed": false
            }"#.to_string(),
        );
        let res = create_app(label_repository, team_repository, todo_repository, user_repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let (labels, label_ids) = label_fixture();
        let user_id = 1;
        let team_id = 1;
        let label_repository = LabelRepositoryForMemory::new();
        let team_repository = TeamRepositoryForMemory::new();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let user_repository = UserRepositoryForMemory::new();
        let _todo = todo_repository
            .create(user_id, Some(team_id), CreateTodo::new("should_delete_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        seed_test_user(&user_repository).await;
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(label_repository, team_repository, todo_repository, user_repository).oneshot(req).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}