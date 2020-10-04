use std::iter::FromIterator;
use crate::math::{
  scalar::int::Int,
  vector::vector::Vector
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Int2(Int, Int);


impl Int2 {
  pub(crate) fn new(x: i32, y: i32) -> Self {
    Int2(Int(x), Int(y))
  }
}

impl Vector<Int> for Int2 {
  fn dim(&self) -> usize {
    2
  }
  
  fn get(&self, i: usize) -> Option<Int> {
    match i {
      0 => Some(self.0),
      1 => Some(self.1),
      _ => None
    }
  }
  
  fn set(&mut self, i: usize, value: Int) {
    match i {
      0 => self.0 = value,
      1 => self.1 = value,
      _ => ()
    };
  }
}

// impl IntoIterator for Int2 {
//   type Item = Int;
//   type IntoIter = std::vec::IntoIter<Self::Item>;
//
//   fn into_iter(self) -> Self::IntoIter {
//     let mut values = vec![];
//
//     for i in 0..self.dim() {
//       let value = self.get(i).unwrap();
//       values.push(value);
//     }
//
//     values.into_iter()
//   }
// }

impl FromIterator<Int> for Int2 {
  fn from_iter<T: IntoIterator<Item = Int>>(iter: T) -> Self {
    let (x, y) = ();
    Int2::new()
  }
}
