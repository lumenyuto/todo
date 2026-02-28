use axum::{
    extract::{Extension, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::{
    models::label::{CreateLabel},
    repositories::label::LabelRepository,
};
use super::{UserIdQuery, ValidatedJson};

pub async fn create_label<T: LabelRepository>(
    Query(query): Query<UserIdQuery>,
    ValidatedJson(payload): ValidatedJson<CreateLabel>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let label = repository
        .create(query.user_id, payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn all_label<T: LabelRepository>(
    Query(query): Query<UserIdQuery>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let labels = repository
        .all(query.user_id)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::OK, Json(labels)))
}

pub async fn delete_label<T: LabelRepository>(
    Path(id): Path<i32>,
    Query(query): Query<UserIdQuery>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id, query.user_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        create_app,
        models::label::{CreateLabel, Label},
        repositories::{
            label::test_utils::LabelRepositoryForMemory,
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

    async fn res_to_label(res: Response) -> Label {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
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
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
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

    #[tokio::test]
    async fn should_create_label() {
        let (labels, _label_ids) = label_fixture();
        let expected = Label::new(1, "should_create_label".to_string(), 1);

        let req = build_req_with_json(
            "/labels?user_id=1",
            Method::POST,
            r#"{ "name": "should_create_label" }"#.to_string(),
        );
        let res = create_app(
            TodoRepositoryForMemory::new(labels.clone()),
            LabelRepositoryForMemory::new(),
            UserRepositoryForMemory::new(),
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

        let req = build_todo_req_with_empty(Method::GET, "/labels?user_id=1");
        let res = create_app(TodoRepositoryForMemory::new(labels.clone()), label_repository, UserRepositoryForMemory::new())
            .oneshot(req)
            .await
            .unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
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
        let req = build_todo_req_with_empty(Method::DELETE, "/labels/1?user_id=1");
        let res = create_app(TodoRepositoryForMemory::new(labels.clone()), label_repository, UserRepositoryForMemory::new())
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}