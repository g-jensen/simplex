
#[cfg(test)]
mod tests {
    use crate::linear_optimization_problem::{self as sut, test};

    #[test]
    fn solves_one_variable_zero_constraint_problem() {
        let objective_function = vec![1_f32];
        let fn_constraints = vec![];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn solves_two_variable_zero_constraint_problem() {
        let objective_function = vec![1_f32, 2_f32];
        let fn_constraints = vec![];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![0_f32, 0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn solves_one_variable_one_constraint_problem() {
        let objective_function = vec![1_f32];
        let functional_constraint = sut::UpperBoundConstraint::new(vec![1_f32],1_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![1_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_unconstrainted_variable() {
        let objective_function = vec![1_f32, 1_f32];
        let functional_constraint = sut::UpperBoundConstraint::new(vec![3_f32, 0_f32],6_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![2_f32, 0_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_one_variable_problem() {
        let objective_function = vec![1_f32];
        let functional_constraint = sut::UpperBoundConstraint::new(vec![3_f32],6_f32);
        let fn_constraints = vec![functional_constraint];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_one_variable_two_constraint_problem() {
        let objective_function = vec![1_f32];
        let fn_constaint_0 = sut::UpperBoundConstraint::new(vec![2_f32],6_f32);
        let fn_constaint_1 = sut::UpperBoundConstraint::new(vec![3_f32], 6_f32);
        let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_two_independent_variable_two_constraint_problem() {
        let objective_function = vec![1_f32, 1_f32];
        let fn_constaint_0 = sut::UpperBoundConstraint::new(vec![2_f32,0_f32],6_f32);
        let fn_constaint_1 = sut::UpperBoundConstraint::new(vec![0_f32,4_f32],8_f32);
        let fn_constraints = vec![fn_constaint_0, fn_constaint_1];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![3_f32,2_f32];
        assert_eq!(expected_solns, solns);
    }

    #[test]
    fn maximizes_two_dependent_variable_one_constraint_symmetric_problem() {
        let objective_function = vec![1_f32, 1_f32];
        let fn_constaint_0 = sut::UpperBoundConstraint::new(vec![2_f32,4_f32],6_f32);
        let fn_constraints = vec![fn_constaint_0];
        let solns = sut::solve_standard_problem(&objective_function,&fn_constraints);
        let expected_solns = vec![3_f32,0_f32];
        assert_eq!(expected_solns, solns);
    }
}