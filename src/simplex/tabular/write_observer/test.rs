use crate::simplex::tabular::write_observer::{self as sut};
use crate::simplex::tabular::{Equation, Problem, ProblemObserver, SimplexRow};
use fraction::{ConstZero, Fraction};

fn make_one_variable_problem() -> Problem {
    Problem {
        objective_equation: Equation {
            coefficients: vec![Fraction::from(-5)],
            constraint: Fraction::ZERO,
        },
        rows: vec![],
        point: vec![Fraction::ZERO],
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
        objective_equation: Equation {
            coefficients: vec![
                Fraction::from(-500),
                Fraction::from(-4),
                Fraction::ZERO,
                Fraction::ZERO,
            ],
            constraint: Fraction::ZERO,
        },
        rows: vec![
            SimplexRow {
                basic_variable: 2,
                equation: Equation {
                    coefficients: vec![
                        Fraction::from(6),
                        Fraction::from(4),
                        Fraction::from(1),
                        Fraction::ZERO,
                    ],
                    constraint: Fraction::from(24),
                },
                ratio: Fraction::from(4),
            },
            SimplexRow {
                basic_variable: 3,
                equation: Equation {
                    coefficients: vec![
                        Fraction::from(1),
                        Fraction::from(2),
                        Fraction::ZERO,
                        Fraction::from(1),
                    ],
                    constraint: Fraction::from(6),
                },
                ratio: Fraction::from(6),
            },
        ],
        point: vec![Fraction::ZERO; 4],
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
        objective_equation: Equation {
            coefficients: vec![Fraction::from(-5)],
            constraint: Fraction::ZERO,
        },
        rows: vec![SimplexRow {
            basic_variable: 1,
            equation: Equation {
                coefficients: vec![Fraction::from(2)],
                constraint: Fraction::from(10),
            },
            ratio: Fraction::from(5),
        }],
        point: vec![Fraction::ZERO],
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
