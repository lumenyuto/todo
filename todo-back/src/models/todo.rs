use validator::Validate;
use serde::{Deserialize, Serialize};
use super::label::Label;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: String,
    pub labels: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub labels: Option<Vec<i32>>,
}

impl TodoEntity {
    pub fn new(id: i32, text:String, labels: Vec<Label>) -> Self {
        Self {
            id,
            text,
            completed: false,
            labels,
        }
    }
}

impl CreateTodo {
    pub fn new(text: String, labels: Vec<i32>) -> Self {
        Self { text, labels }
    }
}