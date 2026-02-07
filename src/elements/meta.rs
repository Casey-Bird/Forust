use crate::elements::Element;

struct MetaElement {
    element: Element,

}


impl MetaElement {


    pub async fn new(element: Element) -> Self {
        Self { element }
    }

    pub async fn run_me() {
        println!("Test 2");
    }




}









