use crate::simplex::{
    tabular::dual::{self as sut},
    test::frac,
};

// #[test]
// fn solves_one_variable_zero_constraint_problem() {
//     let objective_function = vec![frac(1,1)];
//     let fn_constraints = vec![];
//     let problem = sut::Problem::new(&objective_function,&fn_constraints);
//     let solns = sut::solve(problem,&mut EmptyObserver::new());
//     let expected_solns = vec![frac(0,1)];
//     assert_eq!(expected_solns, solns);
// }
