use crate::dto::CategoryResponse;
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
