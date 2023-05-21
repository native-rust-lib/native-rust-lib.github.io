use quiz_core::asynchronous::fetch_categories_async;
use quiz_core::blocking::fetch_categories_blocking;
use tokio::runtime::Runtime;

fn main() {
    fetch_categories_blocking();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        fetch_categories_async().await;
    });
}
