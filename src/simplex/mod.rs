#[cfg(test)]
mod test;

pub mod core;
pub mod tabular;

use crate::simplex::core::*;

#[allow(dead_code)]
pub fn solve(
    objective_fn_coeffs: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let problem = tabular::Problem::new(objective_fn_coeffs,functional_constraints);
        let mut solution = tabular::solve(problem,&mut tabular::EmptyObserver::new());
        solution.truncate(objective_fn_coeffs.len());
        solution
}