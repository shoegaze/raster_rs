use crate::math::{
  scalar::float::Float,
  vector::vector::Vector
};


#[derive(Debug, Copy, Clone)]
pub struct Float2(Float, Float);


impl Float2 {
  pub(crate) fn new(x: f32, y: f32) -> Self {
    Float2(Float(x), Float(y))
  }
}

impl Vector<Float> for Float2 {
  fn dim(&self) -> usize {
    2
  }
  
  fn get(&self, i: usize) -> Option<Float> {
    match i {
      0 => Some(self.0),
      1 => Some(self.1),
      _ => None
    }
  }
  
  fn set(&mut self, i: usize, value: Float) {
    match i {
      0 => self.0 = value,
      1 => self.1 = value,
      _ => ()
    };
  }
}

impl IntoIterator for Float2 {
  type Item = Float;
  type IntoIter = std::vec::IntoIter<Self::Item>;
  
  fn into_iter(self) -> Self::IntoIter {
    let mut values = vec![];
    
    for i in 0..self.dim() {
      let value = self.get(i).unwrap();
      values.push(value);
    }
    
    values.into_iter()
  }
}
