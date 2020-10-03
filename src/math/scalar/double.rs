use crate::math::scalar::ops::{ScalarOps, ScalarFloatOps};

#[derive(Debug, Copy, Clone)]
pub struct Double(pub f64);

impl ScalarOps for Double {
  type Element = Double;

  fn negate(&self) -> Self::Element {
    Double(-self.0)
  }

  fn inverse(&self) -> Self::Element {
    Double(1.0 / self.0)
  }

  fn add(&self, rhs: &Self::Element) -> Self::Element {
    Double(self.0 + rhs.0)
  }

  fn mul(&self, rhs: &Self::Element) -> Self::Element {
    Double(self.0 * rhs.0)
  }

  fn pow(&self, rhs: &Self::Element) -> Self {
    Double(self.0.powf(rhs.0))
  }
}

impl ScalarFloatOps for Double {
  type Element = Double;

  fn sqrt(&self, rhs: &Self) -> Self::Element {
    Double(self.0.sqrt())
  }
}