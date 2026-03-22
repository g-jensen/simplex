mod is_optimal {
    use crate::simplex::{
        tabular::primal::{self as sut, {ObjectiveEquation, Problem}, mvalue::test::zfrac},
    };

    #[test]
    fn empty_objective_is_optimal() {
        let problem = Problem {
            objective_equation: ObjectiveEquation {
                coefficients: vec![],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn positive_objective_is_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![zfrac(1, 1)],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn zero_objective_is_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![zfrac(0, 1)],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn positive_and_zero_objective_is_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![zfrac(1, 1), zfrac(0, 1)],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn negative_objective_is_not_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![-zfrac(1, 1)],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(!sut::is_optimal(&problem));
    }

    #[test]
    fn one_negative_in_objective_is_not_optimal() {
        let problem = sut::Problem {
            objective_equation: sut::ObjectiveEquation {
                coefficients: vec![-zfrac(1, 1), zfrac(0, 1)],
                constraint: zfrac(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(!sut::is_optimal(&problem));
    }
}
