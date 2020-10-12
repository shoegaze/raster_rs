use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    float::ScalarFloat
  },
  vectors::{
    vector3::Vector3,
    ops3::Vector3Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Float3 {
  x: ScalarFloat,
  y: ScalarFloat,
  z: ScalarFloat
}

impl Vector3<ScalarFloat> for Float3 {
  fn new(x: ScalarFloat, y: ScalarFloat, z: ScalarFloat) -> Self {
    Float3 { x, y, z }
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

  fn set_x(&mut self, value: ScalarFloat) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarFloat) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarFloat) {
    self.z = value;
  }
}

impl Vector3Ops<ScalarFloat> for Float3 {}

impl From<(f32, f32, f32)> for Float3 {
  fn from(tuple: (f32, f32, f32)) -> Self {
    Self::new(
      ScalarFloat::new(tuple.0),
      ScalarFloat::new(tuple.1),
      ScalarFloat::new(tuple.2)
    )
  }
}

impl Add for Float3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarFloat>>::add(&self, &rhs)
  }
}

impl Sub for Float3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarFloat>>::sub(&self, &rhs)
  }
}

impl Mul<f32> for Float3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    <Self as Vector3Ops<ScalarFloat>>::mul(&self, &ScalarFloat::new(rhs))
  }
}

impl Div<f32> for Float3 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    <Self as Vector3Ops<ScalarFloat>>::div(&self, &ScalarFloat::new(rhs))
  }
}
