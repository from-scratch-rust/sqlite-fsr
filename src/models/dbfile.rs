use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use crate::models::schema::SchemaRAW;
use std::ops::Deref;
use std::ops::DerefMut;
use std::io;
pub struct DBFile {
    pub file: File,
    pub schema: SchemaRAW
}

impl DBFile {
    pub fn open(path: PathBuf) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let schema = Self::extract_raw_schema_data(&mut file);
        Ok(Self { file, schema })
    }


    fn extract_raw_schema_data<R: Read + Seek>(file: &mut R) -> SchemaRAW {
        // Read the 2-byte page size at offset 16
        let mut page_size_buffer = [0u8; 2];
        file.seek(SeekFrom::Start(16)).expect("seek failed");
        file.read_exact(&mut page_size_buffer).expect("failed to read page size");
        let page_size = u16::from_be_bytes(page_size_buffer);

        // Read the full schema page
        let mut schema_raw_buffer = vec![0; page_size as usize];
        file.seek(SeekFrom::Start(0)).expect("seek failed");
        file.read_exact(&mut schema_raw_buffer).expect("failed to read schema page");

        SchemaRAW::from_bytes(&schema_raw_buffer)
    }
}

impl Deref for DBFile {
    type Target = File;
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for DBFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}