#[cfg(test)]
mod test;

pub mod objectivevalue;
pub mod rowvalue;
pub mod tabular;
pub mod value;

use crate::simplex::rowvalue::RowValue;

pub type Coefficients<R> = Vec<R>;
pub type Variable = usize;

#[derive(Clone)]
pub enum Operator {
    LESSTHANEQUAL,
    EQUAL,
}

#[derive(Clone)]
pub struct Constraint<R: RowValue> {
    pub operator: Operator,
    pub coefficients: Coefficients<R>,
    pub bound: R,
}
