mod api;
mod executor;
mod storage;
mod rejections;

use tokio;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let storage = Arc::new(storage::Storage::init().expect("Failed to initialize storage"));
    warp::serve(api::server(storage)).run(([127, 0, 0, 1], 3030)).await;
}
