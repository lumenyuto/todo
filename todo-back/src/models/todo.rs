use validator::Validate;
use serde::{Deserialize, Serialize};
use super::{
    label::Label,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
    pub user_id: i32,
}

impl TodoEntity {
    pub fn new(id: i32, text:String, labels: Vec<Label>, user_id: i32) -> Self {
        Self {
            id,
            text,
            completed: false,
            labels,
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: String,
    pub label_ids: Vec<i32>,
}

impl CreateTodo {
    pub fn new(text: String, label_ids: Vec<i32>) -> Self {
        Self {
            text,
            label_ids,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub label_ids: Option<Vec<i32>>,
}