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
    todo::{all_todo, create_team_todo, create_personal_todo, delete_todo, find_todo, update_todo},
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
        
        LabelRepositoryForDb::new(pool.clone()),
        TeamRepositoryForDb::new(pool.clone()),
        TodoRepositoryForDb::new(pool.clone()),
        UserRepositoryForDb::new(pool.clone()),
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
struct AppState<Label, Team, Todo, User> {
    label_repository: Arc<Label>,
    team_repository: Arc<Team>,
    todo_repository: Arc<Todo>,
    user_repository: Arc<User>,
}

fn create_app<Label: LabelRepository, Team: TeamRepository, Todo: TodoRepository, User: UserRepository>(
    label_repository: Label,
    team_repository: Team,
    todo_repository: Todo,
    user_repository: User,
) -> Router {
    let state = AppState {
        label_repository: Arc::new(label_repository),
        team_repository: Arc::new(team_repository),
        todo_repository: Arc::new(todo_repository),
        user_repository: Arc::new(user_repository),
    };

    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_personal_todo::<Label, Team, Todo, User>).get(all_todo::<Label, Team, Todo, User>))
        .route(
            "/todos/{id}",
            get(find_todo::<Label, Team, Todo, User>)
                .delete(delete_todo::<Label, Team, Todo, User>)
                .patch(update_todo::<Label, Team, Todo, User>),
        )
        .route(
            "/labels",
            post(create_label::<Label, Team, Todo, User>).get(all_label::<Label, Team, Todo, User>),
        )
        .route("/labels/{id}", delete(delete_label::<Label, Team, Todo, User>))
        .route(
            "/users",
            post(create_user::<Label, Team, Todo, User>),
        )
        .route(
            "/users/me",
            get(find_me::<Label, Team, Todo, User>)
                .patch(update_user::<Label, Team, Todo, User>),
        )
        .route(
            "/teams",
            post(create_team::<Label, Team, Todo, User>)
        )
        .route(
            "/teams/{id}/todos",
            post(create_team_todo::<Label, Team, Todo, User>),
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