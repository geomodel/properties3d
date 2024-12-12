pub type Continuous = f64;
pub type Discrete = i16;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct IJK {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GridConstraints {
    pub(crate) i_max: usize,
    pub(crate) j_max: usize,
    pub(crate) k_max: usize,
    pub(crate) ij_max: usize,
    pub(crate) number: usize,
}
impl GridConstraints {
    pub fn new(i_max: usize, j_max: usize, k_max: usize) -> Self {
        let ij_max = i_max*j_max;
        Self {
            i_max, j_max, k_max,
            ij_max,
            number: ij_max*k_max,
        }
    }

    pub fn get_i_max(&self) -> usize {
        self.i_max
    }
    pub fn get_j_max(&self) -> usize {
        self.j_max
    }
    pub fn get_k_max(&self) -> usize {
        self.k_max
    }
    pub fn get_number(&self) -> usize {
        self.number
    }
}
