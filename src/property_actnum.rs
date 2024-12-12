use anyhow::Result;

use crate::property_trait::IProperty;


//  //  //  //  //  //  //  //
pub struct ActnumProperty {
    data: Box<[bool]>,
}

impl ActnumProperty {
    pub fn from_data( data: Box<[bool]> ) -> Self {
        Self { data }
    }
    pub fn from_file( file_name: &str, size: usize ) -> Result<Self> {
        let data = io3d::load_actnum( file_name, size )?;
        Ok( Self{ data } )
    }
}

impl IProperty for ActnumProperty {
    type Value = bool;

    fn array(&self) -> &Box<[Self::Value]> {
        &self.data
    }
    fn array_mut(&mut self) -> &mut Box<[Self::Value]> {
        &mut self.data
    }
}

impl std::ops::Index<usize> for ActnumProperty {
    type Output = bool;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod actnum {
    use super::*;

    #[test]
    fn create_simple() {
        let ar = [true, false, true, false];
        let act = ActnumProperty::from_data(ar.into());

        assert!( act[0] == true );
        assert!( act[1] == false );
        assert!( act[2] == true );
        assert!( act[3] == false );
    }
}
