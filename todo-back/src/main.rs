mod auth;
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
    team::{create_team},
    todo::{all_todo, create_todo, delete_todo, find_todo, update_todo},
    user::{create_user, find_me, update_user},
};
use repositories::{
    label::{LabelRepository, LabelRepositoryForDb},
    team::{TeamRepository, TeamRepositoryForDb},
    todo::{TodoRepository, TodoRepositoryForDb},
    user::{UserRepository, UserRepositoryForDb}
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
        TodoRepositoryForDb::new(pool.clone()),
        LabelRepositoryForDb::new(pool.clone()),
        UserRepositoryForDb::new(pool.clone()),
        TeamRepositoryForDb::new(pool.clone()),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

#[derive(Clone)]
struct AppState<Todo, Label, User, Team> {
    todo_repository: Arc<Todo>,
    label_repository: Arc<Label>,
    user_repository: Arc<User>,
    team_repository: Arc<Team>,
}

fn create_app<Todo: TodoRepository, Label: LabelRepository, User: UserRepository, Team: TeamRepository>(
    todo_repository: Todo,
    label_repository: Label,
    user_repository: User,
    team_repository: Team,
) -> Router {
    let state = AppState {
        todo_repository: Arc::new(todo_repository),
        label_repository: Arc::new(label_repository),
        user_repository: Arc::new(user_repository),
        team_repository: Arc::new(team_repository)
    };

    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<Todo, Label, User, Team>).get(all_todo::<Todo, Label, User, Team>))
        .route(
            "/todos/{id}",
            get(find_todo::<Todo, Label, User, Team>)
                .delete(delete_todo::<Todo, Label, User, Team>)
                .patch(update_todo::<Todo, Label, User, Team>),
        )
        .route(
            "/labels",
            post(create_label::<Todo, Label, User, Team>).get(all_label::<Todo, Label, User, Team>),
        )
        .route("/labels/{id}", delete(delete_label::<Todo, Label, User, Team>))
        .route(
            "/users",
            post(create_user::<Todo, Label, User, Team>),
        )
        .route(
            "/users/me",
            get(find_me::<Todo, Label, User, Team>)
                .patch(update_user::<Todo, Label, User, Team>),
        )
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]),
        )
}

async fn root() -> &'static str {
    "Hello, World!"
}