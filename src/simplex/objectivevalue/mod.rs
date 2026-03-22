use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::{Coefficients, Constraint};

pub trait ObjectiveValue: RowValue {
    fn is_optimal(&self) -> bool;
    fn initial_objective_equation(
        objective_fn_coeffs: &Coefficients,
        functional_constraints: &Vec<Constraint>,
    ) -> Row<Self>;
}
