use std::{fmt::Display, ops::{Div, DivAssign, Mul, Neg, SubAssign}};

use fraction::{ConstOne, ConstZero, Fraction};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Value {
    pub finite: Fraction,
    pub m: Fraction,
}

impl Value {
    pub fn from(f: Fraction) -> Self {
        Value { finite: f, m: Fraction::ZERO }
    }
}

impl Neg for Value {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Value {
            finite: self.finite.neg(),
            m: self.m,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.finite.fmt(f)
        // if self.m.is_zero() {
        //     self.finite.fmt(f)
        // } else {
        //     self.finite.fmt(f);
        //     if self.m.is_negative() {
        //         write!(f," - ");
        //     } else {
        //         write!(f," + ");
        //     }
        //     self.m.abs().fmt(f);
        //     write!(f,"M")
        // }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Value {
            finite: self.finite.mul(rhs.finite),
            m: self.m
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Value {
            finite: self.finite.div(rhs.finite),
            m: self.m
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        self.finite.div_assign(rhs.finite);
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        self.finite.sub_assign(rhs.finite);
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.finite.partial_cmp(&other.finite)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.finite.cmp(&other.finite)
    }
}

pub fn zero() -> Value {
    Value { finite: Fraction::ZERO, m: Fraction::ZERO }
}

pub fn one() -> Value {
    Value { finite: Fraction::ONE, m: Fraction::ZERO }
}