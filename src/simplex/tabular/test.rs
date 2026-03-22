use crate::simplex::{
    tabular,
    test::{frac, upper_bound_constraint},
    value::Value,
    Constraint,
};

#[test]
fn finds_initial_point_with_no_constraints() {
    let coeffs = Vec::<Value>::new();
    let ub_constraints = Vec::<Constraint<Value>>::new();
    let point = tabular::initial_point(&coeffs, &ub_constraints);
    let expected_point = Vec::<Value>::new();
    assert_eq!(expected_point, point);
}

#[test]
fn finds_initial_point_with_one_constraint_one_variable() {
    let coeffs = vec![frac(1, 1)];
    let ub_constraints = vec![upper_bound_constraint(vec![frac(1, 1)], frac(5, 1))];
    let point = tabular::initial_point(&coeffs, &ub_constraints);
    let expected_point = vec![frac(0, 1), frac(5, 1)];
    assert_eq!(expected_point, point);
}

#[test]
fn finds_initial_point_with_one_constraint_two_variables() {
    let coeffs = vec![frac(1, 1), frac(1, 1)];
    let ub_constraints = vec![upper_bound_constraint(
        vec![frac(1, 1), frac(2, 1)],
        frac(6, 1),
    )];
    let point = tabular::initial_point(&coeffs, &ub_constraints);
    let expected_point = vec![frac(0, 1), frac(0, 1), frac(6, 1)];
    assert_eq!(expected_point, point);
}

#[test]
fn finds_initial_point_with_two_constraints_two_variables() {
    let coeffs = vec![frac(1, 1), frac(1, 1)];
    let ub_constraints = vec![
        upper_bound_constraint(vec![frac(1, 1), frac(2, 1)], frac(6, 1)),
        upper_bound_constraint(vec![frac(3, 1), frac(4, 1)], frac(7, 1)),
    ];
    let point = tabular::initial_point(&coeffs, &ub_constraints);
    let expected_point = vec![frac(0, 1), frac(0, 1), frac(6, 1), frac(7, 1)];
    assert_eq!(expected_point, point);
}
