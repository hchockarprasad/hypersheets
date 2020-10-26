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

use cell::Cell;
use column::{Column, ColumnManager};
use events::{CustomEvent, CustomEventDetail, MousePosition};
use model::DataModel;
use rectangle::Rectangle;
use row::{Row, RowManager};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct HyperSheet {
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
        let mut instance = Self {
            row_manager,
            col_manager,
            canvas,
            h_scroller,
            v_scroller,
            placeholder,
            scroller,
            active_cell: None,
        };
        let mut row = Row::new(3);
        row.set_height(40);
        let mut col = Column::new(4);
        col.set_width(200);
        instance.row_manager.set_row(row);
        instance.col_manager.set_column(col);
        instance.paint();
        instance
    }

    fn get_row_with_idx(&self, idx: u16) -> Option<&Row> {
        self.row_manager.get_row(idx)
    }

    fn get_col_with_idx(&self, idx: u8) -> Option<&Column> {
        self.col_manager.get_column(idx)
    }

    /// Expensive operation to perform..
    fn get_last_visible_row(&self, row_offset: usize) -> (u16, usize) {
        let mut row_count = 0;
        let mut offset = 0;
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
        (row_count, offset)
    }

    /// Expensive operation to perform..
    fn get_last_visible_col(&self, col_offset: usize) -> (u8, usize) {
        let mut col_count = 0;
        let mut offset = 0;
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

    fn get_canvas_bounds(&self) -> Rectangle {
        Rectangle::new(0.0, 0.0, 600.0, 400.0)
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
        if boundary.right() <= 20000.0 {
            let mut cell = Cell::new(next_col_idx, next_row_idx);
            self.placeholder
                .style()
                .set_property("left", &boundary.x_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("width", &boundary.width_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("height", &boundary.height_as_px())
                .unwrap();
            cell.set_boundary(boundary);
            self.active_cell = Some(cell);
            let diff = boundary.right() - self.get_scroller_bounds().right();
            if diff > 0.0 {
                let delta_scroll_left = self.get_scroller_bounds().left() as i32 + diff as i32;
                self.scroller.set_scroll_left(delta_scroll_left);
                self.h_scroller.set_scroll_left(delta_scroll_left);
                self.paint();
            }
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
        if boundary.left() >= 0.0 {
            let mut cell = Cell::new(next_col_idx, next_row_idx);
            self.placeholder
                .style()
                .set_property("left", &boundary.x_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("width", &boundary.width_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("height", &boundary.height_as_px())
                .unwrap();
            cell.set_boundary(boundary);
            self.active_cell = Some(cell);
            let diff = boundary.left() - self.get_scroller_bounds().left();
            if diff < 0.0 {
                let delta_scroll_left = self.get_scroller_bounds().left() as i32 + diff as i32;
                self.scroller.set_scroll_left(delta_scroll_left);
                self.h_scroller.set_scroll_left(delta_scroll_left);
                self.paint();
            }
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
        if boundary.top() >= 0.0 {
            let mut cell = Cell::new(next_col_idx, next_row_idx);
            self.placeholder
                .style()
                .set_property("top", &boundary.y_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("width", &boundary.width_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("height", &boundary.height_as_px())
                .unwrap();
            cell.set_boundary(boundary);
            self.active_cell = Some(cell);
            let diff = boundary.top() - self.get_scroller_bounds().top();
            if diff < 0.0 {
                let delta_scroll_top = self.get_scroller_bounds().top() as i32 + diff as i32;
                self.scroller.set_scroll_top(delta_scroll_top);
                self.v_scroller.set_scroll_top(delta_scroll_top);
                self.paint();
            }
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
        if boundary.bottom() <= 1245560.0 {
            let mut cell = Cell::new(next_col_idx, next_row_idx);
            self.placeholder
                .style()
                .set_property("top", &boundary.y_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("width", &boundary.width_as_px())
                .unwrap();
            self.placeholder
                .style()
                .set_property("height", &boundary.height_as_px())
                .unwrap();
            cell.set_boundary(boundary);
            self.active_cell = Some(cell);
            let diff = boundary.bottom() - self.get_scroller_bounds().bottom();
            if diff > 0.0 {
                let delta_scroll_top = self.get_scroller_bounds().top() as i32 + diff as i32;
                self.scroller.set_scroll_top(delta_scroll_top);
                self.v_scroller.set_scroll_top(delta_scroll_top);
                self.paint();
            }
        }
    }

    fn paint(&self) {
        let top = self.get_scroller_bounds().top();
        let left = self.get_scroller_bounds().left();
        let (row_idx, row_offset) = self.get_last_visible_row(top as usize);
        let (col_idx, col_offset) = self.get_last_visible_col(left as usize);
        let ctx: web_sys::CanvasRenderingContext2d =
            self.canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        let cb = self.get_canvas_bounds();
        ctx.clear_rect(cb.left(), cb.top(), cb.right(), cb.bottom());
        ctx.set_fill_style(&"#000000".into());
        let mut row_offset = row_offset as f64 - top + 20.0;
        let mut col_offset = col_offset as f64 - left + 40.0;
        let mut row_idx = row_idx + 1;
        let mut col_idx = col_idx + 1;
        loop {
            let height = match self.row_manager.get_row(row_idx) {
                Some(rw) => rw.get_height(),
                None => 20,
            };

            let new_offset = height as f64 + row_offset;
            if new_offset <= cb.bottom() {
                ctx.begin_path();
                ctx.rect(0.0, row_offset, 40.0, height as f64);
                ctx.stroke();
                ctx.fill_text(&row_idx.to_string(), 5.0, row_offset + 5.0 + (height / 2) as f64)
                    .unwrap();
                ctx.begin_path();
                ctx.set_line_width(0.3);
                ctx.set_stroke_style(&"#000000".into());
                ctx.move_to(40.0, new_offset);
                ctx.line_to(cb.right(), new_offset);
                ctx.stroke();
                row_idx += 1;
                row_offset = new_offset;
            } else {
                break;
            }
        }

        loop {
            let width = match self.col_manager.get_column(col_idx) {
                Some(rw) => rw.get_width(),
                None => 85,
            };

            let new_offset = width as f64 + col_offset;
            ctx.begin_path();
            ctx.rect(col_offset, 0.0, width as f64, 20.0);
            ctx.stroke();
            ctx.fill_text(&col_idx.to_string(), col_offset + (width / 2) as f64, 15.0)
                .unwrap();
            ctx.begin_path();
            ctx.set_line_width(0.3);
            ctx.set_stroke_style(&"#000000".into());
            ctx.move_to(new_offset, 20.0);
            ctx.line_to(new_offset, cb.bottom());
            ctx.stroke();
            if new_offset <= cb.right() {
                col_idx += 1;
                col_offset = new_offset;
            } else {
                break;
            }
        }

        ctx.begin_path();
        ctx.set_fill_style(&"#dbdbdb".into());
        ctx.fill_rect(0.0, 0.0, 40.0, 20.0);
    }

    pub fn on_right_arrow_keydown(&mut self, _: web_sys::KeyboardEvent) {
        self.move_placeholder_right();
    }

    pub fn on_left_arrow_keydown(&mut self, _: web_sys::KeyboardEvent) {
        self.move_placeholder_left();
    }

    pub fn on_up_arrow_keydown(&mut self, _: web_sys::KeyboardEvent) {
        self.move_placeholder_top();
    }

    pub fn on_down_arrow_keydown(&mut self, _: web_sys::KeyboardEvent) {
        self.move_placeholder_down();
    }

    pub fn on_h_scroll(&mut self, _: web_sys::Event) {
        self.scroller.set_scroll_left(self.h_scroller.scroll_left());
        self.paint();
    }

    pub fn on_v_scroll(&mut self, _: web_sys::Event) {
        self.scroller.set_scroll_top(self.v_scroller.scroll_top());
        self.paint();
    }

    pub fn on_click(&mut self, event: web_sys::MouseEvent) {
        let (row_idx, row_offset) = self.get_last_visible_row(event.offset_y() as usize);
        let (col_idx, col_offset) = self.get_last_visible_col(event.offset_x() as usize);
        let mut cell = Cell::new(col_idx + 1, row_idx + 1);
        let (width, height) = self.get_cell_dimension(col_idx + 1, row_idx + 1);
        let boundary = Rectangle::new(col_offset as f64, row_offset as f64, width as f64, height as f64);
        self.placeholder
            .style()
            .set_property("top", &boundary.y_as_px())
            .unwrap();
        self.placeholder
            .style()
            .set_property("left", &boundary.x_as_px())
            .unwrap();
        self.placeholder
            .style()
            .set_property("width", &boundary.width_as_px())
            .unwrap();
        self.placeholder
            .style()
            .set_property("height", &boundary.height_as_px())
            .unwrap();
        cell.set_boundary(boundary);
        self.active_cell = Some(cell);
    }
}
