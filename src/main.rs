mod simplex;

use std::io::stdout;

use fraction::Fraction;

use crate::simplex::tabular::UpperBoundConstraint;
use crate::simplex::tabular::Value;
use crate::simplex::tabular::write_observer::WriteObserver;
use crate::simplex::tabular::Problem;
use crate::simplex::tabular::solve;

fn main() {
    let mut writer = stdout();
    let mut observer = WriteObserver::new(&mut writer);
    let objective_fn_coeffs = vec![Value::from(Fraction::from(1)), Value::from(Fraction::from(2))];
    let functional_constraints = vec![
        UpperBoundConstraint {
            coefficients: vec![Value::from(Fraction::from(1)),Value::from(Fraction::from(1))],
            bound: Value::from(Fraction::from(4))
        },
        UpperBoundConstraint {
            coefficients: vec![Value::from(Fraction::from(1)),Value::from(Fraction::from(3))],
            bound: Value::from(Fraction::from(8))
        },
    ];
    let problem = Problem::new(&objective_fn_coeffs, &functional_constraints);
    solve(problem,&mut observer);
}
