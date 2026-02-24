mod reduce_equations {
    use crate::simplex::{
        tabular::{self as sut},
        test::frac,
    };

    #[test]
    fn reduces_objective_equation() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(3, 1), frac(2, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1), frac(1, 2)],
                    constraint: frac(4, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(
            vec![frac(0, 1), frac(1, 2)],
            problem.objective_equation.coefficients
        );
        assert_eq!(-frac(12, 1), problem.objective_equation.constraint);
    }

    #[test]
    fn leaves_pivot_row_unchanged() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(1, 1), frac(1, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1), frac(2, 1)],
                    constraint: frac(6, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(
            vec![frac(1, 1), frac(2, 1)],
            problem.rows[0].equation.coefficients
        );
        assert_eq!(frac(6, 1), problem.rows[0].equation.constraint);
    }

    #[test]
    fn reduces_non_pivot_row() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(0, 1), frac(0, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1), frac(2, 1)],
                        constraint: frac(10, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(3, 1), frac(4, 1)],
                        constraint: frac(20, 1),
                    },
                    ratio: frac(0, 1),
                },
            ],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(
            vec![frac(0, 1), -frac(2, 1)],
            problem.rows[1].equation.coefficients
        );
        assert_eq!(-frac(10, 1), problem.rows[1].equation.constraint);
    }

    #[test]
    fn reduces_multiple_non_pivot_rows() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(0, 1), frac(0, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(2, 1), frac(1, 1)],
                        constraint: frac(5, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1), frac(3, 1)],
                        constraint: frac(8, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![frac(4, 1), frac(2, 1)],
                        constraint: frac(12, 1),
                    },
                    ratio: frac(0, 1),
                },
            ],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 1, 0);
        assert_eq!(
            vec![frac(0, 1), -frac(5, 1)],
            problem.rows[0].equation.coefficients
        );
        assert_eq!(-frac(11, 1), problem.rows[0].equation.constraint);
        assert_eq!(
            vec![frac(1, 1), frac(3, 1)],
            problem.rows[1].equation.coefficients
        );
        assert_eq!(frac(8, 1), problem.rows[1].equation.constraint);
        assert_eq!(
            vec![frac(0, 1), -frac(10, 1)],
            problem.rows[2].equation.coefficients
        );
        assert_eq!(-frac(20, 1), problem.rows[2].equation.constraint);
    }

    #[test]
    fn reduces_on_non_first_variable() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(2, 1), frac(4, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(3, 1), frac(1, 1)],
                    constraint: frac(6, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 1);
        assert_eq!(
            vec![-frac(10, 1), frac(0, 1)],
            problem.objective_equation.coefficients
        );
        assert_eq!(-frac(24, 1), problem.objective_equation.constraint);
    }
}
