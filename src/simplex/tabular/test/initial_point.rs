mod initial_point {
    use crate::simplex::{tabular::{self as sut}, test::frac};

    #[test]
    fn finds_initial_point_with_no_constraints() {
        let coeffs = Vec::<sut::Value>::new();
        let ub_constraints = Vec::<sut::UpperBoundConstraint>::new();
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = Vec::<sut::Value>::new();
        assert_eq!(expected_point, point);
    }

    #[test]
    fn finds_initial_point_with_one_constraint_one_variable() {
        let coeffs = vec![frac(1, 1)];
        let ub_constraints = vec![sut::UpperBoundConstraint {
            coefficients: vec![frac(1, 1)],
            bound: frac(5, 1),
        }];
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![frac(0, 1), frac(5, 1)];
        assert_eq!(expected_point, point);
    }

    #[test]
    fn finds_initial_point_with_one_constraint_two_variables() {
        let coeffs = vec![frac(1, 1), frac(1, 1)];
        let ub_constraints = vec![sut::UpperBoundConstraint {
            coefficients: vec![frac(1, 1), frac(2, 1)],
            bound: frac(6, 1),
        }];
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![frac(0, 1), frac(0, 1), frac(6, 1)];
        assert_eq!(expected_point, point);
    }

    #[test]
    fn finds_initial_point_with_two_constraints_two_variables() {
        let coeffs = vec![frac(1, 1), frac(1, 1)];
        let ub_constraints = vec![
            sut::UpperBoundConstraint {
                coefficients: vec![frac(1, 1), frac(2, 1)],
                bound: frac(6, 1),
            },
            sut::UpperBoundConstraint {
                coefficients: vec![frac(3, 1), frac(4, 1)],
                bound: frac(7, 1),
            },
        ];
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![frac(0, 1), frac(0, 1), frac(6, 1), frac(7, 1)];
        assert_eq!(expected_point, point);
    }
}