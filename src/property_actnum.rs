use anyhow::Result;



//  //  //  //  //  //  //  //
pub struct ActnumProperty {
    data: Box<[bool]>,
}

impl ActnumProperty {
    pub fn new( data: Box<[bool]> ) -> Self {
        Self { data }
    }
}
