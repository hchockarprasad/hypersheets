use std::collections::BTreeMap;

use super::cell::Cell;

pub struct DataModel {
  pub items: BTreeMap<String, Cell>,
}

impl DataModel {
  pub fn new() -> Self {
    Self { items: BTreeMap::new() }
  }

  pub fn set_cell(&mut self, cell: Cell) {
    self.items.insert(cell.name(), cell);
  }

  pub fn get_cell(&self, col_idx: u8, row_idx: u16) -> Option<&Cell> {
    let cell = Cell::new(col_idx, row_idx);
    self.items.get(&cell.name())
  }

  pub fn get_cells_within(&self, col_idx: u8, row_idx: u16) -> Vec<&Cell> {
    self.items.values().filter(|x| x.get_col_idx() <= col_idx && x.get_row_idx() <= row_idx).collect()
  }

  pub fn get_cells_within_rows(&self, row_idx: u16) -> Vec<&Cell> {
    self.items.values().filter(|x| x.get_row_idx() <= row_idx).collect()
  }

  pub fn get_cells_within_cols(&self, col_idx: u8) -> Vec<&Cell> {
    self.items.values().filter(|x| x.get_col_idx() <= col_idx).collect()
  }
}
