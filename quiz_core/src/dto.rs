use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryResponse {
    trivia_categories: Vec<Category>,
}
