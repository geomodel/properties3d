use anyhow::Result;


//  //  //  //  //  //  //  //
static FALSER: bool = false;
impl std::ops::Index<usize> for ActnumProperty {
    type Output = bool;
    fn index(&self, i: usize) -> &bool {
        let Some(arr) = &self.data else {
            return &FALSER;
        };
        &arr[ i ]
    }
}


pub struct ActnumProperty {
    data: Option<Box<[bool]>>,
}

impl ActnumProperty {
    pub fn new( data: Box<[bool]> ) -> Self {
        Self { data: Some(data) }
    }
    pub fn from_file( _file_name: &str ) -> Result<Self> {
        todo!();
    }
}

impl Default for ActnumProperty {
    fn default() -> Self {
        Self { data: None }
    }
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod actnum {
    use super::*;

    #[test]
    fn default_values() {
        let act = ActnumProperty::default();
        assert!( act.data == None );
        for i in 0..1000 {
            assert!( act[i] == false );
        }
    }

    #[test]
    fn create_simple() {
        let ar = [true, false, true, false];
        let act = ActnumProperty::new(ar.into());

        assert!( act.data != None );
        assert!( act[0] == true );
        assert!( act[1] == false );
        assert!( act[2] == true );
        assert!( act[3] == false );
    }
}
