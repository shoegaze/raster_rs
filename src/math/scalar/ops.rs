use crate::math::scalar::float::Float;
use crate::math::scalar::double::Double;

pub trait ScalarOps {
  type Element;

  fn negate(&self) -> Self::Element;
  fn inverse(&self) -> Self::Element;
  fn add(&self, rhs: &Self::Element) -> Self::Element;
  fn mul(&self, rhs: &Self::Element) -> Self::Element;
  fn pow(&self, rhs: &Self::Element) -> Self::Element;

  fn sub(&self, rhs: &Self) -> Self::Element {
    self.add(&rhs.negate())
  }

  fn div(&self, rhs: &Self) -> Self::Element {
    self.mul(&rhs.inverse())
  }
}

pub trait ScalarFloatOps {
  type Element;

  fn sqrt(&self, rhs: &Self::Element) -> Self::Element;
}