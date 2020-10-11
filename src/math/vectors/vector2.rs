
pub(crate) trait Vector2<T> {
  fn new(x: T, y: T) -> Self;
  fn x(&self) -> T;
  fn y(&self) -> T;
  fn set_x(&mut self, value: T);
  fn set_y(&mut self, value: T);
}
