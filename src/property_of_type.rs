use anyhow::Result;

//  //  //  //  //  //  //  //
pub struct Property<T>
where
    T: std::str::FromStr + std::fmt::Display,
{
    data: Box<[Option<T>]>,
}

impl<T: std::str::FromStr + std::fmt::Display> Property<T> {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::<Option<T>>::with_capacity(size);
        for _ in 0..size {
            v.push(None);
        }
        Self {
            data: v.into_boxed_slice(),
        }
    }
    pub fn from_data(data: Box<[Option<T>]>) -> Self {
        Self { data }
    }
    pub fn from_file(file_name: &str, size: usize, udnef_value: &str) -> Result<Self> {
        let data = io3d::load_property(file_name, size, udnef_value)?;
        Ok(Self { data })
    }
    pub fn save_to_file(&self, file_name: &str, udnef_value: &str) -> Result<()> {
        io3d::save_property(file_name, &self.data, udnef_value)?;
        Ok(())
    }
}

impl<T: std::str::FromStr + std::fmt::Display> std::ops::Index<usize> for Property<T> {
    type Output = Option<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl<T: std::str::FromStr + std::fmt::Display> std::ops::IndexMut<usize> for Property<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod property_of_type {
    use super::*;

    #[test]
    fn create_new() {
        let size = 111;
        let mut prop = Property::<u64>::new(size);

        assert!(prop.data.len() == size);
        for i in 0..size {
            assert!(prop[i] == None);
        }
        for i in 0..size {
            prop[i] = Some(i as u64);
        }
        for i in 0..size {
            assert!(prop[i] == Some(i as u64));
        }
    }

    #[test]
    fn get_values() {
        let ar = [None, Some(1), Some(2), None];
        let prop = Property::from_data(ar.into());

        assert!(prop[0] == None);
        assert!(prop[1] == Some(1));
        assert!(prop[2] == Some(2));
        assert!(prop[3] == None);
    }

    #[test]
    fn set_values() {
        let ar = [None, Some(1), Some(2), None];
        let mut prop = Property::from_data(ar.into());

        assert!(prop[0] == None);
        assert!(prop[1] == Some(1));
        assert!(prop[2] == Some(2));
        assert!(prop[3] == None);

        prop[1] = None;
        prop[0] = Some(3);
        assert!(prop[0] == Some(3));
        assert!(prop[1] == None);
        assert!(prop[2] == Some(2));
        assert!(prop[3] == None);
    }
}
