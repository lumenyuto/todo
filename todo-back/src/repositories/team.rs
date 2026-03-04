use sqlx::{FromRow, PgPool};
use crate::models::{
    team::{CreateTeam, TeamEntity},
    user::User,
};
use super::RepositoryError;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TeamWithUserFromRow {
    id: i32,
    name: String,
    user_id: Option<i32>,
    user_sub: Option<String>,
    user_name: Option<String>,
    user_email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct TeamFromRow {
    id: i32,
    name: String,
}

fn fold_entities(rows: Vec<TeamWithUserFromRow>) -> Vec<TeamEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<TeamEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let mut teams = accum.iter_mut();
        while let Some(team) = teams.next() {
            if team.id == row.id {
                if let (Some(user_id), Some(user_sub)) = 
                    (row.user_id, row.user_sub.clone())
                {
                    team.users.push(User {
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

        accum.push(TeamEntity {
            id: row.id,
            name: row.name.clone(),
            users,
        });
    }
    accum
} 

pub trait TeamRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTeam) -> impl Future<Output = anyhow::Result<TeamEntity>> + Send;
    fn find(&self, id: i32) -> impl Future<Output = anyhow::Result::<TeamEntity>> + Send;
    fn all_by_user(&self, user_id: i32) -> impl Future<Output = anyhow::Result<Vec<TeamEntity>>> + Send;
    fn is_member(&self, id: i32, user_id: i32) -> impl Future<Output = anyhow::Result<bool>> + Send;
}

#[derive(Debug, Clone)]
pub struct TeamRepositoryForDb {
    pool: PgPool,
}

impl TeamRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TeamRepositoryForDb { pool }
    }
}

impl TeamRepository for TeamRepositoryForDb {
    async fn create(&self, payload: CreateTeam) -> anyhow::Result<TeamEntity> {
        let mut tx = self.pool
            .begin()
            .await?;
        let row = sqlx::query_as::<_, TeamFromRow>(
            r#"
insert into teams (name)
values ($1)
returning id, name
            "#,
            )
            .bind(payload.name.clone())
            .fetch_one(&mut * tx)
            .await?;

        sqlx::query(
            r#"
insert into team_users (team_id, user_id)
select $1, id
from unnest ($2) as t(id)
            "#,
            )
            .bind(row.id)
            .bind(payload.user_ids)
            .execute(&mut * tx)
            .await?;
        
        tx.commit().await?;

        let team = self.find(row.id).await?;
        Ok(team)
    }

    async fn find(&self, id: i32) -> anyhow::Result<TeamEntity> {
        let items = sqlx::query_as::<_, TeamWithUserFromRow>(
            r#"
select teams.id, teams.name,
       users.id as user_id,
       users.sub as user_sub,
       users.name as user_name,
       users.email as user_email
from teams
            left outer join team_users tu on teams.id = tu.team_id
            left outer join users on users.id = tu.user_id
where teams.id = $1
        "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        let teams = fold_entities(items);
        let team = teams.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(team.clone())
    }

    async fn all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<TeamEntity>> {
        let items = sqlx::query_as::<_, TeamWithUserFromRow>(
            r#"
select teams.id, teams.name,
       users.id as user_id,
       users.sub as user_sub,
       users.name as user_name,
       users.email as user_email
from teams
            inner join team_users tu on teams.id = tu.team_id
            left outer join team_users tu2 on teams.id = tu2.team_id
            left outer join users on users.id = tu2.user_id
where tu.user_id = $1
order by teams.id desc;
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let teams = fold_entities(items);
        Ok(teams)
    }

    async fn is_member(&self, id: i32, user_id: i32) -> anyhow::Result<bool> {
        let row = sqlx::query(
            r#"
select 1 from team_users
where team_id = $1 and user_id = $2
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
    use anyhow::Context;
    use::std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };
    use super::*;

    type TeamData = HashMap<i32, TeamEntity>;

    #[derive(Debug, Clone)]
    pub struct TeamRepositoryForMemory {
        store: Arc<RwLock<TeamData>>,
    }

    impl TeamRepositoryForMemory {
        pub fn new() -> Self {
            TeamRepositoryForMemory {
                store: Arc::default(),
            }
        }
        fn write_store_ref(&self) -> RwLockWriteGuard<TeamData> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TeamData> {
            self.store.read().unwrap()
        }
    }

    impl TeamRepository for TeamRepositoryForMemory {
        async fn create(&self, payload: CreateTeam) -> anyhow::Result<TeamEntity> {
            todo!()
        }

        async fn find(&self, id: i32) -> anyhow::Result<TeamEntity> {
            todo!()
        }

        async fn all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<TeamEntity>> {
            todo!()
        }

        async fn is_member(&self, id: i32, user_id: i32) -> anyhow::Result<bool>{
            todo!()
        }
    }
}