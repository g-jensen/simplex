use std::{fmt::Display, ops::{Add, Div, Mul, Neg}};

use crate::simplex::value::Value;

#[cfg(test)]
mod test;

pub mod value;
pub mod tabular;

pub type Coefficients = Vec<Value>;
pub type Variable = usize;

#[derive(Clone)]
pub enum Operator {
    LESSTHANEQUAL,
    EQUAL
}

#[derive(Clone)]
pub struct Constraint {
    pub operator: Operator,
    pub coefficients: Coefficients,
    pub bound: Value
}