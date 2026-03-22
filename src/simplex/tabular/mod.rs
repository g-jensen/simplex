#[cfg(test)]
mod test;

pub mod dual;
pub mod primal;
pub mod write_observer;

use crate::simplex::objectivevalue::ObjectiveValue;
use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::{Coefficients, Constraint, Variable};

pub type Equation<R> = Row<R>;

#[derive(PartialEq, Debug, Clone)]
pub struct SimplexRow<R: RowValue> {
    pub basic_variable: Variable,
    pub equation: Equation<R>,
    pub ratio: R,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Problem<R: RowValue, O: ObjectiveValue<R>> {
    pub objective_equation: Row<O>,
    pub rows: Vec<SimplexRow<R>>,
    pub point: Coefficients<R>,
}

pub trait ProblemObserver<R: RowValue, O: ObjectiveValue<R>> {
    fn observe(&mut self, problem: Problem<R, O>);
}

pub struct EmptyObserver;

impl<R: RowValue, O: ObjectiveValue<R>> ProblemObserver<R, O> for EmptyObserver {
    fn observe(&mut self, _problem: Problem<R, O>) {}
}

impl EmptyObserver {
    pub fn new() -> EmptyObserver {
        EmptyObserver {}
    }
}

impl<R: RowValue, O: ObjectiveValue<R>> Problem<R, O> {
    pub fn new(
        objective_coeffs: &Coefficients<R>,
        functional_constraints: &Vec<Constraint<R>>,
    ) -> Self {
        Self {
            objective_equation: O::initial_objective_equation(
                objective_coeffs,
                functional_constraints,
            ),
            rows: initial_rows(functional_constraints, objective_coeffs.len()),
            point: initial_point(objective_coeffs, functional_constraints),
        }
    }
}

fn initial_rows<R: RowValue>(
    functional_constraints: &Vec<Constraint<R>>,
    nonbasic_var_count: usize,
) -> Vec<SimplexRow<R>> {
    let mut rows = vec![];
    for (var, constraint) in functional_constraints.iter().enumerate() {
        let row = SimplexRow {
            basic_variable: nonbasic_var_count + var,
            equation: equality_constraint(constraint, var, functional_constraints.len()),
            ratio: R::zero(),
        };
        rows.push(row);
    }
    rows
}

fn equality_constraint<R: RowValue>(
    constraint: &Constraint<R>,
    target_var: Variable,
    basic_var_count: usize,
) -> Equation<R> {
    let coeffs = &constraint.coefficients;
    Equation {
        coefficients: with_slack_variable(coeffs, target_var, basic_var_count),
        constraint: constraint.bound.clone(),
    }
}

fn with_slack_variable<R: RowValue>(
    coefficients: &Coefficients<R>,
    target_var: Variable,
    basic_var_count: usize,
) -> Coefficients<R> {
    let mut coeffs = coefficients.clone();
    for var in 0..basic_var_count {
        coeffs.push(if var == target_var {
            R::one()
        } else {
            R::zero()
        });
    }
    coeffs
}

fn initial_point<R: RowValue>(
    objective_fn_coeffs: &Coefficients<R>,
    constraints: &Vec<Constraint<R>>,
) -> Coefficients<R> {
    let mut point = vec![R::zero(); objective_fn_coeffs.len()];
    for constraint in constraints {
        point.push(constraint.bound.clone());
    }
    point
}
