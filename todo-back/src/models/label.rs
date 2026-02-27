use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Label {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl Label {
    pub fn new(id: i32, name: String, user_id: i32) -> Self {
        Self {
            id,
            name,
            user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Validate)]
pub struct CreateLabel {
    #[validate(length(min = 1, message = "Can no be empty"))]
    #[validate(length(max = 100, message = "Over text langth"))]
    pub name: String,
    pub user_id: i32,
}

impl CreateLabel {
    pub fn new(name: String, user_id: i32) -> Self {
        Self {
            name,
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdateLabel {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate, sqlx::FromRow)]
pub struct DeleteLabel {
    pub id: i32,
    pub user_id: i32,
}

impl DeleteLabel {
    pub fn new(id: i32, user_id: i32) -> Self {
        Self {
            id,
            user_id,
        }
    }
}