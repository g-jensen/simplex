#[cfg(test)]
pub mod test;

pub mod mobjectivevalue;

use super::{Problem, ProblemObserver};
use crate::simplex::objectivevalue::ObjectiveValue;
use crate::simplex::rowvalue::{Row, RowValue};
use crate::simplex::tabular;
use crate::simplex::tabular::primal::mobjectivevalue::MObjectiveValue;
use crate::simplex::value::Value;
use crate::simplex::{Coefficients, Variable};

pub type PrimalProblem = Problem<Value, MObjectiveValue>;
pub type MObjectiveEquation = Row<MObjectiveValue>;

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
        tabular::normalize_equation(&mut problem, pivot_row_idx, pivot_variable);
        tabular::reduce_equations(&mut problem, pivot_row_idx, pivot_variable);
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

fn set_new_point<R: RowValue, O: ObjectiveValue<R>>(problem: &mut Problem<R, O>) {
    problem.point.fill(R::zero());
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint.clone();
    }
}
