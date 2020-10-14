use std::ops::{Neg, Add, Sub, Mul, Div};
use super::{
  int::ScalarInt,
  float::ScalarFloat,
  double::ScalarDouble
};

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Scalar<T>(pub(crate) T)
  where T: ScalarInner;

pub trait ScalarInner:
    Sized + Copy + Clone +
    PartialOrd + PartialEq +
    Neg + Add + Sub + Mul + Div {}

impl ScalarInner for i32 {}
impl ScalarInner for f32 {}
impl ScalarInner for f64 {}

pub trait ScalarTrait:
    Default + Sized + Copy +
    From<ScalarInt> + From<ScalarFloat> + From<ScalarDouble> +
    Into<ScalarInt> + Into<ScalarFloat> + Into<ScalarDouble> {
  type Inner: ScalarInner;

  fn zero() -> Self {
    Self::default()
  }

  fn one() -> Self;

  fn new(value: Self::Inner) -> Self;
  fn inner(&self) -> Self::Inner;
}
