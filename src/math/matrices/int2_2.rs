use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::int::ScalarInt,
  vectors::{
    vector2::Vector2,
    int2::Int2
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
pub struct Int2_2 {
  pub(self) a00: ScalarInt,
  pub(self) a01: ScalarInt,
  pub(self) a10: ScalarInt,
  pub(self) a11: ScalarInt
}

impl Matrix2_2<Int2, ScalarInt> for Int2_2 {
  fn a00(&self) -> ScalarInt {
    self.a00
  }

  fn a01(&self) -> ScalarInt {
    self.a01
  }

  fn a10(&self) -> ScalarInt {
    self.a10
  }

  fn a11(&self) -> ScalarInt {
    self.a11
  }
}

impl Matrix2_2Constructor<Int2, ScalarInt> for Int2_2 {
  fn new(a00: ScalarInt, a01: ScalarInt,
         a10: ScalarInt, a11: ScalarInt) -> Self {
    Self { a00, a01,
           a10, a11 }
  }
}

impl From<(Int2, Int2)> for Int2_2 {
  fn from(cols: (Int2, Int2)) -> Self {
    Self::new(
      cols.0.x(), cols.1.x(),
      cols.0.y(), cols.1.y()
    )
  }
}

impl Matrix2_2Ops<Int2, ScalarInt> for Int2_2 {}

impl Add for Int2_2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Int2, ScalarInt>>::add(&self, &rhs)
  }
}

impl Sub for Int2_2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Matrix2_2Ops<Int2, ScalarInt>>::sub(&self, &rhs)
  }
}

impl Mul<ScalarInt> for Int2_2 {
  type Output = Self;

  fn mul(self, rhs: ScalarInt) -> Self::Output {
    <Self as Matrix2_2Ops<Int2, ScalarInt>>::mul(&self, &rhs)
  }
}

impl Div<ScalarInt> for Int2_2 {
  type Output = Self;

  fn div(self, rhs: ScalarInt) -> Self::Output {
    <Self as Matrix2_2Ops<Int2, ScalarInt>>::div(&self, &rhs)
  }
}
