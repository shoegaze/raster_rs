use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble
  },
  vectors::{
    vector2::Vector2,
    ops2::Vector2Ops
  }
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Double2 {
  pub(self) x: ScalarDouble,
  pub(self) y: ScalarDouble
}

impl Vector2<ScalarDouble> for Double2 {
  fn new(x: ScalarDouble, y: ScalarDouble) -> Self {
    Double2 { x, y }
  }

  fn x(&self) -> ScalarDouble {
    self.x
  }

  fn y(&self) -> ScalarDouble {
    self.y
  }

  fn set_x(&mut self, value: ScalarDouble) {
    self.x = value
  }

  fn set_y(&mut self, value: ScalarDouble) {
    self.y = value
  }
}

impl Vector2Ops<ScalarDouble> for Double2 {}

impl From<(f64, f64)> for Double2 {
  fn from(pair: (f64, f64)) -> Self {
    Self::new(
      ScalarDouble::new(pair.0),
      ScalarDouble::new(pair.1)
    )
  }
}

impl Add for Double2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarDouble>>::add(&self, &rhs)
  }
}

impl Sub for Double2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    <Self as Vector2Ops<ScalarDouble>>::sub(&self, &rhs)
  }
}

impl Mul<f64> for Double2 {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    <Self as Vector2Ops<ScalarDouble>>::mul(&self, &ScalarDouble::new(rhs))
  }
}

impl Div<f64> for Double2 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    <Self as Vector2Ops<ScalarDouble>>::div(&self, &ScalarDouble::new(rhs))
  }
}