use axum::async_trait;
use sqlx::PgPool;
use crate::models::user::{CreateUser, User};
use super::RepositoryError;

#[async_trait]
pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateUser) -> anyhow::Result<User>;
    async fn all(&self) -> anyhow::Result<Vec<User>>;
    async fn find_by_name(&self, name: String) -> anyhow::Result<User>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryForDb {
    pool: PgPool,
}

impl UserRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryForDb {
    async fn create(&self, payload: CreateUser) -> anyhow::Result<User> {
        let optional_user = sqlx::query_as::<_, User>(
            r#"
select * from users where name = $1
            "#,
        )
        .bind(payload.name.clone())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(user) = optional_user {
            return Err(RepositoryError::Duplicate(user.id).into());
        }

        let user = sqlx::query_as::<_, User>(
            r#"
insert into users ( name )
values ( $1 )
returning *
            "#,
        )
        .bind(payload.name.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn all(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
select * from users
order by users.id asc;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn find_by_name(&self, name: String) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
select * from users where name = $1
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(0),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(user)
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
        let pool = PgPool::connect(database_url)
            .await
            .expect(&format!("fail connect database, url is [{}]", database_url));

        let repository = UserRepositoryForDb::new(pool.clone());
        let user_name = "test_user";

        // create
        let user = repository
            .create(CreateUser::new(user_name.to_string()))
            .await
            .expect("[create] returned Err");
        assert_eq!(user.name, user_name);

        // all
        let users = repository.all().await.expect("[all] returned Err");
        let user = users.last().unwrap();
        assert_eq!(user.name, user_name);

        // find_by_name
        let user = repository
            .find_by_name(user_name.to_string())
            .await
            .expect("[find_by_name] returned Err");
        assert_eq!(user.name, user_name);

        // cleanup
        sqlx::query(
            r#"
delete from users where id = $1
            "#,
        )
        .bind(user.id)
        .execute(&pool)
        .await
        .expect("[cleanup] failed to delete test user");
    }
}

pub mod test_utils {
    use axum::async_trait;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };
    use super::*;

    type UserData = HashMap<i32, User>;

    #[derive(Debug, Clone)]
    pub struct UserRepositoryForMemory {
        store: Arc<RwLock<UserData>>,
    }

    impl UserRepositoryForMemory {
        pub fn new() -> Self {
            UserRepositoryForMemory {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<UserData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<UserData> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl UserRepository for UserRepositoryForMemory {
        async fn create(&self, payload: CreateUser) -> anyhow::Result<User> {
            let mut store = self.write_store_ref();

            let duplicate = store.values().find(|u| u.name == payload.name);
            if let Some(user) = duplicate {
                return Err(RepositoryError::Duplicate(user.id).into());
            }

            let id = (store.len() + 1) as i32;
            let user = User::new(id, payload.name.clone());
            store.insert(id, user.clone());
            Ok(user)
        }

        async fn all(&self) -> anyhow::Result<Vec<User>> {
            let store = self.read_store_ref();
            Ok(Vec::from_iter(store.values().map(|user| user.clone())))
        }

        async fn find_by_name(&self, name: String) -> anyhow::Result<User> {
            let store = self.read_store_ref();
            let user = store
                .values()
                .find(|u| u.name == name)
                .cloned()
                .ok_or(RepositoryError::NotFound(0))?;
            Ok(user)
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn user_crud_scenario() {
            let name = "test user".to_string();
            let id = 1;
            let expected = User::new(id, name.clone());

            // create
            let repository = UserRepositoryForMemory::new();
            let user = repository
                .create(CreateUser::new(name.clone()))
                .await
                .expect("failed create user");
            assert_eq!(expected, user);

            // all
            let users = repository.all().await.unwrap();
            assert_eq!(vec![expected.clone()], users);

            // find_by_name
            let user = repository
                .find_by_name(name.clone())
                .await
                .expect("failed find_by_name");
            assert_eq!(expected, user);

            // duplicate check
            let res = repository.create(CreateUser::new(name)).await;
            assert!(res.is_err());
        }
    }
}
