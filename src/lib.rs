mod math;

#[cfg(test)]
mod tests {
  use crate::math::scalars::{
    scalar::ScalarTrait,
    ops::ScalarRealOps,
    int::ScalarInt,
    float::ScalarFloat,
    double::ScalarDouble
  };

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
  fn fixed_scalar_ops() {
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
  fn float_scalar_ops() {
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

    let res = sf1.pow(sf2).inner();
    assert!(approx_eq_f32(res, f1.powf(f2)));

    let res = sf1.sqrt().inner();
    assert!(approx_eq_f32(res, f1.sqrt()));
  }

  #[test]
  fn double_scalar_ops() {
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

    let res = sd1.pow(sd2).inner();
    assert!(approx_eq_f64(res, d1.powf(d2)));

    let res = sd1.sqrt().inner();
    assert!(approx_eq_f64(res, d1.sqrt()));
  }
}
