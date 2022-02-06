extern crate package;

use std::env;
use std::error::Error;

mod build;
mod consts;
mod error;
mod pack;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "pack")]
    pack::pack_resource()?;

    if env::var("PROFILE")? == "release" {
        build::generate_build_info()?;
    }

    Ok(())
}
