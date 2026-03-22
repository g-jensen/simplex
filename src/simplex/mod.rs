use crate::simplex::value::Value;

#[cfg(test)]
mod test;

pub mod objectivevalue;
pub mod rowvalue;
pub mod tabular;
pub mod value;

pub type Coefficients = Vec<Value>;
pub type Variable = usize;

#[derive(Clone)]
pub enum Operator {
    LESSTHANEQUAL,
    EQUAL,
}

#[derive(Clone)]
pub struct Constraint {
    pub operator: Operator,
    pub coefficients: Coefficients,
    pub bound: Value,
}
