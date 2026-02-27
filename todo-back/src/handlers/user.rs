use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::models::{
    user::CreateUser,
};
use crate::repositories::user::UserRepository;
use super::ValidatedJson;

pub async fn create_user<T: UserRepository>(
    ValidatedJson(payload): ValidatedJson<CreateUser>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn all_user<T: UserRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let users = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(users)))
}

pub async fn find_user<T: UserRepository>(
    Path(name): Path<String>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .find_by_name(name)
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

    fn build_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_user(res: Response) -> User {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
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
        let expected = User::new(1, "should_create_user".to_string());

        let req = build_req_with_json(
            "/users",
            Method::POST,
            r#"{ "name": "should_create_user" }"#.to_string(),
        );
        let res = create_app(
            TodoRepositoryForMemory::new(labels),
            LabelRepositoryForMemory::new(),
            UserRepositoryForMemory::new(),
        )
        .oneshot(req)
        .await
        .unwrap();
        let user = res_to_user(res).await;

        assert_eq!(expected, user);
    }

    #[tokio::test]
    async fn should_get_all_users() {
        let (labels, _label_ids) = label_fixture();
        let expected = User::new(1, "should_get_all_users".to_string());
        let user_repository = UserRepositoryForMemory::new();
        let _user = user_repository
            .create(CreateUser::new("should_get_all_users".to_string()))
            .await
            .expect("failed create user");

        let req = build_req_with_empty(Method::GET, "/users");
        let res = create_app(
            TodoRepositoryForMemory::new(labels),
            LabelRepositoryForMemory::new(),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let users: Vec<User> = serde_json::from_str(&body)
            .expect(&format!("cannot convert User list instance. body: {}", body));

        assert_eq!(vec![expected], users);
    }

    #[tokio::test]
    async fn should_find_user_by_name() {
        let (labels, _label_ids) = label_fixture();
        let expected = User::new(1, "should_find_user".to_string());
        let user_repository = UserRepositoryForMemory::new();
        let _user = user_repository
            .create(CreateUser::new("should_find_user".to_string()))
            .await
            .expect("failed create user");

        let req = build_req_with_empty(Method::GET, "/users/should_find_user");
        let res = create_app(
            TodoRepositoryForMemory::new(labels),
            LabelRepositoryForMemory::new(),
            user_repository,
        )
        .oneshot(req)
        .await
        .unwrap();
        let user = res_to_user(res).await;
        
        assert_eq!(expected, user);
    }
}
