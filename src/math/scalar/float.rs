use crate::math::scalar::ops::{ScalarOps, ScalarFloatOps};

#[derive(Debug, Copy, Clone)]
pub struct Float(pub f32);

impl ScalarOps for Float {
  type Element = Float;

  fn negate(&self) -> Self::Element {
    Float(-self.0)
  }

  fn inverse(&self) -> Self::Element {
    Float(1.0 / self.0)
  }

  fn add(&self, rhs: &Self::Element) -> Self::Element {
    Float(self.0 + rhs.0)
  }

  fn mul(&self, rhs: &Self::Element) -> Self::Element {
    Float(self.0 * rhs.0)
  }

  fn pow(&self, rhs: &Self::Element) -> Self {
    Float(self.0.powf(rhs.0))
  }
}

impl ScalarFloatOps for Float {
  type Element = Float;

  fn sqrt(&self, rhs: &Self) -> Self::Element {
    Float(self.0.sqrt())
  }
}