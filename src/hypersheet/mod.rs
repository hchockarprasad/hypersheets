mod rectangle;

mod canvas;
mod cell;
mod celleditor;
mod events;
mod model;
mod properties;
mod scroll;

use canvas::{Canvas, CanvasHelper};
use celleditor::CellEditor;
use events::{CustomEvent, CustomEventDetail, MousePosition};
use js_sys::Array;
use model::DataModel;
use properties::HyperSheetProperties;
use scroll::ScrollBar;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement, MouseEvent};

struct CellEvent {}

enum Edge {
  Top,
  Bottom,
  Left,
  Right,
  Empty,
}

pub struct HyperSheet {
  container: HtmlElement,
  canvas: Canvas,
  properties: HyperSheetProperties,
  cellEditor: Option<CellEditor>,
}

impl HyperSheet {
  pub fn new(container_name: &str) -> Self {
    let data_model = DataModel::new();
    let container = HyperSheet::init_container(container_name);
    let canvas = HyperSheet::init_canvas(container_name);
    let instance = Self {
      container: container,
      canvas,
      properties: HyperSheetProperties::default(),
      cellEditor: None,
    };
    instance.reset_grid_border(Edge::Top);
    instance.reset_grid_border(Edge::Bottom);
    instance.reset_grid_border(Edge::Left);
    instance.reset_grid_border(Edge::Right);
    instance
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

  fn create_cell_editor(&self, cell_event: CellEvent) -> CellEditor {
    CellEditor::new("Test")
  }

  fn reset_grid_border(&self, edge: Edge) {
    let style_prefix = "border";
    let (prop_name, style_name) = match edge {
      Top => (self.properties.grid_border_top, [style_prefix, "Top"].concat()),
      Right => (self.properties.grid_border_right, [style_prefix, "Right"].concat()),
      Bottom => (self.properties.grid_border_bottom, [style_prefix, "Bottom"].concat()),
      Left => (self.properties.grid_border_left, [style_prefix, "Left"].concat()),
      Empty => (self.properties.grid_border, [style_prefix, ""].concat()),
    };
    let border = match prop_name {
      true => [
        self.properties.fixed_lines_hwidth.to_string(),
        "px solid".to_string(),
        self.properties.grid_lines_hcolor.clone(),
      ]
      .concat(),
      false => "".to_string(),
    };
    self.canvas.element.style().set_property("border", &border).unwrap();
  }

  fn get_scroll_top(&self) {}

  fn compute_cell_bounds(&self) {
    self.get_scroll_top();
  }
}
