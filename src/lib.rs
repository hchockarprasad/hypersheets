#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement};

mod canvas;
mod helpers;
mod hypersheet;
use canvas::Canvas;
use helpers::dom::element::ElementHelper;
use hypersheet::HyperSheet;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let mut container = document
        .get_element_by_id("container")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let mut canvas = Canvas::new(container);
    console::log_1(&"Sheet found".into());
}

#[cfg(test)]
mod lib {

    use super::*;

    #[test]
    fn check_distance() {}
}
