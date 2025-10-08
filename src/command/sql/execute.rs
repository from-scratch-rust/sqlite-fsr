use std::fs::File;
use std::io::{SeekFrom, Seek, Read};
use crate::models::schema::*;
use crate::models::tablepage::*;
use crate::models::error::SQLCommandError;
use crate::models::record::Record;

use crate::command::sql;

pub fn execute(command_components: Vec<&str>, schema_data: &SchemaRAW, file: &mut File) -> Result<Vec<Record>, SQLCommandError> {
    let target_table = command_components[command_components.len()-1];
    let target_table_schema_entry: SchemaRow= schema_data
                                                        .to_schema_rows()
                                                        .into_iter()
                                                        .find(|entry| entry.table_name == target_table)
                                                        .ok_or_else(|| SQLCommandError::UnknownTable(target_table.to_string()))?; // ← early return if None

                                                    
    let result = match command_components[0] {
                    "SELECT" => Ok(sql::select(command_components, &target_table_schema_entry, &schema_data, file)),
                    _ => Err(SQLCommandError::UnsupportedCommand(command_components[0].to_string()))

                };
    
    return result;
}


