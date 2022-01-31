pub static MAGIC: &'static str = "PAKE";
pub static MINIMAL_HEADER_LEN: usize = MAGIC.len();
pub static COUNTER_LEN: usize = 4;
pub static SIZES_LEN: usize = 8;

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub file: String,
}

impl Item {
    pub fn new<S: Into<String>>(id: S, file: S) -> Item {
        Item { id: id.into(), file: file.into() }
    }
}
