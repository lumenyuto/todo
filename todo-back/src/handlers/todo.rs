use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::models::todo::{CreateTodo, UpdateTodo};
use crate::repositories::todo::TodoRepository;
use super::ValidatedJson;

pub async fn create_todo<T: TodoRepository>(
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todo)))
}
pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        create_app,
        models::{
            label::Label,
            todo::{CreateTodo, TodoEntity},
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
        http::{header, Method, Request, StatusCode},
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

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> TodoEntity {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
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

    #[tokio::test]
    async fn should_creat_todo() {
        let (labels, _label_ids) = label_fixture();
        let expected = TodoEntity::new(1, "should_create_todo".to_string(), labels.clone());
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_req_with_json(
            "/todos",
            Method::POST,
            r#"{ "text": "should_create_todo", "label_ids": [999] }"#.to_string(),
        );
        let res = create_app(todo_repository, label_repository, UserRepositoryForMemory::new()).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let (labels, label_ids) = label_fixture();
        let expected = TodoEntity::new(1, "should_find_todo".to_string(), labels.clone());
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let _todo = todo_repository
            .create(CreateTodo::new("should_find_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(todo_repository, label_repository, UserRepositoryForMemory::new()).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let (labels, label_ids) = label_fixture();
        let expected = TodoEntity::new(1, "should_get_all_todos".to_string(), labels.clone());
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let _todo = todo_repository
            .create(CreateTodo::new("should_get_all_todos".to_string(), label_ids))
            .await
            .expect("failed create todo");
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(todo_repository, label_repository, UserRepositoryForMemory::new()).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todos: Vec<TodoEntity> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo list instance. body: {}", body));
        assert_eq!(vec![expected], todos);

    }

    #[tokio::test]
    async fn should_update_todo() {
        let (labels, label_ids) = label_fixture();
        let expected = TodoEntity::new(1, "should_update_todo".to_string(), labels.clone());
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let _todo = todo_repository
            .create(CreateTodo::new("before_update_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{
                "id": 1,
                "text": "should_update_todo",
                "completed": false
            }"#.to_string(),
        );
        let res = create_app(todo_repository, label_repository, UserRepositoryForMemory::new()).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let (labels, label_ids) = label_fixture();
        let todo_repository = TodoRepositoryForMemory::new(labels.clone());
        let _todo = todo_repository
            .create(CreateTodo::new("should_delete_todo".to_string(), label_ids))
            .await
            .expect("failed create todo");
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(todo_repository, label_repository, UserRepositoryForMemory::new()).oneshot(req).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}