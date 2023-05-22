use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Category {
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryResponse {
    trivia_categories: Vec<Category>,
}

#[derive(Serialize, Debug)]
pub struct QuestionRequest {
    amount: u32,
}

impl QuestionRequest {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }
}
