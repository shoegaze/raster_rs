use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    int::ScalarInt
  },
  vectors::{
    vector3::Vector3,
    ops3::Vector3Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Int3 {
  pub(self) x: ScalarInt,
  pub(self) y: ScalarInt,
  pub(self) z: ScalarInt
}

impl Vector3<ScalarInt> for Int3 {
  fn new(x: ScalarInt, y: ScalarInt, z: ScalarInt) -> Self {
    Int3 { x, y, z }
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

  fn set_x(&mut self, value: ScalarInt) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarInt) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarInt) {
    self.z = value;
  }
}

impl Vector3Ops<ScalarInt> for Int3 {}

impl From<(i32, i32, i32)> for Int3 {
  fn from(tuple: (i32, i32, i32)) -> Self {
    Self::new(
      ScalarInt::new(tuple.0),
      ScalarInt::new(tuple.1),
      ScalarInt::new(tuple.2)
    )
  }
}

impl Add for Int3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarInt>>::add(&self, &rhs)
  }
}

impl Sub for Int3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarInt>>::sub(&self, &rhs)
  }
}

impl Mul<i32> for Int3 {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self::Output {
    <Self as Vector3Ops<ScalarInt>>::mul(&self, &ScalarInt::new(rhs))
  }
}

impl Div<i32> for Int3 {
  type Output = Self;

  fn div(self, rhs: i32) -> Self::Output {
    <Self as Vector3Ops<ScalarInt>>::div(&self, &ScalarInt::new(rhs))
  }
}
