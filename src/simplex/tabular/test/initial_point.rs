mod initial_point {
    use crate::simplex::tabular::{self as sut};

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
        let coeffs = vec![1_f32];
        let ub_constraints = vec![sut::UpperBoundConstraint {
            coefficients: vec![1_f32],
            bound: 5_f32,
        }];
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![0_f32, 5_f32];
        assert_eq!(expected_point, point);
    }

    #[test]
    fn finds_initial_point_with_one_constraint_two_variables() {
        let coeffs = vec![1_f32, 1_f32];
        let ub_constraints = vec![sut::UpperBoundConstraint {
            coefficients: vec![1_f32, 2_f32],
            bound: 6_f32,
        }];
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![0_f32, 0_f32, 6_f32];
        assert_eq!(expected_point, point);
    }

    #[test]
    fn finds_initial_point_with_two_constraints_two_variables() {
        let coeffs = vec![1_f32, 1_f32];
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
        let point = sut::initial_point(&coeffs,&ub_constraints);
        let expected_point = vec![0_f32, 0_f32, 6_f32, 7_f32];
        assert_eq!(expected_point, point);
    }
}