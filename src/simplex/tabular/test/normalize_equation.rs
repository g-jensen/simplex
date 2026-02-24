mod normalize_equation {
    use crate::simplex::{
        tabular::{self as sut},
        test::frac,
    };

    #[test]
    fn sets_single_coefficient_to_one() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(5, 1)],
                    constraint: frac(10, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(vec![frac(1, 1)], problem.rows[0].equation.coefficients);
    }

    #[test]
    fn divides_other_coefficients_by_pivot() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(2, 1), frac(4, 1), frac(6, 1)],
                    constraint: frac(10, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(
            vec![frac(1, 1), frac(2, 1), frac(3, 1)],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn normalizes_on_non_first_variable() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(3, 1), frac(6, 1), frac(9, 1)],
                    constraint: frac(12, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 1);
        assert_eq!(
            vec![frac(1, 2), frac(1, 1), frac(3, 2)],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn leaves_other_coefficients_unchanged_when_pivot_is_one() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1), frac(3, 1), frac(5, 1)],
                    constraint: frac(8, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(
            vec![frac(1, 1), frac(3, 1), frac(5, 1)],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn normalizes_correct_row_when_multiple_rows_exist() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(2, 1), frac(4, 1)],
                        constraint: frac(6, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(3, 1), frac(9, 1)],
                        constraint: frac(12, 1),
                    },
                    ratio: frac(0, 1),
                },
            ],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 1, 1);
        assert_eq!(
            vec![frac(2, 1), frac(4, 1)],
            problem.rows[0].equation.coefficients
        );
        assert_eq!(
            vec![frac(1, 3), frac(1, 1)],
            problem.rows[1].equation.coefficients
        );
    }
}
