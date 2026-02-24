use fraction::{Fraction};

use crate::simplex::{self as sut};

pub fn upper_bound_constraint(
    coefficients: sut::Coefficients,
    bound: sut::Value) -> sut::UpperBoundConstraint {
    sut::UpperBoundConstraint {
        coefficients,
        bound,
    }
}
pub fn frac(n: u64, d: u64) -> Fraction {
    Fraction::new(n,d)
}

#[test]
fn solves_one_variable_zero_constraint_problem() {
    let objective_function = vec![frac(1,1)];
    let fn_constraints = vec![];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_two_variable_zero_constraint_problem() {
    let objective_function = vec![frac(1,1), frac(2,1)];
    let fn_constraints = vec![];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_one_variable_one_constraint_problem() {
    let objective_function = vec![frac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(1,1)], frac(1,1));
    let fn_constraints = vec![functional_constraint];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(1,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_unconstrainted_variable() {
    let objective_function = vec![frac(1,1), frac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3,1), frac(0,1)], frac(6,1));
    let fn_constraints = vec![functional_constraint];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(2,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_problem() {
    let objective_function = vec![frac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3,1)], frac(6,1));
    let fn_constraints = vec![functional_constraint];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(2,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_two_constraint_problem() {
    let objective_function = vec![frac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1)], frac(6,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(3,1)], frac(6,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(2,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_independent_variable_two_constraint_problem() {
    let objective_function = vec![frac(1,1), frac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1), frac(0,1)], frac(6,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(0,1), frac(4,1)], frac(8,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(3,1), frac(2,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_one_constraint_symmetric_problem() {
    let objective_function = vec![frac(1,1), frac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1), frac(4,1)], frac(6,1));
    let fn_constraints = vec![fn_constaint_0];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(3,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_two_constraint_symmetric_problem() {
    let objective_function = vec![frac(1,1), frac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(1,1), frac(2,1)], frac(3,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(2,1), frac(1,1)], frac(3,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let solns = sut::solve(&objective_function, &fn_constraints);
    let expected_solns = vec![frac(1,1), frac(1,1)];
    assert_eq!(expected_solns, solns);
}