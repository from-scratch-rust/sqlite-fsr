use std::path::PathBuf;
use sqlite_fsr::command::{sql};
use sqlite_fsr::command::sql::parser::sql_statement::ToSQLStatement;
use sqlite_fsr::models::{DBFile};
use sqlite_fsr::run;


#[test]
fn test_COUNT_sql_command_executes_without_error() {
    let args = vec![String::from("/path/to/binary"), String::from("./tests/assets/sample.db"), String::from("\"SELECT COUNT(*) FROM apples\"")];
    let result = run(&args);
    assert!(result.is_ok());
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT COUNT(*) FROM apples".to_sql_statment().unwrap(), &mut file).unwrap()
                                                                                    .iter()
                                                                                    .map(|record| record.to_string())
                                                                                    .collect();
    assert_eq!(result, vec!["4"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_2() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT COUNT(*) FROM oranges".to_sql_statment().unwrap(), &mut file).unwrap()
                                                                                    .iter()
                                                                                    .map(|record| record.to_string())
                                                                                    .collect();
    assert_eq!(result, vec!["6"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_3() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/superheroes.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT COUNT(*) FROM superheroes".to_sql_statment().unwrap(), &mut file).unwrap()
                                                                                        .iter()
                                                                                        .map(|record| record.to_string())
                                                                                        .collect();
    assert_eq!(result, vec!["6895"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_4() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/companies.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT COUNT(*) FROM companies".to_sql_statment().unwrap(), &mut file).unwrap()
                                                                        .iter()
                                                                        .map(|record| record.to_string())
                                                                        .collect();
    assert_eq!(result, vec!["55991"]);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT name FROM apples".to_sql_statment().unwrap(), &mut file)
                                .unwrap()
                                .iter()
                                .map(|record| record.to_string())
                                .collect();
                
    assert_eq!(result, vec!["Granny Smith", "Fuji", "Honeycrisp", "Golden Delicious"]);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values_2() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT name, color FROM apples".to_sql_statment().unwrap(), &mut file)
                                .unwrap()
                                .iter()
                                .map(|record| record.to_string())
                                .collect();
                
    assert_eq!(result, vec!["Granny Smith Light Green", "Fuji Red", "Honeycrisp Blush Red", "Golden Delicious Yellow"]);
}



#[test]
fn test_SELECT_sql_command_returns_correct_values_3() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT * FROM apples".to_sql_statment().unwrap(), &mut file)
                                .unwrap()
                                .iter()
                                .map(|record| record.to_string())
                                .collect();
                
    assert_eq!(result, vec!["1 Granny Smith Light Green", "2 Fuji Red", "3 Honeycrisp Blush Red", "4 Golden Delicious Yellow"]);
}




#[test]
fn test_COUNT_sql_command_returns_error_when_table_not_found() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result = "SELECT COUNT(*) FROM seafood".to_sql_statment()
                        .map_err(|e| sqlite_fsr::models::error::SQLCommandError::UnsupportedCommand(e.to_string()))
                        .and_then(|stmt| sql::execute(stmt, &mut file));
    assert!(result.is_err());
}



#[test]
fn test_sql_command_returns_error_when_command_not_supported() {
    let mut file = DBFile::open(PathBuf::from("./tests/assets/sample.db")).unwrap();
    let result = "WHOOPTY COUNT(*) FROM seafood".to_sql_statment()
                        .map_err(|e| sqlite_fsr::models::error::SQLCommandError::UnsupportedCommand(e.to_string()))
                        .and_then(|stmt| sql::execute(stmt, &mut file));
    assert!(result.is_err());
}