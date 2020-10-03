use super::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector2<E> {
  x: E,
  y: E
}

impl Vector for Vector2<E> {
  type Element = E;
  
  fn dim(&self) -> usize {
    2
  }
  
  fn get(&self, i: usize) -> Option<Self::Element> {
    match i {
      0 => Some(self.x),
      1 => Some(self.y),
      _ => None
    }
  }
  
  fn set(&self, i: usize, value: Self::Element) {
    match i {
      0 => self.x = value,
      1 => self.y = value,
      _ => None
    };
  }
}