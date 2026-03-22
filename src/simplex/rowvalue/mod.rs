use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg};

pub trait RowValue:
    PartialOrd
    + Ord
    + PartialEq
    + Debug
    + Clone
    + Neg<Output = Self>
    + Add<Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Display
    + Sized
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_finite(&self) -> bool;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Row<R> {
    pub coefficients: Vec<R>,
    pub constraint: R,
}
