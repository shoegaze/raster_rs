use super::ops::ScalarOps;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Int(pub i32);

impl Int {
  pub fn new(value: i32) -> Self {
    Int(value)
  }
}

impl ScalarOps for Int {
  type Element = Int;

  fn negate(&self) -> Self::Element {
    Int(-self.0)
  }

  fn inverse(&self) -> Self::Element {
    Int(1 / self.0)
  }

  fn add(&self, rhs: &Self::Element) -> Self::Element {
    Int(self.0 + rhs.0)
  }

  fn mul(&self, rhs: &Self::Element) -> Self::Element {
    Int(self.0 * rhs.0)
  }

  fn pow(&self, rhs: &Self::Element) -> Self {
    Int(self.0.pow(rhs.0 as u32))
  }
}


// use std::ops::Add;
// use std::ops::Sub;
// use std::ops::Mul;
// use std::ops::Div;
//
// impl Add for Int {
//   type Output = Int;
//
//   fn add(self, rhs: Self) -> Self::Output {
//     Int(self.0 + rhs.0)
//   }
// }
//
// impl Sub for Int {
//
// }