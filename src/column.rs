use std::collections::BTreeMap;

pub struct Column {
  idx: u8,
  width: u16,
}

impl Column {
  pub fn new(idx: u8) -> Self {
    Self { idx, width: 20 }
  }

  pub fn get_width(&self) -> u16 {
    self.width
  }

  pub fn set_width(&mut self, width: u16) {
    self.width = width;
  }
}

pub struct ColumnManager {
  items: BTreeMap<u8, Column>,
}

impl ColumnManager {
  pub fn new() -> Self {
    Self { items: BTreeMap::new() }
  }

  pub fn set_column(&mut self, column: Column) {
    self.items.insert(column.idx, column);
  }

  pub fn get_column(&self, idx: u8) -> Option<&Column> {
    self.items.get(&idx)
  }

  pub fn get_cols_within(&self, col_idx: u8) -> Vec<&Column> {
    let mut results = vec![];
    let cols: Vec<&u8> = self.items.keys().filter(|x| x <= &&col_idx).collect();
    for col in cols {
      results.push(self.items.get(col).unwrap());
    }
    results
  }
}
