use fraction::Fraction;

use crate::simplex::value::Value;
use crate::simplex::{Coefficients, Constraint, Operator};

pub fn frac(n: u64, d: u64) -> Value {
    Fraction::new(n, d)
}

pub fn upper_bound_constraint(
    coefficients: Coefficients<Value>,
    bound: Value,
) -> Constraint<Value> {
    Constraint {
        operator: Operator::LESSTHANEQUAL,
        coefficients,
        bound,
    }
}

pub fn equality_constraint(coefficients: Coefficients<Value>, bound: Value) -> Constraint<Value> {
    Constraint {
        operator: Operator::EQUAL,
        coefficients,
        bound,
    }
}
