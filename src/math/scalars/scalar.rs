use std::fmt::Debug;
use std::ops::{Neg, Add, Sub, Mul, Div};
use super::{
  int::ScalarInt,
  float::ScalarFloat,
  double::ScalarDouble
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Scalar<T>(pub(crate) T)
  where T: ScalarInner;

pub trait ScalarInner:
    Debug + Sized + Copy + Clone +
    PartialOrd + PartialEq +
    Neg + Add + Sub + Mul + Div {}

impl ScalarInner for i32 {}
impl ScalarInner for f32 {}
impl ScalarInner for f64 {}

pub trait ScalarTrait<T>:
    From<ScalarInt> + From<ScalarFloat> + From<ScalarDouble>
  where T: ScalarInner {
  fn new(value: T) -> Self;
  fn inner(&self) -> T;
}
