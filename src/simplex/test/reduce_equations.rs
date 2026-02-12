mod reduce_equations {
    use crate::simplex::{self as sut};

    #[test]
    fn reduces_objective_equation() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![3_f32, 2_f32],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32, 0.5_f32],
                    constraint: 4_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(
            vec![0_f32, 0.5_f32],
            problem.objective_equation.coefficients
        );
        assert_eq!(-12_f32, problem.objective_equation.constraint);
    }

    #[test]
    fn leaves_pivot_row_unchanged() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![1_f32, 1_f32],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32, 2_f32],
                    constraint: 6_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(vec![1_f32, 2_f32], problem.rows[0].equation.coefficients);
        assert_eq!(6_f32, problem.rows[0].equation.constraint);
    }

    #[test]
    fn reduces_non_pivot_row() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![0_f32, 0_f32],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![1_f32, 2_f32],
                        constraint: 10_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![3_f32, 4_f32],
                        constraint: 20_f32,
                    },
                    ratio: 0_f32,
                },
            ],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 0);
        assert_eq!(vec![0_f32, -2_f32], problem.rows[1].equation.coefficients);
        assert_eq!(-10_f32, problem.rows[1].equation.constraint);
    }

    #[test]
    fn reduces_multiple_non_pivot_rows() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![0_f32, 0_f32],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![2_f32, 1_f32],
                        constraint: 5_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![1_f32, 3_f32],
                        constraint: 8_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![4_f32, 2_f32],
                        constraint: 12_f32,
                    },
                    ratio: 0_f32,
                },
            ],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 1, 0);
        assert_eq!(vec![0_f32, -5_f32], problem.rows[0].equation.coefficients);
        assert_eq!(-11_f32, problem.rows[0].equation.constraint);
        assert_eq!(vec![1_f32, 3_f32], problem.rows[1].equation.coefficients);
        assert_eq!(8_f32, problem.rows[1].equation.constraint);
        assert_eq!(vec![0_f32, -10_f32], problem.rows[2].equation.coefficients);
        assert_eq!(-20_f32, problem.rows[2].equation.constraint);
    }

    #[test]
    fn reduces_on_non_first_variable() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![2_f32, 4_f32],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![3_f32, 1_f32],
                    constraint: 6_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::reduce_equations(&mut problem, 0, 1);
        assert_eq!(
            vec![-10_f32, 0_f32],
            problem.objective_equation.coefficients
        );
        assert_eq!(-24_f32, problem.objective_equation.constraint);
    }
}