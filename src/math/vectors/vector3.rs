pub(crate) trait Vector3<T> {
  fn new(x: T, y: T, z: T) -> Self;

  fn x(&self) -> T;
  fn y(&self) -> T;
  fn z(&self) -> T;
  fn set_x(&mut self, value: T);
  fn set_y(&mut self, value: T);
  fn set_z(&mut self, value: T);

  fn r(&self) -> T {
    self.x()
  }

  fn g(&self) -> T {
    self.y()
  }

  fn b(&self) -> T {
    self.y()
  }
}
