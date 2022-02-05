use crate::defs::*;
use crate::error::{OtherError, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub struct LoadPackage {
    items: Vec<Option<Vec<u8>>>,
}

impl LoadPackage {
    pub fn from_file<S: AsRef<Path>>(filename: S) -> Result<LoadPackage> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        let file_size = reader.read_to_end(&mut buffer)?;

        // magic (4 bytes)
        if buffer.len() < MINIMAL_HEADER_LEN {
            return Err(OtherError::InvalidFileSize.into());
        }
        // check magic
        if &buffer[0..MINIMAL_HEADER_LEN] != MAGIC.as_bytes() {
            return Err(OtherError::InvalidFileFormat.into());
        }

        // files count
        let count_range = MINIMAL_HEADER_LEN..MINIMAL_HEADER_LEN + COUNTER_LEN;
        let count = u32::from_be_bytes(buffer[count_range].try_into()?) as usize;
        let mut sizes = Vec::with_capacity(count);
        let mut total_size = 0;
        for i in 0..count {
            let start = SIZES_LEN * (i + 1);
            let end = start + SIZES_LEN;
            let size = u64::from_be_bytes(buffer[start..end].try_into()?) as usize;
            sizes.push(size);
            if total_size > file_size {
                return Err(OtherError::CorruptedData.into());
            }
            total_size += size;
        }

        let mut skip = MINIMAL_HEADER_LEN + COUNTER_LEN + count * SIZES_LEN;

        // check file size
        if total_size + skip != buffer.len() {
            return Err(OtherError::InvalidFileSize.into());
        }

        // data
        let mut items = Vec::new();
        for size in sizes {
            items.push(Some(Vec::from(&buffer[skip..skip + size])));
            skip += size;
        }

        Ok(LoadPackage { items })
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn take(&mut self, index: usize) -> Option<Vec<u8>> {
        self.items[index].take()
    }
}
