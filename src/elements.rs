use std::collections::HashMap;
use axum::Router;

mod navigation;
mod meta;

///
pub struct Element {
    name: String,
    routers: Vec<Router>,
}

impl Element {


    pub async fn new(name: &str) -> Element {
        Self {
            name: name.to_string(),
            routers: Vec::new(),
        }
    }


}



pub struct ElementManager {
    element_map: HashMap<String, Element>,
}

impl ElementManager {
    pub async fn new() -> ElementManager {
        Self {
            element_map: HashMap::new(),
        }
    }

    pub async fn add(&mut self, element: Element) {
        self.element_map.insert(element.name.clone(), element);
    }

}



