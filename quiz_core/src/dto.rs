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

#[derive(Deserialize, Debug)]
pub struct QuestionResponse {
    pub response_code: u32,
    pub results: Vec<Question>,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    category: String,
    #[serde(rename = "type")]
    question_type: QuestionType,
    difficulty: QuestionDifficulty,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub enum QuestionType {
    #[serde(rename = "multiple")]
    Multiple,
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Deserialize, Debug)]
pub enum QuestionDifficulty {
    #[serde(rename = "easy")]
    Easy,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "hard")]
    Hard,
}
