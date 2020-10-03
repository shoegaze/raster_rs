pub trait Scalar {
  type Primitive;
  
  fn new(value: Self::Primitive) -> Self;
  fn to_primitive(&self) -> Self::Primitive;
}