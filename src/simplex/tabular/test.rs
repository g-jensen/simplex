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
    let functional_constraint =
        simplex_helper::upper_bound_constraint(vec![frac(3, 1)], frac(6, 1));
    let constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_coeffs, &constraints);
    let next_problem = sut::Problem {
        objective_equation: sut::Equation {
            coefficients: vec![frac(0, 1), frac(1, 3)],
            constraint: frac(2, 1),
        },
        rows: vec![sut::SimplexRow {
            basic_variable: 0,
            equation: sut::Equation {
                coefficients: vec![frac(1, 1), frac(1, 3)],
                constraint: frac(2, 1),
            },
            ratio: frac(2, 1),
        }],
        point: vec![frac(2, 1), frac(0, 1)],
    };
    let expected_observations = vec![problem.clone(), next_problem];
    let _solution = sut::solve(problem, &mut observer);
    assert_eq!(expected_observations, observer.observations);
}