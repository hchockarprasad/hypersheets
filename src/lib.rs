mod rectangle;

mod canvas;
mod cell;
mod celleditor;
mod column;
mod events;
mod model;
mod properties;
mod row;
mod scroll;

use canvas::{Canvas, CanvasHelper};
use cell::Cell;
use celleditor::CellEditor;
use column::{Column, ColumnManager};
use events::{CustomEvent, CustomEventDetail, MousePosition};
use js_sys::Array;
use model::DataModel;
use properties::HyperSheetProperties;
use rectangle::{Point, Rectangle, Within};
use row::{Row, RowManager};
use scroll::ScrollBar;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct HyperSheet {
    data_model: DataModel,
    row_manager: RowManager,
    col_manager: ColumnManager,
    canvas: web_sys::HtmlCanvasElement,
    h_scroller: web_sys::HtmlElement,
    v_scroller: web_sys::HtmlElement,
    scroller: web_sys::HtmlElement,
    placeholder: web_sys::HtmlElement,
    active_cell: Option<Cell>,
}

#[wasm_bindgen]
impl HyperSheet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        h_scroller: web_sys::HtmlElement,
        v_scroller: web_sys::HtmlElement,
        scroller: web_sys::HtmlElement,
        placeholder: web_sys::HtmlElement,
    ) -> Self {
        let data_model = DataModel::new();
        let row_manager = RowManager::new();
        let col_manager = ColumnManager::new();
        Self {
            data_model,
            row_manager,
            col_manager,
            canvas,
            h_scroller,
            v_scroller,
            placeholder,
            scroller,
            active_cell: None,
        }
    }

    #[wasm_bindgen]
    pub fn arrow_down(&self) {
        web_sys::console::log_1(&"hello".into());
    }

    fn update_row_manager(&mut self, rows: Vec<Row>) {
        for row in rows {
            self.row_manager.set_row(row);
        }
    }

    fn add_cells(&mut self, cells: Vec<Cell>) {
        for cell in cells {
            self.data_model.set_cell(cell);
        }
    }

    fn get_row_with_idx(&self, idx: u16) -> Option<&Row> {
        self.row_manager.get_row(idx)
    }

    fn get_col_with_idx(&self, idx: u8) -> Option<&Column> {
        self.col_manager.get_column(idx)
    }

    fn get_last_visible_row(&self, row_offset: usize) -> (u16, usize) {
        let mut row_count: u16 = (row_offset / 20) as u16;
        let mut offset = (row_count as usize) * 20;
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
                let new_offset = match self.get_row_with_idx(row_count + 1) {
                    Some(row) => offset + row.get_height() as usize,
                    None => offset + 20,
                };
                if new_offset >= row_offset {
                    break;
                }
                offset = new_offset;
                row_count += 1;
            }
        }
        (row_count, offset)
    }

    fn get_last_visible_col(&self, col_offset: usize) -> (u8, usize) {
        let mut col_count: u8 = (col_offset / 85) as u8;
        let mut offset = (col_count as usize) * 85;
        let cols = self.col_manager.get_column(col_count);
        for col in cols {
            offset = offset + col.get_width() as usize - 85;
        }
        if offset > col_offset {
            loop {
                offset = match self.get_col_with_idx(col_count) {
                    Some(col) => offset - col.get_width() as usize,
                    None => offset - 85,
                };
                col_count -= 1;
                if offset < col_offset {
                    break;
                }
            }
        } else if offset < col_offset {
            loop {
                let new_offset = match self.get_col_with_idx(col_count + 1) {
                    Some(col) => offset + col.get_width() as usize,
                    None => offset + 85,
                };
                if new_offset >= col_offset {
                    break;
                }
                offset = new_offset;
                col_count += 1;
            }
        }
        (col_count, offset)
    }

    pub fn get_last_visible_row_offset(&self, row_offset: usize) -> JsValue {
        let (_, offset) = self.get_last_visible_row(row_offset);
        JsValue::from(offset as f64)
    }

    pub fn get_last_visible_col_offset(&self, col_offset: usize) -> JsValue {
        let (_, offset) = self.get_last_visible_col(col_offset);
        JsValue::from(offset as f64)
    }

    /// Derive cell dimension with cell indices.
    fn get_cell_dimension(&self, col_idx: u8, row_idx: u16) -> (u16, u16) {
        let row = self.row_manager.get_row(row_idx);
        let col = self.col_manager.get_column(col_idx);
        let height: u16 = match row {
            Some(rw) => rw.get_height(),
            None => 20,
        };
        let width: u16 = match col {
            Some(cl) => cl.get_width(),
            None => 85,
        };
        (width, height)
    }

    fn get_scroller_bounds(&self) -> Rectangle {
        let left = self.scroller.scroll_left() as f64;
        let top = self.scroller.scroll_top() as f64;
        Rectangle::new(left, top, 560.0, 380.0)
    }

    fn move_placeholder_right(&mut self) {
        let (next_col_idx, next_row_idx, boundary) = match self.active_cell {
            Some(mut cell) => {
                let next_col_idx = cell.get_col_idx() + 1;
                let next_row_idx = cell.get_row_idx();
                let left = cell.get_boundary().unwrap().right();
                let top = cell.get_boundary().unwrap().top();
                let (width, height) = self.get_cell_dimension(next_col_idx, next_row_idx);
                let boundary = Rectangle::new(left, top, width as f64, height as f64);
                (next_col_idx, next_row_idx, boundary)
            }
            None => {
                let (width, height) = self.get_cell_dimension(1, 1);
                (1, 1, Rectangle::new(0.0, 0.0, width as f64, height as f64))
            }
        };
        let mut cell = Cell::new(next_col_idx, next_row_idx);
        self.placeholder
            .style()
            .set_property("left", &boundary.x_as_px())
            .unwrap();
        cell.set_boundary(boundary);
        self.active_cell = Some(cell);
        let diff = boundary.right() - self.get_scroller_bounds().right();
        if diff > 0.0 {
            self.scroller
                .set_scroll_left(self.get_scroller_bounds().left() as i32 + diff as i32);
        }
    }

    fn move_placeholder_left(&mut self) {
        let (next_col_idx, next_row_idx, boundary) = match self.active_cell {
            Some(mut cell) => {
                let next_col_idx = cell.get_col_idx() - 1;
                let next_row_idx = cell.get_row_idx();
                let origin = cell.get_boundary().unwrap().get_origin();
                let (width, height) = self.get_cell_dimension(next_col_idx, next_row_idx);
                let boundary = Rectangle::new(origin.x() - width as f64, origin.y(), width as f64, height as f64);
                (next_col_idx, next_row_idx, boundary)
            }
            None => {
                let (width, height) = self.get_cell_dimension(1, 1);
                (1, 1, Rectangle::new(0.0, 0.0, width as f64, height as f64))
            }
        };
        let mut cell = Cell::new(next_col_idx, next_row_idx);
        self.placeholder
            .style()
            .set_property("left", &boundary.x_as_px())
            .unwrap();
        cell.set_boundary(boundary);
        self.active_cell = Some(cell);
        let diff = boundary.left() - self.get_scroller_bounds().left();
        if diff < 0.0 {
            self.scroller
                .set_scroll_left(self.get_scroller_bounds().left() as i32 + diff as i32);
        }
    }

    fn move_placeholder_top(&mut self) {
        let (next_col_idx, next_row_idx, boundary) = match self.active_cell {
            Some(mut cell) => {
                let next_col_idx = cell.get_col_idx();
                let next_row_idx = cell.get_row_idx() - 1;
                let origin = cell.get_boundary().unwrap().get_origin();
                let (width, height) = self.get_cell_dimension(next_col_idx, next_row_idx);
                let boundary = Rectangle::new(origin.x(), origin.y() - height as f64, width as f64, height as f64);
                (next_col_idx, next_row_idx, boundary)
            }
            None => {
                let (width, height) = self.get_cell_dimension(1, 1);
                (1, 1, Rectangle::new(0.0, 0.0, width as f64, height as f64))
            }
        };
        let mut cell = Cell::new(next_col_idx, next_row_idx);
        self.placeholder
            .style()
            .set_property("top", &boundary.y_as_px())
            .unwrap();
        cell.set_boundary(boundary);
        self.active_cell = Some(cell);
        let diff = boundary.top() - self.get_scroller_bounds().top();
        if diff < 0.0 {
            self.scroller
                .set_scroll_top(self.get_scroller_bounds().top() as i32 + diff as i32);
        }
    }

    fn move_placeholder_down(&mut self) {
        let (next_col_idx, next_row_idx, boundary) = match self.active_cell {
            Some(mut cell) => {
                let next_col_idx = cell.get_col_idx();
                let next_row_idx = cell.get_row_idx() + 1;
                let left = cell.get_boundary().unwrap().left();
                let bottom = cell.get_boundary().unwrap().bottom();
                let (width, height) = self.get_cell_dimension(next_col_idx, next_row_idx);
                let boundary = Rectangle::new(left, bottom, width as f64, height as f64);
                (next_col_idx, next_row_idx, boundary)
            }
            None => {
                let (width, height) = self.get_cell_dimension(1, 1);
                (1, 1, Rectangle::new(0.0, 0.0, width as f64, height as f64))
            }
        };
        let mut cell = Cell::new(next_col_idx, next_row_idx);
        self.placeholder
            .style()
            .set_property("top", &boundary.y_as_px())
            .unwrap();
        cell.set_boundary(boundary);
        self.active_cell = Some(cell);
        let diff = boundary.bottom() - self.get_scroller_bounds().bottom();
        if diff > 0.0 {
            self.scroller
                .set_scroll_top(self.get_scroller_bounds().top() as i32 + diff as i32);
        }
    }

    fn paint(&self) {}

    pub fn on_right_arrow_keydown(&mut self, event: web_sys::KeyboardEvent) {
        self.move_placeholder_right();
    }

    pub fn on_left_arrow_keydown(&mut self, event: web_sys::KeyboardEvent) {
        self.move_placeholder_left();
    }

    pub fn on_up_arrow_keydown(&mut self, event: web_sys::KeyboardEvent) {
        self.move_placeholder_top();
    }

    pub fn on_down_arrow_keydown(&mut self, event: web_sys::KeyboardEvent) {
        self.move_placeholder_down();
    }
}
