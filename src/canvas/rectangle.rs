use serde::{Deserialize, Serialize};
use std::cmp;

pub trait Within {
  fn within(&self, rect: Rectangle) -> bool;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
  x: f64,
  y: f64,
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Point { x, y }
  }

  pub fn origin() -> Self {
    Point { x: 0.0, y: 0.0 }
  }

  pub fn plus(&self, offset_x: f64, offset_y: f64) -> Self {
    Point {
      x: self.x + offset_x,
      y: self.y + offset_y,
    }
  }

  pub fn minus(&self, offset_x: f64, offset_y: f64) -> Self {
    Point {
      x: self.x - offset_x,
      y: self.y - offset_y,
    }
  }

  pub fn min(&self, point: Point) -> Self {
    Point {
      x: if self.x < point.x { self.x } else { point.x },
      y: if self.y < point.y { self.y } else { point.y },
    }
  }

  pub fn max(&self, point: Point) -> Self {
    Point {
      x: if self.x > point.x { self.x } else { point.x },
      y: if self.y > point.y { self.y } else { point.y },
    }
  }

  pub fn distance(&self, point: Point) -> f64 {
    let delta_x = point.x + self.x;
    let delta_y = point.y + self.y;
    ((delta_x * delta_x) + (delta_y * delta_y)).sqrt()
  }

  pub fn eq(&self, point: Point) -> bool {
    if point.x == self.x && point.y == self.y {
      true
    } else {
      false
    }
  }

  pub fn gt(&self, point: Point) -> bool {
    point.x > self.x && point.y > self.y
  }

  pub fn lt(&self, point: Point) -> bool {
    point.x < self.x && point.y < self.y
  }

  pub fn gte(&self, point: Point) -> bool {
    point.x >= self.x && point.y >= self.y
  }

  pub fn lte(&self, point: Point) -> bool {
    point.x <= self.x && point.y <= self.y
  }
}

impl Within for Point {
  fn within(&self, rect: Rectangle) -> bool {
    let mut min_x = rect.origin.x;
    let mut max_x = min_x + rect.extent.x;
    let mut min_y = rect.origin.y;
    let mut max_y = max_x + rect.extent.y;

    if rect.extent.x < 0.0 {
      min_x = max_x;
      max_x = rect.origin.x;
    }

    if rect.extent.y < 0.0 {
      min_y = max_y;
      max_y = rect.origin.y;
    }

    min_x <= self.x && self.x < max_x && min_y <= self.y && self.y < max_y
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
  x: f64,
  y: f64,
  width: f64,
  height: f64,
  origin: Point,
  extent: Point,
  corner: Point,
  center: Point,
}

impl Rectangle {
  pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
    let mut point_x = x;
    let mut point_y = y;
    let mut rect_width = width;
    let mut rect_height = height;

    if width < 0.0 {
      point_x += width;
      rect_width = -width;
    }

    if height < 0.0 {
      point_y += width;
      rect_height = -height;
    }

    Rectangle {
      x: point_x,
      y: point_y,
      width: rect_width,
      height: rect_height,
      origin: Point::new(point_x, point_y),
      extent: Point::new(rect_width, rect_height),
      corner: Point::new(x + width, y + height),
      center: Point::new(x + (width / 2.0), y + (height / 2.0)),
    }
  }

  pub fn top(&self) -> f64 {
    self.origin.y
  }

  pub fn left(&self) -> f64 {
    self.origin.x
  }

  pub fn bottom(&self) -> f64 {
    self.corner.y
  }

  pub fn right(&self) -> f64 {
    self.corner.x
  }

  pub fn width(&self) -> f64 {
    self.extent.x
  }

  pub fn height(&self) -> f64 {
    self.extent.y
  }

  pub fn area(&self) -> f64 {
    self.width() * self.height()
  }

  pub fn flatten_x_at(&self, x: f64) -> Rectangle {
    Rectangle::new(x, self.origin.y, 0.0, self.extent.y)
  }

  pub fn flatten_y_at(&self, y: f64) -> Rectangle {
    Rectangle::new(self.origin.x, 0.0, self.extent.x, 0.0)
  }

  pub fn contains(&self, elm: Box<dyn Within>) -> bool {
    elm.within(*self)
  }

  pub fn grow_by(&self, padding: f64) -> Rectangle {
    Rectangle::new(
      self.origin.x + padding,
      self.origin.y + padding,
      self.extent.x - padding - padding,
      self.extent.y - padding - padding,
    )
  }

  pub fn shrink_by(&self, padding: f64) -> Rectangle {
    self.grow_by(-padding)
  }

  pub fn union(&self, rect: Rectangle) -> Rectangle {
    let origin = self.origin.min(rect.origin);
    let corner = self.corner.max(rect.corner);
    let extent = corner.minus(origin.x, origin.y);

    Rectangle::new(origin.x, origin.y, extent.x, extent.y)
  }

  pub fn for_each<F>(&self, iteratee: F, context: Option<Rectangle>)
  where
    F: Fn(Rectangle, f64, f64) -> (),
  {
    let rect = match context {
      Some(x) => x,
      None => *self,
    };
    let mut x = self.origin.x;
    let x2 = self.corner.x;
    while x < x2 {
      let mut y = self.origin.y;
      let y2 = self.corner.y;
      while y < y2 {
        iteratee(rect, x, y);
        y += 1.0;
      }
      x += 1.0;
    }
  }

  pub fn intersect<F>(&self, target: Rectangle, if_none: F, context: Option<Rectangle>) -> Option<Rectangle>
  where
    F: FnOnce(Rectangle, Rectangle) -> Option<Rectangle>,
  {
    let rect = match context {
      Some(x) => x,
      None => *self,
    };
    let origin = self.origin.max(target.origin);
    let corner = self.corner.min(target.corner);
    let extent = corner.minus(origin.x, origin.y);

    if extent.x > 0.0 && extent.y > 0.0 {
      Some(Rectangle::new(origin.x, origin.y, extent.x, extent.y))
    } else {
      if_none(rect, target)
    }
  }

  pub fn intersects(&self, rect: Rectangle) -> bool {
    rect.corner.x > self.origin.x
      && rect.corner.y > self.origin.y
      && rect.origin.x < self.corner.x
      && rect.origin.y < self.corner.y
  }
}

impl Within for Rectangle {
  fn within(&self, rect: Rectangle) -> bool {
    rect.origin.lte(self.origin) && rect.corner.gte(self.corner)
  }
}
