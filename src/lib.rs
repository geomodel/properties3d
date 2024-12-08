mod property_actnum;
mod property_of_type;

//  //  //  //  //  //  //  //
use anyhow::Result;

pub use io3d::Discrete as Discrete;
pub use io3d::Continuous as Continuous;

pub use property_actnum::*;
use property_of_type::*;


pub fn load_discrete_property( file_name: &str, size: usize ) -> Result<Property<Discrete>> {
    let data = io3d::load_discrete_property(file_name, size)?;
    Ok( Property::from_data( data ) )
}
pub fn load_coutinuous_property( file_name: &str, size: usize ) -> Result<Property<Continuous>> {
    let data = io3d::load_continuous_property(file_name, size)?;
    Ok( Property::from_data( data ) )
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod entry {
    //use super::*;


    //#[test]
    //fn it_works() {
    //}
}
