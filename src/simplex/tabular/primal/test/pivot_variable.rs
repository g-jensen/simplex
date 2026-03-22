mod pivot_variable {
    use crate::simplex::tabular::primal::{self as sut, mvalue::test::mvalue_from};

    #[test]
    fn empty_objective_has_no_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(None, sut::pivot_variable(&problem));
    }

    #[test]
    fn single_var_is_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![mvalue_from(1, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(Some(0), sut::pivot_variable(&problem));
    }

    #[test]
    fn smallest_var_is_pivot() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![mvalue_from(1, 1), -mvalue_from(2, 1), mvalue_from(0, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert_eq!(Some(1), sut::pivot_variable(&problem));
    }
}
