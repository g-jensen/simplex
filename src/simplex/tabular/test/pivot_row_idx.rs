mod pivot_row_idx {
    use crate::simplex::{tabular::{self as sut}, test::frac};

    #[test]
    fn returns_none_for_no_rows() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_index_for_single_positive_ratio() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1)],
                    constraint: frac(5, 1),
                },
                ratio: frac(5, 1),
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn returns_none_for_zero_ratio() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1)],
                    constraint: frac(0, 1),
                },
                ratio: frac(0, 1),
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_none_for_negative_ratio() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![sut::SimplexRow {
                basic_variable: 0,
                equation: sut::Equation {
                    coefficients: vec![frac(1, 1)],
                    constraint: -frac(5, 1),
                },
                ratio: -frac(5, 1),
            }],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }

    #[test]
    fn returns_index_of_minimum_positive_ratio() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(10, 1),
                    },
                    ratio: frac(10, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(3, 1),
                    },
                    ratio: frac(3, 1),
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(7, 1),
                    },
                    ratio: frac(7, 1),
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn ignores_non_positive_ratios() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: -frac(2, 1),
                    },
                    ratio: -frac(2, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(0, 1),
                    },
                    ratio: frac(0, 1),
                },
                sut::SimplexRow {
                    basic_variable: 2,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(5, 1),
                    },
                    ratio: frac(5, 1),
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn returns_none_when_all_ratios_non_positive() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![
                sut::SimplexRow {
                    basic_variable: 0,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: -frac(3, 1),
                    },
                    ratio: -frac(3, 1),
                },
                sut::SimplexRow {
                    basic_variable: 1,
                    equation: sut::Equation {
                        coefficients: vec![frac(1, 1)],
                        constraint: frac(0, 1),
                    },
                    ratio: frac(0, 1),
                },
            ],
            point: vec![],
        };
        let result = sut::pivot_row_idx(&problem);
        assert_eq!(None, result);
    }
}