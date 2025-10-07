use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


use crate::schema::*;
use crate::tablepage::*;
use crate::error::*;
use crate::record::Record;

pub fn dbinfo(schema_data: &SchemaRAW) -> (u16, usize) {
    return (schema_data.page_size, schema_data.cells.len());
}


pub fn tables(schema_data: &SchemaRAW) -> Vec<String> {
    let mut table_names: Vec<String>  = Vec::from([]); 
    for schemarow_header in schema_data.to_schema_rows() {
        if !schemarow_header.name.starts_with("sqlite_") { 
            table_names.push(schemarow_header.table_name); 
        }
    }
    return table_names;
}


pub fn sql_command(command_components: Vec<&str>, schema_data: &SchemaRAW, file: &mut File) -> Result<Vec<Record>, SQLCommandError> {
    let target_table = command_components[command_components.len()-1];
    let target_table_schema_entry: Option<SchemaRow> = schema_data
                                                        .to_schema_rows()
                                                        .into_iter()
                                                        .find(|entry| entry.table_name == target_table);

    let result = match target_table_schema_entry {
                        Some(entry) => {
                            file.seek(SeekFrom::Start(schema_data.page_size as u64 * (entry.rootpage-1)  as u64)).expect("seek failed");
                            let mut table_page_buf = vec![0; schema_data.page_size as usize];
                            file.read_exact(&mut table_page_buf).expect("failed to read row size for table");
                            let mut table_page: TablePage = match table_page_buf[0] {
                                                                0x0D => TablePage::Leaf(LeafTablePage::from_bytes(&table_page_buf)),
                                                                0x05 => TablePage::Interior(InteriorTablePage::from_bytes(&table_page_buf, file)),
                                                                _    => panic!("unsupported page type"),
                                                            };
                            let table_rows = table_page.to_table_rows();
                            Ok(table_rows)
                        },
                        None => Err(SQLCommandError::UnknownTable(target_table.to_string()))
                    };
    
    return result;
}
