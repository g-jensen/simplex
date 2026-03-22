#[cfg(test)]
pub mod test;

use std::{fmt::Display, ops::{Add, Div, Mul, Neg}};

use fraction::{Signed, Zero};

use crate::simplex::{Value, value};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MValue {
    finite: Value,
    m: Value
}

impl MValue {
    pub fn from(finite: Value) -> MValue {
        MValue { finite: finite, m: value::zero() }
    }

    pub fn from_m(finite: Value, m: Value) -> MValue {
        MValue { finite: finite, m: m }
    }

    pub fn zero() -> MValue {
        MValue::from(value::zero())
    }
}

impl PartialOrd for MValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.m.partial_cmp(&other.m) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.finite.partial_cmp(&other.finite)
    }
}

impl Ord for MValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.m.cmp(&other.m) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.finite.cmp(&other.finite)
    }
}

impl Add for MValue {
    type Output = MValue;

    fn add(self, rhs: MValue) -> Self::Output {
        MValue{
            finite: self.finite + rhs.finite,
            m : self.m + rhs.m
        }
    }
}

impl Neg for MValue {
    type Output = MValue;

    fn neg(self) -> Self::Output {
        MValue{
            finite: self.finite.neg(),
            m: self.m.neg()
        }
    }
}

impl Mul<Value> for MValue {
    type Output = MValue;

    fn mul(self, rhs: Value) -> Self::Output {
        MValue{
            finite: self.finite * rhs,
            m : self.m * rhs
        }
    }
}

impl Div<Value> for MValue {
    type Output = MValue;

    fn div(self, rhs: Value) -> Self::Output {
        MValue{
            finite: self.finite / rhs,
            m : self.m / rhs
        }
    }
}

impl Display for MValue {
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