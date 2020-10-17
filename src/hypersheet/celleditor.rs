use wasm_bindgen::JsCast;

pub struct CellEditor {
  element: web_sys::HtmlInputElement,
}

impl CellEditor {
  pub fn new(container_name: &str) -> Self {
    let window = web_sys::window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let wrapper = document
      .get_element_by_id(container_name)
      .unwrap()
      .dyn_into::<web_sys::HtmlElement>()
      .unwrap();
    let element = document
      .create_element("input")
      .unwrap()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap();
    element.set_class_name("hypersheet-editor");
    wrapper.append_child(&element).unwrap();
    Self { element }
  }
}
