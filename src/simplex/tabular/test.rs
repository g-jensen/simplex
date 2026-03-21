mod initial_point;

mod is_optimal;

mod pivot_variable;

mod set_ratios;

mod pivot_row_idx;

mod normalize_equation;

mod reduce_equations;

use crate::simplex::test::{zfrac, zfrac_m};
use crate::simplex::value::{self, Value, ZValue};
use crate::simplex::tabular::{self as sut, EmptyObserver};
use crate::simplex::{
    test::frac,
};

struct MockObserver {
    observations: Vec<sut::Problem>,
}

impl MockObserver {
    pub fn new() -> Self {
        Self {
            observations: Vec::new(),
        }
    }
}

impl sut::ProblemObserver for MockObserver {
    fn observe(&mut self, problem: sut::Problem) {
        self.observations.push(problem);
    }
}

pub fn upper_bound_constraint(
    coefficients: sut::Coefficients,
    bound: Value) -> sut::UpperBoundConstraint {
    sut::UpperBoundConstraint {
        coefficients,
        bound,
    }
}

#[test]
fn solves_one_variable_zero_constraint_problem() {
    let objective_function = vec![zfrac(1,1)];
    let fn_constraints = vec![];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_two_variable_zero_constraint_problem() {
    let objective_function = vec![zfrac(1,1), zfrac(2,1)];
    let fn_constraints = vec![];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_one_variable_one_constraint_problem() {
    let objective_function = vec![zfrac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(1,1)], frac(1,1));
    let fn_constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(1,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_unconstrainted_variable() {
    let objective_function = vec![zfrac(1,1), zfrac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3,1), frac(0,1)], frac(6,1));
    let fn_constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(2,1), frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_problem() {
    let objective_function = vec![zfrac(1,1)];
    let functional_constraint = upper_bound_constraint(vec![frac(3,1)], frac(6,1));
    let fn_constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(2,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_one_variable_two_constraint_problem() {
    let objective_function = vec![zfrac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1)], frac(6,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(3,1)], frac(6,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(2,1), frac(2,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_independent_variable_two_constraint_problem() {
    let objective_function = vec![zfrac(1,1), zfrac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1), frac(0,1)], frac(6,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(0,1), frac(4,1)], frac(8,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(3,1), frac(2,1), frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_one_constraint_symmetric_problem() {
    let objective_function = vec![zfrac(1,1), zfrac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(2,1), frac(4,1)], frac(6,1));
    let fn_constraints = vec![fn_constaint_0];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(3,1), frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn maximizes_two_dependent_variable_two_constraint_symmetric_problem() {
    let objective_function = vec![zfrac(1,1), zfrac(1,1)];
    let fn_constaint_0 = upper_bound_constraint(vec![frac(1,1), frac(2,1)], frac(3,1));
    let fn_constaint_1 = upper_bound_constraint(vec![frac(2,1), frac(1,1)], frac(3,1));
    let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
    let problem = sut::Problem::new(&objective_function,&fn_constraints);
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(1,1), frac(1,1), frac(0,1), frac(0,1)];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solves_big_m_problem() {
    let problem = sut::Problem{
        objective_equation: sut::ObjectiveEquation{
            coefficients: vec![
                zfrac_m(-frac(2,1), -frac(2,1)),
                zfrac_m(-frac(3,1), -frac(1,1)),
                ZValue::zero(),
                ZValue::zero()
            ],
            constraint: zfrac_m(value::zero(),-frac(3,1))
        },
        rows: vec![
            sut::SimplexRow{
                basic_variable: 2,
                equation: sut::Equation{
                    coefficients: vec![
                        frac(1,1),
                        frac(2,1),
                        frac(1,1),
                        value::zero()
                    ],
                    constraint: frac(4,1)
                },
                ratio: value::zero()
            },
            sut::SimplexRow{
                basic_variable: 3,
                equation: sut::Equation{
                    coefficients: vec![
                        frac(1,1),
                        frac(1,1),
                        value::zero(),
                        frac(1,1)
                    ],
                    constraint: frac(3,1)
                },
                ratio: value::zero()
            }
        ],
        point: vec![value::zero(),value::zero(),frac(4,1),frac(3,1)]
    };
    let solns = sut::solve(problem,&mut EmptyObserver::new());
    let expected_solns = vec![frac(3,1), value::zero(), frac(1,1), value::zero()];
    assert_eq!(expected_solns, solns);
}

#[test]
fn solve_observes_empty_problem() {
    let mut observer = MockObserver::new();
    let coeffs = vec![];
    let constraints = vec![];
    let problem = sut::Problem::new(&coeffs, &constraints);
    let expected_observations = vec![problem.clone()];
    let _solution = sut::solve(problem, &mut observer);
    assert_eq!(expected_observations, observer.observations);
}

#[test]
fn solve_observes_steps_of_problem() {
    let mut observer = MockObserver::new();
    let objective_coeffs = vec![zfrac(1, 1)];
    let functional_constraint = upper_bound_constraint(
        vec![frac(3, 1)], 
        frac(6, 1)
    );
    let constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_coeffs, &constraints);
    let mut middle_problem = problem.clone();
    sut::set_ratios(&mut middle_problem,0);
    let mut solved_problem = middle_problem.clone();
    sut::set_basic_variable(&mut solved_problem, 0, 0);
    sut::normalize_equation(&mut solved_problem, 0, 0);
    sut::reduce_equations(&mut solved_problem, 0, 0);
    sut::set_new_point(&mut solved_problem);

    let expected_observations = vec![middle_problem, solved_problem];
    let _solution = sut::solve(problem, &mut observer);
    assert_eq!(expected_observations, observer.observations);
}