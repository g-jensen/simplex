#[cfg(test)]
pub mod test;

use std::{fmt::Display, ops::{Add, Div, Mul, Neg}};

use fraction::{Signed, Zero};

use crate::simplex::{Value, value};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MObjectiveValue {
    finite: Value,
    m: Value
}

impl MObjectiveValue {
    pub fn from(finite: Value) -> MObjectiveValue {
        MObjectiveValue { finite: finite, m: value::zero() }
    }

    pub fn from_m(finite: Value, m: Value) -> MObjectiveValue {
        MObjectiveValue { finite: finite, m: m }
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
        MObjectiveValue{
            finite: self.finite + rhs.finite,
            m : self.m + rhs.m
        }
    }
}

impl Neg for MObjectiveValue {
    type Output = MObjectiveValue;

    fn neg(self) -> Self::Output {
        MObjectiveValue{
            finite: self.finite.neg(),
            m: self.m.neg()
        }
    }
}

impl Mul<Value> for MObjectiveValue {
    type Output = MObjectiveValue;

    fn mul(self, rhs: Value) -> Self::Output {
        MObjectiveValue{
            finite: self.finite * rhs,
            m : self.m * rhs
        }
    }
}

impl Div<Value> for MObjectiveValue {
    type Output = MObjectiveValue;

    fn div(self, rhs: Value) -> Self::Output {
        MObjectiveValue{
            finite: self.finite / rhs,
            m : self.m / rhs
        }
    }
}

impl Display for MObjectiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.m.is_zero() {
            self.finite.fmt(f)
        } else if self.finite.is_zero() {
            let _ = self.m.fmt(f);
            write!(f,"M")
        } else {
            let _ = self.finite.fmt(f);
            if self.m.is_negative() {
                let _ = write!(f," - ");
            } else {
                let _ = write!(f," + ");
            }
            let _ = self.m.abs().fmt(f);
            write!(f,"M")
        }
    }
}