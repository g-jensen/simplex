mod pivot_variable {
    use crate::simplex::{self as sut};

    #[test]
    fn empty_objective_has_no_pivot() {
        let problem = sut::TabularSimplex {
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
        let problem = sut::TabularSimplex {
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
        let problem = sut::TabularSimplex {
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