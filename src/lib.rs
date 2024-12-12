mod property_trait;
mod property_of_type;
mod property_actnum;

mod index3d;
mod types;

//  //  //  //  //  //  //  //
//use anyhow::Result;


/*
use property_actnum::*;
use property_of_type::*;


pub fn load_discrete_property( file_name: &str, size: usize ) -> Result<Property<Discrete>> {
    let data = io3d::load_discrete_property(file_name, size)?;
    Ok( Property::from_data( data ) )
}
pub fn load_coutinuous_property( file_name: &str, size: usize ) -> Result<Property<Continuous>> {
    let data = io3d::load_continuous_property(file_name, size)?;
    Ok( Property::from_data( data ) )
}
*/
