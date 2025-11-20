use crate::command::sql::parser::sql_statement::SQLStatement;
use crate::models::{schema::*, DBFile};
use crate::models::error::SQLCommandError;
use crate::models::record::{Record, Records};

use crate::command::sql;

pub fn execute(sql_statement: SQLStatement, file: &mut DBFile) -> Result<Records, SQLCommandError> {

    let records = match sql_statement {
                        SQLStatement::Select(statement) => {
                            let target_table_schema_entry: SchemaRow = file.schema
                                                                        .to_schema_rows()
                                                                        .into_iter()
                                                                        .find(|entry| entry.table_name == statement.table_name)
                                                                        .ok_or_else(|| SQLCommandError::UnknownTable(statement.table_name.to_string()))?;
                            let results = sql::select(statement, &target_table_schema_entry, file);
                            Ok(Records::from(results))
                        },
                        _ => Err(SQLCommandError::UnsupportedCommand(String::new()))
                    };


        
    return records;
}


