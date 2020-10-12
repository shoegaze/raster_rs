use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble,
    ops::ScalarRealOps
  },
  vectors::vector4::Vector4
};

pub(crate) trait Vector4Ops<S>
  where Self: Sized + Vector4<S>,
        S: ScalarTrait +
           Add<Output = S> + Sub<Output = S> +
           Mul<Output = S> + Div<Output = S> {

 fn add(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() + rhs.x(),
      self.y() + rhs.y(),
      self.z() + rhs.z(),
      self.w() + rhs.w()
    )
  }

  fn sub(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() - rhs.x(),
      self.y() - rhs.y(),
      self.z() - rhs.z(),
      self.w() - rhs.w()
    )
  }

  fn mul(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() * s,
      self.y() * s,
      self.z() * s,
      self.w() * s,
    )
  }

  fn div(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() / s,
      self.y() / s,
      self.z() / s,
      self.w() / s,
    )
  }

  fn dot(&self, rhs: &Self) -> S {
    let prod_x = self.x() * rhs.x();
    let prod_y = self.y() * rhs.y();
    let prod_z = self.z() * rhs.z();
    let prod_w = self.w() * rhs.w();

    prod_x + prod_y + prod_z + prod_w
  }

  fn len_2(&self) -> S {
    self.dot(self)
  }

  fn len(&self) -> ScalarDouble {
    let len_2: ScalarDouble = self.len_2().into();
    len_2.sqrt()
  }

  fn normalize(&self) -> Self {
    let len: S = self.len().into();
    self.div(&len)
  }
}
