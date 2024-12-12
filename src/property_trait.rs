
//  //  //  //  //  //  //  //
#[allow(dead_code)]
pub(crate) trait IProperty {
    type Value;

    fn array(&self) -> &Box<[Self::Value]>;
    fn array_mut(&mut self) -> &mut Box<[Self::Value]>;

    fn get(&self, index: usize) -> &Self::Value {
        &self.array()[index]
    }
    fn get_mut(&mut self, index: usize) -> &mut Self::Value {
        &mut self.array_mut()[index]
    }
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod actnum {
    use super::*;

    struct SomeProperty {
        data: Box<[bool]>,
    }
    impl IProperty for SomeProperty {
        type Value = bool;

        fn array(&self) -> &Box<[Self::Value]> {
            &self.data
        }
        fn array_mut(&mut self) -> &mut Box<[Self::Value]> {
            &mut self.data
        }
    }

    #[test]
    fn create_immutable() {
        let ar = [true, false, true, false];

        let imm = SomeProperty{ data: ar.into() };
        assert!( *imm.get(0) == true );
    }
    #[test]
    fn create_mutable() {
        let ar = [true, false, true, false];

        let mut mm = SomeProperty{ data: ar.into() };
        assert!( *mm.get(0) == true );
        *mm.get_mut(0) = false;
        assert!( *mm.get(0) == false );
    }
}
