use std::path::PathBuf;
use sqlite_fsr::command::{sql};
use sqlite_fsr::models::{DBFile};
use sqlite_fsr::run;


#[test]
fn test_COUNT_sql_command_executes_without_error() {
    let args = vec![String::from("/path/to/binary"), String::from("./sample.db"), String::from("\"SELECT COUNT(*) FROM apples\"")];
    let result = run(&args);
    assert!(result.is_ok());
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows() {
    let mut file = DBFile::open(PathBuf::from("./sample.db")).unwrap();
    let result = sql::execute("SELECT COUNT(*) FROM apples", &mut file).unwrap();
    assert_eq!(result.len(), 4);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_2() {
    let mut file = DBFile::open(PathBuf::from("./sample.db")).unwrap();
    let result = sql::execute("SELECT COUNT(*) FROM oranges", &mut file).unwrap();
    assert_eq!(result.len(), 6);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_3() {
    let mut file = DBFile::open(PathBuf::from("./superheroes.db")).unwrap();
    let result = sql::execute("SELECT COUNT(*) FROM superheroes", &mut file).unwrap();
    assert_eq!(result.len(), 6895);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_4() {
    let mut file = DBFile::open(PathBuf::from("./companies.db")).unwrap();
    let result = sql::execute("SELECT COUNT(*) FROM companies", &mut file).unwrap();
    assert_eq!(result.len(), 55991);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values() {
    let mut file = DBFile::open(PathBuf::from("./sample.db")).unwrap();
    let result: Vec<String> = sql::execute("SELECT name FROM apples", &mut file)
                                .unwrap()
                                .iter()
                                .map(|record| record.to_string())
                                .collect();
                
    assert_eq!(result, vec!["Granny Smith", "Fuji", "Honeycrisp", "Golden Delicious"]);
}




#[test]
fn test_COUNT_sql_command_returns_error_when_table_not_found() {
    let mut file = DBFile::open(PathBuf::from("./sample.db")).unwrap();
    let result = sql::execute("SELECT COUNT(*) FROM seafood", &mut file);
    assert!(result.is_err());
}


#[test]
fn test_sql_command_returns_error_when_command_not_supported() {
    let mut file = DBFile::open(PathBuf::from("./sample.db")).unwrap();
    let result = sql::execute("WHOOPTY COUNT(*) FROM seafood", &mut file);
    assert!(result.is_err());
}