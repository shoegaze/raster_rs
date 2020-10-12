use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble
  },
  vectors::{
    vector4::Vector4,
    ops4::Vector4Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Double4 {
  pub(self) x: ScalarDouble,
  pub(self) y: ScalarDouble,
  pub(self) z: ScalarDouble,
  pub(self) w: ScalarDouble
}

impl Vector4<ScalarDouble> for Double4 {
  fn new(x: ScalarDouble, y: ScalarDouble, z: ScalarDouble, w: ScalarDouble) -> Self {
    Double4 { x, y, z, w }
  }

  fn x(&self) -> ScalarDouble {
    self.x
  }

  fn y(&self) -> ScalarDouble {
    self.y
  }

  fn z(&self) -> ScalarDouble {
    self.z
  }

  fn w(&self) -> ScalarDouble {
    self.w
  }

  fn set_x(&mut self, value: ScalarDouble) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarDouble) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarDouble) {
    self.z = value;
  }

  fn set_w(&mut self, value: ScalarDouble) {
    self.w = value;
  }
}

impl Vector4Ops<ScalarDouble> for Double4 {}

impl From<(f64, f64, f64, f64)> for Double4 {
  fn from(tuple: (f64, f64, f64, f64)) -> Self {
    Self::new(
      ScalarDouble::new(tuple.0),
      ScalarDouble::new(tuple.1),
      ScalarDouble::new(tuple.2),
      ScalarDouble::new(tuple.3),
    )
  }
}

impl Add for Double4 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarDouble>>::add(&self, &rhs)
  }
}

impl Sub for Double4 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector4Ops<ScalarDouble>>::sub(&self, &rhs)
  }
}

impl Mul<f64> for Double4 {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    <Self as Vector4Ops<ScalarDouble>>::mul(&self, &ScalarDouble::new(rhs))
  }
}

impl Div<f64> for Double4 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    <Self as Vector4Ops<ScalarDouble>>::div(&self, &ScalarDouble::new(rhs))
  }
}
