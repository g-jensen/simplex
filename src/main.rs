mod simplex;

use std::io::stdout;

use fraction::Fraction;

use crate::simplex::core::UpperBoundConstraint;
use crate::simplex::tabular::write_observer::WriteObserver;
use crate::simplex::tabular::Problem;
use crate::simplex::tabular::solve;

fn main() {
    let mut writer = stdout();
    let mut observer = WriteObserver::new(&mut writer);
    let objective_fn_coeffs = vec![Fraction::from(2), Fraction::from(6), Fraction::from(9)];
    let functional_constraints = vec![
        UpperBoundConstraint {
            coefficients: vec![Fraction::from(1),Fraction::from(0),Fraction::from(1)],
            bound: Fraction::from(3)
        },
        UpperBoundConstraint {
            coefficients: vec![Fraction::from(0),Fraction::from(1),Fraction::from(2)],
            bound: Fraction::from(5)
        },
    ];
    let problem = Problem::new(&objective_fn_coeffs, &functional_constraints);
    solve(problem,&mut observer);
}
