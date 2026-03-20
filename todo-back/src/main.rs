mod middlewares;
mod handlers;
mod models;
mod repositories;

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
    team::{all_team, create_team},
    todo::{all_user_todo, all_team_todo, create_team_todo, create_user_todo, delete_team_todo, delete_user_todo, find_user_todo, update_team_todo, update_user_todo},
    user::{create_user, find_me, update_user},
};
use repositories::{
    label::LabelRepositoryForDb,
    team::TeamRepositoryForDb,
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

    let app = create_app(
        LabelRepositoryForDb::new(pool.clone()),
        TeamRepositoryForDb::new(pool.clone()),
        TodoRepositoryForDb::new(pool.clone()),
        UserRepositoryForDb::new(pool.clone()),
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
    pub team_repository: Arc<dyn repositories::team::TeamRepository>,
    pub todo_repository: Arc<dyn repositories::todo::TodoRepository>,
    pub user_repository: Arc<dyn repositories::user::UserRepository>,
}

pub fn create_app(
    label_repository: impl repositories::label::LabelRepository,
    team_repository: impl repositories::team::TeamRepository,
    todo_repository: impl repositories::todo::TodoRepository,
    user_repository: impl repositories::user::UserRepository,
) -> Router {
    let state = AppState {
        label_repository: Arc::new(label_repository),
        team_repository: Arc::new(team_repository),
        todo_repository: Arc::new(todo_repository),
        user_repository: Arc::new(user_repository),
    };

    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_user_todo).get(all_user_todo))
        .route(
            "/todos/{id}",
            get(find_user_todo)
                .delete(delete_user_todo)
                .patch(update_user_todo),
        )
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
            "/teams",
            post(create_team).get(all_team)
        )
        .route(
            "/teams/{id}/todos",
            post(create_team_todo).get(all_team_todo),
        )
        .route(
            "/teams/{id}/todos/{id}",
            delete(delete_team_todo).patch(update_team_todo),
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
