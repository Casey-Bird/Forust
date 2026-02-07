
use crate::elements::{Element, ElementManager};


struct NavigationElement {
    module: Element,

}


impl NavigationElement {


    pub async fn new() -> Self {

        let navigation_element: Element = Element::new("navigation").await;


        Self {
            module: navigation_element,
        }
    }


}







