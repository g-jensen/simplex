use fraction::{ConstZero, Fraction};

use crate::simplex::tabular::Value;

pub fn frac(n: u64, d: u64) -> Value {
    Value {
        finite: Fraction::new(n,d),
        m: Fraction::ZERO
    }
}