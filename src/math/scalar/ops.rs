pub trait ScalarNaturalOps {
  type Natural: ScalarNaturalOps<Natural = Self::Natural>;
  
  fn negation(&self) -> Self::Natural;
  fn add(&self, rhs: &Self::Natural) -> Self::Natural;
  fn mul(&self, rhs: &Self::Natural) -> Self::Natural;
  fn pow(&self, rhs: &Self::Natural) -> Self::Natural;

  fn sub(&self, rhs: &Self::Natural) -> Self::Natural {
    self.add(&rhs.negation())
  }
}

pub trait ScalarRealOps: ScalarNaturalOps {
  type Real: ScalarRealOps<Real = Self::Real>;
  
  fn inverse(&self) -> Self::Real;
  fn mul(&self, rhs: &Self::Real) -> Self::Real;
  fn sqrt(&self) -> Self::Real;
  
  fn div(&self, rhs: &Self::Real) -> Self::Real {
    ScalarRealOps::mul(self, &rhs.inverse())
  }
}