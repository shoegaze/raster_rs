use std::ops::{Neg, Add, Sub, Mul};
use super::{
  scalar::{Scalar, ScalarTrait},
  ops::ScalarNaturalOps,
  float::ScalarFloat,
  double::ScalarDouble,
};

pub type ScalarInt = Scalar<i32>;

impl ScalarTrait<i32> for ScalarInt {
  fn new(value: i32) -> Self {
    Scalar(value)
  }

  fn inner(&self) -> i32 {
    self.0
  }
}

impl From<ScalarFloat> for ScalarInt {
  fn from(scalar: ScalarFloat) -> Self {
    ScalarInt::new(scalar.inner() as i32)
  }
}

impl From<ScalarDouble> for ScalarInt {
  fn from(scalar: ScalarDouble) -> Self {
    ScalarInt::new(scalar.inner() as i32)
  }
}

impl ScalarNaturalOps for ScalarInt {
  type Output = ScalarInt;
}

impl Neg for ScalarInt {
  type Output = ScalarInt;
  
  fn neg(self) -> Self::Output {
    ScalarInt::new(-self.inner())
  }
}

impl Add for ScalarInt {
  type Output = ScalarInt;

  fn add(self, rhs: Self) -> Self::Output {
    ScalarInt::new(self.inner() + rhs.inner())
  }
}

impl Sub for ScalarInt {
  type Output = ScalarInt;

  fn sub(self, rhs: Self) -> Self::Output {
    ScalarInt::new(self.inner() - rhs.inner())
  }
}

impl Mul for ScalarInt {
  type Output = ScalarInt;

  fn mul(self, rhs: Self) -> Self::Output {
    ScalarInt::new(self.inner() * rhs.inner())
  }
}
