use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    AppState,
    middlewares::auth::AuthenticatedUser,
    models::label::CreateLabel,
};
use super::ValidatedJson;

pub async fn create_label(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateLabel>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let label = state.label_repository
        .create(user.id, payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn all_label(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let labels = state.label_repository
        .all(user.id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::OK, Json(labels)))
}

pub async fn delete_label(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let user = state.user_repository
        .find_by_sub(auth_user.sub)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    state.label_repository
        .delete(id, user.id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        create_app,
        models::{
            label::{CreateLabel, Label},
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
    use crate::repositories::{label::LabelRepository, user::UserRepository};

    const TEST_SUB: &str = "auth0|test_sub";

    async fn res_to_label(res: Response) -> Label {
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let label: Label = serde_json::from_str(&body)
            .expect(&format!("cannot convert Label instance. body: {}", body));
        label
    }

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
    async fn should_create_label() {
        let (labels, _label_ids) = label_fixture();
        let expected = Label::new(1, "should_create_label".to_string(), 1);
        let user_repository = UserRepositoryForMemory::new();
        seed_test_user(&user_repository).await;

        let req = build_req_with_json(
            "/labels",
            Method::POST,
            r#"{ "name": "should_create_label" }"#.to_string(),
        );
        let res = create_app(
            LabelRepositoryForMemory::new(),
            TeamRepositoryForMemory::new(),
            TodoRepositoryForMemory::new(labels.clone()),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        let label = res_to_label(res).await;
        assert_eq!(expected, label);
    }

    #[tokio::test]
    async fn should_get_all_label() {
        let (labels, _label_ids) = label_fixture();
        let expected = Label::new(1, "should_get_all_label".to_string(), 1);
        let label_repository = LabelRepositoryForMemory::new();
        let _label = label_repository
            .create(1, CreateLabel::new("should_get_all_label".to_string()))
            .await
            .expect("failed create label");
        let user_repository = UserRepositoryForMemory::new();
        seed_test_user(&user_repository).await;

        let req = build_req_with_empty(Method::GET, "/labels");
        let res = create_app(
            label_repository,
            TeamRepositoryForMemory::new(),
            TodoRepositoryForMemory::new(labels.clone()),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let labels: Vec<Label> = serde_json::from_str(&body).expect(&format!(
            "cannot convert Label list instance. body: {}",
            body
        ));
        assert_eq!(vec![expected], labels);
    }

    #[tokio::test]
    async fn should_delete_label() {
        let (labels, _label_ids) = label_fixture();
        let label_repository = LabelRepositoryForMemory::new();
        let _label = label_repository
            .create(1, CreateLabel::new("should_delete_label".to_string()))
            .await
            .expect("failed create label");
        let user_repository = UserRepositoryForMemory::new();
        seed_test_user(&user_repository).await;
        let req = build_req_with_empty(Method::DELETE, "/labels/1");
        let res = create_app(
            label_repository,
            TeamRepositoryForMemory::new(),
            TodoRepositoryForMemory::new(labels.clone()),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}
