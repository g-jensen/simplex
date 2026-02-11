#[cfg(test)]
mod tests {
    use crate::linear_optimization_problem::{self as sut};

    pub fn upper_bound_constraint(
        coefficients: sut::Coefficients,
        bound: sut::Value,
    ) -> sut::UpperBoundConstraint {
        sut::UpperBoundConstraint {
            coefficients,
            bound,
        }
    }

    #[test]
    fn solves_one_variable_zero_constraint_problem() {
        let objective_function = vec![1_f32];
        let fn_constraints = vec![];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn solves_two_variable_zero_constraint_problem() {
        let objective_function = vec![1_f32, 2_f32];
        let fn_constraints = vec![];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![0_f32, 0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn solves_one_variable_one_constraint_problem() {
        let objective_function = vec![1_f32];
        let functional_constraint = upper_bound_constraint(vec![1_f32], 1_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![1_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_unconstrainted_variable() {
        let objective_function = vec![1_f32, 1_f32];
        let functional_constraint = upper_bound_constraint(vec![3_f32, 0_f32], 6_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![2_f32, 0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_one_variable_problem() {
        let objective_function = vec![1_f32];
        let functional_constraint = upper_bound_constraint(vec![3_f32], 6_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_one_variable_two_constraint_problem() {
        let objective_function = vec![1_f32];
        let fn_constaint_0 = upper_bound_constraint(vec![2_f32], 6_f32);
        let fn_constaint_1 = upper_bound_constraint(vec![3_f32], 6_f32);
        let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_two_independent_variable_two_constraint_problem() {
        let objective_function = vec![1_f32, 1_f32];
        let fn_constaint_0 = upper_bound_constraint(vec![2_f32, 0_f32], 6_f32);
        let fn_constaint_1 = upper_bound_constraint(vec![0_f32, 4_f32], 8_f32);
        let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![3_f32, 2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_two_dependent_variable_one_constraint_symmetric_problem() {
        let objective_function = vec![1_f32, 1_f32];
        let fn_constaint_0 = upper_bound_constraint(vec![2_f32, 4_f32], 6_f32);
        let fn_constraints = vec![fn_constaint_0];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![3_f32, 0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_two_dependent_variable_two_constraint_symmetric_problem() {
        let objective_function = vec![1_f32, 1_f32];
        let fn_constaint_0 = upper_bound_constraint(vec![1_f32, 2_f32], 3_f32);
        let fn_constaint_1 = upper_bound_constraint(vec![2_f32, 1_f32], 3_f32);
        let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
        let solns = sut::solve_standard_problem(&objective_function, &fn_constraints);
        let expected_solns = vec![1_f32, 1_f32];
        assert_eq!(expected_solns, solns);
    }

    mod initial_point {
        use super::*;

        #[test]
        fn finds_initial_point_with_no_constraints() {
            let ub_constraints = Vec::<sut::UpperBoundConstraint>::new();
            let point = sut::initial_point(&ub_constraints);
            let expected_point = Vec::<sut::Value>::new();
            assert_eq!(expected_point, point);
        }

        #[test]
        fn finds_initial_point_with_one_constraint_one_variable() {
            let ub_constraints = vec![sut::UpperBoundConstraint {
                coefficients: vec![1_f32],
                bound: 5_f32,
            }];
            let point = sut::initial_point(&ub_constraints);
            let expected_point = vec![0_f32, 5_f32];
            assert_eq!(expected_point, point);
        }

        #[test]
        fn finds_initial_point_with_one_constraint_two_variables() {
            let ub_constraints = vec![sut::UpperBoundConstraint {
                coefficients: vec![1_f32, 2_f32],
                bound: 6_f32,
            }];
            let point = sut::initial_point(&ub_constraints);
            let expected_point = vec![0_f32, 0_f32, 6_f32];
            assert_eq!(expected_point, point);
        }

        #[test]
        fn finds_initial_point_with_two_constraints_two_variables() {
            let ub_constraints = vec![
                sut::UpperBoundConstraint {
                    coefficients: vec![1_f32, 2_f32],
                    bound: 6_f32,
                },
                sut::UpperBoundConstraint {
                    coefficients: vec![3_f32, 4_f32],
                    bound: 7_f32,
                },
            ];
            let point = sut::initial_point(&ub_constraints);
            let expected_point = vec![0_f32, 0_f32, 6_f32, 7_f32];
            assert_eq!(expected_point, point);
        }
    }

    mod is_optimal {
        use super::*;

        #[test]
        fn empty_objective_is_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(sut::is_optimal(&problem));
        }

        #[test]
        fn positive_objective_is_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![1_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(sut::is_optimal(&problem));
        }

        #[test]
        fn zero_objective_is_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![0_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(sut::is_optimal(&problem));
        }

        #[test]
        fn positive_and_zero_objective_is_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![1_f32, 0_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(sut::is_optimal(&problem));
        }

        #[test]
        fn negative_objective_is_not_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![-1_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(!sut::is_optimal(&problem));
        }

        #[test]
        fn one_negative_in_objective_is_not_optimal() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![-1_f32, 0_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert!(!sut::is_optimal(&problem));
        }
    }

    mod pivot_variable {
        use super::*;

        #[test]
        fn empty_objective_has_no_pivot() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert_eq!(None, sut::pivot_variable(&problem));
        }

        #[test]
        fn single_var_is_pivot() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![1_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert_eq!(Some(0), sut::pivot_variable(&problem));
        }

        #[test]
        fn smallest_var_is_pivot() {
            let problem = sut::SimplexProblem {
                objective_equation: sut::Equation {
                    coefficients: vec![1_f32, -2_f32, 0_f32],
                    constraint: 0_f32,
                },
                rows: vec![],
                point: vec![],
            };
            assert_eq!(Some(1), sut::pivot_variable(&problem));
        }
    }

    mod set_ratios {
        use super::*;

        #[test]
        fn recalculates_ratio_for_row() {
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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

    mod pivot_row_idx {
        use super::*;

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

    mod normalize_equation {
        use super::*;

        #[test]
        fn sets_single_coefficient_to_one() {
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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

    mod reduce_equations {
        use super::*;

        #[test]
        fn reduces_objective_equation() {
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
            let mut problem = sut::SimplexProblem {
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
}
