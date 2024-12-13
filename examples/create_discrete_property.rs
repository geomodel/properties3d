use anyhow::Result;
use properties3d::*;

fn main() -> Result<()> {
    let grid = Grid::new(2,2,2);

    let src_file_name = r#"../../properties3d/assets/ijk.ascii"#;
    println!("loading 1..");
    let mut property = Property::<Discrete>::from_file(src_file_name, 8)?;
    println!("loading 2..");
    let property2 = Property::<Continuous>::from_file(src_file_name, 8)?;

    /*
    let bw_file_name = r#"d:\raaDir\ForUserDefined\exchange\bw.ascii"#;
    let _bw1 = load_discrete_bw(bw_file_name)?;
    let _bw2 = load_continuous_bw(bw_file_name)?;

    for value in property.array().iter_mut() {
        if let Some(v) = value {
            *value = Some(*v + 5);
        }
    }
    */

    for index in 0..grid.get_number() {
        if let Some(v) = property[index] {
            property[index] = Some(v + 5);
        }
    }

    let dest_file_name = r#"../../properties3d/assets/result.ascii"#;
    println!("saving 1..");
    property.save_to_file(dest_file_name)?;
    let dest_file_name = r#"../../properties3d/assets/result2.ascii"#;
    println!("saving 2..");
    property2.save_to_file(dest_file_name)?;

    println!("{}", "Ok-k-k!!");
    Ok(())
}
