pub mod write_observer;

#[cfg(test)]
mod test;

use std::{fmt::Display, ops::{Div, DivAssign, Mul, Neg, SubAssign}};

use fraction::{ConstOne, ConstZero, Fraction};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Value {
    pub finite: Fraction,
    pub m: Fraction,
}

impl Value {
    pub fn from(f: Fraction) -> Self {
        Value { finite: f, m: Fraction::ZERO }
    }
}

impl Neg for Value {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Value {
            finite: self.finite.neg(),
            m: self.m,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.finite.fmt(f)
        // if self.m.is_zero() {
        //     self.finite.fmt(f)
        // } else {
        //     self.finite.fmt(f);
        //     if self.m.is_negative() {
        //         write!(f," - ");
        //     } else {
        //         write!(f," + ");
        //     }
        //     self.m.abs().fmt(f);
        //     write!(f,"M")
        // }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Value {
            finite: self.finite.mul(rhs.finite),
            m: self.m
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Value {
            finite: self.finite.div(rhs.finite),
            m: self.m
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        self.finite.div_assign(rhs.finite);
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        self.finite.sub_assign(rhs.finite);
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.finite.partial_cmp(&other.finite)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.finite.cmp(&other.finite)
    }
}

fn zero() -> Value {
    Value { finite: Fraction::ZERO, m: Fraction::ZERO }
}

fn one() -> Value {
    Value { finite: Fraction::ONE, m: Fraction::ZERO }
}

pub type Coefficients = Vec<Value>;
pub type Variable = usize;

pub struct UpperBoundConstraint {
    pub coefficients: Coefficients,
    pub bound: Value
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
    pub objective_equation: Equation,
    pub rows: Vec<SimplexRow>,
    pub point: Vec<Value>,
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
        objective_fn_coeffs: &Coefficients,
        functional_constraints: &Vec<UpperBoundConstraint>,
    ) -> Self {
        Self {
            objective_equation: initial_objective_equation(
                objective_fn_coeffs,
                functional_constraints.len(),
            ),
            rows: initial_rows(&functional_constraints, objective_fn_coeffs.len()),
            point: initial_point(objective_fn_coeffs, functional_constraints),
        }
    }
}

fn initial_objective_equation(
    objective_fn_coeffs: &Coefficients,
    nonbasic_var_count: usize,
) -> Equation {
    let mut coefficients = objective_fn_coeffs.clone();
    for coeff in &mut coefficients {
        *coeff = -coeff.clone();
    }
    coefficients.append(&mut vec![zero(); nonbasic_var_count]);
    Equation {
        coefficients: coefficients,
        constraint: zero(),
    }
}

fn initial_rows(
    functional_constraints: &Vec<UpperBoundConstraint>,
    nonbasic_var_count: usize,
) -> Vec<SimplexRow> {
    let mut rows = vec![];
    for (var, constraint) in functional_constraints.iter().enumerate() {
        let row = SimplexRow {
            basic_variable: nonbasic_var_count + var,
            equation: equality_constraint(constraint, var, functional_constraints.len()),
            ratio: zero(),
        };
        rows.push(row);
    }
    rows
}

fn equality_constraint(
    constraint: &UpperBoundConstraint,
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
    coefficients: &Vec<Value>,
    target_var: Variable,
    basic_var_count: usize,
) -> Vec<Value> {
    let mut coeffs = coefficients.clone();
    for var in 0..basic_var_count {
        coeffs.push(if var == target_var {
            one()
        } else {
            zero()
        });
    }
    coeffs
}

fn initial_point(
    objective_fn_coeffs: &Coefficients,
    constraints: &Vec<UpperBoundConstraint>,
) -> Vec<Value> {
    let mut point = vec![zero(); objective_fn_coeffs.len()];
    for constraint in constraints {
        point.push(constraint.bound.clone());
    }
    point
}

pub fn solve(mut problem: Problem, observer: &mut impl ProblemObserver) -> Vec<Value> {
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
        .all(|v| *v >= zero())
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
            row.ratio > zero() && row.ratio != Value::from(Fraction::Infinity(fraction::Sign::Plus))
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
            coeffs[var] = one();
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
    reduce_equation(
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
        *coeff -= factor.clone() * pivot_equation.coefficients[k].clone();
    }
    equation.constraint -= factor * pivot_equation.constraint.clone();
}

fn set_new_point(problem: &mut Problem) {
    problem.point.fill(zero());
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint.clone();
    }
}
