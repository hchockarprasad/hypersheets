use wasm_bindgen::JsCast;

pub enum ScrollBarOrientation {
  Vertical,
  Horizontal
}

impl std::string::ToString for ScrollBarOrientation {
  fn to_string(&self) -> String {
    match self {
      Vertical => "Vertical".to_string(),
      Horizontal => "Horizontal".to_string()
    }
  }
}

pub struct ScrollBar {
  orientation: ScrollBarOrientation,
  min: u32,
  max: u32,
  index: u32,
  content_size: u32,
  oh: OrientationHash,
  class_prefix: Option<String>,
  bar: web_sys::HtmlElement,
  thumb: web_sys::HtmlElement,
  thumb_max: u32,
  increment: u32,
  delta_x_factor: u32,
  delta_y_factor: u32,
  delta_z_factor: u32,
}

pub struct ScrollBarOption {
  index: Option<u32>,
  range: Option<(u32, u32)>,
}

struct OrientationHash {
  coordinate: &'static str,
  axis: &'static str,
  size: &'static str,
  outside: &'static str,
  inside: &'static str,
  leading: &'static str,
  trailing: &'static str,
  margin_leading: &'static str,
  margin_trailing: &'static str,
  thickness: &'static str,
  delta: &'static str,
}

struct Axis {
  top: ScrollBarOrientation,
  bottom: ScrollBarOrientation,
  height: ScrollBarOrientation,
  left: ScrollBarOrientation,
  right: ScrollBarOrientation,
  width: ScrollBarOrientation,
}

const axis: Axis = Axis {
  top: ScrollBarOrientation::Vertical,
  bottom: ScrollBarOrientation::Vertical,
  height: ScrollBarOrientation::Vertical,
  left: ScrollBarOrientation::Horizontal,
  right: ScrollBarOrientation::Horizontal,
  width: ScrollBarOrientation::Horizontal,
};

const VERTICAL_ORIENTATION_HASH: OrientationHash = OrientationHash {
  coordinate: "clientY",
  axis: "pageY",
  size: "height",
  outside: "right",
  inside: "left",
  leading: "top",
  trailing: "bottom",
  margin_leading: "marginTop",
  margin_trailing: "marginBottom",
  thickness: "width",
  delta: "deltaY",
};

const HORIZONTAL_ORIENTATION_HASH: OrientationHash = OrientationHash {
  coordinate: "clientX",
  axis: "pageX",
  size: "width",
  outside: "bottom",
  inside: "top",
  leading: "left",
  trailing: "right",
  margin_leading: "marginLeft",
  margin_trailing: "marginRight",
  thickness: "height",
  delta: "deltaX",
};

impl ScrollBar {
  fn new(options: ScrollBarOption) -> Self {
    let window = web_sys::window().expect("No global window exist");
    let document = window.document().expect("Should have a doc on window");
    let thumb = document.create_element("div").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
    let class_list = js_sys::Array::new();
    class_list.set(0, "thumb".into());
    thumb.class_list().add(&class_list).unwrap();
    thumb.set_attribute("style", "Thumbstyle").unwrap();

    let bar = document.create_element("div").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
    class_list.set(0, "bar-vertical".into());
    bar.class_list().add(&class_list).unwrap();
    bar.set_attribute("style", "Barstyle").unwrap();
    bar.append_child(&thumb).unwrap();

    let (min, max) = match options.range {
      Some(range) => (range.0, range.1),
      None => (0, 100)
    };

    let index = match options.index {
      Some(idx) => idx,
      None => 0
    };

    let content_size = (max - min + 1) as u32;
    
    Self {
      orientation: ScrollBarOrientation::Vertical,
      min,
      max,
      content_size,
      index,
      oh: VERTICAL_ORIENTATION_HASH,
      class_prefix: None,
      thumb,
      bar,
      thumb_max: 1,
      increment: 1,
      delta_x_factor: 1,
      delta_y_factor: 1,
      delta_z_factor: 1
    }
  }

  pub fn set_orientation(&mut self, orientation: ScrollBarOrientation) {
    self.orientation = orientation;
    self.oh = match self.orientation {
      ScrollBarOrientation::Horizontal => HORIZONTAL_ORIENTATION_HASH,
      ScrollBarOrientation::Vertical => VERTICAL_ORIENTATION_HASH
    }
  }

  pub fn get_orientation(&self) -> &ScrollBarOrientation {
    &self.orientation
  }

  pub fn set_class_prefix(&mut self, prefix: &str) {
    let class_list = js_sys::Array::new();
    if self.class_prefix.is_some() {
      class_list.set(0, self.orientation.to_string().into());
      self.bar.class_list().remove(&class_list).unwrap();
    }
    self.class_prefix = Some(prefix.to_string());
    class_list.set(0, [prefix, "-", &self.orientation.to_string()].concat().into());
    self.bar.class_list().add(&class_list ).unwrap();
  }

  pub fn get_range(&self) -> (u32, u32) {
    (self.min, self.max)
  }

  pub fn set_range(&mut self, min: u32, max: u32) {
    if max < min {
      self.min = min;
      self.max = max;
      self.content_size = self.max - self.min + 1;
    }
    
  }

  pub fn set_index(&mut self, idx: u32) {
    let idx = std::cmp::min(self.max, std::cmp::max(self.min, idx));
    self.set_scroll(idx, None);
  }

  pub fn set_scroll(&mut self, idx: u32, scaled: Option<u32>) {
    self.index = idx;

    let scaled_value = if scaled.is_none() {
      (idx - self.min) / (self.max - self.min) + self.thumb_max
    } else {
      scaled.unwrap()
    };

    self.thumb.style().set_property(self.oh.leading, &[scaled_value.to_string(), "px".to_string()].concat()).unwrap();
  }

  pub fn short_stop(&self, event: web_sys::Event) {
    event.stop_propagation();
  }

  pub fn on_wheel(&mut self, event: web_sys::Event) {
    self.index += 5;
    event.stop_propagation();
    event.prevent_default();
  }
}