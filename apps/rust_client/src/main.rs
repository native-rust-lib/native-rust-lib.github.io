use quiz_core::asynchronous::{fetch_categories_async, fetch_questions_async};
use quiz_core::blocking::{fetch_categories_blocking, fetch_questions_blocking};
use quiz_core::dto::QuestionRequest;
use tokio::runtime::Runtime;

fn main() {
    fetch_categories_blocking();
    fetch_questions_blocking(QuestionRequest::new(10));
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        fetch_categories_async().await;
        fetch_questions_async(QuestionRequest::new(10)).await;
    });
}
