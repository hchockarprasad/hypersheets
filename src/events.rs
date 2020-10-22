use super::*;

use rectangle::{BoundingRect, Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CustomEventDetail {
  event: Option<String>,
  mouse: Option<Point>,
}

impl CustomEventDetail {
  pub fn new(mouse: Point) -> Self {
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

pub trait CustomEvent {
  fn create_custom_event(
    &self,
    name: &str,
    bubbles: bool,
    cancelable: bool,
    detail: CustomEventDetail,
  ) -> web_sys::CustomEvent;
}

pub trait MousePosition {
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

impl MousePosition for web_sys::MouseEvent {
  fn location(&self, boundary: BoundingRect, zoom_factor: f64) -> Point {
    let x = self.client_x() as f64 / zoom_factor - boundary.left();
    let y = self.client_y() as f64 / zoom_factor - boundary.top();
    Point::new(x, y)
  }
}
