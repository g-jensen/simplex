mod is_optimal {
    use crate::simplex::tabular::primal::{
        self as sut, MObjectiveEquation, PrimalProblem, mobjectivevalue::test::mvalue_from
    };

    #[test]
    fn empty_objective_is_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn positive_objective_is_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![mvalue_from(1, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn zero_objective_is_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![mvalue_from(0, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn positive_and_zero_objective_is_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![mvalue_from(1, 1), mvalue_from(0, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(sut::is_optimal(&problem));
    }

    #[test]
    fn negative_objective_is_not_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![-mvalue_from(1, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(!sut::is_optimal(&problem));
    }

    #[test]
    fn one_negative_in_objective_is_not_optimal() {
        let problem = PrimalProblem {
            objective_equation: MObjectiveEquation {
                coefficients: vec![-mvalue_from(1, 1), mvalue_from(0, 1)],
                constraint: mvalue_from(0, 1),
            },
            rows: vec![],
            point: vec![],
        };
        assert!(!sut::is_optimal(&problem));
    }
}
