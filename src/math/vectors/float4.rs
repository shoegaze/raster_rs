use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    float::ScalarFloat
  },
  vectors::{
    vector4::Vector4,
    ops4::Vector4Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Float4 {
  pub(self) x: ScalarFloat,
  pub(self) y: ScalarFloat,
  pub(self) z: ScalarFloat,
  pub(self) w: ScalarFloat
}

impl Vector4<ScalarFloat> for Float4 {
  fn new(x: ScalarFloat, y: ScalarFloat, z: ScalarFloat, w: ScalarFloat) -> Self {
    Float4 { x, y, z, w }
  }

  fn x(&self) -> ScalarFloat {
    self.x
  }

  fn y(&self) -> ScalarFloat {
    self.y
  }

  fn z(&self) -> ScalarFloat {
    self.z
  }

  fn w(&self) -> ScalarFloat {
    self.w
  }

  fn set_x(&mut self, value: ScalarFloat) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarFloat) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarFloat) {
    self.z = value;
  }

  fn set_w(&mut self, value: ScalarFloat) {
    self.w = value;
  }
}

impl Vector4Ops<ScalarFloat> for Float4 {}

impl From<(f32, f32, f32, f32)> for Float4 {
  fn from(tuple: (f32, f32, f32, f32)) -> Self {
    Self::new(
      ScalarFloat::new(tuple.0),
      ScalarFloat::new(tuple.1),
      ScalarFloat::new(tuple.2),
      ScalarFloat::new(tuple.3),
    )
  }
}

impl Add for Float4 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarFloat>>::add(&self, &rhs)
  }
}

impl Sub for Float4 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarFloat>>::sub(&self, &rhs)
  }
}

impl Mul<f32> for Float4 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    <Self as Vector4Ops<ScalarFloat>>::mul(&self, &ScalarFloat::new(rhs))
  }
}

impl Div<f32> for Float4 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    <Self as Vector4Ops<ScalarFloat>>::div(&self, &ScalarFloat::new(rhs))
  }
}
