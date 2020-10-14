// Convert an array to a 2d array
pub trait ArrayUtils<T> {
  fn to_2d(&self) -> Vec<Vec<&T>>;
  fn reduce<F>(&self, iteratee: F, accumulator: T, init_from_array: bool) -> T
  where
    F: Fn(T, T, u32, &Vec<T>) -> T,
    T: Copy;
}

pub trait Array2DUtils<T> {
  fn pivot(&self) -> Vec<Vec<Option<&T>>>;
}

impl<T> ArrayUtils<T> for Vec<T> {
  fn to_2d(&self) -> Vec<Vec<&T>> {
    let mut target = Vec::new();
    for n in self {
      target.push(vec![n]);
    }
    target
  }

  fn reduce<F>(&self, iteratee: F, accumulator: T, init_from_array: bool) -> T
  where
    F: Fn(T, T, u32, &Vec<T>) -> T,
    T: Copy,
  {
    let mut index: i32 = -1;
    let mut result = accumulator;

    if init_from_array && (*self).len() > 0 {
      index += 1;
      result = *self.into_iter().nth(index as usize).unwrap();
    }

    index += 1;

    while index < self.len() as i32 {
      result = iteratee(result, (*self)[index as usize], index as u32, self);
      index += 1;
    }
    result
  }
}

impl<T> Array2DUtils<T> for Vec<Vec<T>> {
  fn pivot(&self) -> Vec<Vec<Option<&T>>> {
    let row_count = self.len();
    let col_count = self[0].len();
    if row_count == 0 || col_count == 0 {
      return vec![vec![]];
    }

    let mut pivoted: Vec<Vec<Option<&T>>> = vec![];
    let mut i = 0;
    while i < row_count {
      let mut j = 0;
      while j < col_count {
        if pivoted.iter().nth(j).is_none() {
          pivoted.push(vec![]);
        }
        pivoted[j].push(match self.iter().nth(i) {
          Some(outer) => match outer.iter().nth(j) {
            Some(inner) => Some(inner),
            None => None,
          },
          None => None,
        });
        j += 1;
      }
      i += 1;
    }
    pivoted
  }
}
