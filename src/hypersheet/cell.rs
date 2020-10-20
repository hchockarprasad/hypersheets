use std::char::from_digit;

pub struct Cell {
  row_idx: u16,
  col_idx: u8,
  width: usize,
  height: usize,
}

impl Cell {
  pub fn new(row_idx: u16, col_idx: u8) -> Self {
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

  fn name(&self) -> String {
    [self.get_column_name(), self.row_idx.to_string()].concat()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn new_cell() {
    let mut cell = Cell::new(65535, 255);
    println!("{}", cell.get_column_name());
    println!("{}", cell.name());
  }
}
