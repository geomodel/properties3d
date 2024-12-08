use crate::{Continuous, Discrete};





pub trait IsDefined {
    fn is_defined(&self, index: usize) -> bool {
        false
    }
}
pub trait DiscreteProperty {
    fn disc_value(&mut self, index: usize) -> &Option<Discrete>;
}
pub trait ContinuousProperty {
    fn cont_value(&mut self, index: usize) -> &Option<Continuous>;
}
