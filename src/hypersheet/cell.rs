pub struct Cell {
  row_idx: u16,
  col_idx: u8,
  width: usize,
  height: usize,
}

impl Cell {
  pub fn new(col_idx: u8, row_idx: u16) -> Self {
    Self {
      row_idx,
      col_idx,
      width: 20,
      height: 20,
    }
  }

  fn get_column_name(&self) -> String {
    let mut dividend = self.col_idx;
    let mut column_name = String::new();
    let mut modulo: u8;

    while dividend > 0 {
      modulo = (dividend - 1) % 26;
      column_name = [((65 + modulo) as char).to_string(), column_name].concat();
      dividend = (dividend - modulo) / 26;
    }
    column_name
  }

  pub fn name(&self) -> String {
    [self.get_column_name(), self.row_idx.to_string()].concat()
  }

  pub fn get_col_idx(&self) -> u8 {
    self.col_idx
  }

  pub fn get_row_idx(&self) -> u16 {
    self.row_idx
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }

  pub fn set_width(&mut self, width: usize) {
    self.width = width;
  }

  pub fn set_height(&mut self, height: usize) {
    self.height = height;
  }
}
