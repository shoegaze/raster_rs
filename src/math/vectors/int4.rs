use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    int::ScalarInt
  },
  vectors::{
    vector4::Vector4,
    ops4::Vector4Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Int4 {
  pub(self) x: ScalarInt,
  pub(self) y: ScalarInt,
  pub(self) z: ScalarInt,
  pub(self) w: ScalarInt
}

impl Vector4<ScalarInt> for Int4 {
  fn new(x: ScalarInt, y: ScalarInt, z: ScalarInt, w: ScalarInt) -> Self {
    Int4 { x, y, z, w }
  }

  fn x(&self) -> ScalarInt {
    self.x
  }

  fn y(&self) -> ScalarInt {
    self.y
  }

  fn z(&self) -> ScalarInt {
    self.z
  }

  fn w(&self) -> ScalarInt {
    self.w
  }

  fn set_x(&mut self, value: ScalarInt) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarInt) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarInt) {
    self.z = value;
  }

  fn set_w(&mut self, value: ScalarInt) {
    self.w = value;
  }
}

impl Vector4Ops<ScalarInt> for Int4 {}

impl From<(i32, i32, i32, i32)> for Int4 {
  fn from(tuple: (i32, i32, i32, i32)) -> Self {
    Self::new(
      ScalarInt::new(tuple.0),
      ScalarInt::new(tuple.1),
      ScalarInt::new(tuple.2),
      ScalarInt::new(tuple.3),
    )
  }
}

impl Add for Int4 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarInt>>::add(&self, &rhs)
  }
}

impl Sub for Int4 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarInt>>::sub(&self, &rhs)
  }
}

impl Mul<i32> for Int4 {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self::Output {
    <Self as Vector4Ops<ScalarInt>>::mul(&self, &ScalarInt::new(rhs))
  }
}

impl Div<i32> for Int4 {
  type Output = Self;

  fn div(self, rhs: i32) -> Self::Output {
    <Self as Vector4Ops<ScalarInt>>::div(&self, &ScalarInt::new(rhs))
  }
}
