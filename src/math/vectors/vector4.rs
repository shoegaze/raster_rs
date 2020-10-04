// use super::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector4<E> {
  x: E,
  y: E,
  z: E,
  w: E
}

// impl<E> Vector for Vector4<E> {
//   type Element = E;
//
//   fn dim(&self) -> usize {
//     4
//   }
//
//   fn get(&self, i: usize) -> Option<Self::Element> {
//     match i {
//       0 => Some(self.x),
//       1 => Some(self.y),
//       2 => Some(self.z),
//       3 => Some(self.w),
//       _ => None
//     }
//   }
//
//   fn set(&mut self, i: usize, value: Self::Element) {
//     match i {
//       0 => self.x = value,
//       1 => self.y = value,
//       2 => self.z = value,
//       3 => self.w = value,
//       _ => ()
//     };
//   }
// }