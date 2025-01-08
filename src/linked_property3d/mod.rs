use anyhow::{anyhow, Result};

use io3d::data_types::*;
use crate::*;

pub mod matching;

//  //  //  //  //  //  //  //
pub struct LinkedProperty3D<'a, T>
where
    T: std::str::FromStr + std::fmt::Display,
{
    grid: &'a Grid,
    data: Property<T>,
}

impl<T: std::str::FromStr + std::fmt::Display> LinkedProperty3D<'_, T> {
    pub fn from_property<'g>(data: Property<T>, grid: &'g Grid) -> LinkedProperty3D<'g, T> {
        LinkedProperty3D::<T> { grid, data }
    }
}

impl<T: std::str::FromStr + std::fmt::Display> Into<Property<T>> for LinkedProperty3D<'_, T> {
    fn into(self) -> Property<T> {
        self.data
    }
}

//  //  //  //  //  //  //  //
impl<T: std::str::FromStr + std::fmt::Display> LinkedProperty3D<'_, T> {
    pub fn value(&self, coord: &IJK) -> Result<&Option<T>> {
        let Some(grid_index) = self.grid.coord_to_index(coord) else {
            return Err(anyhow!("unable convert IJK into grid_index in value(..)"));
        };
        Ok(&self.data[grid_index])
    }
    pub fn value_mut(&mut self, coord: &IJK) -> Result<&mut Option<T>> {
        let Some(grid_index) = self.grid.coord_to_index(coord) else {
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
        let grid = Grid::new(3, 3, 1);
        let array = [None, A, A, A, B, A, A, A, A];
        let data = Property::<Discrete>::from_data(array.into());
        let mut property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        assert!(None == *property.value(&IJK{i: 0, j: 0, k: 0})?);
        assert!(B == *property.value(&IJK{i: 1, j: 1, k: 0})?);
        *property.value_mut(&IJK{i: 0, j: 0, k: 0})? = B;
        assert!(B == *property.value(&IJK{i: 0, j: 0, k: 0})?);

        Ok(())
    }
}
