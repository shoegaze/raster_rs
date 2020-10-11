use std::ops::{Neg, Add, Sub, Mul, Div};

// Fixed-point scalars are closed under unary -, +, -, *
pub trait ScalarNaturalOps:
    Sized +
    Neg<Output = <Self as ScalarNaturalOps>::Output> +
    Add<Output = <Self as ScalarNaturalOps>::Output> +
    Sub<Output = <Self as ScalarNaturalOps>::Output> +
    Mul<Output = <Self as ScalarNaturalOps>::Output> +
    Div<Output = <Self as ScalarNaturalOps>::Output> {
  type Output: ScalarNaturalOps;
}

pub trait ScalarRealOps:
    Sized +
    ScalarNaturalOps<Output = <Self as ScalarRealOps>::Output> {
  type Output: ScalarRealOps;

  fn powf(&self, rhs: Self) -> <Self as ScalarRealOps>::Output;
  fn sqrt(&self) -> <Self as ScalarRealOps>::Output;
}
