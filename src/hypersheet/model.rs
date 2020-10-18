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

  pub fn set_value(&mut self, row_idx: usize, col_idx: usize, value: String) {
    let max_row_digits = 5;
    let max_col_digits = 3;
    let mut row_key = row_idx.to_string();
    let mut col_key = col_idx.to_string();
    while row_key.len() <= max_row_digits {
      row_key = ["0".to_string(), row_key].concat();
    }
    while col_key.len() <= max_col_digits {
      col_key = ["0".to_string(), col_key].concat();
    }
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
    let max_row_digits = 5;
    let mut row_key = row_idx.to_string();
    while row_key.len() <= max_row_digits {
      row_key = ["0".to_string(), row_key].concat();
    }
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
      dm.set_value(10, 4, "Cell D".to_string());
      dm.set_value(1, 3, "Cell C".to_string());
      dm.set_value(1, 2, "Cell B".to_string());
      let row = dm.get_row(1);
      println!("{:?}", row);
      assert_eq!(2 + 2, 4);
    }
}
