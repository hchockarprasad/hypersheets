use std::collections::BTreeMap;

#[derive(Debug)]
pub struct DataModel {
  pub items: BTreeMap<String, String>,
}



impl DataModel {
  pub fn new() -> Self {
    Self {
      items: BTreeMap::new()
    }
  }

  fn zero_pad_index(&self, idx: usize, padding_size: u8) -> String {
    let mut key = idx.to_string();
    while key.len() <= padding_size as usize {
      key = ["0".to_string(), key].concat();
    }
    key
  }

  pub fn set_value(&mut self, row_idx: usize, col_idx: usize, value: String) {
    let row_key = self.zero_pad_index(row_idx, 5);
    let col_key = self.zero_pad_index(col_idx, 3);
    let key = [row_key, "x".to_string(), col_key].concat();
    self.items.insert(key, value);
  }

  pub fn get_value(&self, row_idx: usize, col_idx: usize) -> String {
    let key = [row_idx.to_string(), "x".to_string(), col_idx.to_string()].concat();
    match self.items.get(&key) {
      Some(x) => x.to_string(),
      None => "".to_string()
    }
  }

  pub fn get_row(&self, row_idx: usize) -> BTreeMap<String, String> {
    let row_key = self.zero_pad_index(row_idx, 5);
    let key = [row_key, "x".to_string()].concat();
    let key2 = key.clone();
    let mut set = BTreeMap::new();
    for (k, v) in self.items.range(key..).take_while(|x| x.0.starts_with(&key2)) {
      set.insert(k.clone(), v.clone());
    }
    set
  }

  pub fn get_all(&self) -> BTreeMap<String, String> {
    self.items.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn data_model() {
      let mut dm = DataModel::new();
      dm.set_value(1, 1, "Cell A".to_string());
      dm.set_value(1, 4, "Cell D".to_string());
      dm.set_value(1, 3, "Cell C".to_string());
      dm.set_value(1, 2, "Cell B".to_string());
      let row = dm.get_row(1);
      println!("{:?}", row);
      assert_eq!(2 + 2, 4);
    }
}
