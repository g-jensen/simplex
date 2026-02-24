use fraction::Fraction;

pub type Value = Fraction;
pub type Coefficients = Vec<Value>;
pub type Variable = usize;

pub struct UpperBoundConstraint {
    pub coefficients: Coefficients,
    pub bound: Value
}