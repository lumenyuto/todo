use validator::Validate;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl User {
    pub fn new(id: i32, sub: String, name: Option<String>, email: Option<String>) -> Self {
        Self { id, sub, name, email }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub sub: String,
    pub name: String,
    pub email: String,
}

impl CreateUser {
    pub fn new(sub: String, name: String, email: String) -> Self {
        Self { sub, name, email }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}
