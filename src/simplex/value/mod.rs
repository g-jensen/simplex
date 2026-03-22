use fraction::{ConstOne, ConstZero, Fraction};

pub type Value = Fraction;

pub fn zero() -> Value {
    Fraction::ZERO
}

pub fn one() -> Value {
    Fraction::ONE
}