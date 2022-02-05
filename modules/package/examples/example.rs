extern crate package;

use package::{CreatePackage, Result, LoadPackage};

fn main() -> Result<()> {
    let list_file = "examples/resource.list";
    let output_file = "examples/resource.package";

    let res = CreatePackage::from_list(list_file)?;
    println!("Package file index: ");
    for (id, name) in res.build_index() {
        println!(" - {name}: {id}");
    }
    res.pack(output_file)?;

    let mut pack = LoadPackage::from_file(output_file)?;
    println!("Package contain {} item(s)", pack.count());

    let index = 0;
    match pack.take(index) {
        Some(_) => println!("Data resource at index {index} taken!"),
        None => println!("Data resource at index {index} not found!"),
    }
    match pack.take(index) {
        Some(_) => println!("OMG, I have another resource at index {index}!"),
        None => println!("Ok. Index {index} is empty!"),
    }

    Ok(())
}
