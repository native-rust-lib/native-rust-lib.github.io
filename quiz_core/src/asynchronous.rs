use crate::dto::CategoryResponse;
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
