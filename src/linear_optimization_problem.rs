use std::f32::INFINITY;

pub type Value = f32;
pub type Coefficients = Vec<Value>;

pub struct UpperBoundConstraint {
    coefficients: Coefficients,
    bound: Value
}

pub fn solve_standard_problem(
    objective_fn_coeffs: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let problem = SimplexProblem::new(objective_fn_coeffs,functional_constraints);
        if functional_constraints.len() == 0 {return problem.point}
        let mut solution = solve_simplex_problem(problem);
        solution.truncate(objective_fn_coeffs.len());
        solution
}

fn solve_simplex_problem(mut problem: SimplexProblem) -> Vec<Value> {
    if is_optimal(&problem) {
        return problem.point;
    }
    let Some(pivot_variable) = pivot_variable(&problem) else {
        return problem.point;
    };
    set_ratios(&mut problem,pivot_variable);
    let Some(pivot_row_idx) = pivot_row_idx(&problem) else {
        return problem.point;
    };
    set_basic_variable(&mut problem,pivot_row_idx,pivot_variable);
    normalize_equation(&mut problem,pivot_row_idx,pivot_variable);
    reduce_equations(&mut problem,pivot_row_idx,pivot_variable);
    set_new_point(&mut problem);
    solve_simplex_problem(problem)
}

fn is_optimal(problem: &SimplexProblem) -> bool {
    problem.objective_equation.coefficients.iter().all(|v| *v >= 0_f32)
}

fn pivot_variable(problem: &SimplexProblem) -> Option<Variable> {
    problem.objective_equation.coefficients.iter().enumerate()
        .min_by(|(_, v1),(_, v2)| v1.total_cmp(v2))
        .unzip().0
}

fn set_ratios(problem: &mut SimplexProblem, pivot_column: Variable) {
    for row in &mut problem.rows {
        row.ratio = row.equation.constraint / row.equation.coefficients[pivot_column];
    }
}

fn pivot_row_idx(problem: &SimplexProblem) -> Option<usize> {
    problem.rows.iter().enumerate()
        .filter(|(_, row)| row.ratio > 0_f32 && row.ratio != INFINITY)
        .min_by(|(_, r1),(_, r2)| r1.ratio.total_cmp(&r2.ratio))
        .unzip().0
}

fn set_basic_variable(problem: &mut SimplexProblem, var_idx: usize, new_var: usize) {
    problem.rows[var_idx].basic_variable = new_var;
}

fn normalize_equation(problem: &mut SimplexProblem, equation_idx: usize, variable: Variable) {
    let coeffs = &mut problem.rows[equation_idx].equation.coefficients;
    let coeff = coeffs[variable];
    let var_count = coeffs.len();
    for var in  0..var_count {
        if var == variable {
            coeffs[var] = 1_f32;
        } else {
            coeffs[var] /= coeff;
        }
    }
    problem.rows[equation_idx].equation.constraint /= coeff;
}

fn reduce_equations(problem: &mut SimplexProblem, pivot_row_idx: usize, variable: Variable) {
    let (pivot_row, other_rows) = iter_around_mut(&mut problem.rows, pivot_row_idx);
    for row in other_rows {
        reduce_equation(&mut row.equation,&pivot_row.equation,variable);
    }
    reduce_equation(&mut problem.objective_equation,&pivot_row.equation,variable);
}

fn set_new_point(problem: &mut SimplexProblem) {
    problem.point.fill(0_f32);
    for row in problem.rows.iter() {
        problem.point[row.basic_variable] = row.equation.constraint;
    }
}

fn iter_around_mut<T>(slice: &mut [T], index: usize) -> (&mut T, impl Iterator<Item = &mut T>) {
    let (before, rest) = slice.split_at_mut(index);
    let (item, after) = rest.split_first_mut().unwrap();
    (item, before.iter_mut().chain(after.iter_mut()))
}

fn reduce_equation(equation: &mut Equation, pivot_equation: &Equation, variable: Variable) {
    let factor = equation.coefficients[variable];
    for (k, coeff) in equation.coefficients.iter_mut().enumerate() {
        *coeff -= factor * pivot_equation.coefficients[k];
    }
    equation.constraint -= factor * pivot_equation.constraint;
}

type Variable = usize;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Equation {
    coefficients: Coefficients,
    constraint: Value
}

#[derive(PartialEq)]
#[derive(Debug)]
struct SimplexRow {
    basic_variable: Variable,
    equation: Equation,
    ratio: Value
}

#[derive(Debug)]
struct SimplexProblem {
    objective_equation: Equation,
    rows: Vec<SimplexRow>,
    point: Vec<Value>,
}

impl SimplexProblem {
    pub fn new(objective_fn_coeffs: &Coefficients, functional_constraints: &Vec<UpperBoundConstraint>) -> Self {
        Self {
            objective_equation: initial_objective_equation(objective_fn_coeffs,functional_constraints.len()),
            rows: initial_rows(&functional_constraints,objective_fn_coeffs.len()),
            point: initial_point(objective_fn_coeffs,functional_constraints),
        }
    }
}

fn initial_objective_equation(objective_fn_coeffs: &Coefficients, nonbasic_var_count: usize) -> Equation {
    let mut coefficients = objective_fn_coeffs.clone();
    for coeff in &mut coefficients {
        *coeff = -*coeff;
    }
    coefficients.append(&mut vec![0_f32;nonbasic_var_count]);
    Equation{
        coefficients: coefficients, 
        constraint: 0_f32
    }
}

fn initial_rows(functional_constraints: &Vec<UpperBoundConstraint>, nonbasic_var_count: usize) -> Vec<SimplexRow> {
    let mut rows = vec![];
    for (var, constraint) in functional_constraints.iter().enumerate() {
        let row = SimplexRow {
            basic_variable: nonbasic_var_count+var,
            equation: equality_constraint(constraint,var,functional_constraints.len()),
            ratio: 0_f32
        };
        rows.push(row);
    }
    rows
}

fn equality_constraint(
    constraint: &UpperBoundConstraint, 
    target_var: Variable,
    basic_var_count: usize) -> Equation {
        let coeffs = &constraint.coefficients;
        Equation{
            coefficients: with_slack_variable(coeffs, target_var, basic_var_count),
            constraint: constraint.bound
        }
}

fn with_slack_variable(
    coefficients: &Vec<Value>,
    target_var: Variable, 
    basic_var_count: usize) -> Vec<Value> {
        let mut coeffs = coefficients.clone();
        for var in 0..basic_var_count {
            coeffs.push(if var == target_var {1_f32} else {0_f32});
        }
        coeffs
}

fn initial_point(objective_fn_coeffs: &Coefficients, constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
    let mut point = vec![0_f32; objective_fn_coeffs.len()];
    for constraint in constraints {
        point.push(constraint.bound);
    }
    point
}

#[cfg(test)]
mod test;