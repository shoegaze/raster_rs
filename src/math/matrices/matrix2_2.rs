use crate::math::vectors::vector2::Vector2;
use crate::math::scalars::scalar::ScalarTrait;

#[allow(non_camel_case_types)]
pub(crate) trait Matrix2_2<V, S>
  where V: Vector2<S>,
        S: ScalarTrait {

  fn a00(&self) -> S;
  fn a01(&self) -> S;
  fn a10(&self) -> S;
  fn a11(&self) -> S;

  fn row(&self, i: usize) -> Option<V> {
    match i {
      0 => Some(V::new(self.a00().to_owned(), self.a01().to_owned())),
      1 => Some(V::new(self.a10().to_owned(), self.a11().to_owned())),
      _ => None
    }
  }

  fn col(&self, j: usize) -> Option<V> {
    match j {
      0 => Some(V::new(self.a00().to_owned(), self.a10().to_owned())),
      1 => Some(V::new(self.a01().to_owned(), self.a11().to_owned())),
      _ => None
    }
  }
}

#[allow(non_camel_case_types)]
pub(crate) trait Matrix2_2Constructor<V, S>
  where Self: From<(V, V)>,
        V: Vector2<S>,
        S: ScalarTrait {

  fn new(a00: S, a01: S,
         a10: S, a11: S) -> Self;
}
