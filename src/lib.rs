mod math;

#[cfg(test)]
mod tests {
  use crate::math::{
    scalars::{
      scalar::ScalarTrait,
      ops::ScalarRealOps,
      int::ScalarInt,
      float::ScalarFloat,
      double::ScalarDouble
    },
    vectors::{
      ops2::Vector2Ops,
      int2::Int2,
      float2::Float2
    }
  };
  use crate::math::vectors::double2::Double2;

  fn approx_eq_f32(lhs: f32, rhs: f32) -> bool {
    use std::f32::EPSILON;

    (lhs - rhs) < EPSILON
  }

  fn approx_eq_f64(lhs: f64, rhs: f64) -> bool {
    use std::f64::EPSILON;

    (lhs - rhs) < EPSILON
  }

  #[test]
  fn scalar_conversion() {
    use std::f32::consts::PI as PI_f32;
    use std::f64::consts::PI as PI_f64;

    let sd = ScalarDouble::new(PI_f64);

    assert_eq!(ScalarInt::from(sd).inner(), 3);

    let res = ScalarFloat::from(sd).inner();
    assert!(approx_eq_f32(res, PI_f32));

    let res = ScalarDouble::from(sd).inner();
    assert!(approx_eq_f64(res, PI_f64));
  }

  #[test]
  fn int_ops() {
    let i1 = 1;
    let i2 = 2;
    let si1 = ScalarInt::new(i1);
    let si2 = ScalarInt::new(i2);

    assert_eq!(-si1, ScalarInt::new(-i1));
    assert_eq!(si1 + si2, ScalarInt::new(i1 + i2));
    assert_eq!(si1 - si2, ScalarInt::new(i1 - i2));
    assert_eq!(si1 * si2, ScalarInt::new(i1 * i2));
  }

  #[test]
  fn float_ops() {
    let f1 = 0.75_f32;
    let f2 = 1.5_f32;
    let sf1 = ScalarFloat::new(f1);
    let sf2 = ScalarFloat::new(f2);

    let res = (-sf1).inner();
    assert!(approx_eq_f32(res, -f1));

    let res = (sf1 + sf2).inner();
    assert!(approx_eq_f32(res, f1 + f2));

    let res = (sf1 - sf2).inner();
    assert!(approx_eq_f32(res, f1 - f2));

    let res = (sf1 * sf2).inner();
    assert!(approx_eq_f32(res, f1 * f2));

    let res = (sf1 / sf2).inner();
    assert!(approx_eq_f32(res, f1 / f2));

    let res = sf1.powf(sf2).inner();
    assert!(approx_eq_f32(res, f1.powf(f2)));

    let res = sf1.sqrt().inner();
    assert!(approx_eq_f32(res, f1.sqrt()));
  }

  #[test]
  fn double_ops() {
    let d1 = 0.75_f64;
    let d2 = 0.125_f64;
    let sd1 = ScalarDouble::new(d1);
    let sd2 = ScalarDouble::new(d2);

    let res = (-sd1).inner();
    assert!(approx_eq_f64(res, -d1));

    let res = (sd1 + sd2).inner();
    assert!(approx_eq_f64(res, d1 + d2));

    let res = (sd1 - sd2).inner();
    assert!(approx_eq_f64(res, d1 - d2));

    let res = (sd1 * sd2).inner();
    assert!(approx_eq_f64(res, d1 * d2));

    let res = (sd1 / sd2).inner();
    assert!(approx_eq_f64(res, d1 / d2));

    let res = sd1.powf(sd2).inner();
    assert!(approx_eq_f64(res, d1.powf(d2)));

    let res = sd1.sqrt().inner();
    assert!(approx_eq_f64(res, d1.sqrt()));
  }

  #[test]
  fn vector2_add() {
    let v2i1 = Int2::from((1, 2));
    let v2i2 = Int2::from((3, 4));

    assert_eq!(v2i1 + v2i2, Int2::from((4, 6)));


    let v2f1 = Float2::from((1.5, 2.25));
    let v2f2 = Float2::from((3.75, 5.0));

    assert_eq!(v2f1 + v2f2, Float2::from((5.25, 7.25)));


    let v2d1 = Double2::from((1.5, 2.25));
    let v2d2 = Double2::from((3.75, 5.0));

    assert_eq!(v2d1 + v2d2, Double2::from((5.25, 7.25)));
  }

  #[test]
  fn vector2_sub() {
    let v2i1 = Int2::from((1, 2));
    let v2i2 = Int2::from((3, 4));

    assert_eq!(v2i1 - v2i2, Int2::from((-2, -2)));


    let v2f1 = Float2::from((1.5, 2.25));
    let v2f2 = Float2::from((3.75, 5.0));

    assert_eq!(v2f1 - v2f2, Float2::from((-2.25, -2.75)));

    let v2d1 = Double2::from((1.5, 2.25));
    let v2d2 = Double2::from((3.75, 5.0));

    assert_eq!(v2d1 - v2d2, Double2::from((-2.25, -2.75)));
  }

  #[test]
  fn vector2_mul() {
    let v2i1 = Int2::from((1, 2));
    let i1 = 3;

    assert_eq!(v2i1 * i1, Int2::from((3, 6)));


    let v2f1 = Float2::from((1.5, 2.25));
    let f1 = 3.75;

    assert_eq!(v2f1 * f1, Float2::from((5.625, 8.4375)));


    let v2d1 = Double2::from((1.5, 2.25));
    let d1 = 3.75;

    assert_eq!(v2d1 * d1, Double2::from((5.625, 8.4375)));
  }

  #[test]
  fn vector2_div() {
    let v2i1 = Int2::from((1, 2));
    let i1 = 2;

    assert_eq!(v2i1 / i1, Int2::from((0, 1)));


    let v2f1 = Float2::from((1.5, 2.25));
    let f1 = 3.75;

    assert_eq!(v2f1 / f1, Float2::from((0.4, 0.6)));


    let v2d1 = Double2::from((1.5, 2.25));
    let d1 = 3.75;

    assert_eq!(v2d1 * d1, Double2::from((5.625, 8.4375)));
  }

  #[test]
  fn vector2_dot() {
    let v2i1 = Int2::from((1, -1));
    let v2i2 = Int2::from((2, 3));

    assert_eq!(v2i1.dot(&v2i2).inner(), -1);


    let v2f1 = Float2::from((1.5, 2.25));
    let v2f2 = Float2::from((3.75, 5.0));

    assert_eq!(v2f1.dot(&v2f2).inner(), 16.875);


    let v2d1 = Double2::from((1.5, 2.25));
    let v2d2 = Double2::from((3.75, 5.0));

    assert_eq!(v2d1.dot(&v2d2).inner(), 16.875);
  }

  #[test]
  fn vector2_len_2() {
    let v2i1 = Int2::from((2, 3));

    assert_eq!(v2i1.len_2().inner(), 13);


    let v2f1 = Float2::from((1.5, 2.25));

    assert_eq!(v2f1.len_2().inner(), 7.3125);


    let v2d1 = Double2::from((1.5, 2.25));

    assert_eq!(v2d1.len_2().inner(), 7.3125);
  }

  #[test]
  fn vector2_length() {
    let (x, y) = (2, 3);
    let v2i1 = Int2::from((x, y));
    let len = (x*x + y*y) as f64;

    assert_eq!(v2i1.len().inner(), len.sqrt());

    let (x, y) = (1.5, 2.25);
    let v2f1 = Float2::from((x, y));
    let len = (x*x + y*y) as f64;

    assert_eq!(v2f1.len().inner(), len.sqrt());


    let (x, y) = (1.5, 2.25);
    let v2d1 = Double2::from((x, y));
    let len = (x*x + y*y) as f64;

    assert_eq!(v2d1.len().inner(), len.sqrt());
  }
}
