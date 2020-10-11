use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    float::ScalarFloat
  },
  vectors::vector2::*
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Float2 {
  pub(self) x: ScalarFloat,
  pub(self) y: ScalarFloat
}

impl Vector2<ScalarFloat> for Float2 {
  fn new(x: ScalarFloat, y: ScalarFloat) -> Self {
    Float2 { x, y }
  }

  fn x(&self) -> ScalarFloat {
    self.x
  }

  fn y(&self) -> ScalarFloat {
    self.y
  }

  fn set_x(&mut self, value: ScalarFloat) {
    self.x = value
  }

  fn set_y(&mut self, value: ScalarFloat) {
    self.y = value
  }
}

impl Vector2Ops<ScalarFloat> for Float2 {}

impl From<(f32, f32)> for Float2 {
  fn from(pair: (f32, f32)) -> Self {
    Self::new(
      ScalarFloat::new(pair.0),
      ScalarFloat::new(pair.1)
    )
  }
}

impl Add for Float2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarFloat>>::add(&self, &rhs)
  }
}

impl Sub for Float2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarFloat>>::sub(&self, &rhs)
  }
}

impl Mul<f32> for Float2 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    <Self as Vector2Ops<ScalarFloat>>::mul(&self, &ScalarFloat::new(rhs))
  }
}

impl Div<f32> for Float2 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    <Self as Vector2Ops<ScalarFloat>>::div(&self, &ScalarFloat::new(rhs))
  }
}