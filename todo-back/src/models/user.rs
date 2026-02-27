use validator::Validate;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub name: String,
}

impl CreateUser {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
