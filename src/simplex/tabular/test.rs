mod initial_point;

mod is_optimal;

mod pivot_variable;

mod set_ratios;

mod pivot_row_idx;

mod normalize_equation;

mod reduce_equations;

// use crate::simplex::tabular::Problem;

use crate::simplex::tabular as sut;
use crate::simplex::test::{self as simplex_helper};

struct MockObserver {
    observations: Vec<sut::Problem>
}

impl MockObserver {
    pub fn new() -> Self {
        Self {
            observations: Vec::new()
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
    let _solution = sut::solve(problem,&mut observer);
    assert_eq!(expected_observations,observer.observations);
}

#[test]
fn solve_observes_steps_of_problem() {
    let mut observer = MockObserver::new();
    let objective_coeffs = vec![1_f32];
    let functional_constraint = simplex_helper::upper_bound_constraint(vec![3_f32], 6_f32);
    let constraints = vec![functional_constraint];
    let problem = sut::Problem::new(&objective_coeffs, &constraints);
    let next_problem = sut::Problem { 
        objective_equation: sut::Equation { 
            coefficients: vec![0.0, 0.33333334], 
            constraint: 2.0 
        }, 
        rows: vec![
            sut::SimplexRow { 
                basic_variable: 0, 
                equation: sut::Equation { 
                    coefficients: vec![1.0, 0.33333334], 
                    constraint: 2.0 
                }, 
                ratio: 2.0 
            }
        ], 
        point: vec![2.0, 0.0] 
    };
    let expected_observations = vec![problem.clone(), next_problem];
    let _solution = sut::solve(problem,&mut observer);
    assert_eq!(expected_observations,observer.observations);
}