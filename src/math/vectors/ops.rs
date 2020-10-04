use crate::math::scalar::ops::{ScalarNaturalOps, ScalarRealOps};

// pub trait VectorNaturalOps: IntoIterator {
//   type Natural: ScalarNaturalOps<Natural = Self::Natural>;
//   type Real: ScalarRealOps<Real = Self::Real>;
  
  // fn negate(&self) -> Self {
  //   self.into_iter()
  //       .map(|v| v.negation())
  //       .collect()
  // }
  
  // fn add(&self, rhs: &Self) -> Self {
  //   self.into_iter<Item = Self::Natural>()
  //       .zip(rhs)
  //       .map(|(l, r)| l + r))
  //       .collect()
  // }
  
  // fn sub(&self, rhs: &Self) -> Self {
  //   self.add(rhs.negate())
  // }
  
  // fn mul(&self, rhs: &Self) -> Self::Natural {
  //   self.into_iter()
  //       .map(|v| v.mul(rhs)))
  //       .collect()
  // }
  
  // fn dot(&self, rhs: &Self) -> Self::Natural {
  //   self.into_iter()
  //       .zip(rhs)
  //       .map(|(l, r)| l * r)
  //       .sum()
  // }
  
  // fn length_2(&self) -> Self::Natural {
  //   self.dot(self)
  // }
// }

// pub trait VectorRealOps: VectorNaturalOps<Real = Self::Real> where Self: ScalarRealOps {
  // fn inverse(&self) -> Self::Real {
  //   self.into_iter()
  //       .map(|v| v.inverse())
  //       .collect()
  // }
  
  // fn mul(&self, rhs: &Self::Real) -> Self::Real {
  //   self.into_iter()
  //       .map(|v| v.mul(rhs))
  //       .collect()
  // }
  
  // fn div(&self, rhs: &Self::Real) -> Self::Real {
  //   self.mul(rhs.inverse())
  // }
  
  // fn dot(&self, rhs: &Self::Real) -> Self::Real {
  //   self.into_iter()
  //       .zip(rhs)
  //       .map(|(l, r)| l * r)
  //       .sum()
  // }
  
  // fn length_2(&self) -> Self::Real {
  //   self.dot(self)
  // }
  
  // fn length(&self) -> Self::Real {
  //   self.length_2().sqrt()
  // }
  
  // fn normalized(&self) -> Self {
  //   self.div(self.length())
  // }
// }
