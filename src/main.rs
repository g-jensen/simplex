mod simplex;

use std::io::stdout;

use fraction::Fraction;

use crate::simplex::tabular::Constraint;
use crate::simplex::tabular::Operator;
use crate::simplex::value::Value;
use crate::simplex::tabular::write_observer::WriteObserver;
use crate::simplex::tabular::Problem;
use crate::simplex::tabular::solve;

fn main() {
    let mut writer = stdout();
    let mut observer = WriteObserver::new(&mut writer);
    let objective_fn_coeffs = vec![Fraction::from(1), Fraction::from(2)];
    let functional_constraints = vec![
        Constraint {
            operator: Operator::LESSTHANEQUAL,
            coefficients: vec![Value::from(Fraction::from(1)),Value::from(Fraction::from(1))],
            bound: Value::from(Fraction::from(4))
        },
        Constraint {
            operator: Operator::LESSTHANEQUAL,
            coefficients: vec![Value::from(Fraction::from(1)),Value::from(Fraction::from(3))],
            bound: Value::from(Fraction::from(8))
        },
    ];
    let problem = Problem::new(&objective_fn_coeffs, &functional_constraints);
    solve(problem,&mut observer);
}
