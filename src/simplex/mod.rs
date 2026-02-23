#[cfg(test)]
mod test;

pub mod core;
mod tabular;

use crate::simplex::core::*;

pub fn solve(
    objective_fn_coeffs: &Coefficients, 
    functional_constraints: &Vec<UpperBoundConstraint>) -> Vec<Value> {
        let problem = tabular::TabularSimplex::new(objective_fn_coeffs,functional_constraints);
        if functional_constraints.len() == 0 {return problem.point}
        let mut solution = tabular::solve(problem);
        solution.truncate(objective_fn_coeffs.len());
        solution
}