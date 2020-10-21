use std::collections::BTreeMap;

pub struct Row {
  idx: u16,
  height: u16,
}

impl Row {
  pub fn new(idx: u16) -> Self {
    Self { idx, height: 20 }
  }

  pub fn get_height(&self) -> u16 {
    self.height
  }

  pub fn set_height(&mut self, height: u16) {
    self.height = height;
  }
}

pub struct RowManager {
  items: BTreeMap<u16, Row>,
}

impl RowManager {
  pub fn new() -> Self {
    Self { items: BTreeMap::new() }
  }

  pub fn set_row(&mut self, row: Row) {
    self.items.insert(row.idx, row);
  }

  pub fn get_row(&self, idx: u16) -> Option<&Row> {
    self.items.get(&idx)
  }

  pub fn get_rows_within(&self, row_idx: u16) -> Vec<&Row> {
    let mut results = vec![];
    let rows: Vec<&u16> = self.items.keys().filter(|x| x <= &&row_idx).collect();
    for row in rows {
      results.push(self.items.get(row).unwrap());
    }
    results
  }
}
