use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use crate::models::{
    workspace::{CreateWorkspace, WorkspaceEntity},
    user::User,
};
use super::RepositoryError;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct WorkspaceWithUserFromRow {
    id: i32,
    name: String,
    is_personal: bool,
    user_id: Option<i32>,
    user_sub: Option<String>,
    user_name: Option<String>,
    user_email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct WorkspaceFromRow {
    id: i32,
    name: String,
    is_personal: bool,
}

fn fold_entities(rows: Vec<WorkspaceWithUserFromRow>) -> Vec<WorkspaceEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<WorkspaceEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let mut workspaces = accum.iter_mut();
        while let Some(ws) = workspaces.next() {
            if ws.id == row.id {
                if let (Some(user_id), Some(user_sub)) =
                    (row.user_id, row.user_sub.clone())
                {
                    ws.users.push(User {
                        id: user_id,
                        sub: user_sub,
                        name: row.user_name.clone(),
                        email: row.user_email.clone(),
                    });
                }
                continue 'outer;
            }
        }

        let users = if let (Some(user_id), Some(user_sub))
            = (row.user_id, row.user_sub.clone())
        {
            vec![User {
                id: user_id,
                sub: user_sub,
                name: row.user_name.clone(),
                email: row.user_email.clone(),
            }]
        } else {
            vec![]
        };

        accum.push(WorkspaceEntity {
            id: row.id,
            name: row.name.clone(),
            is_personal: row.is_personal,
            users,
        });
    }
    accum
}

#[async_trait]
pub trait WorkspaceRepository: Send + Sync + 'static {
    async fn create(&self, user_id: i32, payload: CreateWorkspace) -> anyhow::Result<WorkspaceEntity>;
    async fn find(&self, id: i32) -> anyhow::Result<WorkspaceEntity>;
    async fn all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<WorkspaceEntity>>;
    async fn is_member(&self, id: i32, user_id: i32) -> anyhow::Result<bool>;
}

#[derive(Debug, Clone)]
pub struct WorkspaceRepositoryForDb {
    pool: PgPool,
}

impl WorkspaceRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        WorkspaceRepositoryForDb { pool }
    }
}

#[async_trait]
impl WorkspaceRepository for WorkspaceRepositoryForDb {
    async fn create(&self, user_id: i32, payload: CreateWorkspace) -> anyhow::Result<WorkspaceEntity> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, WorkspaceFromRow>(
            r#"
insert into workspaces (name, is_personal)
values ($1, $2)
returning id, name, is_personal
            "#,
        )
        .bind(payload.name.clone())
        .bind(payload.is_personal)
        .fetch_one(&mut *tx)
        .await?;

        // 作成者を必ずメンバーに追加
        sqlx::query(
            r#"
insert into workspace_users (workspace_id, user_id)
values ($1, $2)
            "#,
        )
        .bind(row.id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // emailで指定された追加メンバーを追加
        if !payload.user_emails.is_empty() {
            sqlx::query(
                r#"
insert into workspace_users (workspace_id, user_id)
select $1, users.id
from unnest ($2::text[]) as t(email)
inner join users on users.email = t.email
where users.id != $3
                "#,
            )
            .bind(row.id)
            .bind(payload.user_emails)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        let workspace = self.find(row.id).await?;
        Ok(workspace)
    }

    async fn find(&self, id: i32) -> anyhow::Result<WorkspaceEntity> {
        let items = sqlx::query_as::<_, WorkspaceWithUserFromRow>(
            r#"
select workspaces.id, workspaces.name, workspaces.is_personal,
       users.id as user_id,
       users.sub as user_sub,
       users.name as user_name,
       users.email as user_email
from workspaces
            left outer join workspace_users wu on workspaces.id = wu.workspace_id
            left outer join users on users.id = wu.user_id
where workspaces.id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        let workspaces = fold_entities(items);
        let workspace = workspaces.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(workspace.clone())
    }

    async fn all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<WorkspaceEntity>> {
        let items = sqlx::query_as::<_, WorkspaceWithUserFromRow>(
            r#"
select workspaces.id, workspaces.name, workspaces.is_personal,
       users.id as user_id,
       users.sub as user_sub,
       users.name as user_name,
       users.email as user_email
from workspaces
            inner join workspace_users wu on workspaces.id = wu.workspace_id
            left outer join workspace_users wu2 on workspaces.id = wu2.workspace_id
            left outer join users on users.id = wu2.user_id
where wu.user_id = $1
order by workspaces.is_personal desc, workspaces.id desc
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let workspaces = fold_entities(items);
        Ok(workspaces)
    }

    async fn is_member(&self, id: i32, user_id: i32) -> anyhow::Result<bool> {
        let row = sqlx::query(
            r#"
select 1 from workspace_users
where workspace_id = $1 and user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.is_some())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
}

#[cfg(test)]
pub mod test_utils {
    use::std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };
    use super::*;

    type WorkspaceData = HashMap<i32, WorkspaceEntity>;

    #[derive(Debug, Clone)]
    pub struct WorkspaceRepositoryForMemory {
        store: Arc<RwLock<WorkspaceData>>,
    }

    impl WorkspaceRepositoryForMemory {
        pub fn new() -> Self {
            WorkspaceRepositoryForMemory {
                store: Arc::default(),
            }
        }
        fn write_store_ref(&self) -> RwLockWriteGuard<WorkspaceData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<WorkspaceData> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl WorkspaceRepository for WorkspaceRepositoryForMemory {
        async fn create(&self, _user_id: i32, payload: CreateWorkspace) -> anyhow::Result<WorkspaceEntity> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let workspace = WorkspaceEntity::new(id, payload.name, payload.is_personal, vec![]);
            store.insert(id, workspace.clone());
            Ok(workspace)
        }

        async fn find(&self, id: i32) -> anyhow::Result<WorkspaceEntity> {
            let store = self.read_store_ref();
            store.get(&id).cloned().ok_or(anyhow::anyhow!("not found"))
        }

        async fn all_by_user(&self, _user_id: i32) -> anyhow::Result<Vec<WorkspaceEntity>> {
            let store = self.read_store_ref();
            Ok(store.values().cloned().collect())
        }

        async fn is_member(&self, _id: i32, _user_id: i32) -> anyhow::Result<bool> {
            Ok(true)
        }
    }
}
