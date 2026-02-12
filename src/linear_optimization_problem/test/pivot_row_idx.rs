mod pivot_row_idx {
    use crate::linear_optimization_problem::{self as sut};

    #[test]
    fn returns_none_for_no_rows() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_index_for_single_positive_ratio() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32],
                    constraint: 5_f32,
                },
                ratio: 5_f32,
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn returns_none_for_zero_ratio() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32],
                    constraint: 0_f32,
                },
                ratio: 0_f32,
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_none_for_negative_ratio() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![1_f32],
                    constraint: -5_f32,
                },
                ratio: -5_f32,
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_index_of_minimum_positive_ratio() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 10_f32,
                    },
                    ratio: 10_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 3_f32,
                    },
                    ratio: 3_f32,
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 7_f32,
                    },
                    ratio: 7_f32,
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn ignores_non_positive_ratios() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: -2_f32,
                    },
                    ratio: -2_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 0_f32,
                    },
                    ratio: 0_f32,
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 5_f32,
                    },
                    ratio: 5_f32,
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn returns_none_when_all_ratios_non_positive() {
        let problem = sut::SimplexProblem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: 0_f32,
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: -3_f32,
                    },
                    ratio: -3_f32,
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![1_f32],
                        constraint: 0_f32,
                    },
                    ratio: 0_f32,
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }
}