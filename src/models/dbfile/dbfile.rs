use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use crate::models::dbfile::dbtable::Records;
use crate::models::dbfile::schema::SchemaRAW;
use crate::command::sql::parser::sql_statement::SQLStatement;
use crate::models::dbfile::schema::schemarow::SchemaRow;
use crate::command::sql;
use crate::models::dbfile::dbtable::DBTable;
use std::ops::Deref;
use std::ops::DerefMut;
use std::io;
use crate::SQLCommandError;

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


    fn get_table(&mut self, table_name: &String) -> Result<DBTable, SQLCommandError> {
        let target_table_schema_entry: SchemaRow = self.schema
                                                        .to_schema_rows()
                                                        .into_iter()
                                                        .find(|entry| entry.table_name == *table_name)
                                                        .ok_or_else(|| SQLCommandError::UnknownTable(table_name.to_string()))?;

        let mut table = DBTable::new(target_table_schema_entry, self);
        return Ok(table);
    }


    pub fn get_dbinfo(&self) -> (u16, usize) {
        return (self.schema.page_size, self.schema.cells.len());
    }

    pub fn get_table_names(&self) -> Vec<String> {
        let mut table_names: Vec<String>  = Vec::from([]); 
        for schemarow_header in self.schema.to_schema_rows() {
            if !schemarow_header.name.starts_with("sqlite_") { 
                table_names.push(schemarow_header.table_name); 
            }
        }
        return table_names;
    }

    pub fn execute(&mut self, sql_statement: SQLStatement) -> Result<Records, SQLCommandError> {
        match sql_statement {
            SQLStatement::Select(statement) => {
                let mut table = self.get_table(&statement.table_name)?;
                let results = sql::select(&mut table, statement);
                Ok(Records::from(results))
            },
            _ => Err(SQLCommandError::UnsupportedCommand(String::new()))
        }
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