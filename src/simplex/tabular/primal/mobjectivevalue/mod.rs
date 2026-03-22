#[cfg(test)]
pub mod test;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg},
};

use fraction::{Signed, Zero};

use crate::simplex::{
    objectivevalue::{ObjectiveEquation, ObjectiveValue},
    value, Coefficients, Constraint, Operator, Value,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MObjectiveValue {
    finite: Value,
    m: Value,
}

impl MObjectiveValue {
    pub fn from(finite: Value) -> MObjectiveValue {
        MObjectiveValue {
            finite: finite,
            m: value::zero(),
        }
    }

    pub fn from_m(finite: Value, m: Value) -> MObjectiveValue {
        MObjectiveValue {
            finite: finite,
            m: m,
        }
    }

    pub fn zero() -> MObjectiveValue {
        MObjectiveValue::from(value::zero())
    }
}

impl PartialOrd for MObjectiveValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.m.partial_cmp(&other.m) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.finite.partial_cmp(&other.finite)
    }
}

impl Ord for MObjectiveValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.m.cmp(&other.m) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.finite.cmp(&other.finite)
    }
}

impl Add for MObjectiveValue {
    type Output = MObjectiveValue;

    fn add(self, rhs: MObjectiveValue) -> Self::Output {
        MObjectiveValue {
            finite: self.finite + rhs.finite,
            m: self.m + rhs.m,
        }
    }
}

impl Neg for MObjectiveValue {
    type Output = MObjectiveValue;

    fn neg(self) -> Self::Output {
        MObjectiveValue {
            finite: self.finite.neg(),
            m: self.m.neg(),
        }
    }
}

impl Mul<Value> for MObjectiveValue {
    type Output = MObjectiveValue;

    fn mul(self, rhs: Value) -> Self::Output {
        MObjectiveValue {
            finite: self.finite * rhs,
            m: self.m * rhs,
        }
    }
}

impl Div<Value> for MObjectiveValue {
    type Output = MObjectiveValue;

    fn div(self, rhs: Value) -> Self::Output {
        MObjectiveValue {
            finite: self.finite / rhs,
            m: self.m / rhs,
        }
    }
}

impl Display for MObjectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.m.is_zero() {
            self.finite.fmt(f)
        } else if self.finite.is_zero() {
            let _ = self.m.fmt(f);
            write!(f, "M")
        } else {
            let _ = self.finite.fmt(f);
            if self.m.is_negative() {
                let _ = write!(f, " - ");
            } else {
                let _ = write!(f, " + ");
            }
            let _ = self.m.abs().fmt(f);
            write!(f, "M")
        }
    }
}

fn find_equality_constraint(functional_constraints: &Vec<Constraint>) -> Option<Constraint> {
    for constraint in functional_constraints {
        match constraint.operator {
            Operator::EQUAL => return Some(constraint.clone()),
            _ => {}
        }
    }
    return None;
}

fn initial_objective_coeffs(
    coeffs: &Coefficients,
    equality_constraint_opt: &Option<Constraint>,
) -> Vec<MObjectiveValue> {
    let mut obj_coeffs = vec![MObjectiveValue::zero(); coeffs.len()];
    for i in 0..coeffs.len() {
        obj_coeffs[i] = -MObjectiveValue::from(coeffs[i].clone());
    }
    match equality_constraint_opt {
        Some(equality_constraint) => {
            for i in 0..equality_constraint.coefficients.len() {
                obj_coeffs[i] = obj_coeffs[i].clone()
                    + -MObjectiveValue::from_m(value::zero(), equality_constraint.coefficients[i])
            }
        }
        None => {}
    }
    obj_coeffs
}

impl ObjectiveValue for MObjectiveValue {
    fn is_optimal(&self) -> bool {
        *self >= MObjectiveValue::zero()
    }

    fn initial_objective_equation(
        objective_fn_coeffs: &Coefficients,
        functional_constraints: &Vec<Constraint>,
    ) -> ObjectiveEquation<Self> {
        let nonbasic_var_count = functional_constraints.len();
        let equality_constraint_opt = find_equality_constraint(functional_constraints);
        let mut coefficients =
            initial_objective_coeffs(objective_fn_coeffs, &equality_constraint_opt);
        coefficients.append(&mut vec![MObjectiveValue::zero(); nonbasic_var_count]);
        ObjectiveEquation {
            coefficients: coefficients,
            constraint: match equality_constraint_opt {
                Some(equality_constraint) => {
                    -MObjectiveValue::from_m(value::zero(), equality_constraint.bound)
                }
                None => MObjectiveValue::zero(),
            },
        }
    }
}
