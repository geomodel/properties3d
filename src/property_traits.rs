//use crate::{Continuous, Discrete};





pub trait DiscreteProperty {
    fn disc_value(&mut self, index: usize) -> &Option<Discrete>;
}
pub trait ContinuousProperty {
    fn cont_value(&mut self, index: usize) -> &Option<Continuous>;
}
