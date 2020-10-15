use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement};

mod helpers;
mod rectangle;
use helpers::dom::element::DomHelper;
use rectangle::Point;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let body = document.body().expect("Document should have a body");
    let canvas = document
        .get_element_by_id("rustcanvas")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let offset = canvas.offset();
    console::log_1(&format!("Top {}, Left {}", offset.top, offset.left).into());
    let pt = Point::new(10, 10);
}

#[cfg(test)]
mod lib {

    use super::*;

    #[test]
    fn check_distance() {
        let pt1 = Point::new(5, 0);
        let pt2 = Point::origin();

        let dist = pt1.distance(pt2);
        assert_eq!(5.0, dist);
    }
}
