use std::ops::{Neg, Add, Sub, Mul, Div};
use super::{
  scalar::{Scalar, ScalarTrait},
  ops::{ScalarNaturalOps, ScalarRealOps},
  int::ScalarInt,
  float::ScalarFloat
};

pub type ScalarDouble = Scalar<f64>;

impl ScalarTrait for ScalarDouble {
  type Inner = f64;

  fn one() -> Self {
    Self::new(1.0)
  }

  fn new(value: f64) -> Self {
    Scalar(value)
  }
  
  fn inner(&self) -> f64 {
    self.0
  }
}

impl From<ScalarInt> for ScalarDouble {
  fn from(scalar: ScalarInt) -> Self {
    ScalarDouble::new(f64::from(scalar.inner()))
  }
}

impl From<ScalarFloat> for ScalarDouble {
  fn from(scalar: ScalarFloat) -> Self {
    ScalarDouble::new(f64::from(scalar.inner()))
  }
}

impl ScalarNaturalOps for ScalarDouble {
  type Output = ScalarDouble;
}

impl Neg for ScalarDouble {
  type Output = ScalarDouble;

  fn neg(self) -> Self::Output {
    ScalarDouble::new(-self.inner())
  }
}

impl Add for ScalarDouble {
  type Output = ScalarDouble;

  fn add(self, rhs: Self) -> Self::Output {
    ScalarDouble::new(self.inner() + rhs.inner())
  }
}

impl Sub for ScalarDouble {
  type Output = ScalarDouble;

  fn sub(self, rhs: Self) -> Self::Output {
    ScalarDouble::new(self.inner() - rhs.inner())
  }
}

impl Mul for ScalarDouble {
  type Output = ScalarDouble;

  fn mul(self, rhs: Self) -> Self::Output {
    ScalarDouble::new(self.inner() * rhs.inner())
  }
}

impl ScalarRealOps for ScalarDouble {
  type Output = ScalarDouble;

  fn powf(&self, rhs: Self) -> <Self as ScalarRealOps>::Output {
    ScalarDouble::new(self.inner().powf(rhs.inner()))
  }

  fn sqrt(&self) -> <Self as ScalarRealOps>::Output {
    ScalarDouble::new(self.inner().sqrt())
  }
}

impl Div for ScalarDouble {
  type Output = ScalarDouble;

  fn div(self, rhs: Self) -> Self::Output {
    ScalarDouble::new(self.inner() / rhs.inner())
  }
}
