use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble
  },
  vectors::{
    vector3::Vector3,
    ops3::Vector3Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Double3 {
  pub(self) x: ScalarDouble,
  pub(self) y: ScalarDouble,
  pub(self) z: ScalarDouble
}

impl Vector3<ScalarDouble> for Double3 {
  fn new(x: ScalarDouble, y: ScalarDouble, z: ScalarDouble) -> Self {
    Double3 { x, y, z }
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

  fn set_x(&mut self, value: ScalarDouble) {
    self.x = value;
  }

  fn set_y(&mut self, value: ScalarDouble) {
    self.y = value;
  }

  fn set_z(&mut self, value: ScalarDouble) {
    self.z = value;
  }
}

impl Vector3Ops<ScalarDouble> for Double3 {}

impl From<(f64, f64, f64)> for Double3 {
  fn from(tuple: (f64, f64, f64)) -> Self {
    Self::new(
      ScalarDouble::new(tuple.0),
      ScalarDouble::new(tuple.1),
      ScalarDouble::new(tuple.2)
    )
  }
}

impl Add for Double3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarDouble>>::add(&self, &rhs)
  }
}

impl Sub for Double3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector3Ops<ScalarDouble>>::sub(&self, &rhs)
  }
}

impl Mul<f64> for Double3 {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    <Self as Vector3Ops<ScalarDouble>>::mul(&self, &ScalarDouble::new(rhs))
  }
}

impl Div<f64> for Double3 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    <Self as Vector3Ops<ScalarDouble>>::div(&self, &ScalarDouble::new(rhs))
  }
}
