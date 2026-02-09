pub type Value = f32;
pub type Coefficients = Vec<Value>;

#[derive(Clone)]
pub struct UpperBoundConstraint {
    coefficients: Coefficients,
    bound: Value
}

impl UpperBoundConstraint {
    pub fn new(coefficients: &Coefficients, bound: Value) -> Self {
        Self {coefficients: coefficients.clone(), bound}
    }
}

pub fn solve_standard_problem(
    objective_function: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let mut problem = SimplexProblem::new(objective_function,functional_constraints);
        let variable_count = objective_function.len();
        let mut solns = vec![0_f32; variable_count];
        for var in 0..variable_count {
            solns[var] = next_maximal_val(var,&mut problem.equations);
        }
        solns
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