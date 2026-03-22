mod is_optimal;

mod pivot_variable;

mod set_ratios;

mod pivot_row_idx;

mod normalize_equation;

mod reduce_equations;

use crate::simplex::rowvalue::Row;
use crate::simplex::tabular::primal::mobjectivevalue::test::mvalue_from_m;
use crate::simplex::tabular::primal::mobjectivevalue::MObjectiveValue;
use crate::simplex::tabular::primal::{self as sut};
use crate::simplex::tabular::{EmptyObserver, Equation, Problem, ProblemObserver, SimplexRow};
use crate::simplex::test::{equality_constraint, frac, upper_bound_constraint};
use crate::simplex::value;
use crate::simplex::value::Value;

pub type MProblem = Problem<Value, MObjectiveValue>;
pub type MObjectiveEquation = Row<MObjectiveValue>;

struct MockObserver {
    observations: Vec<MProblem>,
}

impl MockObserver {
    pub fn new() -> Self {
        Self {
            observations: Vec::new(),
        }
    }
}

impl ProblemObserver<Value, MObjectiveValue> for MockObserver {
    fn observe(&mut self, problem: MProblem) {
        self.observations.push(problem);
    }
}

#[test]
fn solves_one_variable_zero_constraint_problem() {
    let objective_function = vec![frac(1, 1)];
    let fn_constraints = vec![];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_two_variable_zero_constraint_problem() {
    let objective_function = vec![frac(1, 1), frac(2, 1)];
    let fn_constraints = vec![];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(0, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_one_variable_one_constraint_problem() {
    let objective_function = vec![frac(1, 1)];
    let functional_constraint = upper_bound_constraint(vec![frac(1, 1)], frac(1, 1));
    let fn_constraints = vec![functional_constraint];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(1, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_unconstrainted_variable() {
    let objective_function = vec![frac(1, 1), frac(1, 1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3, 1), frac(0, 1)], frac(6, 1));
    let fn_constraints = vec![functional_constraint];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(2, 1), frac(0, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_problem() {
    let objective_function = vec![frac(1, 1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3, 1)], frac(6, 1));
    let fn_constraints = vec![functional_constraint];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(2, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_two_constraint_problem() {
    let objective_function = vec![frac(1, 1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2, 1)], frac(6, 1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(3, 1)], frac(6, 1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(2, 1), frac(2, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_independent_variable_two_constraint_problem() {
    let objective_function = vec![frac(1, 1), frac(1, 1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2, 1), frac(0, 1)], frac(6, 1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(0, 1), frac(4, 1)], frac(8, 1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(3, 1), frac(2, 1), frac(0, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_one_constraint_symmetric_problem() {
    let objective_function = vec![frac(1, 1), frac(1, 1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2, 1), frac(4, 1)], frac(6, 1));
    let fn_constraints = vec![fn_constaint_0];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(3, 1), frac(0, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_two_constraint_symmetric_problem() {
    let objective_function = vec![frac(1, 1), frac(1, 1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(1, 1), frac(2, 1)], frac(3, 1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(2, 1), frac(1, 1)], frac(3, 1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = MProblem::new(&objective_function, &fn_constraints);
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(1, 1), frac(1, 1), frac(0, 1), frac(0, 1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_big_m_problem() {
    let problem = MProblem {
        objective_equation: MObjectiveEquation {
            coefficients: vec![
                mvalue_from_m(-frac(2, 1), -frac(2, 1)),
                mvalue_from_m(-frac(3, 1), -frac(1, 1)),
                MObjectiveValue::zero(),
                MObjectiveValue::zero(),
            ],
            constraint: mvalue_from_m(value::zero(), -frac(3, 1)),
        },
        rows: vec![
            SimplexRow {
                basic_variable: 2,
                equation: Equation {
                    coefficients: vec![frac(1, 1), frac(2, 1), frac(1, 1), value::zero()],
                    constraint: frac(4, 1),
                },
                ratio: value::zero(),
            },
            SimplexRow {
                basic_variable: 3,
                equation: Equation {
                    coefficients: vec![frac(1, 1), frac(1, 1), value::zero(), frac(1, 1)],
                    constraint: frac(3, 1),
                },
                ratio: value::zero(),
            },
        ],
        point: vec![value::zero(), value::zero(), frac(4, 1), frac(3, 1)],
    };
    let solns = sut::solve(problem, &mut EmptyObserver::new());
    let expected_solns = vec![frac(3, 1), value::zero(), frac(1, 1), value::zero()];
    assert_eq!(expected_solns, solns);
}

#[test]
fn creates_big_m_problem() {
    let objective_function = vec![frac(2, 1), frac(3, 1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(1, 1), frac(2, 1)], frac(4, 1));
    let fn_constaint_1 = equality_constraint(vec![frac(1, 1), frac(2, 1)], frac(3, 1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];

    let problem = MProblem::new(&objective_function, &fn_constraints);
    let expected_problem = MProblem {
        objective_equation: MObjectiveEquation {
            coefficients: vec![
                mvalue_from_m(-frac(2, 1), -frac(1, 1)),
                mvalue_from_m(-frac(3, 1), -frac(2, 1)),
                MObjectiveValue::zero(),
                MObjectiveValue::zero(),
            ],
            constraint: mvalue_from_m(value::zero(), -frac(3, 1)),
        },
        rows: vec![
            SimplexRow {
                basic_variable: 2,
                equation: Equation {
                    coefficients: vec![frac(1, 1), frac(2, 1), frac(1, 1), value::zero()],
                    constraint: frac(4, 1),
                },
                ratio: value::zero(),
            },
            SimplexRow {
                basic_variable: 3,
                equation: Equation {
                    coefficients: vec![frac(1, 1), frac(2, 1), value::zero(), frac(1, 1)],
                    constraint: frac(3, 1),
                },
                ratio: value::zero(),
            },
        ],
        point: vec![value::zero(), value::zero(), frac(4, 1), frac(3, 1)],
    };
    assert_eq!(
        expected_problem.objective_equation,
        problem.objective_equation
    );
    assert_eq!(expected_problem.point, problem.point);
    assert_eq!(expected_problem.rows, problem.rows);
    assert_eq!(expected_problem, problem);
}

#[test]
fn solve_observes_empty_problem() {
    let mut observer = MockObserver::new();
    let coeffs = vec![];
    let constraints = vec![];
    let problem = MProblem::new(&coeffs, &constraints);
    let expected_observations = vec![problem.clone()];
    let _solution = sut::solve(problem, &mut observer);
    assert_eq!(expected_observations, observer.observations);
}

#[test]
fn solve_observes_steps_of_problem() {
    let mut observer = MockObserver::new();
    let objective_coeffs = vec![frac(1, 1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3, 1)], frac(6, 1));
    let constraints = vec![functional_constraint];
    let problem = MProblem::new(&objective_coeffs, &constraints);
    let mut middle_problem = problem.clone();
    sut::set_ratios(&mut middle_problem, 0);
    let mut solved_problem = middle_problem.clone();
    sut::set_basic_variable(&mut solved_problem, 0, 0);
    sut::normalize_equation(&mut solved_problem, 0, 0);
    sut::reduce_equations(&mut solved_problem, 0, 0);
    sut::set_new_point(&mut solved_problem);

    let expected_observations = vec![middle_problem, solved_problem];
    let _solution = sut::solve(problem, &mut observer);
    assert_eq!(expected_observations, observer.observations);
}
