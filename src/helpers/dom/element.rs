use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlElement, HtmlInputElement, Selection, Window};

pub struct ElementOffset {
  left: f64,
  top: f64,
}

pub trait ElementHelper {
  fn offset(&self) -> ElementOffset;
  fn is_input(&self) -> bool;
  fn is_outside_input(&self) -> bool;
  fn select_element_if_allowed(&self);
  fn has_horizontal_scrollbar(&self) -> bool;
  fn has_vertical_scrollbar(&self) -> bool;
  fn set_caret_position(&self, start_pos: u32, end_pos: Option<u32>);
  fn get_caret_position(&self) -> u32;
  fn get_selection_end_position(&self) -> u32;
}

pub trait WindowHelper {
  fn get_selection_text(&self) -> String;
  fn clear_selection_text(&self);
}

impl WindowHelper for Window {
  fn get_selection_text(&self) -> String {
    let selection = JsValue::from(self.get_selection().unwrap().unwrap())
      .as_string()
      .unwrap();
    selection
  }

  fn clear_selection_text(&self) {
    self.get_selection().unwrap().unwrap().empty().unwrap();
  }
}

impl ElementHelper for HtmlElement {
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

  fn is_input(&self) -> bool {
    let inputs: [&str; 3] = ["INPUT", "SELECT", "TEXTAREA"];
    inputs.contains(&&*self.node_name()) || self.content_editable() == "true"
  }

  fn is_outside_input(&self) -> bool {
    self.is_input() && self.has_attribute("data-hyper-input") == false
  }

  fn select_element_if_allowed(&self) {
    let active_element = self
      .owner_document()
      .unwrap()
      .active_element()
      .unwrap()
      .dyn_into::<HtmlInputElement>()
      .unwrap();
    if !active_element.is_outside_input() {
      active_element.select();
    }
  }

  fn has_horizontal_scrollbar(&self) -> bool {
    self.offset_height() != self.client_height()
  }

  fn has_vertical_scrollbar(&self) -> bool {
    self.offset_width() != self.client_width()
  }

  fn set_caret_position(&self, start_pos: u32, end_pos: Option<u32>) {
    let start = start_pos;
    let end = end_pos.unwrap_or(start_pos);
    let element = self.dyn_ref::<HtmlInputElement>().unwrap();
    if self.is_input() {
      element.focus().unwrap();
      match element.set_selection_range(start, end) {
        Ok(_) => {}
        Err(_) => {
          let parent = element.parent_element().unwrap().dyn_into::<HtmlElement>().unwrap();
          let parent_display_value = parent.style().get_property_value("display").unwrap();
          parent.style().set_property("display", "block").unwrap();
          element.set_selection_range(start, end).unwrap();
          parent.style().set_property("display", &parent_display_value).unwrap();
        }
      }
    }
  }

  fn get_caret_position(&self) -> u32 {
    if self.is_input() {
      return self
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .selection_start()
        .unwrap()
        .unwrap();
    }
    0
  }

  fn get_selection_end_position(&self) -> u32 {
    if self.is_input() {
      return self
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .selection_end()
        .unwrap()
        .unwrap();
    }
    0
  }
}
