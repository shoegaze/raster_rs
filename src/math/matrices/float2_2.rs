use crate::math::vectors::vector2::Vector2;
use crate::math::matrices::matrix2_2::Matrix2_2;
use crate::math::scalars::float::ScalarFloat;
use crate::math::vectors::float2::Float2;

#[allow(non_camel_case_types)]
pub struct Float2_2 {
  pub(self) a00: ScalarFloat,
  pub(self) a01: ScalarFloat,
  pub(self) a10: ScalarFloat,
  pub(self) a11: ScalarFloat
}

impl Matrix2_2<Float2, ScalarFloat> for Float2_2 {
  fn new(a00: ScalarFloat, a01: ScalarFloat,
         a10: ScalarFloat, a11: ScalarFloat) -> Self {
    Self { a00, a01,
           a10, a11 }
  }

  fn a00(&self) -> &ScalarFloat {
    &self.a00
  }

  fn a01(&self) -> &ScalarFloat {
    &self.a01
  }

  fn a10(&self) -> &ScalarFloat {
    &self.a10
  }

  fn a11(&self) -> &ScalarFloat {
    &self.a11
  }
}
