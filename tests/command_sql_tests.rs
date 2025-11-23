use std::path::PathBuf;
use sqlite_fsr::command::{sql};
use sqlite_fsr::command::sql::parser::sql_statement::ToSQLStatement;
use sqlite_fsr::models::{DBFile};


#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT COUNT(*) FROM apples";
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
    assert_eq!(result, vec!["4"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_2() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT COUNT(*) FROM oranges";
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
    assert_eq!(result, vec!["6"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_3() {
    let mut file = DBFile::open("./tests/assets/superheroes.db").unwrap();
    let sql_statement = "SELECT COUNT(*) FROM superheroes";
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
    assert_eq!(result, vec!["6895"]);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_4() {
    let mut file = DBFile::open("./tests/assets/companies.db").unwrap();
    let sql_statement = "SELECT COUNT(*) FROM companies";    
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
    assert_eq!(result, vec!["55991"]);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT name FROM apples";    
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
                
    assert_eq!(result, vec!["Granny Smith", "Fuji", "Honeycrisp", "Golden Delicious"]);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values_2() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT name, color FROM apples";    

    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
                
    assert_eq!(result, vec!["Granny Smith Light Green", "Fuji Red", "Honeycrisp Blush Red", "Golden Delicious Yellow"]);
}



#[test]
fn test_SELECT_sql_command_returns_correct_values_3() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT * FROM apples";
    let result: Vec<String> = file.execute(sql_statement).unwrap()
                                                         .iter()
                                                         .map(|record| record.to_string())
                                                         .collect();
                
    assert_eq!(result, vec!["1 Granny Smith Light Green", "2 Fuji Red", "3 Honeycrisp Blush Red", "4 Golden Delicious Yellow"]);
}


#[test]
fn test_COUNT_sql_command_returns_error_when_table_not_found() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let result = file.execute("SELECT COUNT(*) FROM seafood");
    assert!(result.is_err());
}



#[test]
fn test_sql_command_returns_error_when_command_not_supported() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let result = file.execute("WHOOPTY COUNT(*) FROM seafood");
    assert!(result.is_err());
}