#[cfg(test)]
mod test;

use std::{fmt::Display, ops::{Div, DivAssign, Mul, Neg, SubAssign}};

use fraction::{ConstOne, ConstZero, Fraction, Signed, Zero};


pub type Value = Fraction;

pub fn zero() -> Value {
    Fraction::ZERO
}

pub fn one() -> Value {
    Fraction::ONE
}

#[derive(Debug,PartialEq,Eq)]
pub struct ZValue {
    finite: Value,
    m: Value
}

impl ZValue {
    pub fn from(finite: Value) -> ZValue {
        ZValue { finite: finite, m: zero() }
    }

    pub fn from_m(finite: Value, m: Value) -> ZValue {
        ZValue { finite: finite, m: m }
    }
}

impl Neg for ZValue {
    type Output = ZValue;

    fn neg(self) -> Self::Output {
        ZValue{
            finite: self.finite.neg(),
            m: self.m.neg()
        }
    }
}

impl Mul<Value> for ZValue {
    type Output = ZValue;

    fn mul(self, rhs: Value) -> Self::Output {
        ZValue{
            finite: self.finite * rhs,
            m : self.m * rhs
        }
    }
}

impl Div<Value> for ZValue {
    type Output = ZValue;

    fn div(self, rhs: Value) -> Self::Output {
        ZValue{
            finite: self.finite / rhs,
            m : self.m / rhs
        }
    }
}

impl Display for ZValue {
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