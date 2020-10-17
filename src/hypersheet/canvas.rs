use super::super::*;
use super::*;
use rectangle::{BoundingRect, Point};

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlElement};

pub trait CanvasHelper {
  fn get_bounding_rect(&self) -> BoundingRect;
}

impl CanvasHelper for HtmlCanvasElement {
  fn get_bounding_rect(&self) -> BoundingRect {
    let rect = self.get_bounding_client_rect();
    BoundingRect::new(
      rect.left(),
      rect.top(),
      rect.top(),
      rect.right(),
      rect.bottom(),
      rect.left(),
      rect.width(),
      rect.height(),
    )
  }
}

pub struct Canvas {
  element: HtmlCanvasElement,
  info_element: HtmlElement,
  mouse_location: Point,
}

impl Canvas {
  pub fn new(wrapper: HtmlElement) -> Self {
    let document = wrapper.owner_document().unwrap();
    let info_element = document
      .create_element("div")
      .unwrap()
      .dyn_into::<HtmlElement>()
      .unwrap();
    info_element.set_class_name("info");
    wrapper.append_child(&info_element).unwrap();
    let canvas = document
      .create_element("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    canvas.style().set_property("outline", "none").unwrap();
    wrapper.append_child(&canvas).unwrap();
    Canvas {
      info_element,
      element: canvas,
      mouse_location: Point::new(-1.0, -1.0),
    }
  }

  pub fn init_listeners(&self) {
    let document = self.element.owner_document().unwrap();
    let elm = self.element.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let pt = event.location(elm.get_bounding_rect(), 1.0);
      let ce = event.create_custom_event("test", false, false, CustomEventDetail::new(pt));
      let dispatched = elm.dispatch_event(&ce).unwrap();
      let mut state = HYPER_SHEETS.lock().unwrap();
      let x = (*state.get_mut(&1).unwrap()).add();
      console::log_1(&format!("Mouse down triggerd, {:?}", x.to_string()).into());
    }) as Box<dyn FnMut(_)>);
    document
      .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
      .unwrap();
    closure.forget();
  }
}
