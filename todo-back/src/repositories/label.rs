use axum::async_trait;
use sqlx::PgPool;
use crate::models::label::{CreateLabel, Label};
use super::RepositoryError;

#[async_trait]
pub trait LabelRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, user_id: i32, payload: CreateLabel) -> anyhow::Result<Label>;
    async fn all(&self, user_id: i32) -> anyhow::Result<Vec<Label>>;
    async fn delete(&self, id: i32, user_id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct LabelRepositoryForDb {
    pool: PgPool,
}

impl LabelRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForDb {
    async fn create(&self, user_id: i32, payload: CreateLabel) -> anyhow::Result<Label> {
        let label = sqlx::query_as::<_, Label>(
            r#"
insert into labels (name, user_id)
values($1, $2)
on conflict (user_id, name) do update set name = excluded.name
returning *
        "#,
        )
        .bind(payload.name.clone())
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(label)
    }

    async fn all(&self, user_id: i32) -> anyhow::Result<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>(
            r#"
select * from labels
where user_id = $1
order by labels.id asc;
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

    Ok(labels)
    }

    async fn delete(&self, id: i32, user_id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
delete from labels
where id = $1 and user_id = $2
        "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
    use crate::models::user::CreateUser;
    use crate::repositories::user::{UserRepository, UserRepositoryForDb};
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("undifined [DATABASE_URL]");
        let pool = PgPool::connect(database_url)
            .await
            .expect(&format!("fail connect database, url is [{}]", database_url));

        // create test user
        let user_repository = UserRepositoryForDb::new(pool.clone());
        let test_user = user_repository
            .create(CreateUser::new("test label_user".to_string()))
            .await
            .expect("Failed to create test user");
        let test_user_id = test_user.id;

        let label_repository = LabelRepositoryForDb::new(pool.clone());
        let test_label = "test label";

        // create
        let label = label_repository
            .create(test_user_id, CreateLabel::new(test_label.to_string()))
            .await
            .expect("[create] returned Err");
        assert_eq!(label.name, test_label);
        assert_eq!(label.user_id, test_user_id);

        // all
        let labels = label_repository.all(test_user_id).await.expect("[all] returned Err");
        let label = labels.last().unwrap();
        assert_eq!(label.name, test_label);

        // delete
        let _ = label_repository
            .delete(label.id, test_user_id)
            .await
            .expect("[delete] returned Err");
    }
}

pub mod test_utils {
    use axum::async_trait;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };
    use super::*;

    type LabelData = HashMap<i32, Label>;

    #[derive(Debug, Clone)]
    pub struct LabelRepositoryForMemory {
        store: Arc<RwLock<LabelData>>,
    }

    impl LabelRepositoryForMemory {
        pub fn new() -> Self {
            LabelRepositoryForMemory {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<LabelData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<LabelData> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl LabelRepository for LabelRepositoryForMemory {
        async fn create(&self, user_id: i32, payload: CreateLabel) -> anyhow::Result<Label> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let label = Label::new(id, payload.name.clone(), user_id);
            store.insert(id, label.clone());
            Ok(label)
        }

        async fn all(&self, user_id: i32) -> anyhow::Result<Vec<Label>> {
            let store = self.read_store_ref();
            let labels = store
                .values()
                .filter(|label| label.user_id == user_id)
                .cloned()
                .collect();
            Ok(labels)
        }

        async fn delete(&self, id: i32, user_id: i32) -> anyhow::Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn label_crud_scenario() {
            let name = "label name".to_string();
            let id = 1;
            let user_id = 1;
            let expected = Label::new(id, name.clone(), user_id);

            // create
            let repository = LabelRepositoryForMemory::new();
            let label = repository
                .create(user_id, CreateLabel::new(name))
                .await
                .expect("failed create label");
            assert_eq!(expected, label);

            // all
            let label = repository.all(user_id).await.unwrap();
            assert_eq!(vec![expected], label);

            // delete
            let res = repository.delete(id, user_id).await;
            assert!(res.is_ok())
        }
    }
}