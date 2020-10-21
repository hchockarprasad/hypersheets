mod rectangle;

mod canvas;
mod cell;
mod celleditor;
mod events;
mod model;
mod properties;
mod row;
mod scroll;

use canvas::{Canvas, CanvasHelper};
use cell::Cell;
use celleditor::CellEditor;
use events::{CustomEvent, CustomEventDetail, MousePosition};
use js_sys::Array;
use model::DataModel;
use properties::HyperSheetProperties;
use row::{Row, RowManager};
use scroll::ScrollBar;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct HyperSheet {
  data_model: DataModel,
  row_manager: RowManager,
}

impl HyperSheet {
  pub fn new() -> Self {
    let data_model = DataModel::new();
    let row_manager = RowManager::new();
    Self {
      data_model,
      row_manager,
    }
  }
  pub fn update_row_manager(&mut self, rows: Vec<Row>) {
    for row in rows {
      self.row_manager.set_row(row);
    }
  }

  pub fn add_cells(&mut self, cells: Vec<Cell>) {
    for cell in cells {
      self.data_model.set_cell(cell);
    }
  }

  pub fn get_row_with_idx(&self, idx: u16) -> Option<&Row> {
    self.row_manager.get_row(idx)
  }

  pub fn get_last_visible_row_idx(&self, row_offset: usize) -> u16 {
    let mut row_count: u16 = (row_offset / 20) as u16;
    let mut offset = row_offset;
    let rows = self.row_manager.get_rows_within(row_count);
    for row in rows {
      offset = offset + row.get_height() as usize - 20;
    }
    if offset > row_offset {
      loop {
        offset = match self.get_row_with_idx(row_count) {
          Some(row) => offset - row.get_height() as usize,
          None => offset - 20,
        };
        row_count -= 1;
        if offset < row_offset {
          break;
        }
      }
    } else if offset < row_offset {
      loop {
        offset = match self.get_row_with_idx(row_count + 1) {
          Some(row) => offset + row.get_height() as usize,
          None => offset + 20,
        };
        if offset > row_offset {
          break;
        }
        row_count += 1;
      }
    }
    row_count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_rows() {
    let mut sheet = HyperSheet::new();
    let mut row1 = Row::new(1);
    row1.set_height(10);
    let mut row2 = Row::new(2);
    row2.set_height(10);
    sheet.update_row_manager(vec![row1, row2]);
    let x = sheet.get_last_visible_row_idx(4010);
    println!("{}", x);
  }
}
