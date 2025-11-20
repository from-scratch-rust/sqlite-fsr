pub mod utils;
pub mod models;
pub mod command;
use std::path::PathBuf;
use crate::models::error::*;
use crate::command::sql;
use crate::command::sql::parser::sql_statement::ToSQLStatement;
// use crate::command::tables;
// use crate::command::dbinfo;
use crate::models::DBFile;

pub fn run(args: &[String]) -> Result<String, RunError> {
    if args.len() <= 1 {
        return Err(CommandArgsError::MissingArgs)?;
    }
    if args.len() == 2 {
        return Err(CommandArgsError::MissingCommand)?;
    }

    let db_path = PathBuf::from(&args[1]);
    let command: Vec<&str> = args[2]
                                .trim_matches('"')
                                .split(" ")
                                .collect();
    
    let mut file = match DBFile::open(db_path) {
                        Ok(file) => file,
                        Err(e) => return Err(CommandArgsError::Io(e))?
                    };

    let raw_schema = &file.schema;
    let output = match command[0] {
                        ".dbinfo" => {
                            let (page_size, table_count) = file.get_dbinfo();
                            Ok(format!(
                                "database page size: {}\nnumber of tables: {}",
                                page_size, table_count
                            ))
                        }
                        ".tables" => {
                            let tables = file.get_table_names();
                            Ok(format!("{}", tables.join(" ")))
                        }
                        "SELECT" => {
                            let sql_statement = command.to_sql_statment()
                                                        .map_err(|err| SQLCommandError::UnsupportedCommand(err.to_string()))?;

                            let result = file.execute(sql_statement);
                            match result {
                                Ok(rows) => Ok(format!("{}", rows)),
                                Err(e) => Err(e)?
                            }
                        }
                        _ => Err(CommandArgsError::InvalidCommand(command[0].to_owned()))?
                    };
    return output;
}


