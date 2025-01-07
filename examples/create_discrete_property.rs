use anyhow::Result;
use properties3d::*;
use data_types::*;

static U: &str = "-999";

fn main() -> Result<()> {
    let grid = Grid::new(2,2,2);

    let src_file_name = r#"../../properties3d/assets/ijk.ascii"#;
    println!("loading 1..");
    let mut property = Property::<Discrete>::from_file(src_file_name, 8, U)?;
    println!("loading 2..");
    let property2 = Property::<Continuous>::from_file(src_file_name, 8, U)?;

    let bw_file_name = r#"../../properties3d/assets/upscaled_ijk.ascii"#;
    println!("loading 3..");
    let _bw = UpscdProperty::<Continuous>::from_file(bw_file_name)?;

    for index in 0..grid.get_size() {
        if let Some(v) = property[index] {
            property[index] = Some(v + 5);
        }
    }

    let dest_file_name = r#"../../properties3d/assets/result.ascii"#;
    println!("saving 1..");
    property.save_to_file(dest_file_name,  U)?;
    let dest_file_name = r#"../../properties3d/assets/result2.ascii"#;
    println!("saving 2..");
    property2.save_to_file(dest_file_name, U)?;

    println!("{}", "Ok-k-k!!");
    Ok(())
}
