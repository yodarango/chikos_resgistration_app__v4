
use warp::Filter;
mod api;
use api::server;

#[tokio::main]
async fn main() {

server::start().await;

}
