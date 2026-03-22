use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg};

use crate::simplex::value::Value;

pub trait RowValue:
    PartialOrd
    + Ord
    + PartialEq
    + Debug
    + Clone
    + Neg<Output = Self>
    + Add<Output = Self>
    + Mul<Value, Output = Self>
    + Div<Value, Output = Self>
    + Display
    + Sized
{
}

impl RowValue for Value {}

#[derive(PartialEq, Debug, Clone)]
pub struct Row<R: RowValue> {
    pub coefficients: Vec<R>,
    pub constraint: R,
}
