use crate::math::{
  scalar::double::Double,
  vector::vector::Vector
};
use crate::math::vector::ops::VectorOps;


#[derive(Debug, Copy, Clone)]
pub struct Double2(Double, Double);


impl Double2 {
  pub(crate) fn new(x: f64, y: f64) -> Self {
    Double2(Double(x), Double(y))
  }
}

impl Vector<Double> for Double2 {
  fn dim(&self) -> usize {
    2
  }
  
  fn get(&self, i: usize) -> Option<Double> {
    match i {
      0 => Some(self.0),
      1 => Some(self.1),
      _ => None
    }
  }
  
  fn set(&mut self, i: usize, value: Double) {
    match i {
      0 => self.0 = value,
      1 => self.1 = value,
      _ => ()
    };
  }
}

impl IntoIterator for Double2 {
  type Item = Double;
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
