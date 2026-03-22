use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg};

use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::{Coefficients, Constraint};

pub trait ObjectiveValue<R: RowValue>:
    PartialOrd
    + Ord
    + PartialEq
    + Debug
    + Clone
    + Neg<Output = Self>
    + Add<Output = Self>
    + Mul<R, Output = Self>
    + Div<R, Output = Self>
    + Display
    + Sized
{
    fn is_optimal(&self) -> bool;
    fn initial_objective_equation(
        objective_fn_coeffs: &Coefficients<R>,
        functional_constraints: &Vec<Constraint<R>>,
    ) -> Row<Self>;
}
