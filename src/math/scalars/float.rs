use std::ops::{Neg, Add, Sub, Mul, Div};
use super::{
  scalar::{Scalar, ScalarTrait},
  ops::{ScalarNaturalOps, ScalarRealOps},
  int::ScalarInt,
  double::ScalarDouble
};

pub type ScalarFloat = Scalar<f32>;

impl ScalarTrait<f32> for ScalarFloat {
  fn new(value: f32) -> Self {
    Scalar(value)
  }
  
  fn inner(&self) -> f32 {
    self.0
  }
}

impl From<ScalarInt> for ScalarFloat {
  fn from(scalar: ScalarInt) -> Self {
    ScalarFloat::new(scalar.inner() as f32)
  }
}

impl From<ScalarDouble> for ScalarFloat {
  fn from(scalar: ScalarDouble) -> Self {
    ScalarFloat::new(scalar.inner() as f32)
  }
}

impl ScalarNaturalOps for ScalarFloat {
  type Output = ScalarFloat;
}

impl Neg for ScalarFloat {
  type Output = ScalarFloat;

  fn neg(self) -> Self::Output {
    ScalarFloat::new(-self.inner())
  }
}

impl Add for ScalarFloat {
  type Output = ScalarFloat;

  fn add(self, rhs: Self) -> Self::Output {
    ScalarFloat::new(self.inner() + rhs.inner())
  }
}

impl Sub for ScalarFloat {
  type Output = ScalarFloat;

  fn sub(self, rhs: Self) -> Self::Output {
    ScalarFloat::new(self.inner() - rhs.inner())
  }
}

impl Mul for ScalarFloat {
  type Output = ScalarFloat;

  fn mul(self, rhs: Self) -> Self::Output {
    ScalarFloat::new(self.inner() * rhs.inner())
  }
}

impl ScalarRealOps for ScalarFloat {
  type Output = ScalarFloat;

  fn pow(&self, rhs: Self) -> <Self as ScalarRealOps>::Output {
    ScalarFloat::new(self.inner().powf(rhs.inner()))
  }

  fn sqrt(&self) -> <Self as ScalarRealOps>::Output {
    ScalarFloat::new(self.inner().sqrt())
  }
}

impl Div for ScalarFloat {
  type Output = ScalarFloat;

  fn div(self, rhs: Self) -> Self::Output {
    ScalarFloat::new(self.inner() / rhs.inner())
  }
}