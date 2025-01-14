use anyhow::{anyhow, Result};
use std::rc::Rc;

use io3d::types3d::*;
use grid3d::IndexFromCoord3D as Grid;
use crate::*;

pub mod matching;

//  //  //  //  //  //  //  //
//#[derive(Debug)]
pub struct LinkedProperty3D<T>
where
    T: std::str::FromStr + std::fmt::Display,
{
    pub grid: Rc<dyn Grid>,
    pub data: Property<T>,
}

impl<T: std::str::FromStr + std::fmt::Display> LinkedProperty3D<T> {
    pub fn from_property(data: Property<T>, grid: Rc<dyn Grid>) -> Self {
        Self { grid, data }
    }
}

impl<T: std::str::FromStr + std::fmt::Display> Into<Property<T>> for LinkedProperty3D<T> {
    fn into(self) -> Property<T> {
        self.data
    }
}

//  //  //  //  //  //  //  //
impl<T: std::str::FromStr + std::fmt::Display> LinkedProperty3D<T> {
    pub fn value(&self, coord: &IJK) -> Result<&Option<T>> {
        let Some(grid_index) = self.grid.index_from(&coord) else {
            return Err(anyhow!("unable convert IJK into grid_index in value(..)"));
        };
        Ok(&self.data[grid_index])
    }
    pub fn value_mut(&mut self, coord: &IJK) -> Result<&mut Option<T>> {
        let Some(grid_index) = self.grid.index_from(&coord) else {
            return Err(anyhow!("unable convert IJK into grid_index in value_mut(..)"));
        };
        Ok(&mut self.data[grid_index])
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_write {
    use super::*;
    static A: Option<Discrete> = Some(1);
    static B: Option<Discrete> = Some(2);

    #[test]
    fn check_1() -> Result<()> {
        let grid = LightGrid::new_rc(3, 3, 1);
        let array = [None, A, A, A, B, A, A, A, A];
        let data = Property::<Discrete>::from_data(array.into());
        let mut property = LinkedProperty3D::<Discrete>::from_property(data, grid);

        assert!(None == *property.value(&IJK{i: 0, j: 0, k: 0})?);
        assert!(B == *property.value(&IJK{i: 1, j: 1, k: 0})?);
        *property.value_mut(&IJK{i: 0, j: 0, k: 0})? = B;
        assert!(B == *property.value(&IJK{i: 0, j: 0, k: 0})?);

        Ok(())
    }
}
