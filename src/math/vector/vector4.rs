use super::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector3<E> {
  x: E,
  y: E,
  z: E
}

impl Vector for Vector3<E> {
  type Element = E;
  
  fn dim(&self) -> usize {
    3
  }
  
  fn get(&self, i: usize) -> Option<Self::Element> {
    match i {
      0 => Some(self.x),
      1 => Some(self.y),
      2 => Some(self.z),
      _ => None
    }
  }
  
  fn set(&self, i: usize, value: Self::Element) {
    match i {
      0 => self.x = value,
      1 => self.y = value,
      2 => self.z = value,
      _ => ()
    };
  }
}