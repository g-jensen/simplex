use fraction::{ConstOne, ConstZero, Fraction};

use crate::simplex::rowvalue::RowValue;

pub type Value = Fraction;

pub fn zero() -> Value {
    Fraction::ZERO
}

pub fn one() -> Value {
    Fraction::ONE
}

impl RowValue for Value {
    fn zero() -> Self {
        zero()
    }

    fn one() -> Self {
        one()
    }

    fn is_finite(&self) -> bool {
        !matches!(self, Fraction::Infinity(_) | Fraction::NaN)
    }
}
