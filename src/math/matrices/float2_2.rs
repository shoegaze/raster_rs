use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::float::ScalarFloat,
  vectors::{
    vector2::Vector2,
    float2::Float2
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
pub struct Float2_2 {
  pub(self) a00: ScalarFloat,
  pub(self) a01: ScalarFloat,
  pub(self) a10: ScalarFloat,
  pub(self) a11: ScalarFloat
}

impl Matrix2_2<Float2, ScalarFloat> for Float2_2 {
  fn a00(&self) -> ScalarFloat {
    self.a00
  }

  fn a01(&self) -> ScalarFloat {
    self.a01
  }

  fn a10(&self) -> ScalarFloat {
    self.a10
  }

  fn a11(&self) -> ScalarFloat {
    self.a11
  }
}

impl Matrix2_2Constructor<Float2, ScalarFloat> for Float2_2 {
  fn new(a00: ScalarFloat, a01: ScalarFloat,
         a10: ScalarFloat, a11: ScalarFloat) -> Self {
    Self { a00, a01,
           a10, a11 }
  }
}

impl From<(Float2, Float2)> for Float2_2 {
  fn from(cols: (Float2, Float2)) -> Self {
    Self::new(
      cols.0.x(), cols.1.x(),
      cols.0.y(), cols.1.y()
    )
  }
}

impl Matrix2_2Ops<Float2, ScalarFloat> for Float2_2 {}

impl Add for Float2_2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Float2, ScalarFloat>>::add(&self, &rhs)
  }
}

impl Sub for Float2_2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Float2, ScalarFloat>>::sub(&self, &rhs)
  }
}

impl Mul<ScalarFloat> for Float2_2 {
  type Output = Self;

  fn mul(self, rhs: ScalarFloat) -> Self::Output {
    <Self as Matrix2_2Ops<Float2, ScalarFloat>>::mul(&self, &rhs)
  }
}

impl Div<ScalarFloat> for Float2_2 {
  type Output = Self;

  fn div(self, rhs: ScalarFloat) -> Self::Output {
    <Self as Matrix2_2Ops<Float2, ScalarFloat>>::div(&self, &rhs)
  }
}
