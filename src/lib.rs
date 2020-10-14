use wasm_bindgen::prelude::*;
use web_sys::window;

mod rectangle;

use rectangle::Point;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let body = document.body().expect("Document should have a body");

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
