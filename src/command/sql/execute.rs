use std::fs::File;
use crate::models::{schema::*, DBFile};
use crate::models::error::SQLCommandError;
use crate::models::record::Record;

use crate::command::sql;

pub fn execute(command_components: Vec<&str>, file: &mut DBFile) -> Result<Vec<Record>, SQLCommandError> {
    let target_table = command_components[command_components.len()-1];
    let target_table_schema_entry: SchemaRow = file.schema
                                                .to_schema_rows()
                                                .into_iter()
                                                .find(|entry| entry.table_name == target_table)
                                                .ok_or_else(|| SQLCommandError::UnknownTable(target_table.to_string()))?; 

                                                    
    let result = match command_components[0] {
                        "SELECT" => Ok(sql::select(command_components, &target_table_schema_entry, file)),
                        _ => Err(SQLCommandError::UnsupportedCommand(command_components[0].to_string()))

                    };
        
    return result;
}


