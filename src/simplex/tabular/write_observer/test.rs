use crate::simplex::tabular::primal::mobjectivevalue::test::mvalue_from;
use crate::simplex::tabular::write_observer::{self as sut};
use crate::simplex::tabular::primal::{Equation, ObjectiveEquation, Problem, ProblemObserver, SimplexRow};
use crate::simplex::test::{frac};

fn make_one_variable_problem() -> Problem {
    Problem {
        objective_equation: ObjectiveEquation {
            coefficients: vec![-mvalue_from(5,1)],
            constraint: mvalue_from(0,1),
        },
        rows: vec![],
        point: vec![frac(0,1)],
    }
}

#[test]
fn formats_objective_row_only() {
    let mut output = Vec::new();
    {
        let mut observer = sut::WriteObserver::new(&mut output);
        observer.observe(make_one_variable_problem());
    }
    let result = String::from_utf8(output).unwrap();
    let expected = "\
| BV | x0 | RHS | Ratio |
|----|----|-----|-------|
| Z  | -5 |   0 |       |
";
    assert_eq!(expected, result);
}

#[test]
fn formats_two_variable_two_constraint_problem() {
    let problem = Problem {
        objective_equation: ObjectiveEquation {
            coefficients: vec![
                -mvalue_from(500,1),
                -mvalue_from(4,1),
                mvalue_from(0,1),
                mvalue_from(0,1),
            ],
            constraint: mvalue_from(0,1),
        },
        rows: vec![
            SimplexRow {
                basic_variable: 2,
                equation: Equation {
                    coefficients: vec![
                        frac(6,1),
                        frac(4,1),
                        frac(1,1),
                        frac(0,1),
                    ],
                    constraint: frac(24,1),
                },
                ratio: frac(4,1),
            },
            SimplexRow {
                basic_variable: 3,
                equation: Equation {
                    coefficients: vec![
                        frac(1,1),
                        frac(2,1),
                        frac(0,1),
                        frac(1,1),
                    ],
                    constraint: frac(6,1),
                },
                ratio: frac(6,1),
            },
        ],
        point: vec![frac(0,1); 4],
    };
    let mut output = Vec::new();
    {
        let mut observer = sut::WriteObserver::new(&mut output);
        observer.observe(problem);
    }
    let result = String::from_utf8(output).unwrap();
    let expected = "\
| BV |   x0 | x1 | x2 | x3 | RHS | Ratio |
|----|------|----|----|----|-----|-------|
| Z  | -500 | -4 |  0 |  0 |   0 |       |
| x2 |    6 |  4 |  1 |  0 |  24 |     4 |
| x3 |    1 |  2 |  0 |  1 |   6 |     6 |
";
    assert_eq!(expected, result);
}

#[test]
fn formats_with_one_constraint_row() {
    let problem = Problem {
        objective_equation: ObjectiveEquation {
            coefficients: vec![-mvalue_from(5,1)],
            constraint: mvalue_from(0,1),
        },
        rows: vec![SimplexRow {
            basic_variable: 1,
            equation: Equation {
                coefficients: vec![frac(2,1)],
                constraint: frac(10,1),
            },
            ratio: frac(5,1),
        }],
        point: vec![frac(0,1)],
    };
    let mut output = Vec::new();
    {
        let mut observer = sut::WriteObserver::new(&mut output);
        observer.observe(problem);
    }
    let result = String::from_utf8(output).unwrap();
    let expected = "\
| BV | x0 | RHS | Ratio |
|----|----|-----|-------|
| Z  | -5 |   0 |       |
| x1 |  2 |  10 |     5 |
";
    assert_eq!(expected, result);
}
