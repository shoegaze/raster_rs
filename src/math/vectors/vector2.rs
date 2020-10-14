use crate::math::scalars::scalar::ScalarTrait;

pub(crate) trait Vector2<S>
  where Self: Sized,
        S: ScalarTrait {
  fn new(x: S, y: S) -> Self;

  fn zero() -> Self {
    Self::new(S::zero(), S::zero())
  }

  fn ones() -> Self {
    Self::new(S::one(), S::one())
  }

  fn right() -> Self {
    Self::new(S::one(), S::zero())
  }

  fn up() -> Self {
    Self::new(S::zero(), S::one())
  }

  fn x(&self) -> S;
  fn y(&self) -> S;
  fn set_x(&mut self, value: S);
  fn set_y(&mut self, value: S);
}
