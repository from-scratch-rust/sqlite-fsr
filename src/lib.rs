pub mod varint;
pub mod schema;
pub mod error;
pub mod tablepage;
pub mod command;
pub mod record;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::convert::TryInto;

use crate::schema::*;
use crate::varint::parse_varint;
use crate::error::*;
use crate::command::*;

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
    
    let mut file = match File::open(db_path) {
                        Ok(file) => file,
                        Err(e) => return Err(CommandArgsError::Io(e))?
                    };

    let raw_schema = extract_raw_schema_data(&mut file);
    let output = match command[0] {
                        ".dbinfo" => {
                            let (page_size, table_count) = dbinfo(&raw_schema);
                            Ok(format!(
                                "database page size: {}\nnumber of tables: {}",
                                page_size, table_count
                            ))
                        }
                        ".tables" => {
                            let tables = tables(&raw_schema);
                            Ok(format!("{}", tables.join(" ")))
                        }
                        "SELECT" => {
                            let result = sql_command(command, &raw_schema, &mut file);
                            match result {
                                Ok(rows) => Ok(format!("{}", rows.len())),
                                Err(e) => Err(e)?
                            }
                        }
                        _ => Err(CommandArgsError::InvalidCommand(command[0].to_owned()))?
                    };
    return output;
}


