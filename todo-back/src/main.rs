mod handlers;
mod models;
mod repositories;

use crate::repositories::{
    label::{LabelRepository, LabelRepositoryForDb},
    todo::{TodoRepository, TodoRepositoryForDb},
    user::{UserRepository, UserRepositoryForDb}
};
use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,
};
use handlers::{
    label::{all_label, create_label, delete_label},
    todo::{all_todo, create_todo, delete_todo, find_todo, update_todo},
    user::{all_user, create_user, find_user}
};
use std::net::SocketAddr;
use std::{
    env,
    sync::Arc,
};
use hyper::header::CONTENT_TYPE;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer, Origin};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    unsafe {
        env::set_var("RUST_LOG", log_level);
    }
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start_connect database...");
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));

    let app = create_app(
        TodoRepositoryForDb::new(pool.clone()),
        LabelRepositoryForDb::new(pool.clone()),
        UserRepositoryForDb::new(pool.clone()),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<Todo: TodoRepository, Label: LabelRepository, User: UserRepository>(
    todo_repository: Todo,
    label_repository: Label,
    user_repository: User,
) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<Todo>).get(all_todo::<Todo>))
        .route(
            "/todos/:id",
            get(find_todo::<Todo>)
                .delete(delete_todo::<Todo>)
                .patch(update_todo::<Todo>),
        )
        .route(
            "/labels",
            post(create_label::<Label>).get(all_label::<Label>),
        )
        .route("/labels/:id", delete(delete_label::<Label>))
        .route(
            "/users",
            post(create_user::<User>).get(all_user::<User>),
        )
        .route("/users/:name", get(find_user::<User>))
        .layer(Extension(Arc::new(todo_repository)))
        .layer(Extension(Arc::new(label_repository)))
        .layer(Extension(Arc::new(user_repository)))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:3001".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

async fn root() -> &'static str {
    "Hello, World!"
}