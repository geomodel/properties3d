pub type Continuous = f64;
pub type Discrete = i16;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct IJK {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}

