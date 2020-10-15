use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement};

pub struct ElementOffset {
  pub left: f64,
  pub top: f64,
}

pub trait DomHelper {
  fn offset(&self) -> ElementOffset;
}

impl DomHelper for HtmlElement {
  fn offset(&self) -> ElementOffset {
    let root_documemt = self.owner_document().unwrap();
    let root_window = root_documemt.default_view().unwrap();
    let document_element = root_documemt.document_element().unwrap();

    let mut element_to_check = Some(self.clone());
    let mut offset_left: f64 = 0 as f64;
    let mut offset_top: f64 = 0 as f64;
    let mut last_elem = element_to_check.clone();
    element_to_check = Some(
      element_to_check
        .clone()
        .unwrap()
        .parent_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap(),
    );

    loop {
      if element_to_check.is_none() || element_to_check.clone().unwrap() == root_documemt.body().unwrap() {
        break;
      }
      offset_left += element_to_check
        .clone()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .offset_left() as f64;
      offset_top += element_to_check
        .clone()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .offset_top() as f64;
      element_to_check = Some(
        element_to_check
          .clone()
          .unwrap()
          .parent_element()
          .unwrap()
          .dyn_into::<HtmlElement>()
          .unwrap(),
      );
      last_elem = element_to_check.clone();
    }
    if last_elem.is_some() && last_elem.unwrap().style().get_property_value("position").unwrap() == "fixed" {
      offset_left += match root_window.page_x_offset() {
        Ok(offset) => offset,
        _ => document_element.scroll_left() as f64,
      };
      offset_top += match root_window.page_y_offset() {
        Ok(offset) => offset,
        _ => document_element.scroll_top() as f64,
      };
    }
    ElementOffset {
      left: offset_left,
      top: offset_top,
    }
  }
}
