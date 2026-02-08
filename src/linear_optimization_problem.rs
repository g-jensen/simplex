pub type Value = f32;
pub type Coefficients = Vec<Value>;

#[derive(Clone)]
pub struct UpperBoundConstraint {
    coefficients: Coefficients,
    constraint: Value
}

impl UpperBoundConstraint {
    pub fn new(coefficients: &Coefficients, constraint: Value) -> Self {
        Self {coefficients: coefficients.clone(), constraint: constraint}
    }
}

pub fn solve_standard_problem(
    objective_function: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let mut equality_constraints = equality_constraints(functional_constraints);
        let variable_count = objective_function.len();
        let mut solns = vec![0_f32; variable_count];
        for var in 0..variable_count {
            solns[var] = next_maximal_val(var,&mut equality_constraints);
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

struct OptimizationProblem {
    objective_function: Coefficients,
    functional_constraints: Vec<Equation>,
    basic_variables: Vec<Variable>
}

impl OptimizationProblem {
    pub fn new(objective_function: &Coefficients, functional_constraints: &Vec<UpperBoundConstraint>) -> Self {
        Self {
            objective_function: objective_function.clone(),
            functional_constraints: equality_constraints(functional_constraints),
            basic_variables: Vec::new()
        }
    }
}

fn equality_constraints(functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Equation> {
    let count = functional_constraints.len();
    functional_constraints
        .iter().enumerate()
        .map(|(idx, constraint)| equality_constraint(idx, constraint, count))
        .collect()
}

fn equality_constraint(
    constraint_index: usize,
    bound_constraint: &UpperBoundConstraint, 
    constraint_count: usize) -> Equation {
        let coeffs = &bound_constraint.coefficients;
        let coeffs = with_slack_variable(constraint_index, coeffs, constraint_count);
        Equation{
            coefficients: coeffs,
            constraint: bound_constraint.constraint
        }
}

fn with_slack_variable(
    constraint_index: usize, 
    coefficients: &Vec<Value>,
    constraint_count: usize) -> Vec<Value> {
        let mut coeffs = coefficients.clone();
        for i in 0..constraint_count {
            coeffs.push(if i == constraint_index {1_f32} else {0_f32});
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