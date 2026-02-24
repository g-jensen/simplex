mod is_optimal {
    use crate::simplex::{
        tabular::{self as sut},
        test::frac,
    };

    #[test]
    fn empty_objective_is_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
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
                coefficients: vec![frac(1, 1)],
                constraint: frac(0, 1),
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
                coefficients: vec![frac(0, 1)],
                constraint: frac(0, 1),
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
                coefficients: vec![frac(1, 1), frac(0, 1)],
                constraint: frac(0, 1),
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
                coefficients: vec![-frac(1, 1)],
                constraint: frac(0, 1),
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
                coefficients: vec![-frac(1, 1), frac(0, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(!sut::is_optimal(&problem));
    }
}
