use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble,
    ops::ScalarRealOps
  },
  vectors::vector3::Vector3
};

pub(crate) trait Vector3Ops<S>
  where Self: Sized + Vector3<S>,
        S: ScalarTrait +
           Add<Output = S> + Sub<Output = S> +
           Mul<Output = S> + Div<Output = S> {

  fn add(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() + rhs.x(),
      self.y() + rhs.y(),
      self.z() + rhs.z()
    )
  }

  fn sub(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() - rhs.x(),
      self.y() - rhs.y(),
      self.z() - rhs.z()
    )
  }

  fn mul(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() * s,
      self.y() * s,
      self.z() * s
    )
  }

  fn div(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() / s,
      self.y() / s,
      self.z() / s
    )
  }

  fn dot(&self, rhs: &Self) -> S {
    let prod_x = self.x() * rhs.x();
    let prod_y = self.y() * rhs.y();
    let prod_z = self.z() * rhs.z();

    prod_x + prod_y + prod_z
  }

  fn cross(&self, rhs: &Self) -> Self {
    let x = self.y() * rhs.z() - self.z() * rhs.y();
    let y = self.z() * rhs.x() - self.x() * rhs.z();
    let z = self.x() * rhs.y() - self.y() * rhs.x();

    Self::new(x, y, z)
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