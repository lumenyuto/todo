use sqlx::PgPool;
use crate::models::user::{CreateUser, UpdateUser, User};
use super::RepositoryError;

pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateUser) -> impl Future<Output = anyhow::Result<User>> + Send;
    fn find(&self, id: i32) -> impl Future<Output = anyhow::Result<User>> + Send;
    fn find_by_sub(&self, sub: String) -> impl Future<Output = anyhow::Result<User>> + Send;
    fn update_name(&self, sub: String, payload: UpdateUser) -> impl Future<Output = anyhow::Result<User>> + Send;
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

impl UserRepository for UserRepositoryForDb {
    async fn create(&self, payload: CreateUser) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
insert into users (sub, name, email)
values ($1, $2, $3)
on conflict (sub) do update set email = $3
returning *
            "#,
        )
        .bind(payload.sub.clone())
        .bind(payload.name.clone())
        .bind(payload.email.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find(&self, id: i32) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
select * from users where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(0),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(user)
    }

    async fn find_by_sub(&self, sub: String) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
select * from users where sub = $1
            "#,
        )
        .bind(sub)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(0),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(user)
    }

    async fn update_name(&self, sub: String, payload: UpdateUser) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
update users set name = $1 where sub = $2
returning *
            "#,
        )
        .bind(payload.name.clone())
        .bind(sub)
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
    use dotenvy::dotenv;
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
        let user_sub = "auth0|test_user";
        let user_name = "test_user";
        let user_email = "test_user@example.com";

        // create
        let created = repository
            .create(CreateUser::new(user_sub.to_string(), user_name.to_string(), user_email.to_string()))
            .await
            .expect("[create] returned Err");
        assert_eq!(created.name, Some(user_name.to_string()));

        // find
        let user = repository
            .find(created.id)
            .await
            .expect("[find] returned Err");
        assert_eq!(created, user);
    }
}

#[cfg(test)]
pub mod test_utils {
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
    
    impl UserRepository for UserRepositoryForMemory {
        async fn create(&self, payload: CreateUser) -> anyhow::Result<User> {
            let mut store = self.write_store_ref();

            let existing_id = store
                .values()
                .find(|u| u.sub == payload.sub)
                .map(|u| u.id);

            if let Some(id) = existing_id {
                let updated = User::new(
                    id,
                    payload.sub.clone(),
                    Some(payload.name.clone()),
                    Some(payload.email.clone()),
                );
                store.insert(id, updated.clone());
                return Ok(updated);
            }

            let id = (store.len() + 1) as i32;
            let user = User::new(id, payload.sub.clone(), Some(payload.name.clone()), Some(payload.email.clone()));
            store.insert(id, user.clone());
            Ok(user)
        }

        async fn find(&self, id: i32) -> anyhow::Result<User> {
            let store = self.read_store_ref();
            let user = store
                .values()
                .find(|u| u.id == id)
                .cloned()
                .ok_or(RepositoryError::NotFound(0))?;
            Ok(user)
        }

        async fn find_by_sub(&self, sub: String) -> anyhow::Result<User> {
            let store = self.read_store_ref();
            let user = store
                .values()
                .find(|u| u.sub == sub)
                .cloned()
                .ok_or(RepositoryError::NotFound(0))?;
            Ok(user)
        }

        async fn update_name(&self, sub: String, payload: UpdateUser) -> anyhow::Result<User> {
            let mut store = self.write_store_ref();
            let user = store
                .values_mut()
                .find(|u| u.sub == sub)
                .ok_or(RepositoryError::NotFound(0))?;
            user.name = Some(payload.name);
            Ok(user.clone())
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn user_crud_scenario() {
            let sub = "auth0|user".to_string();
            let name = "user_name".to_string();
            let email = "user@example.com".to_string();
            let id = 1;
            let expected = User::new(id, sub.clone(), Some(name.clone()), Some(email.clone()));

            // create
            let repository = UserRepositoryForMemory::new();
            let user = repository
                .create(CreateUser::new(sub.clone(), name.clone(), email.clone()))
                .await
                .expect("failed create user");
            assert_eq!(expected, user);

            // find
            let user = repository
                .find(user.id)
                .await
                .expect("failed find user");
            assert_eq!(expected, user);

            // find_by_sub
            let user = repository
                .find_by_sub(sub.clone())
                .await
                .expect("failed find_by_sub");
            assert_eq!(expected, user);
        }
    }
}
