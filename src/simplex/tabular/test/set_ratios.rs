mod set_ratios {
    use crate::simplex::{
        tabular::{self as sut},
        test::frac,
    };

    #[test]
    fn recalculates_ratio_for_row() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(2, 1)],
                    constraint: frac(6, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 0);
        assert_eq!(frac(3, 1), problem.rows[0].ratio);
    }

    #[test]
    fn recalculates_ratio_for_rows() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(2, 1)],
                        constraint: frac(6, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(2, 1)],
                        constraint: frac(8, 1),
                    },
                    ratio: frac(0, 1),
                },
            ],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 0);
        assert_eq!(frac(3, 1), problem.rows[0].ratio);
        assert_eq!(frac(4, 1), problem.rows[1].ratio);
    }

    #[test]
    fn recalculates_ratio_for_specified_var() {
        let mut problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(2, 1), frac(3, 1)],
                    constraint: frac(6, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        sut::set_ratios(&mut problem, 1);
        assert_eq!(frac(2, 1), problem.rows[0].ratio);
    }
}
