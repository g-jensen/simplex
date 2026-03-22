use fraction::{ConstZero, Fraction};

use crate::simplex::{Value, tabular::primal::mvalue::{self as sut}};

// reconcile with other frac fn.
fn frac(n: u64, d: u64) -> Fraction {
    Fraction::new(n,d)
}

pub fn mvalue_from(n: u64, d: u64) -> sut::MValue {
    sut::MValue::from(frac(n,d))
}

pub fn mvalue_from_m(finite: Value, m: Value) -> sut::MValue {
    sut::MValue::from_m(finite,m)
}

#[test]
fn mvalue_constructs_from_fraction() {
    let value = sut::MValue::from(frac(1,2));
    assert_eq!(sut::MValue{
        finite: frac(1,2),
        m: Fraction::ZERO
    }, value);

    let value = sut::MValue::from(frac(2,3));
    assert_eq!(sut::MValue{
        finite: frac(2,3),
        m: Fraction::ZERO
    }, value);
}

#[test]
fn mvalue_constructs_from_fraction_and_m() {
    let value = sut::MValue::from_m(frac(1,2),frac(3,4));
    assert_eq!(sut::MValue{
        finite: frac(1,2),
        m: frac(3,4)
    }, value);

    let value = sut::MValue::from_m(frac(2,3),frac(4,5));
    assert_eq!(sut::MValue{
        finite: frac(2,3),
        m: frac(4,5)
    }, value);
}

#[test]
fn mvalue_add_finite() {
    let f1 = sut::MValue::from(frac(1,2));
    let f2 = sut::MValue::from(frac(2,3));
    let prod = sut::MValue::from(frac(7,6));
    assert_eq!(prod, f1+f2);

    let f1 = sut::MValue::from(frac(5,2));
    let f2 = sut::MValue::from(frac(3,2));
    let prod = sut::MValue::from(frac(8,2));
    assert_eq!(prod, f1+f2);
}

#[test]
fn mvalue_add_finite_and_m() {
    let f1 = sut::MValue::from_m(frac(1,2), frac(3,4));
    let f2 = sut::MValue::from_m(frac(5,6),frac(7,8));
    let prod = sut::MValue::from_m(frac(8,6),frac(13,8));
    assert_eq!(prod, f1+f2);
}

#[test]
fn mvalue_calculates_negative_of_finite() {
    let value = sut::MValue::from(frac(1,2));
    assert_eq!(sut::MValue{
        finite: -frac(1,2),
        m: Fraction::ZERO
    }, -value);

    let value = sut::MValue::from(frac(2,3));
    assert_eq!(sut::MValue{
        finite: -frac(2,3),
        m: Fraction::ZERO
    }, -value);
}

#[test]
fn mvalue_calculates_negative_of_m() {
    let value = sut::MValue::from_m(Fraction::ZERO,frac(3,4));
    assert_eq!(sut::MValue{
        finite: Fraction::ZERO,
        m: -frac(3,4)
    }, -value);

    let value = sut::MValue::from_m(Fraction::ZERO,frac(4,5));
    assert_eq!(sut::MValue{
        finite: Fraction::ZERO,
        m: -frac(4,5)
    }, -value);
}

#[test]
fn mvalue_formats_finite() {
    let value = sut::MValue::from(frac(1,2));
    assert_eq!("1/2", value.to_string());

    let value = sut::MValue::from(frac(2,4));
    assert_eq!("1/2", value.to_string());

    let value = sut::MValue::from(-frac(6,9));
    assert_eq!("-2/3", value.to_string());
}

#[test]
fn mvalue_formats_m() {
    let value = sut::MValue::from_m(Fraction::ZERO,frac(1,2));
    assert_eq!("1/2M", value.to_string());

    let value = sut::MValue::from_m(Fraction::ZERO,frac(2,4));
    assert_eq!("1/2M", value.to_string());

    let value = sut::MValue::from_m(Fraction::ZERO,-frac(6,9));
    assert_eq!("-2/3M", value.to_string());
}

#[test]
fn mvalue_formats_finite_and_m() {
    let value = sut::MValue::from_m(frac(1,2),frac(3,4));
    assert_eq!("1/2 + 3/4M", value.to_string());

    let value = sut::MValue::from_m(frac(1,2),-frac(3,4));
    assert_eq!("1/2 - 3/4M", value.to_string());

    let value = sut::MValue::from_m(-frac(1,2),frac(3,4));
    assert_eq!("-1/2 + 3/4M", value.to_string());

    let value = sut::MValue::from_m(-frac(1,2),-frac(3,4));
    assert_eq!("-1/2 - 3/4M", value.to_string());
}

#[test]
fn mvalue_mul_finite() {
    let f1 = sut::MValue::from(frac(1,2));
    let f2 = Value::from(frac(2,3));
    let prod = sut::MValue::from(frac(1,3));
    assert_eq!(prod, f1*f2);

    let f1 = sut::MValue::from(frac(5,2));
    let f2 = Value::from(frac(3,2));
    let prod = sut::MValue::from(frac(15,4));
    assert_eq!(prod, f1*f2);
}

#[test]
fn mvalue_mul_finite_and_m() {
    let a = frac(1,2);
    let b = frac(3,4);
    let c = frac(5,6);
    let f1 = sut::MValue::from_m(a,b);
    let f2 = Value::from(c);
    let prod = sut::MValue::from_m(a*c,b*c);
    assert_eq!(prod, f1*f2);
}

#[test]
fn mvalue_div_finite() {
    let f1 = sut::MValue::from(frac(1,2));
    let f2 = Value::from(frac(2,3));
    let prod = sut::MValue::from(frac(3,4));
    assert_eq!(prod, f1/f2);

    let f1 = sut::MValue::from(frac(5,2));
    let f2 = Value::from(frac(3,2));
    let prod = sut::MValue::from(frac(5,3));
    assert_eq!(prod, f1/f2);
}

#[test]
fn mvalue_div_finite_and_m() {
    let f1 = sut::MValue::from_m(frac(1,2), frac(3,4));
    let f2 = Value::from(frac(5,6));
    let prod = sut::MValue::from_m(frac(6,10),frac(18,20));
    assert_eq!(prod, f1/f2);
}