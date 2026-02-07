pub type Value = f32;
pub type Coefficients = Vec<Value>;

#[derive(Clone)]
pub struct UpperBoundConstraint {
    coefficients: Coefficients,
    constraint: Value
}

impl UpperBoundConstraint {
    pub fn new(coefficients: Coefficients, constraint: Value) -> Self {
        Self {coefficients: coefficients, constraint: constraint}
    }
}

type Variable = usize;

pub fn solve_standard_problem(
    objective_function: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let mut functional_constraints = functional_constraints.clone();
        let variable_count = objective_function.len();
        let mut solns = vec![0_f32; variable_count];
        for var in 0..variable_count {
            solns[var] = next_maximal_val(var,&mut functional_constraints);
        }
        solns
}

fn next_maximal_val(variable: Variable, constraints: &mut Vec<UpperBoundConstraint>) -> Value {
    constraints.iter_mut()
        .filter(|constraint| constraint.coefficients[variable] != 0_f32)
        .map(|constraint| maximal_val(variable,constraint))
        .min_by(|a,b| a.total_cmp(b))
        .unwrap_or(0_f32)
}

fn maximal_val(variable: Variable, constraint: &mut UpperBoundConstraint) -> Value {
    let upper_bound = constraint.constraint;
    let coeff = constraint.coefficients[variable];
    constraint.constraint -= upper_bound; // eventually remove i think
    upper_bound / coeff
}

#[cfg(test)]
mod test;