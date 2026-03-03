use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct TeamEntity {
    pub id: i32,
    pub name: String,
    pub users: Vec<User>,
}

impl TeamEntity {
    pub fn new(id: i32, name: String, users: Vec<User>) -> Self {
        Self {
            id,
            name,
            users,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTeam {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over name length"))]
    pub name: String,
    pub user_ids: Vec<i32>,
}

impl CreateTeam {
    fn new(name: String, user_ids: Vec<i32>) -> Self {
        Self {
            name,
            user_ids,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTeam {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over name length"))]
    pub name: String,
    pub user_ids: Vec<i32>,
}