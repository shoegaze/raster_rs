use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    int::ScalarInt,
  },
  vectors::{
    vector2::Vector2,
    ops2::Vector2Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Int2 {
  pub(self) x: ScalarInt,
  pub(self) y: ScalarInt
}

impl Vector2<ScalarInt> for Int2 {
  fn new(x: ScalarInt, y: ScalarInt) -> Self {
    Int2 { x, y }
  }

  fn x(&self) -> ScalarInt {
    self.x
  }

  fn y(&self) -> ScalarInt {
    self.y
  }

  fn set_x(&mut self, value: ScalarInt) {
    self.x = value
  }

  fn set_y(&mut self, value: ScalarInt) {
    self.y = value
  }
}

impl Vector2Ops<ScalarInt> for Int2 {}

impl From<(i32, i32)> for Int2 {
  fn from(pair: (i32, i32)) -> Self {
    Self::new(
      ScalarInt::new(pair.0),
      ScalarInt::new(pair.1)
    )
  }
}

impl Add for Int2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarInt>>::add(&self, &rhs)
  }
}

impl Sub for Int2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarInt>>::sub(&self, &rhs)
  }
}

impl Mul<i32> for Int2 {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self::Output {
    <Self as Vector2Ops<ScalarInt>>::mul(&self, &ScalarInt::new(rhs))
  }
}

impl Div<i32> for Int2 {
  type Output = Self;

  fn div(self, rhs: i32) -> Self::Output {
    <Self as Vector2Ops<ScalarInt>>::div(&self, &ScalarInt::new(rhs))
  }
}