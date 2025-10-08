pub mod select;
pub use select::select;

use std::fs::File;
use std::io::{SeekFrom, Seek, Read};
use crate::models::schema::*;
use crate::models::tablepage::*;
use crate::models::error::SQLCommandError;
use crate::models::record::Record;

pub fn execute(command_components: Vec<&str>, schema_data: &SchemaRAW, file: &mut File) -> Result<Vec<Record>, SQLCommandError> {
    let target_table = command_components[command_components.len()-1];
    let target_table_schema_entry: Option<SchemaRow> = schema_data
                                                        .to_schema_rows()
                                                        .into_iter()
                                                        .find(|entry| entry.table_name == target_table);

    let result = match target_table_schema_entry {
                        Some(entry) => {
                            let table_rows = select(command_components, &entry, &schema_data, file);
                            Ok(table_rows)
                        },
                        None => Err(SQLCommandError::UnknownTable(target_table.to_string()))
                    };
    
    return result;
}
