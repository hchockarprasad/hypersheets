#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

mod helpers;
mod hypersheet;
use hypersheet::HyperSheet;

lazy_static! {
    pub static ref HYPER_SHEETS: Mutex<HashMap<usize, StateRenderer>> = {
        let mut x = HashMap::new();
        x.insert(1, StateRenderer::new());
        Mutex::new(x)
    };
}

pub struct StateRenderer {
    idx: u8,
}

impl StateRenderer {
    pub fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn add(&mut self) -> u8 {
        self.idx += 1;
        self.idx
    }
}

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let mut sheet = HyperSheet::new("container");
    console::log_1(&"Sheet found".into());
}
