#[cfg(test)]
mod test;

pub mod mobjectivevalue;

use fraction::Fraction;

use super::{Equation, Problem, ProblemObserver};
use crate::simplex::objectivevalue::ObjectiveValue;
use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::value::Value;
use crate::simplex::{value, Coefficients, Variable};

pub fn solve<O: ObjectiveValue>(
    mut problem: Problem<O>,
    observer: &mut impl ProblemObserver<O>,
) -> Coefficients {
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

fn is_optimal<O: ObjectiveValue>(problem: &Problem<O>) -> bool {
    problem
        .objective_equation
        .coefficients
        .iter()
        .all(|v| v.is_optimal())
}

fn pivot_variable<O: ObjectiveValue>(problem: &Problem<O>) -> Option<Variable> {
    problem
        .objective_equation
        .coefficients
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unzip()
        .0
}

fn set_ratios<O: ObjectiveValue>(problem: &mut Problem<O>, pivot_column: Variable) {
    for row in &mut problem.rows {
        row.ratio = row.equation.constraint.clone()
            / row.equation.coefficients[pivot_column.clone()].clone();
    }
}

fn pivot_row_idx<O: ObjectiveValue>(problem: &Problem<O>) -> Option<usize> {
    problem
        .rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            row.ratio > value::zero()
                && row.ratio != Value::from(Fraction::Infinity(fraction::Sign::Plus))
        })
        .min_by(|(_, r1), (_, r2)| r1.ratio.cmp(&r2.ratio))
        .unzip()
        .0
}

fn set_basic_variable<O: ObjectiveValue>(problem: &mut Problem<O>, var_idx: usize, new_var: usize) {
    problem.rows[var_idx].basic_variable = new_var;
}

fn normalize_equation<O: ObjectiveValue>(
    problem: &mut Problem<O>,
    equation_idx: usize,
    variable: Variable,
) {
    let coeffs = &mut problem.rows[equation_idx].equation.coefficients;
    let coeff = coeffs[variable].clone();
    let var_count = coeffs.len();
    for var in 0..var_count {
        if var == variable {
            coeffs[var] = value::one();
        } else {
            coeffs[var] /= coeff.clone();
        }
    }
    problem.rows[equation_idx].equation.constraint /= coeff;
}

fn reduce_equations<O: ObjectiveValue>(
    problem: &mut Problem<O>,
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

fn reduce_row<R: RowValue>(row: &mut Row<R>, pivot_equation: &Equation, variable: Variable) {
    let factor = row.coefficients[variable].clone();
    for (k, coeff) in row.coefficients.iter_mut().enumerate() {
        *coeff = coeff.clone() + -(factor.clone() * pivot_equation.coefficients[k].clone());
    }
    row.constraint = row.constraint.clone() + -(factor * pivot_equation.constraint.clone());
}

fn set_new_point<O: ObjectiveValue>(problem: &mut Problem<O>) {
    problem.point.fill(value::zero());
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint.clone();
    }
}
