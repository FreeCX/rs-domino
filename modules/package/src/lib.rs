//! Small and simple library to pack your resource in one file
mod create;
mod defs;
mod error;
mod load;

// re-export
pub use create::CreatePackage;
pub use error::{Result, Error};
pub use load::LoadPackage;
