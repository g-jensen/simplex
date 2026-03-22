pub mod write_observer;

#[cfg(test)]
mod test;

use fraction::{Fraction};

use crate::simplex::{value::Value, mvalue::ZValue};
use crate::simplex::{Coefficients, Constraint, Operator, Variable, value};

pub type ObjectiveCoefficients = Vec<ZValue>;

#[derive(PartialEq, Debug, Clone)]
pub struct ObjectiveEquation {
    pub coefficients: ObjectiveCoefficients,
    pub constraint: ZValue,
}

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
pub struct Problem {
    pub objective_equation: ObjectiveEquation,
    pub rows: Vec<SimplexRow>,
    pub point: Coefficients,
}

pub trait ProblemObserver {
    fn observe(&mut self, problem: Problem);
}

pub struct EmptyObserver;

impl ProblemObserver for EmptyObserver {
    fn observe(&mut self, _problem: Problem) {}
}

impl EmptyObserver {
    pub fn new() -> EmptyObserver {
        EmptyObserver {}
    }
}

impl Problem {
    pub fn new(
        objective_coeffs: &Coefficients,
        functional_constraints: &Vec<Constraint>,
    ) -> Self {
        Self {
            objective_equation: initial_objective_equation(
                objective_coeffs,
                functional_constraints
            ),
            rows: initial_rows(&functional_constraints, objective_coeffs.len()),
            point: initial_point(&objective_coeffs, functional_constraints),
        }
    }
}

fn find_equality_constraint(functional_constraints: &Vec<Constraint>) -> Option<Constraint> {
    for constraint in functional_constraints {
        match constraint.operator {
            Operator::EQUAL => {return Some(constraint.clone())}
            _ => {}
        }
    }
    return None
}

fn initial_objective_coeffs(coeffs: &Coefficients, equality_constraint_opt: &Option<Constraint>) -> ObjectiveCoefficients {
    let mut obj_coeffs = vec![ZValue::zero(); coeffs.len()];
    for i in 0..coeffs.len() {
        obj_coeffs[i] = -ZValue::from(coeffs[i].clone());
    }
    match equality_constraint_opt {
        Some(equality_constraint) => {
            for i in 0..equality_constraint.coefficients.len() {
                obj_coeffs[i] = obj_coeffs[i].clone() + -ZValue::from_m(value::zero(),equality_constraint.coefficients[i])
            }
        }
        None => {}
    }
    obj_coeffs
}

fn initial_objective_equation(
    objective_fn_coeffs: &Coefficients,
    functional_constraints: &Vec<Constraint>,
) -> ObjectiveEquation {
    let nonbasic_var_count = functional_constraints.len();
    let equality_constraint_opt = find_equality_constraint(functional_constraints);
    let mut coefficients = initial_objective_coeffs(objective_fn_coeffs,&equality_constraint_opt);
    coefficients.append(&mut vec![ZValue::zero(); nonbasic_var_count]);
    ObjectiveEquation {
        coefficients: coefficients,
        constraint: match equality_constraint_opt {
            Some(equality_constraint) => -ZValue::from_m(value::zero(),equality_constraint.bound),
            None => ZValue::zero(),
        },
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

// reconcile with test::equality_constraint. maybe merge Equation into Condition?
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

pub fn solve(mut problem: Problem, observer: &mut impl ProblemObserver) -> Coefficients {
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

fn is_optimal(problem: &Problem) -> bool {
    problem
        .objective_equation
        .coefficients
        .iter()
        .all(|v| *v >= ZValue::zero())
}

fn pivot_variable(problem: &Problem) -> Option<Variable> {
    problem
        .objective_equation
        .coefficients
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unzip()
        .0
}

fn set_ratios(problem: &mut Problem, pivot_column: Variable) {
    for row in &mut problem.rows {
        row.ratio = row.equation.constraint.clone() / row.equation.coefficients[pivot_column.clone()].clone();
    }
}

fn pivot_row_idx(problem: &Problem) -> Option<usize> {
    problem
        .rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            row.ratio > value::zero() && row.ratio != Value::from(Fraction::Infinity(fraction::Sign::Plus))
        })
        .min_by(|(_, r1), (_, r2)| r1.ratio.cmp(&r2.ratio))
        .unzip()
        .0
}

fn set_basic_variable(problem: &mut Problem, var_idx: usize, new_var: usize) {
    problem.rows[var_idx].basic_variable = new_var;
}

fn normalize_equation(problem: &mut Problem, equation_idx: usize, variable: Variable) {
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

fn reduce_equations(problem: &mut Problem, pivot_row_idx: usize, variable: Variable) {
    let (pivot_row, other_rows) = iter_around_mut(&mut problem.rows, pivot_row_idx);
    for row in other_rows {
        reduce_equation(&mut row.equation, &pivot_row.equation, variable);
    }
    reduce_objective_equation(
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

fn reduce_equation(equation: &mut Equation, pivot_equation: &Equation, variable: Variable) {
    let factor = equation.coefficients[variable].clone();
    for (k, coeff) in equation.coefficients.iter_mut().enumerate() {
        *coeff = coeff.clone() + -(factor.clone() * pivot_equation.coefficients[k].clone());
    }
    equation.constraint = equation.constraint.clone() + -(factor * pivot_equation.constraint.clone());
}

fn reduce_objective_equation(equation: &mut ObjectiveEquation, pivot_equation: &Equation, variable: Variable) {
    let factor = equation.coefficients[variable].clone();
    for (k, coeff) in equation.coefficients.iter_mut().enumerate() {
        *coeff = coeff.clone() + -(factor.clone() * pivot_equation.coefficients[k].clone());
    }
    equation.constraint = equation.constraint.clone() + -(factor * pivot_equation.constraint.clone());
}

fn set_new_point(problem: &mut Problem) {
    problem.point.fill(value::zero());
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint.clone();
    }
}
