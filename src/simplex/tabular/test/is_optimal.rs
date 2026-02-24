mod is_optimal {
    use crate::simplex::tabular::{self as sut};

    #[test]
    fn empty_objective_is_optimal() {
        let problem = sut::Problem {
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
        let problem = sut::Problem {
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
        let problem = sut::Problem {
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
        let problem = sut::Problem {
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
        let problem = sut::Problem {
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
        let problem = sut::Problem {
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