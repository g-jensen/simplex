mod set_ratios {
    use crate::simplex::tabular::{self as sut};

    #[test]
    fn recalculates_ratio_for_row() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![2_f32],
                    constraint: 6_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 0);
        assert_eq!(3_f32, problem.rows[0].ratio);
    }

    #[test]
    fn recalculates_ratio_for_rows() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![2_f32],
                        constraint: 6_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![2_f32],
                        constraint: 8_f32,
                    },
                    ratio: 0_f32,
                },
            ],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 0);
        assert_eq!(3_f32, problem.rows[0].ratio);
        assert_eq!(4_f32, problem.rows[1].ratio);
    }

    #[test]
    fn recalculates_ratio_for_specified_var() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![2_f32, 3_f32],
                    constraint: 6_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 1);
        assert_eq!(2_f32, problem.rows[0].ratio);
    }
}