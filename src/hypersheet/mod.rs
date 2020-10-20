mod rectangle;

mod canvas;
mod cell;
mod celleditor;
mod events;
mod model;
mod properties;
mod scroll;

use cell::Cell;
use canvas::{Canvas, CanvasHelper};
use celleditor::CellEditor;
use events::{CustomEvent, CustomEventDetail, MousePosition};
use js_sys::Array;
use model::DataModel;
use properties::HyperSheetProperties;
use scroll::ScrollBar;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct HyperSheet {
  data_model: DataModel
}

impl HyperSheet {
  pub fn new() -> Self {
    let data_model = DataModel::new();
    Self {
      data_model: data_model,
    }
  }

  pub fn add_cells(&mut self, cells: Vec<Cell>) {
    for cell in cells {
      self.data_model.set_cell(cell);
    }
  }

  pub fn get_last_visible_row(&self, row_offset: u16) {
    let mut rows: u16 = row_offset / 20;
    let mut offset = (row_offset - rows * 20) as usize;
    let cells = self.data_model.get_cells_within_rows(rows);
    for cell in cells {
      offset = offset + cell.get_height() - 20;
    }
    println!("{}", offset);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_cell() {
    let mut sheet = HyperSheet::new();
    let mut cell1 = Cell::new(1, 1);
    cell1.set_height(40);
    let mut cell2 = Cell::new(1, 2);
    cell2.set_height(10);
    sheet.add_cells(vec![cell1, cell2]);
    sheet.get_last_visible_row(4010);
  }
}

