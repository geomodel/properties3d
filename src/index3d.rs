use crate::types::*;

//  //  //  //  //  //  //  //
#[allow(dead_code)]
pub fn coord_to_index(coord: &IJK, bounds: &GridConstraints) -> Option<usize> {
    let GridConstraints{ i_max, j_max, k_max, .. } = *bounds;
    let IJK{ i, j, k} = *coord;
    if i >= i_max {
        return None;
    }
    if j >= j_max {
        return None;
    }
    if k >= k_max {
        return None;
    }

    let result = i + (j + k*j_max)*i_max;
    Some( result )
}
#[allow(dead_code)]
pub fn index_to_coord(index: usize, bounds: &GridConstraints) -> Option<IJK> {
    let GridConstraints{ i_max, ij_max, number, .. } = *bounds;
    if index >= number {
        return None;
    }
    let wo_k = index % ij_max;
    let i = wo_k % i_max;
    let j = (wo_k - i)/i_max;
    let k = (index - (i + j*i_max))/ij_max;
    Some( IJK{ i,j,k } )
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod ijk_to_index {
    use super::*;

    #[test]
    fn simple_cube() {
        let i_max = 3;
        let j_max = 5;
        let k_max = 7;
        let bounds = GridConstraints::new( i_max, j_max, k_max );
        let mut index = 0;
        for k in 0..k_max {
            for j in 0..j_max {
                for i in 0..i_max {
                    let coords = IJK{ i,j,k };
                    let try_index = coord_to_index( &coords, &bounds );
                    assert!( try_index != None, "get None for {:?}", coords );
                    assert!( try_index.unwrap() == index,
                        "get try_index={} for {:?} instead of {}", try_index.unwrap(), coords, index );

                    let restored = index_to_coord(index, &bounds);
                    assert!( restored.is_some(), "get restored=None for {}", index );
                    assert!( restored.unwrap() == coords,
                        "get restored={:?} for index={} instead of {:?}", restored, index, coords );

                    index += 1;
                }
            }
        }
    }

    #[test]
    fn i_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let bounds = GridConstraints::new( v_max, 1, 1 );
                let coords = IJK{ i:v, j:0, k:0 };
                let index = coord_to_index(&coords, &bounds);
                assert!( index != None, "get None for v_max={}, v={}", v_max, v );
                assert!( index.unwrap() == v, "get index={} for v_max={}, v={}", index.unwrap(), v_max, v );

                let restored = index_to_coord(index.unwrap(), &bounds);
                assert!( restored.is_some(), "get restored=None for v_max={}, v={}", v_max, v );
                assert!( restored.unwrap() == coords, "get restored={:?} for v_max={}, v={} instead of {:?}", restored, v_max, v, coords );
            }
        }
    }
    #[test]
    fn j_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let bounds = GridConstraints::new( 1, v_max, 1 );
                let coords = IJK{ i:0, j:v, k:0 };
                let index = coord_to_index(&coords, &bounds);
                assert!( index != None, "get None for v_max={}, v={}", v_max, v );
                assert!( index.unwrap() == v );

                let restored = index_to_coord(index.unwrap(), &bounds);
                assert!( restored.is_some(), "get restored=None for v_max={}, v={}", v_max, v );
                assert!( restored.unwrap() == coords, "get restored={:?} for v_max={}, v={} instead of {:?}", restored, v_max, v, coords );
            }
        }
    }
    #[test]
    fn k_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let bounds = GridConstraints::new( 1, 1, v_max );
                let coords = IJK{ i:0, j:0, k:v };
                let index = coord_to_index(&coords, &bounds);
                assert!( index != None, "get None for v_max={}, v={}", v_max, v );
                assert!( index.unwrap() == v );

                let restored = index_to_coord(index.unwrap(), &bounds);
                assert!( restored.is_some(), "get restored=None for v_max={}, v={}", v_max, v );
                assert!( restored.unwrap() == coords, "get restored={:?} for v_max={}, v={} instead of {:?}", restored, v_max, v, coords );
            }
        }
    }

    #[test]
    fn k_bounds_error() {
        let bounds = GridConstraints::new( 1, 1, 1 );
        let coords = IJK{ i:0, j:0, k:1 };
        assert!( coord_to_index( &coords, &bounds) == None );
    }
    #[test]
    fn j_bounds_error() {
        let bounds = GridConstraints::new( 1, 1, 1 );
        let coords = IJK{ i:0, j:1, k:0 };
        assert!( coord_to_index( &coords, &bounds) == None );
    }
    #[test]
    fn i_bounds_error() {
        let bounds = GridConstraints::new( 1, 1, 1 );
        let coords = IJK{ i:1, j:0, k:0 };
        assert!( coord_to_index( &coords, &bounds) == None );
    }
}
