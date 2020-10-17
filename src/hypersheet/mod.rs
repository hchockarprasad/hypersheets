mod rectangle;

mod canvas;
mod celleditor;
mod events;
mod properties;

use canvas::{Canvas, CanvasHelper};
use celleditor::CellEditor;
use events::{CustomEvent, CustomEventDetail, MousePosition};
use js_sys::Array;
use properties::HyperSheetProperties;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement, MouseEvent};

pub struct HyperSheet {
  container: HtmlElement,
  canvas: Canvas,
  properties: HyperSheetProperties,
  cellEditor: Option<CellEditor>,
}

impl HyperSheet {
  pub fn new(container_name: &str) -> Self {
    let container = HyperSheet::init_container(container_name);
    let canvas = HyperSheet::init_canvas(container_name);
    Self {
      container: container,
      canvas,
      properties: HyperSheetProperties::default(),
      cellEditor: None,
    }
  }

  fn init_container(container_name: &str) -> HtmlElement {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let container = document
      .get_element_by_id(container_name)
      .unwrap()
      .dyn_into::<HtmlElement>()
      .unwrap();
    let prevent_context_menu_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    container.set_oncontextmenu(Some(prevent_context_menu_closure.as_ref().unchecked_ref()));
    prevent_context_menu_closure.forget();
    container.remove_attribute("tabindex").unwrap();
    let class_list = Array::new();
    class_list.set(0, "hypersheet-container".into());
    container.class_list().add(&class_list).unwrap();
    container
  }

  fn init_canvas(container_name: &str) -> Canvas {
    let window = window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let wrapper = document
      .get_element_by_id(container_name)
      .unwrap()
      .dyn_into::<HtmlElement>()
      .unwrap();
    let canvas = Canvas::new(wrapper);
    canvas.init_listeners();
    canvas
  }

  fn get_container_id(&self) -> String {
    self.container.get_attribute("id").unwrap()
  }

  fn attach_cell_editor(&mut self) {
    let ce = CellEditor::new(&self.get_container_id());
    self.cellEditor = Some(ce);
  }

  fn drop_cell_editor(&mut self) {
    self.cellEditor = None;
  }

  fn has_cell_editor(&self) -> bool {
    match self.cellEditor {
      Some(_) => true,
      None => false,
    }
  }
}
