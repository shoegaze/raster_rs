use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::double::ScalarDouble,
  vectors::{
    vector2::Vector2,
    double2::Double2
  },
  matrices::{
    matrix2_2::{
      Matrix2_2,
      Matrix2_2Constructor
    },
    ops2_2::Matrix2_2Ops
  }
};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub struct Double2_2 {
  pub(self) a00: ScalarDouble,
  pub(self) a01: ScalarDouble,
  pub(self) a10: ScalarDouble,
  pub(self) a11: ScalarDouble
}

impl Matrix2_2<Double2, ScalarDouble> for Double2_2 {
  fn a00(&self) -> ScalarDouble {
    self.a00
  }

  fn a01(&self) -> ScalarDouble {
    self.a01
  }

  fn a10(&self) -> ScalarDouble {
    self.a10
  }

  fn a11(&self) -> ScalarDouble {
    self.a11
  }
}

impl Matrix2_2Constructor<Double2, ScalarDouble> for Double2_2 {
  fn new(a00: ScalarDouble, a01: ScalarDouble,
         a10: ScalarDouble, a11: ScalarDouble) -> Self {
    Self { a00, a01,
           a10, a11 }
  }
}

impl From<(Double2, Double2)> for Double2_2 {
  fn from(cols: (Double2, Double2)) -> Self {
    Self::new(
      cols.0.x(), cols.1.x(),
      cols.0.y(), cols.1.y()
    )
  }
}

impl Matrix2_2Ops<Double2, ScalarDouble> for Double2_2 {}

impl Add for Double2_2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Double2, ScalarDouble>>::add(&self, &rhs)
  }
}

impl Sub for Double2_2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Double2, ScalarDouble>>::sub(&self, &rhs)
  }
}

impl Mul<ScalarDouble> for Double2_2 {
  type Output = Self;

  fn mul(self, rhs: ScalarDouble) -> Self::Output {
    <Self as Matrix2_2Ops<Double2, ScalarDouble>>::mul(&self, &rhs)
  }
}

impl Div<ScalarDouble> for Double2_2 {
  type Output = Self;

  fn div(self, rhs: ScalarDouble) -> Self::Output {
    <Self as Matrix2_2Ops<Double2, ScalarDouble>>::div(&self, &rhs)
  }
}
