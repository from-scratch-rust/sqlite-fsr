pub mod utils;
pub mod models;
pub mod command;
use crate::models::error::*;
use crate::command::sql;
use std::path::PathBuf;
use crate::models::DBFile;


fn main()  {
    let args = std::env::args().collect::<Vec<String>>();
    let output = run(&args);
    match output {
        Ok(o) => println!("{}", o),
        Err(e) => println!("ERROR: {}", e)
    }
    
}




fn run(args: &[String]) -> Result<String, RunError> {
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

    let output = match command[0] {
                        ".dbinfo" => {
                            let (page_size, table_count) = file.get_dbinfo();
                            Ok(format!("database page size: {}\nnumber of tables: {}", page_size, table_count))
                        }
                        ".tables" => {
                            let tables = file.get_table_names();
                            Ok(format!("{}", tables.join(" ")))
                        }
                        "SELECT" => {
                            let result = file.execute(command);
                            match result {
                                Ok(rows) => Ok(format!("{}", rows)),
                                Err(e) => Err(e)?
                            }
                        }
                        _ => Err(CommandArgsError::InvalidCommand(command[0].to_owned()))?
                    };
    return output;
}







#[test]
fn test_run_fails_when_missing_all_args() {
    let args = vec![String::new()];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::MissingArgs)));
}

#[test]
fn test_run_fails_when_missing_one_args() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::MissingCommand)));
}

#[test]
fn test_run_fails_when_invalid_command() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from(".dbpictures")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::InvalidCommand(_))));
}

#[test]
fn test_run_succeeds_when_valid_command() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from(".dbinfo")];
    let result = run(&args);
    assert!(result.is_ok());
}


#[test]
fn test_run_returns_correct_output() {
    let args = vec![String::new(), String::from("./tests/assets/sample.db"), String::from("SELECT * FROM apples;")];
    let result = run(&args).unwrap();
    assert_eq!(result, String::from("1 Granny Smith Light Green\n2 Fuji Red\n3 Honeycrisp Blush Red\n4 Golden Delicious Yellow\n"));
}


#[test]
fn test_run_fails_when_invalid_filepath() {
    let args = vec![String::new(), String::from("../tests/assets/fake/index.db"), String::from(".dbinfo")];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::Io(_))));
}


