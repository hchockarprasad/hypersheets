pub mod rectangle;
use rectangle::{Point, Rectangle};
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCanvasElement, HtmlElement, MouseEvent};

#[derive(Serialize, Deserialize)]
struct CustomEventDetail {
  event: Option<String>,
  mouse: Option<Point>,
}

impl CustomEventDetail {
  fn new(mouse: Point) -> Self {
    CustomEventDetail {
      event: None,
      mouse: None,
    }
  }

  fn set_mouse_point(&mut self, point: Point) {
    self.mouse = Some(point);
  }

  fn set_event(&mut self, event: String) {
    self.event = Some(event);
  }
}

trait CanvasHelper {
  fn get_bounding_rect(&self) -> BoundingRect;
}

trait CustomEvent {
  fn create_custom_event(
    &self,
    name: &str,
    bubbles: bool,
    cancelable: bool,
    detail: CustomEventDetail,
  ) -> web_sys::CustomEvent;
}

trait MousePosition {
  fn location(&self, boundary: BoundingRect, zoom_factor: f64) -> Point;
}

impl CustomEvent for web_sys::Event {
  fn create_custom_event(
    &self,
    name: &str,
    bubbles: bool,
    cancelable: bool,
    mut detail: CustomEventDetail,
  ) -> web_sys::CustomEvent {
    let mut custom_event_init = web_sys::CustomEventInit::new();
    custom_event_init.bubbles(bubbles);
    custom_event_init.cancelable(cancelable);
    detail.set_event(self.type_());
    custom_event_init.detail(&JsValue::from_serde(&detail).unwrap());
    web_sys::CustomEvent::new_with_event_init_dict(name, &custom_event_init).unwrap()
  }
}

impl MousePosition for MouseEvent {
  fn location(&self, boundary: BoundingRect, zoom_factor: f64) -> Point {
    let x = self.client_x() as f64 / zoom_factor - boundary.left;
    let y = self.client_y() as f64 / zoom_factor - boundary.top;
    Point::new(x, y)
  }
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

pub struct BoundingRect {
  x: f64,
  y: f64,
  top: f64,
  right: f64,
  bottom: f64,
  left: f64,
  width: f64,
  height: f64,
}

impl BoundingRect {
  pub fn new(x: f64, y: f64, top: f64, right: f64, bottom: f64, left: f64, width: f64, height: f64) -> Self {
    BoundingRect {
      x,
      y,
      top,
      right,
      bottom,
      left,
      width,
      height,
    }
  }
}

pub struct Canvas {
  wrapper: HtmlElement,
  canvas: HtmlCanvasElement,
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
    let canvas_clone = canvas.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let pt = event.location(canvas_clone.get_bounding_rect(), 1.0);
      let ce = event.create_custom_event("test", false, false, CustomEventDetail::new(pt));
      let dispatched = canvas_clone.dispatch_event(&ce).unwrap();
      console::log_1(&format!("Mouse moved, {:?}", dispatched).into());
    }) as Box<dyn FnMut(_)>);
    document
      .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
      .unwrap();
    closure.forget();

    Canvas {
      wrapper,
      info_element,
      canvas,
      mouse_location: Point::new(-1.0, -1.0),
    }
  }

  fn get_local(&self) {}
}
