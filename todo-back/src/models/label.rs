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
}

impl CreateLabel {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdateLabel {
    pub id: i32,
    pub name: String,
}