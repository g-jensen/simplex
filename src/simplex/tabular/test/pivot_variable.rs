mod pivot_variable {
    use crate::simplex::{
        tabular::{self as sut},
        test::frac,
    };

    #[test]
    fn empty_objective_has_no_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![],
                constraint: frac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(None, sut::pivot_variable(&problem));
    }

    #[test]
    fn single_var_is_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(1, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(Some(0), sut::pivot_variable(&problem));
    }

    #[test]
    fn smallest_var_is_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::Equation {
                coefficients: vec![frac(1, 1), -frac(2, 1), frac(0, 1)],
                constraint: frac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(Some(1), sut::pivot_variable(&problem));
    }
}
