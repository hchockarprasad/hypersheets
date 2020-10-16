#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement};

mod helpers;
mod hypersheet;
mod rectangle;
use helpers::dom::element::ElementHelper;
use hypersheet::HyperSheet;
use rectangle::Point;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn start() {
    let sheet_no = 1;
    let mut sheet = HyperSheet::new("container");
    console::log_1(&"Sheet found".into());
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
