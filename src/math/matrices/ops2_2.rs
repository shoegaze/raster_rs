use std::ops::{Add, Sub, Mul, Div};
use crate::math::{
  scalars::scalar::ScalarTrait,
  vectors::{
    vector2::Vector2,
    ops2::Vector2Ops
  },
  matrices::{
    matrix2_2::{
      Matrix2_2,
      Matrix2_2Constructor
    },
    transpose2_2::Transpose2_2
  }
};

#[allow(non_camel_case_types)]
pub(crate) trait Matrix2_2Ops<V, S>
  where Self: Sized + Matrix2_2<V, S> + Matrix2_2Constructor<V, S>,
        V: Vector2<S> + Vector2Ops<S>,
        S: ScalarTrait +
           Add<Output = S> + Sub<Output = S> +
           Mul<Output = S> + Div<Output = S> {

  fn identity() -> Self {
    Self::new(S::one(),  S::zero(),
              S::zero(), S::one())
  }

  fn transpose(&self) -> Transpose2_2<Self, V, S> {
    Transpose2_2::new(self)
  }

  fn add(&self, rhs: &Self) -> Self {
    Self::new(
      self.a00() + rhs.a00(), self.a01() + rhs.a01(),
      self.a10() + rhs.a10(), self.a11() + rhs.a11()
    )
  }

  fn sub(&self, rhs: &Self) -> Self {
    Self::new(
      self.a00() - rhs.a00(), self.a01() - rhs.a01(),
      self.a10() - rhs.a10(), self.a11() - rhs.a11()
    )
  }

  fn mul(&self, rhs: &S) -> Self {
    let s = rhs.to_owned();

    Self::new(
      self.a00() * s, self.a01() * s,
      self.a10() * s, self.a11() * s
    )
  }

  fn div(&self, rhs: &S) -> Self {
    let s = rhs.to_owned();

    Self::new(
      self.a00() / s, self.a01() / s,
      self.a10() / s, self.a11() / s
    )
  }

  fn dot(&self, rhs: &Self) -> Self {
    let row0 = self.row(0).unwrap();
    let row1 = self.row(1).unwrap();
    let col0 = rhs.col(0).unwrap();
    let col1 = rhs.col(1).unwrap();

    Self::new(
      row0.dot(&col0), row0.dot(&col1),
      row1.dot(&col0), row1.dot(&col1)
    )
  }

  fn dot_vector(&self, rhs: &V) -> V {
    let row0 = self.row(0).unwrap();
    let row1 = self.row(1).unwrap();

    V::new(
      row0.dot(rhs),
      row1.dot(rhs)
    )
  }

  fn det(&self) -> S {
    let a = self.a00();
    let b = self.a01();
    let c = self.a10();
    let d = self.a11();

    a*d - b*c
  }
}
