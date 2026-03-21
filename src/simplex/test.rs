use fraction::{Fraction};

use crate::simplex::value::{Value, ZValue};

pub fn frac(n: u64, d: u64) -> Value {
    Fraction::new(n,d)
}

pub fn zfrac(n: u64, d: u64) -> ZValue {
    ZValue::from(frac(n,d))
}