mod math;

#[cfg(test)]
mod tests {
  use crate::math::vector::int2::Int2;
  use crate::math::scalar::int::Int;
  use crate::math::scalar::float::Float;
  // use crate::math::scalar::double::Double;
  use crate::math::scalar::scalar::Scalar;
  use crate::math::scalar::ops::ScalarRealOps;
  
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
  
  #[test]
  fn scalar_int() {
    let i1: Int = Int::new(12);
    let i2: Int = Int::new(35);
    
    assert_eq!((i1 - i2).to_primitive(), -23);
  }
  
  fn approx(lhs: Float, rhs: Float) -> bool {
    (lhs - rhs).0 < f32::EPSILON
  }
  
  #[test]
  fn scalar_float() {
    let f1: Float = Float::new(1.5);
    let f2: Float = Float::new(0.3);
    
    assert!(approx(f1 / f2, Float(5.0)));
    assert!(approx(Float(16.0).sqrt(), Float(4.0)));
  }
}
