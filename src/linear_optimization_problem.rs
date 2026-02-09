use std::ops::Neg;

pub type Value = f32;
pub type Coefficients = Vec<Value>;

pub struct UpperBoundConstraint {
    coefficients: Coefficients,
    bound: Value
}

pub fn solve_standard_problem(
    objective_function: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let problem = SimplexProblem::new(objective_function,functional_constraints);
        let mut solution = solve_simplex_problem(problem);
        solution.truncate(objective_function.len());
        solution
        // let variable_count = objective_function.len();
        // let mut solns = vec![0_f32; variable_count];
        // for var in 0..variable_count {
        //     solns[var] = next_maximal_val(var,&mut problem.equations);
        // }
        // solns
}

fn solve_simplex_problem(problem: SimplexProblem) -> Vec<Value> {
    if is_optimal(&problem) {
        return problem.point;
    }
    let Some(pivot_variable) = pivot_variable(&problem) else {
        return problem.point;
    };
    let Some(pivot_equation) = pivot_equation(&problem,pivot_variable) else {
        return problem.point;
    };
    normalize_equation(&mut problem,pivot_equation,pivot_variable);
    reduce_equations(&mut problem,pivot_equation,pivot_variable);
}

fn reduce_equations(problem: &mut SimplexProblem, equation: usize, variable: Variable) {
    
}

fn normalize_equation(problem: &mut SimplexProblem, equation: usize, variable: Variable) {
    let ref mut coeffs = problem.equations[equation].coefficients;
    let coeff = coeffs[variable];
    let var_count = coeffs.len();
    for var in  0..var_count {
        coeffs[var] /= coeff;
    }
}

fn is_optimal(problem: &SimplexProblem) -> bool {
    problem.objective_function.iter().all(|v| !v.is_sign_positive())
}

fn pivot_variable(problem: &SimplexProblem) -> Option<Variable> {
    problem.objective_function.iter().enumerate()
        .max_by(|(_, v1),(_, v2)| v1.total_cmp(v2))
        .unzip().0
}

fn pivot_equation(problem: &SimplexProblem, pivot_column: Variable) -> Option<usize> {
    problem.equations.iter().enumerate()
        .map(|(i, row)| (i, row.constraint / row.coefficients[pivot_column]))
        .filter(|(_, ratio)| *ratio > 0_f32)
        .min_by(|(_, r1),(_, r2)| r1.total_cmp(r2))
        .unzip().0
}

type Variable = usize;

#[derive(PartialEq)]
#[derive(Debug)]
struct Equation {
    coefficients: Coefficients,
    constraint: Value
}

struct SimplexProblem {
    objective_function: Coefficients,
    equations: Vec<Equation>,
    point: Vec<Value>
}

impl SimplexProblem {
    pub fn new(objective_function: &Coefficients, functional_constraints: &Vec<UpperBoundConstraint>) -> Self {
        Self {
            objective_function: objective_function.clone(),
            equations: equality_constraints(functional_constraints),
            point: initial_point(functional_constraints)
        }
    }
}

fn initial_point(constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
    let mut point = vec![];
    if constraints.len() > 0 {
        point = vec![0_f32; constraints[0].coefficients.len()];
        for constraint in constraints {
            point.push(constraint.bound);
        }
    }
    point
}

fn equality_constraints(functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Equation> {
    let len = functional_constraints.len();
    functional_constraints
        .iter().enumerate()
        .map(|(var, constraint)| equality_constraint(constraint, var, len))
        .collect()
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

fn next_maximal_val(variable: Variable, constraints: &mut Vec<Equation>) -> Value {
    constraints.iter_mut()
        .filter(|constraint| constraint.coefficients[variable] != 0_f32)
        .map(|constraint| maximal_val(variable,constraint))
        .min_by(|a,b| a.total_cmp(b))
        .unwrap_or(0_f32)
}

fn maximal_val(variable: Variable, constraint: &mut Equation) -> Value {
    let upper_bound = constraint.constraint;
    let coeff = constraint.coefficients[variable];
    constraint.constraint -= upper_bound; // eventually remove i think
    upper_bound / coeff
}

#[cfg(test)]
mod test;