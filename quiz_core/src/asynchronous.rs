use crate::dto::{CategoryResponse, QuestionRequest, QuestionResponse};
use reqwest::Client;

pub async fn fetch_categories_async() {
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api_category.php")
        .send()
        .await
        .unwrap()
        .json::<CategoryResponse>()
        .await
        .unwrap();

    println!("{:#?}", res);
}

pub async fn fetch_questions_async(query_params: QuestionRequest) {
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api.php")
        .query(&query_params)
        .send()
        .await
        .unwrap()
        .json::<QuestionResponse>()
        .await
        .unwrap();

    println!("{:#?}", res);
}
