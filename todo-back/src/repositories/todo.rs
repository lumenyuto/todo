use axum::async_trait;
use sqlx::{FromRow, PgPool};
use crate::models::{
    label::Label,
    todo::{CreateTodo, TodoEntity, UpdateTodo}
};
use super::RepositoryError;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TodoWithLabelFromRow {
    id: i32,
    text: String,
    completed: bool,
    user_id: i32,
    label_id: Option<i32>,
    label_name: Option<String>,
    label_user_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TodoFromRow {
    id: i32,
    text: String,
    completed: bool,
}

fn fold_entities(rows: Vec<TodoWithLabelFromRow>) -> Vec<TodoEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<TodoEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let mut todos = accum.iter_mut();
        while let Some(todo) = todos.next() {
            if todo.id == row.id {
                if let (Some(label_id), Some(label_name), Some(label_user_id)) =
                    (row.label_id, row.label_name.clone(), row.label_user_id)
                {
                    todo.labels.push(Label {
                        id: label_id,
                        name: label_name,
                        user_id: label_user_id,
                    });
                }
                continue 'outer;
            }
        }

        let labels = if let (Some(label_id), Some(label_name), Some(label_user_id)) =
            (row.label_id, row.label_name.clone(), row.label_user_id)
        {
            vec![Label {
                id: label_id,
                name: label_name,
                user_id: label_user_id,
            }]
        } else {
            vec![]
        };

        accum.push(TodoEntity::new(
            row.id,
            row.text.clone(),
            labels,
            row.user_id,
        ));
    }
    accum
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send +
std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity>;
    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity>;
    async fn all(&self, user_id: i32) -> anyhow::Result<Vec<TodoEntity>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new (pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, TodoFromRow>(
            r#"
insert into todos (text, completed, user_id)
values ($1, false, $2)
returning id, text, completed
            "#,
            )
            .bind(payload.text.clone())
            .bind(payload.user_id)
            .fetch_one(&self.pool)
            .await?;

        sqlx::query(
            r#"
insert into todo_labels (todo_id, label_id)
select $1, id
from unnest ($2) as t(id);
        "#,
        )
        .bind(row.id)
        .bind(payload.label_ids)
        .execute(&self.pool)
        .await?;

        tx.commit().await?;

        let todo = self.find(row.id).await?;
        Ok(todo)

    }

    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"
select todos.id, todos.text, todos.completed, todos.user_id,
       labels.id as label_id, labels.name as label_name, labels.user_id as label_user_id
from todos
            left outer join todo_labels tl on todos.id = tl.todo_id
            left outer join labels on labels.id = tl.label_id
where todos.id=$1;
        "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        let todos = fold_entities(items);
        let todo = todos.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(todo.clone())
    }

    async fn all(&self, user_id: i32) -> anyhow::Result<Vec<TodoEntity>> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"
select todos.id, todos.text, todos.completed, todos.user_id,
       labels.id as label_id, labels.name as label_name, labels.user_id as label_user_id
from todos
            left outer join todo_labels tl on todos.id = tl.todo_id
            left outer join labels on labels.id = tl.label_id
where todos.user_id=$1
order by todos.id desc;
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(fold_entities(items))
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;

        let old_todo = self.find(id).await?;
        sqlx::query(
            r#"
update todos set text=$1, completed=$2
where id=$3
returning *
        "#,
        )
        .bind(payload.text.unwrap_or(old_todo.text))
        .bind(payload.completed.unwrap_or(old_todo.completed))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if let Some(labels) = payload.label_ids {
            sqlx::query(
                r#"
    delete from todo_labels where todo_id=$1
            "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;

            sqlx::query(
                r#"
    insert into todo_labels (todo_id, label_id)
    select $1, id
    from unnest($2) as t(id);
            "#,
            )
            .bind(id)
            .bind(labels)
            .execute(&self.pool)
            .await?;
        };

        tx.commit().await?;
        let todo = self.find(id).await?;

        Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let tx = self.pool.begin().await?;

        sqlx::query(
            r#"
delete from todo_labels where todo_id=$1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        sqlx::query(
            r#"
delete from todos where id=$1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        tx.commit().await?;

        Ok(())
    }
}


#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
    use crate::models::user::CreateUser;
    use crate::repositories::user::{UserRepository, UserRepositoryForDb};
    use crate::repositories::label::{LabelRepository, LabelRepositoryForDb};
    use crate::models::label::CreateLabel;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[test]
    fn fold_entities_test() {
        let user_id = 1;
        let label_1 = Label {
            id: 1,
            name: String::from("label 1"),
            user_id,
        };
        let label_2 = Label {
            id: 2,
            name: String::from("label 2"),
            user_id,
        };
        let rows = vec![
            TodoWithLabelFromRow {
                id: 1,
                text: String::from("todo 1"),
                completed: false,
                user_id,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
                label_user_id: Some(user_id),
            },
            TodoWithLabelFromRow {
                id: 1,
                text: String::from("todo 1"),
                completed: false,
                user_id,
                label_id: Some(label_2.id),
                label_name: Some(label_2.name.clone()),
                label_user_id: Some(user_id),
            },
            TodoWithLabelFromRow {
                id: 2,
                text: String::from("todo 2"),
                completed: false,
                user_id,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
                label_user_id: Some(user_id),
            },
        ];
        let res = fold_entities(rows);
        assert_eq!(
            res,
            vec![
                TodoEntity::new(
                    1,
                    String::from("todo 1"),
                    vec![label_1.clone(), label_2.clone()],
                    user_id,
                ),
                TodoEntity::new(
                    2,
                    String::from("todo 2"),
                    vec![label_1.clone()],
                    user_id,
                ),
            ]
        );
    }

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
        let pool = PgPool::connect(database_url)
            .await
            .expect(&format!("fail connect database, url is [{}]", database_url));

        // create test user
        let user_repository = UserRepositoryForDb::new(pool.clone());
        let test_user = user_repository
            .create(CreateUser::new("test_todo_user".to_string()))
            .await
            .expect("Failed to create test user");

        // create test label
        let label_repository = LabelRepositoryForDb::new(pool.clone());
        let label_1 = label_repository
            .create(CreateLabel::new("test label".to_string(), test_user.id))
            .await
            .expect("Failed to create test label");

        let repository = TodoRepositoryForDb::new(pool.clone());
        let todo_text = "[crud_scenario] text";

        // create
        let created = repository
            .create(CreateTodo::new(todo_text.to_string(), vec![label_1.id], test_user.id))
            .await
            .expect("[create] returned Err");
        assert_eq!(created.text, todo_text);
        assert!(!created.completed);
        assert_eq!(*created.labels.first().unwrap(), label_1);

        // find
        let todo = repository
            .find(created.id)
            .await
            .expect("[find] returned Err");
        assert_eq!(created, todo);

        // all
        let todos = repository.all(test_user.id).await.expect("[all] returned Err");
        let todo = todos.first().unwrap();
        assert_eq!(created, *todo);

        // update
        let updated_text = "[crud_scenario] updated text";
        let todo = repository
            .update(
                todo.id,
                UpdateTodo {
                    text: Some(updated_text.to_string()),
                    completed: Some(true),
                    label_ids: Some(vec![]),
                    user_id: test_user.id,
                },
            )
            .await
            .expect("[update] returned Err");
        assert_eq!(created.id, todo.id);
        assert_eq!(todo.text, updated_text);
        assert!(todo.labels.len() == 0);

        // delete
        let _ = repository
            .delete(todo.id)
            .await
            .expect("[delete] returned Err");
        let res = repository.find(created.id).await;
        assert!(res.is_err());

        let todo_rows = sqlx::query(
            r#"
select * from todos where id=$1
        "#,
        )
        .bind(todo.id)
        .fetch_all(&pool)
        .await
        .expect("[delete] todo_labels fetch error");
        assert!(todo_rows.len() == 0);

        let rows = sqlx::query(
            r#"
select * from todo_labels where todo_id=$1
        "#,
        )
        .bind(todo.id)
        .fetch_all(&pool)
        .await
        .expect("[delete] todo_labels fetch error");
        assert!(rows.len() == 0);
    }
}

pub mod test_utils {
    use anyhow::Context;
    use axum::async_trait;
    use::std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };
    use super::*;

    type TodoData = HashMap<i32, TodoEntity>;

    #[derive(Debug, Clone)]
    pub struct TodoRepositoryForMemory {
        store: Arc<RwLock<TodoData>>,
        labels: Vec<Label>,
    }

    impl TodoRepositoryForMemory {
        pub fn new(labels: Vec<Label>) -> Self {
            TodoRepositoryForMemory {
                store: Arc::default(),
                labels,
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
            self.store.read().unwrap()
        }

        fn resolve_labels(&self, labels: Vec<i32>) -> Vec<Label> {
            labels
                .iter()
                .filter_map(|id| {
                    self.labels
                        .iter()
                        .find(|label| label.id == *id)
                        .cloned()
                })
                .collect()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryForMemory {
        async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let labels = self.resolve_labels(payload.label_ids);
            let todo = TodoEntity::new(id, payload.text.clone(), labels, payload.user_id);
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
            let store = self.read_store_ref();
            let todo = store
                .get(&id)
                .map(|todo| todo.clone())
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self, user_id: i32) -> anyhow::Result<Vec<TodoEntity>> {
            let store = self.read_store_ref();
            Ok(store.values()
                .filter(|todo| todo.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
            let text = payload.text.unwrap_or(todo.text.clone());
            let completed = payload.completed.unwrap_or(todo.completed);
            let labels = match payload.label_ids {
                Some(label_ids) => self.resolve_labels(label_ids),
                None => todo.labels.clone(),
            };
            let todo = TodoEntity::new(id, text, labels, payload.user_id);
            let todo = TodoEntity {
                completed,
                ..todo
            };
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> anyhow::Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn todo_crud_scenario() {
            let text = "todo text".to_string();
            let id = 1;
            let user_id = 1;
            let label = Label::new(id, "test label".to_string(), user_id);
            let labels = vec![label.clone()];
            let expected = TodoEntity::new(id, text.clone(), labels.clone(), user_id);

            // create
            let repository = TodoRepositoryForMemory::new(labels.clone());
            let todo = repository
                .create(CreateTodo::new(text, vec![label.id], user_id))
                .await
                .expect("failed create todo");
            assert_eq!(expected, todo);

            let todo = repository.find(todo.id).await.unwrap();
            assert_eq!(expected, todo);

            let todo = repository
                .all(user_id)
                .await
                .expect("failed get all todo");
            assert_eq!(vec![expected], todo);

            let text = "update todo text".to_string();
            let todo = repository
                .update(
                    1,
                    UpdateTodo {
                        text: Some(text.clone()),
                        completed: Some(true),
                        label_ids: Some(vec![]),
                        user_id,
                    },
                )
                .await
                .expect("failed update todo.");
            assert_eq!(
                TodoEntity {
                    id,
                    text,
                    completed: true,
                    labels: vec![],
                    user_id,
                },
                todo
            );

            let res = repository.delete(id).await;
            assert!(res.is_ok())
        }
    }
}