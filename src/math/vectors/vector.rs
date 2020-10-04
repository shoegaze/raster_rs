use crate::math::scalar::ops::ScalarNaturalOps;
use std::iter::FromIterator;
use std::iter;

pub trait Vector<E>: Copy + Clone + IntoIterator<Item = E> + FromIterator<E>
  where E: ScalarNaturalOps {
  fn dim(&self) -> usize;
  fn get(&self, i: usize) -> Option<E>;
  fn set(&mut self, i: usize, value: E);
}

impl IntoIterator for dyn Vector<E>
  where E: ScalarOps {
  type Item = E;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    let values = [Self::Item; 3];
    
    values.iter()
    
    // let mut values = vec![];
    //
    // for i in 0..self.dim() {
    //   values.push(self.get(i).unwrap());
    // }
    //
    // values.into_iter()
  }
}
