use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    AppState,
    middlewares::auth::AuthenticatedUser,
    models::user::{CreateUser, UpdateUser},
    repositories::{
        label::LabelRepository,
        team::TeamRepository,
        todo::TodoRepository,
        user::UserRepository,
    },
};
use super::ValidatedJson;

pub async fn create_user<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    _user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn find_me<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    auth_user: AuthenticatedUser,
    State(state): State<AppState<Label, Team, Todo, User>>,
    ValidatedJson(payload): ValidatedJson<UpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .update_name(auth_user.sub, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        create_app,
        models::{
            label::Label,
            user::{CreateUser, User},
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
        http::{header, Method, Request},
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

    fn build_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header("X-Test-Sub", TEST_SUB)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_user(res: Response) -> User {
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body)
            .expect(&format!("cannot convert User instance. body: {}", body));
        user
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

    #[tokio::test]
    async fn should_create_user() {
        let (labels, _label_ids) = label_fixture();
        let expected = User::new(
            1,
            TEST_SUB.to_string(),
            Some("should_create_user".to_string()),
            Some("test@example.com".to_string()),
        );

        let req = build_req_with_json(
            "/users",
            Method::POST,
            format!(
                r#"{{ "sub": "{}", "name": "should_create_user", "email": "test@example.com" }}"#,
                TEST_SUB
            ),
        );
        let res = create_app(
            LabelRepositoryForMemory::new(),
            TeamRepositoryForMemory::new(),
            TodoRepositoryForMemory::new(labels.clone()),
            UserRepositoryForMemory::new(),
            )
            .oneshot(req)
            .await
            .unwrap();
        let user = res_to_user(res).await;

        assert_eq!(expected, user);
    }

    #[tokio::test]
    async fn should_find_me() {
        let (labels, _label_ids) = label_fixture();
        let expected = User::new(
            1,
            TEST_SUB.to_string(),
            Some("test_user".to_string()),
            Some("test@example.com".to_string()),
        );
        let user_repository = UserRepositoryForMemory::new();
        let _user = user_repository
            .create(CreateUser::new(
                TEST_SUB.to_string(),
                "test_user".to_string(),
                "test@example.com".to_string(),
            ))
            .await
            .expect("failed create user");

        let req = build_req_with_empty(Method::GET, "/users/me");
        let res = create_app(
            LabelRepositoryForMemory::new(),
            TeamRepositoryForMemory::new(),
            TodoRepositoryForMemory::new(labels.clone()),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        let user = res_to_user(res).await;

        assert_eq!(expected, user);
    }
}
