use crate::math::{
  matrices::{
    matrix2_2::Matrix2_2Constructor,
    ops2_2::Matrix2_2Ops,
    int2_2::Int2_2,
    float2_2::Float2_2
  },
  vectors::{
    int2::Int2,
    float2::Float2
  },
  scalars::{
    scalar::ScalarTrait,
    int::ScalarInt,
    float::ScalarFloat
  }
};

#[test]
fn mat2_2_identity() {
  let m2id = Int2_2::identity();
  let zero = ScalarInt::zero();
  let one = ScalarInt::one();

  assert_eq!(m2id,
             Int2_2::new(
               one, zero,
               zero, one
             ));

  let m2id = Float2_2::identity();
  let zero = ScalarFloat::zero();
  let one = ScalarFloat::one();

  assert_eq!(m2id,
             Float2_2::new(
               one, zero,
               zero, one
             ));
}

#[test]
fn mat2_2_transpose() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  assert_eq!(m2i1.transpose().as_matrix(),
             Int2_2::from((
               Int2::from((1, 3)),
               Int2::from((2, 4))
             )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  assert_eq!(m2f1.transpose().as_matrix(),
             Float2_2::from((
               Float2::from((1.25, 3.75)),
               Float2::from((2.5,  5.0))
             )));
}

#[test]
fn mat2_2_add() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  let col0 = Int2::from((5, 6));
  let col1 = Int2::from((7, 8));
  let m2i2 = Int2_2::from((col0, col1));

  assert_eq!(m2i1 + m2i2, Int2_2::from((
    Int2::from((6, 8)),
    Int2::from((10, 12))
  )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  let col0 = Float2::from((6.25, 7.5));
  let col1 = Float2::from((8.75, 10.0));
  let m2f2 = Float2_2::from((col0, col1));

  assert_eq!(m2f1 + m2f2, Float2_2::from((
    Float2::from((7.5,  10.0)),
    Float2::from((12.5, 15.0))
  )));
}

#[test]
fn mat2_2_sub() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  let col0 = Int2::from((5, 6));
  let col1 = Int2::from((7, 8));
  let m2i2 = Int2_2::from((col0, col1));

  assert_eq!(m2i1 - m2i2, Int2_2::from((
    Int2::from((-4, -4)),
    Int2::from((-4, -4))
  )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  let col0 = Float2::from((6.25, 7.5));
  let col1 = Float2::from((8.75, 10.0));
  let m2f2 = Float2_2::from((col0, col1));

  assert_eq!(m2f1 - m2f2, Float2_2::from((
    Float2::from((-5.0, -5.0)),
    Float2::from((-5.0, -5.0))
  )));
}

#[test]
fn mat2_2_mul() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  assert_eq!(m2i1 * ScalarInt::new(2), Int2_2::from((
    Int2::from((2, 4)),
    Int2::from((6, 8))
  )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  assert_eq!(m2f1 * ScalarFloat::new(2.0), Float2_2::from((
    Float2::from((2.5, 5.0)),
    Float2::from((7.5, 10.0))
  )));
}

#[test]
fn mat2_2_div() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  assert_eq!(m2i1 / ScalarInt::new(2), Int2_2::from((
    Int2::from((0, 1)),
    Int2::from((1, 2))
  )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  assert_eq!(m2f1 / ScalarFloat::new(2.0), Float2_2::from((
    Float2::from((0.625, 1.25)),
    Float2::from((1.875, 2.5))
  )));
}

#[test]
fn mat2_2_dot() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  let col0 = Int2::from((5, 6));
  let col1 = Int2::from((7, 8));
  let m2i2 = Int2_2::from((col0, col1));

  assert_eq!(m2i1.dot(&m2i2), Int2_2::from((
    Int2::from((23, 34)),
    Int2::from((31, 46))
  )));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  let col0 = Float2::from((6.25, 7.5));
  let col1 = Float2::from((8.75, 10.0));
  let m2f2 = Float2_2::from((col0, col1));

  assert_eq!(m2f1.dot(&m2f2), Float2_2::from((
    Float2::from((35.9375, 53.125)),
    Float2::from((48.4375, 71.875))
  )));
}

#[test]
fn mat2_2_dot_vector() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  let v2i1 = Int2::from((5, 6));

  assert_eq!(m2i1.dot_vector(&v2i1), Int2::from((23, 34)));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  let v2f1 = Float2::from((6.25, 7.5));

  assert_eq!(m2f1.dot_vector(&v2f1), Float2::from((35.9375, 53.125)));
}

#[test]
fn mat2_2_det() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  assert_eq!(m2i1.det(), ScalarInt::new(-2));


  let col0 = Float2::from((1.25, 2.5));
  let col1 = Float2::from((3.75, 5.0));
  let m2f1 = Float2_2::from((col0, col1));

  assert_eq!(m2f1.det(), ScalarFloat::new(-3.125));
}
