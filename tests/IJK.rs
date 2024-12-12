use anyhow::Result;

use properties3d::*;

#[test]
fn printer() -> Result<()> {
    /*
    let src_file_name = r#"assets/ijk.ascii"#;
    let i_property = load_property<Discrete>(src_file_name, 8)?;

    let ijk_max = IJK{ i:2,j:2,k:2 };

    for k in 0..ijk_max.k {
        for j in 0..ijk_max.j {
            for i in 0..ijk_max.i {
                let arr_ind = ijk_to_array(&IJK{i,j,k},&ijk_max);
                let value = i_property[arr_ind.unwrap()];
                println!("ijk = {}.{}.{} -> {:?} -> {:?}", i, j, k, arr_ind, value);
            }
        }
    }
    */

    Ok(())
}
