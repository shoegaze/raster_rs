// use crate::math::{
//   vector::{
//     vector::Vector
//   },
//   scalar::{
//     ops::ScalarOps,
//     int::Int,
//     float::Float,
//     double::Double
//   }
// };

// pub trait Vector2<E> {
//   type Output;
//
//   fn new(x: E, y: E) -> Self::Output
// }

// impl<E> Vector for dyn Vector2<E> {
//   type Element = E;
//
//   fn dim(&self) -> usize {
//     2
//   }
//
//   fn get(&self, i: usize) -> Option<&Self::Element> {
//     match i {
//       0 => Some(self.x()),
//       1 => Some(self.y()),
//       _ => None
//     }
//   }
//
//   fn set(&mut self, i: usize, value: Self::Element) {
//     match i {
//
//     };
//   }
// }

// #[derive(Debug, PartialEq, Copy, Clone)]
// pub struct Vector2<E: ScalarOps> {
//   pub(crate) x: E,
//   pub(crate) y: E
// }
//

// impl<E> Vector2<E> where E: ScalarOps {
//   fn new(x: E, y: E) -> Vector2<E> {
//     Vector2 { x, y }
//   }
// }

// impl<E> Vector for Vector2<E> where E: ScalarOps {
//   type Element = E;
//
//   fn dim(&self) -> usize {
//     2
//   }
//
//   fn get(&self, i: usize) -> Option<&Self::Element> {
//     match i {
//       0 => Some(&self.x),
//       1 => Some(&self.y),
//       _ => None
//     }
//   }
//
//   fn set(&mut self, i: usize, value: Self::Element) {
//     match i {
//       0 => self.x = value,
//       1 => self.y = value,
//       _ => ()
//     };
//   }
// }

// impl<E> IntoIterator for Vector2<E> where E: ScalarOps {
//   type Item = E;
//   type IntoIter = std::vec::IntoIter<Self::Item>;
//
//   fn into_iter(self) -> Self::IntoIter {
//     vec![self.x, self.y].into_iter()
//   }
// }