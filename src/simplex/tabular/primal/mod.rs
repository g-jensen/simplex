#[cfg(test)]
mod test;

pub mod mobjectivevalue;

use std::ops::{Add, Mul, Neg};

use super::{Equation, Problem, ProblemObserver};
use crate::simplex::objectivevalue::ObjectiveValue;
use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::{Coefficients, Variable};

pub fn solve<R: RowValue, O: ObjectiveValue<R>>(
    mut problem: Problem<R, O>,
    observer: &mut impl ProblemObserver<R, O>,
) -> Coefficients<R> {
    while !is_optimal(&problem) {
        let Some(pivot_variable) = pivot_variable(&problem) else {
            return problem.point;
        };
        set_ratios(&mut problem, pivot_variable);
        let Some(pivot_row_idx) = pivot_row_idx(&problem) else {
            return problem.point;
        };
        observer.observe(problem.clone());
        set_basic_variable(&mut problem, pivot_row_idx, pivot_variable);
        normalize_equation(&mut problem, pivot_row_idx, pivot_variable);
        reduce_equations(&mut problem, pivot_row_idx, pivot_variable);
        set_new_point(&mut problem);
    }
    observer.observe(problem.clone());
    return problem.point;
}

fn is_optimal<R: RowValue, O: ObjectiveValue<R>>(problem: &Problem<R, O>) -> bool {
    problem
        .objective_equation
        .coefficients
        .iter()
        .all(|v| v.is_optimal())
}

fn pivot_variable<R: RowValue, O: ObjectiveValue<R>>(problem: &Problem<R, O>) -> Option<Variable> {
    problem
        .objective_equation
        .coefficients
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unzip()
        .0
}

fn set_ratios<R: RowValue, O: ObjectiveValue<R>>(
    problem: &mut Problem<R, O>,
    pivot_column: Variable,
) {
    for row in &mut problem.rows {
        row.ratio =
            row.equation.constraint.clone() / row.equation.coefficients[pivot_column].clone();
    }
}

fn pivot_row_idx<R: RowValue, O: ObjectiveValue<R>>(problem: &Problem<R, O>) -> Option<usize> {
    problem
        .rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.ratio > R::zero() && row.ratio.is_finite())
        .min_by(|(_, r1), (_, r2)| r1.ratio.cmp(&r2.ratio))
        .unzip()
        .0
}

fn set_basic_variable<R: RowValue, O: ObjectiveValue<R>>(
    problem: &mut Problem<R, O>,
    var_idx: usize,
    new_var: usize,
) {
    problem.rows[var_idx].basic_variable = new_var;
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

fn set_new_point<R: RowValue, O: ObjectiveValue<R>>(problem: &mut Problem<R, O>) {
    problem.point.fill(R::zero());
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint.clone();
    }
}
