use std::marker::PhantomData;
use crate::math::{
  matrices::{
    matrix2_2::{
      Matrix2_2,
      Matrix2_2Constructor
    },
  },
  vectors::vector2::Vector2,
  scalars::scalar::ScalarTrait
};

#[allow(non_camel_case_types)]
pub(crate) struct Transpose2_2<'a, M, V, S>
  where M: Matrix2_2<V, S> + Matrix2_2Constructor<V, S>,
        V: Vector2<S>,
        S: ScalarTrait {

  mat: &'a M,
  phantom_vector: PhantomData<V>,
  phantom_scalar: PhantomData<S>
}

#[allow(dead_code)]
impl<'a, M, V, S> Transpose2_2<'a, M, V, S>
  where M: Matrix2_2<V, S> + Matrix2_2Constructor<V, S>,
        V: Vector2<S>,
        S: ScalarTrait {

  pub(crate) fn new(mat: &'a M) -> Self {
    Self {
      mat,
      phantom_vector: PhantomData,
      phantom_scalar: PhantomData
    }
  }

  pub(crate) fn as_matrix(&self) -> M {
    M::new(
      self.a00(), self.a01(),
      self.a10(), self.a11()
    )
  }
}

impl<M, V, S> Matrix2_2<V, S> for Transpose2_2<'_, M, V, S>
  where M: Matrix2_2<V, S> + Matrix2_2Constructor<V, S>,
        V: Vector2<S>,
        S: ScalarTrait {

  fn a00(&self) -> S {
    self.mat.a00()
  }

  fn a01(&self) -> S {
    self.mat.a10()
  }

  fn a10(&self) -> S {
    self.mat.a01()
  }

  fn a11(&self) -> S {
    self.mat.a11()
  }
}

// impl<M, V, S> Matrix2_2Ops<V, S> for Transpose2_2<'_, M, V, S>
//   where M: Sized + Matrix2_2<V, S> + Matrix2_2Constructor<V, S>,
//         V: Vector2<S> + Vector2Ops<S>,
//         S: ScalarTrait +
//            Add<Output = S> + Sub<Output = S> +
//            Mul<Output = S> + Div<Output = S> {}
