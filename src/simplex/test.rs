use fraction::{ConstZero, Fraction};

use crate::simplex::value::Value;

pub fn frac(n: u64, d: u64) -> Value {
    Fraction::new(n,d)
}