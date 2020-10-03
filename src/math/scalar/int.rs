use super::ops::ScalarNaturalOps;
use super::scalar::Scalar;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Int(pub i32);

impl Scalar for Int {
  type Primitive = i32;
  
  fn new(value: Self::Primitive) -> Self {
    Int(value)
  }
  
  fn to_primitive(&self) -> Self::Primitive {
    self.0
  }
}

impl ScalarNaturalOps for Int {
  type Natural = Int;
  
  fn negation(&self) -> Self {
    Int(-self.0)
  }

  fn add(&self, rhs: &Self) -> Self {
    Int(self.0 + rhs.0)
  }

  fn mul(&self, rhs: &Self) -> Self {
    Int(self.0 * rhs.0)
  }

  fn pow(&self, rhs: &Self) -> Self {
    Int(self.0.pow(rhs.0 as u32))
  }
}

impl Add for Int {
  type Output = Int;

  fn add(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::add(&self, &rhs)
  }
}

impl Sub for Int {
  type Output = Int;
  
  fn sub(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::sub(&self, &rhs)
  }
}

impl Mul for Int {
  type Output = Int;
  
  fn mul(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::mul(&self, &rhs)
  }
}
