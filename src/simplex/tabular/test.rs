mod initial_point;

mod is_optimal;

mod pivot_variable;

mod set_ratios;

mod pivot_row_idx;

mod normalize_equation;

mod reduce_equations;

use crate::simplex::tabular as sut;
use crate::simplex::test::{self as simplex_helper, frac};

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
    let objective_coeffs = vec![frac(1, 1)];
    let functional_constraint = simplex_helper::upper_bound_constraint(
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