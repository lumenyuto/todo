use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct WorkspaceEntity {
    pub id: i32,
    pub name: String,
    pub is_personal: bool,
    pub users: Vec<User>,
}

impl WorkspaceEntity {
    pub fn new(id: i32, name: String, is_personal: bool, users: Vec<User>) -> Self {
        Self {
            id,
            name,
            is_personal,
            users,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateWorkspace {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over name length"))]
    pub name: String,
    #[serde(default)]
    pub is_personal: bool,
    #[serde(default)]
    pub user_emails: Vec<String>,
}

impl CreateWorkspace {
    pub fn new(name: String, is_personal: bool, user_emails: Vec<String>) -> Self {
        Self {
            name,
            is_personal,
            user_emails,
        }
    }
}
