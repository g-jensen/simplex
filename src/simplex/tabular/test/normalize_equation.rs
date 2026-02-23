mod normalize_equation {
    use crate::simplex::tabular::{self as sut};

    #[test]
    fn sets_single_coefficient_to_one() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![5_f32],
                    constraint: 10_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(vec![1_f32], problem.rows[0].equation.coefficients);
    }

    #[test]
    fn divides_other_coefficients_by_pivot() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![2_f32, 4_f32, 6_f32],
                    constraint: 10_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(
            vec![1_f32, 2_f32, 3_f32],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn normalizes_on_non_first_variable() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![3_f32, 6_f32, 9_f32],
                    constraint: 12_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 1);
        assert_eq!(
            vec![0.5_f32, 1_f32, 1.5_f32],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn leaves_other_coefficients_unchanged_when_pivot_is_one() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32, 3_f32, 5_f32],
                    constraint: 8_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 0, 0);
        assert_eq!(
            vec![1_f32, 3_f32, 5_f32],
            problem.rows[0].equation.coefficients
        );
    }

    #[test]
    fn normalizes_correct_row_when_multiple_rows_exist() {
        let mut problem = sut::TabularSimplex {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![2_f32, 4_f32],
                        constraint: 6_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![3_f32, 9_f32],
                        constraint: 12_f32,
                    },
                    ratio: 0_f32,
                },
            ],
            point: vec![],
        };
        sut::normalize_equation(&mut problem, 1, 1);
        assert_eq!(vec![2_f32, 4_f32], problem.rows[0].equation.coefficients);
        assert_eq!(
            vec![1_f32 / 3_f32, 1_f32],
            problem.rows[1].equation.coefficients
        );
    }
}