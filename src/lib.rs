#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use web_sys::{console, window};

mod helpers;
mod hypersheet;
use hypersheet::HyperSheet;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let sheet = HyperSheet::new();
    console::log_1(&"Sheet found".into());
}
