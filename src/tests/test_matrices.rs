use crate::math::matrices::int2_2::Int2_2;
use crate::math::vectors::int2::Int2;
use crate::math::matrices::ops2_2::Matrix2_2Ops;
use crate::math::matrices::matrix2_2::Matrix2_2Constructor;
use crate::math::scalars::int::ScalarInt;
use crate::math::scalars::scalar::ScalarTrait;

#[test]
fn mat2_2_identity() {
  let m2id = Int2_2::identity();
  let zero = ScalarInt::zero();
  let one = ScalarInt::one();

  assert_eq!(m2id, Int2_2::new(one, zero,
                               zero, one));
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
}

#[test]
fn mat2_2_dot_vector() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  let v2i1 = Int2::from((5, 6));

  assert_eq!(m2i1.dot_vector(&v2i1), Int2::from((23, 34)));
}

#[test]
fn mat2_2_det() {
  let col0 = Int2::from((1, 2));
  let col1 = Int2::from((3, 4));
  let m2i1 = Int2_2::from((col0, col1));

  assert_eq!(m2i1.det(), ScalarInt::new(-2));
}
