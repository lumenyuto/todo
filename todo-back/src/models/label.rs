use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Validate)]
pub struct CreateLabel {
    #[validate(length(min = 1, message = "Can no be empty"))]
    #[validate(length(max = 100, message = "Over text langth"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdateLabel {
    pub id: i32,
    pub name: String,
}

impl Label {
    pub fn new(id: i32, name:String) -> Self {
        Self {
            id,
            name,
        }
    }
}

impl CreateLabel {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}