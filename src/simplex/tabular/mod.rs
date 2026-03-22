#[cfg(test)]
mod test;

pub mod dual;
pub mod primal;
pub mod write_observer;

use crate::simplex::objectivevalue::{ObjectiveEquation, ObjectiveValue};
use crate::simplex::value::Value;
use crate::simplex::{value, Coefficients, Constraint, Variable};

#[derive(PartialEq, Debug, Clone)]
pub struct Equation {
    pub coefficients: Coefficients,
    pub constraint: Value,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SimplexRow {
    pub basic_variable: Variable,
    pub equation: Equation,
    pub ratio: Value,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Problem<O: ObjectiveValue> {
    pub objective_equation: ObjectiveEquation<O>,
    pub rows: Vec<SimplexRow>,
    pub point: Coefficients,
}

pub trait ProblemObserver<O: ObjectiveValue> {
    fn observe(&mut self, problem: Problem<O>);
}

pub struct EmptyObserver;

impl<O: ObjectiveValue> ProblemObserver<O> for EmptyObserver {
    fn observe(&mut self, _problem: Problem<O>) {}
}

impl EmptyObserver {
    pub fn new() -> EmptyObserver {
        EmptyObserver {}
    }
}

impl<O: ObjectiveValue> Problem<O> {
    pub fn new(objective_coeffs: &Coefficients, functional_constraints: &Vec<Constraint>) -> Self {
        Self {
            objective_equation: O::initial_objective_equation(
                objective_coeffs,
                functional_constraints,
            ),
            rows: initial_rows(&functional_constraints, objective_coeffs.len()),
            point: initial_point(&objective_coeffs, functional_constraints),
        }
    }
}

fn initial_rows(
    functional_constraints: &Vec<Constraint>,
    nonbasic_var_count: usize,
) -> Vec<SimplexRow> {
    let mut rows = vec![];
    for (var, constraint) in functional_constraints.iter().enumerate() {
        let row = SimplexRow {
            basic_variable: nonbasic_var_count + var,
            equation: equality_constraint(constraint, var, functional_constraints.len()),
            ratio: value::zero(),
        };
        rows.push(row);
    }
    rows
}

fn equality_constraint(
    constraint: &Constraint,
    target_var: Variable,
    basic_var_count: usize,
) -> Equation {
    let coeffs = &constraint.coefficients;
    Equation {
        coefficients: with_slack_variable(coeffs, target_var, basic_var_count),
        constraint: constraint.bound.clone(),
    }
}

fn with_slack_variable(
    coefficients: &Coefficients,
    target_var: Variable,
    basic_var_count: usize,
) -> Coefficients {
    let mut coeffs = coefficients.clone();
    for var in 0..basic_var_count {
        coeffs.push(if var == target_var {
            value::one()
        } else {
            value::zero()
        });
    }
    coeffs
}

fn initial_point(
    objective_fn_coeffs: &Coefficients,
    constraints: &Vec<Constraint>,
) -> Coefficients {
    let mut point = vec![value::zero(); objective_fn_coeffs.len()];
    for constraint in constraints {
        point.push(constraint.bound.clone());
    }
    point
}
