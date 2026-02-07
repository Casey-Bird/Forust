mod util;
mod elements;

use std::path::Path;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

use elements::{Element, ElementManager};


#[tokio::main]
pub async fn main() {

    let app: Router = create_app();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3080").await.expect("Failed to bind tcp listener");

    // Create a new element manager to load, store and process all elements in the elements directory
    let mut element_manager: ElementManager = ElementManager::new().await;

    // TODO Load elements

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

}

fn create_app() -> Router {

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))



}


pub async fn load_elements(element_manager: ElementManager)  {
    let elements_dir: &Path = Path::new("src/elements");

    if !elements_dir.exists() {
        println!("Couldn't find elements directory");
    }


}


