use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg},
};

use crate::simplex::{value::Value, Coefficients, Constraint};

pub trait ObjectiveValue:
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
    fn is_optimal(&self) -> bool;
    fn initial_objective_equation(
        objective_fn_coeffs: &Coefficients,
        functional_constraints: &Vec<Constraint>,
    ) -> ObjectiveEquation<Self>;
}

pub type ObjectiveCoefficients<O> = Vec<O>;

#[derive(PartialEq, Debug, Clone)]
pub struct ObjectiveEquation<O: ObjectiveValue> {
    pub coefficients: ObjectiveCoefficients<O>,
    pub constraint: O,
}
