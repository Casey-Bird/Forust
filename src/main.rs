mod util;

use std::path::Path;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;


#[tokio::main]
pub async fn main() {

    let app: Router = create_app();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3080").await.expect("Failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

}

fn create_app() -> Router {

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))



}

