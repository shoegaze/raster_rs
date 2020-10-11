use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::{
    scalar::ScalarTrait,
    double::ScalarDouble,
    ops::ScalarRealOps
  },
  vectors::vector2::Vector2
};

pub(crate) trait Vector2Ops<S>
  where Self: Sized + Vector2<S>,
        S: ScalarTrait +
           Add<Output = S> + Sub<Output = S> +
           Mul<Output = S> + Div<Output = S> {

  fn add(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() + rhs.x(),
      self.y() + rhs.y()
    )
  }

  fn sub(&self, rhs: &Self) -> Self {
    Self::new(
      self.x() - rhs.x(),
      self.y() - rhs.y()
    )
  }

  fn mul(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() * s,
      self.y() * s
    )
  }

  fn div(&self, rhs: &S) -> Self {
    let s = *rhs;
    Self::new(
      self.x() / s,
      self.y() / s
    )
  }

  fn dot(&self, rhs: &Self) -> S {
    let prod_x = self.x() * rhs.x();
    let prod_y = self.y() * rhs.y();

    prod_x + prod_y
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
