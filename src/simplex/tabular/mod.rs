#[cfg(test)]
mod test;

pub mod dual;
pub mod primal;
pub mod write_observer;

use std::ops::{Add, Mul, Neg};

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

fn normalize_equation<R: RowValue, O: ObjectiveValue<R>>(
    problem: &mut Problem<R, O>,
    equation_idx: usize,
    variable: Variable,
) {
    let coeffs = &mut problem.rows[equation_idx].equation.coefficients;
    let coeff = coeffs[variable].clone();
    let var_count = coeffs.len();
    for var in 0..var_count {
        if var == variable {
            coeffs[var] = R::one();
        } else {
            coeffs[var] = coeffs[var].clone() / coeff.clone();
        }
    }
    let constraint = &mut problem.rows[equation_idx].equation.constraint;
    *constraint = constraint.clone() / coeff;
}

fn reduce_equations<R: RowValue, O: ObjectiveValue<R>>(
    problem: &mut Problem<R, O>,
    pivot_row_idx: usize,
    variable: Variable,
) {
    let (pivot_row, other_rows) = iter_around_mut(&mut problem.rows, pivot_row_idx);
    for row in other_rows {
        reduce_row(&mut row.equation, &pivot_row.equation, variable);
    }
    reduce_row(
        &mut problem.objective_equation,
        &pivot_row.equation,
        variable,
    );
}

fn iter_around_mut<T>(slice: &mut [T], index: usize) -> (&mut T, impl Iterator<Item = &mut T>) {
    let (before, rest) = slice.split_at_mut(index);
    let (item, after) = rest.split_first_mut().unwrap();
    (item, before.iter_mut().chain(after.iter_mut()))
}

fn reduce_row<T, R: RowValue>(row: &mut Row<T>, pivot_equation: &Equation<R>, variable: Variable)
where
    T: Clone + Add<Output = T> + Neg<Output = T> + Mul<R, Output = T>,
{
    let factor = row.coefficients[variable].clone();
    for (k, coeff) in row.coefficients.iter_mut().enumerate() {
        *coeff = coeff.clone() + -(factor.clone() * pivot_equation.coefficients[k].clone());
    }
    row.constraint = row.constraint.clone() + -(factor * pivot_equation.constraint.clone());
}