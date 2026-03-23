mod middlewares;
mod handlers;
mod models;
mod repositories;
pub mod services;

use axum::{
    Router,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue,
    },
    routing::{delete, get, post},
};
use std::net::SocketAddr;
use std::{
    env,
    sync::Arc,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use tokio::net::TcpListener;

use handlers::{
    label::{all_label, create_label, delete_label},
    workspace::{all_workspace, create_workspace},
    todo::{all_todo, create_todo, delete_todo, update_todo, recommend_todos},
    user::{create_user, find_me, update_user},
};
use repositories::{
    label::LabelRepositoryForDb,
    workspace::WorkspaceRepositoryForDb,
    todo::TodoRepositoryForDb,
    user::UserRepositoryForDb,
};

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

    tracing::info!("running database migrations...");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");

    let gemini_api_key = env::var("GROQ_API_KEY").unwrap_or_default();

    let app = create_app(
        LabelRepositoryForDb::new(pool.clone()),
        WorkspaceRepositoryForDb::new(pool.clone()),
        TodoRepositoryForDb::new(pool.clone()),
        UserRepositoryForDb::new(pool.clone()),
        gemini_api_key,
    );
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub label_repository: Arc<dyn repositories::label::LabelRepository>,
    pub workspace_repository: Arc<dyn repositories::workspace::WorkspaceRepository>,
    pub todo_repository: Arc<dyn repositories::todo::TodoRepository>,
    pub user_repository: Arc<dyn repositories::user::UserRepository>,
    pub gemini_api_key: String,
}

pub fn create_app(
    label_repository: impl repositories::label::LabelRepository,
    workspace_repository: impl repositories::workspace::WorkspaceRepository,
    todo_repository: impl repositories::todo::TodoRepository,
    user_repository: impl repositories::user::UserRepository,
    gemini_api_key: String,
) -> Router {
    let state = AppState {
        label_repository: Arc::new(label_repository),
        workspace_repository: Arc::new(workspace_repository),
        todo_repository: Arc::new(todo_repository),
        user_repository: Arc::new(user_repository),
        gemini_api_key,
    };

    Router::new()
        .route("/", get(root))
        .route(
            "/labels",
            post(create_label).get(all_label),
        )
        .route("/labels/{id}", delete(delete_label))
        .route(
            "/users",
            post(create_user),
        )
        .route(
            "/users/me",
            get(find_me)
                .patch(update_user),
        )
        .route(
            "/workspaces",
            post(create_workspace).get(all_workspace),
        )
        .route("/workspaces/{id}/todos/recommend", post(recommend_todos))
        .route(
            "/workspaces/{id}/todos",
            post(create_todo).get(all_todo),
        )
        .route(
            "/workspaces/{id}/todos/{todo_id}",
            delete(delete_todo).patch(update_todo),
        )
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(
                    env::var("CORS_ORIGIN")
                        .unwrap_or_else(|_| "http://localhost:3001".to_string())
                        .parse::<HeaderValue>()
                        .unwrap()
                )
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]),
        )
}

async fn root() -> &'static str {
    "Hello, World!"
}
