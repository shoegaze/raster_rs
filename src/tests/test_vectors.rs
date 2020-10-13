use crate::math::{
  scalars::scalar::ScalarTrait,
  vectors::{
    ops2::Vector2Ops,
    int2::Int2,
    float2::Float2,
    double2::Double2
  }
};

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
  let len = (x * x + y * y) as f64;

  assert_eq!(v2i1.len().inner(), len.sqrt());

  let (x, y) = (1.5, 2.25);
  let v2f1 = Float2::from((x, y));
  let len = (x * x + y * y) as f64;

  assert_eq!(v2f1.len().inner(), len.sqrt());


  let (x, y) = (1.5, 2.25);
  let v2d1 = Double2::from((x, y));
  let len = (x*x + y*y) as f64;

  assert_eq!(v2d1.len().inner(), len.sqrt());
}
