use crate::defs::{Id, Item, MAGIC};
use crate::error::{OtherError, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct CreatePackage {
    files: Vec<Item>,
    parent: String,
}

impl CreatePackage {
    pub fn new() -> CreatePackage {
        CreatePackage::default()
    }

    pub fn from_list<S: AsRef<Path> + Copy>(filename: S) -> Result<CreatePackage> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let mut package = CreatePackage {
            parent: filename.as_ref().parent().ok_or(OtherError::EmptyParentPath)?.display().to_string(),
            ..Default::default()
        };
        for (index, line) in buffer.lines().enumerate() {
            let mut tokens = line.split_whitespace();
            let identifier = tokens.next().ok_or(OtherError::IncorrectFormat(index + 1))?;
            let filename = tokens.next().ok_or(OtherError::IncorrectFormat(index + 1))?;
            package.add(identifier, filename);
        }
        Ok(package)
    }

    pub fn build_index(&self) -> Vec<(Id, String)> {
        self.files.iter().enumerate().map(|(index, item)| (index as Id, item.id.clone())).collect()
    }

    pub fn add(&mut self, id: &str, filename: &str) {
        self.files.push(Item::new(id, filename));
    }

    pub fn pack<S: AsRef<Path>>(self, filename: S) -> Result<()> {
        // magic (4 bytes)
        let mut header = Vec::from(MAGIC);
        let mut data = Vec::new();

        // items count (4 bytes)
        header.extend_from_slice(&(self.files.len() as u32).to_be_bytes());

        for item in self.files {
            let path: PathBuf = vec![self.parent.clone(), item.file].iter().collect();
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            let size = reader.read_to_end(&mut buffer)?;

            // file size (8 bytes)
            header.extend_from_slice(&(size as u64).to_be_bytes());
            // data (*size* bytes)
            data.append(&mut buffer);
        }

        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&header)?;
        writer.write_all(&data)?;

        Ok(())
    }
}
