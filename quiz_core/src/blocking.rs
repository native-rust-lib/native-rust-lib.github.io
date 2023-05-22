use crate::dto::{CategoryResponse, QuestionRequest};
use reqwest::blocking::Client;

pub fn fetch_categories_blocking() {
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api_category.php")
        .send()
        .unwrap()
        .json::<CategoryResponse>()
        .unwrap();

    println!("{:#?}", res);
}

pub fn fetch_questions_blocking(query_params: QuestionRequest) {
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api.php")
        .query(&query_params)
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{:#?}", res);
}
