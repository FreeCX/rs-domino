#![allow(dead_code)]
use crate::consts::GEN_HEADER;
use package::{CreatePackage, Result};
use std::fs::File;
use std::io::Write;

pub fn pack_resource() -> Result<()> {
    // load data
    let bundle = CreatePackage::from_list("resources/resource.list")?;

    // create index for app
    let mut source_code = String::new();
    source_code.push_str(GEN_HEADER);
    for (id, name) in bundle.build_index() {
        source_code.push_str(&format!("pub static {name}: u16 = {id};\n"));
    }
    // and write it to file
    File::create("src/resource.rs").and_then(|mut file| file.write_all(source_code.as_bytes()))?;

    // don't forget to save bundle file
    bundle.pack("resources/resource.package")?;

    Ok(())
}
