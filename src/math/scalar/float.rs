use crate::math::scalar::ops::{ScalarNaturalOps, ScalarRealOps};
use crate::math::scalar::scalar::Scalar;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Float(pub f32);

impl Scalar for Float {
  type Primitive = f32;
  
  fn new(value: Self::Primitive) -> Self {
    Float(value)
  }
  
  fn to_primitive(&self) -> Self::Primitive {
    self.0
  }
}

impl ScalarNaturalOps for Float {
  type Natural = Float;

  fn negation(&self) -> Self::Natural {
    Float(-self.0)
  }

  fn add(&self, rhs: &Self::Natural) -> Self::Natural {
    Float(self.0 + rhs.0)
  }

  fn mul(&self, rhs: &Self::Natural) -> Self::Natural {
    Float(self.0 * rhs.0)
  }

  fn pow(&self, rhs: &Self::Natural) -> Self {
    Float(self.0.powf(rhs.0))
  }
}

impl ScalarRealOps for Float {
  type Real = Float;
  
  fn inverse(&self) -> Self::Natural {
    Float(1.0 / self.0)
  }
  
  fn mul(&self, rhs: &Self::Real) -> Self::Real {
    Float(self.0 * rhs.0)
  }
  
  fn sqrt(&self) -> Self::Natural {
    Float(self.0.sqrt())
  }
}

impl Add for Float {
  type Output = Float;
  
  fn add(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::add(&self, &rhs)
  }
}

impl Sub for Float {
  type Output = Float;
  
  fn sub(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::sub(&self, &rhs)
  }
}

impl Mul for Float {
  type Output = Float;
  
  fn mul(self, rhs: Self) -> Self::Output {
    ScalarRealOps::mul(&self, &rhs)
  }
}

impl Div for Float {
  type Output = Float;
  
  fn div(self, rhs: Self) -> Self::Output {
    ScalarRealOps::div(&self, &rhs)
  }
}