use anyhow::Result;

use data_types::IJK;

//  //  //  //  //  //  //  //
pub struct UpscdProperty<T>
where
    T: std::str::FromStr,
{
    data: Box<[(IJK, T)]>,
}

impl<T: std::str::FromStr + Clone> UpscdProperty<T> {
    pub fn from_data(data: Box<[(IJK, T)]>) -> Self {
        Self { data }
    }
    pub fn from_file(file_name: &str) -> Result<Self> {
        let data = io3d::load_bw(file_name)?;
        Ok(Self { data })
    }

    pub fn get_by_coord(&self, coord: &IJK) -> Option<T> {
        for item in &self.data {
            if item.0 == *coord {
                return Some(item.1.clone());
            }
        }
        None
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn get_via_index(&self, index: usize) -> &(IJK, T) {
        &self.data[index]
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod ups_property_of_type {
    use super::*;

    #[test]
    fn check_idex() {
        let a = (IJK { i: 1, j: 1, k: 1 }, 111);
        let b = (IJK { i: 2, j: 2, k: 2 }, 222);
        let c = (IJK { i: 1, j: 2, k: 3 }, 123);
        let data = [a, b, c];
        let bw = UpscdProperty::from_data(Box::new(data));

        assert!(bw.len() == data.len());
        assert!(*bw.get_via_index(1) == b);
    }

    #[test]
    fn create_new() {
        let a = (IJK { i: 1, j: 1, k: 1 }, 111);
        let b = (IJK { i: 2, j: 2, k: 2 }, 222);
        let c = (IJK { i: 1, j: 2, k: 3 }, 123);
        let data = [a, b, c];
        let bw = UpscdProperty::from_data(Box::new(data));

        assert!(bw.get_by_coord(&a.0) == Some(a.1));
        assert!(bw.get_by_coord(&b.0) == Some(b.1));
        assert!(bw.get_by_coord(&c.0) == Some(c.1));
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    let coord = IJK { i, j, k };
                    if coord != a.0 {
                        if coord != b.0 {
                            if coord != c.0 {
                                assert!(bw.get_by_coord(&IJK { i, j, k }) == None);
                            }
                        }
                    }
                }
            }
        }
    }
}
