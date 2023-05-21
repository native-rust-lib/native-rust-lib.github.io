use reqwest::blocking::Client;

use crate::dto::CategoryResponse;

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
