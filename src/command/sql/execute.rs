use crate::command::sql::parser::sql_statement::{SQLStatement, ToSQLStatement};
use crate::models::{schema::*, DBFile};
use crate::models::error::SQLCommandError;
use crate::models::record::Record;

use crate::command::sql;

pub fn execute(sql_statment_string: &str, file: &mut DBFile) -> Result<Vec<Record>, SQLCommandError> {

    let sql_statement = sql_statment_string.to_sql_statment().unwrap();

    let result = match sql_statement {
                        SQLStatement::Select(statement) => {
                            let target_table_schema_entry: SchemaRow = file.schema
                                                                        .to_schema_rows()
                                                                        .into_iter()
                                                                        .find(|entry| entry.table_name == statement.table_name)
                                                                        .ok_or_else(|| SQLCommandError::UnknownTable(statement.table_name.to_string()))?;
                            Ok(sql::select(statement, &target_table_schema_entry, file))
                        },
                        _ => Err(SQLCommandError::UnsupportedCommand(sql_statment_string.to_string()))
                    };


        
    return result;
}


