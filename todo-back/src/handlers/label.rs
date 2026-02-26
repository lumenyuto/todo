use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::models::label::CreateLabel;
use crate::repositories::label::LabelRepository;
use super::ValidatedJson;

pub async fn create_label<T: LabelRepository>(
    ValidatedJson(payload): ValidatedJson<CreateLabel>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let label = repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn all_label<T: LabelRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let labels = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(labels)))
}

pub async fn delete_label<T: LabelRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
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
        (
            vec![Label {
                id,
                name: String::from("test label"),
            }],
            vec![id],
        )
    }

    #[tokio::test]
    async fn should_create_label() {
        let (labels, _label_ids) = label_fixture();
        let expected = Label::new(1, "should_create_label".to_string());

        let req = build_req_with_json(
            "/labels",
            Method::POST,
            r#"{ "name": "should_create_label" }"#.to_string(),
        );
        let res = create_app(
            TodoRepositoryForMemory::new(labels.clone()),
            LabelRepositoryForMemory::new(),
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
        let expected = Label::new(1, "should_get_all_label".to_string());
        let label_repository = LabelRepositoryForMemory::new();
        let _label = label_repository
            .create(CreateLabel::new("should_get_all_label".to_string()))
            .await
            .expect("failed create label");

        let req = build_todo_req_with_empty(Method::GET, "/labels");
        let res = create_app(TodoRepositoryForMemory::new(labels.clone()), label_repository)
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
            .create(CreateLabel::new("should_delete_label".to_string()))
            .await
            .expect("failed create label");
        let req = build_todo_req_with_empty(Method::DELETE, "/labels/1");
        let res = create_app(TodoRepositoryForMemory::new(labels.clone()), label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}