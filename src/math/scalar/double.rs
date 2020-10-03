use crate::math::scalar::ops::{ScalarNaturalOps, ScalarRealOps};
use crate::math::scalar::scalar::Scalar;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Double(pub f64);

impl Scalar for Double {
  type Primitive = f64;
  
  fn new(value: Self::Primitive) -> Self {
    Double(value)
  }
  
  fn to_primitive(&self) -> Self::Primitive {
    self.0
  }
}

impl ScalarNaturalOps for Double {
  type Natural = Double;
  
  fn negation(&self) -> Self::Natural {
    Double(-self.0)
  }
  
  fn add(&self, rhs: &Self::Natural) -> Self::Natural {
    Double(self.0 + rhs.0)
  }
  
  fn mul(&self, rhs: &Self::Natural) -> Self::Natural {
    Double(self.0 * rhs.0)
  }
  
  fn pow(&self, rhs: &Self::Natural) -> Self::Natural {
    Double(self.0.powf(rhs.0))
  }
}

impl ScalarRealOps for Double {
  type Real = Double;
  
  fn inverse(&self) -> Self {
    Double(1.0 / self.0)
  }
  
  fn mul(&self, rhs: &Self::Real) -> Self::Real {
    Double(self.0 * rhs.0)
  }
  
  fn sqrt(&self) -> Self {
    Double(self.0.sqrt())
  }
}

impl Add for Double {
  type Output = Double;
  
  fn add(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::add(&self, &rhs)
  }
}

impl Sub for Double {
  type Output = Double;
  
  fn sub(self, rhs: Self) -> Self::Output {
    ScalarNaturalOps::sub(&self, &rhs)
  }
}

impl Mul for Double {
  type Output = Double;
  
  fn mul(self, rhs: Self) -> Self::Output {
    ScalarRealOps::mul(&self, &rhs)
  }
}

impl Div for Double {
  type Output = Double;
  
  fn div(self, rhs: Self) -> Self::Output {
    ScalarRealOps::div(&self, &rhs)
  }
}