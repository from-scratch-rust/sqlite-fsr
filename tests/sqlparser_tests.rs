use sqlite_fsr::command::sql::parser::{sql_statement::{SQLStatement, ToSQLStatement}, sql_token::{ Symbol, ToSQLToken, Tokenize}, SQLToken};

#[test]
fn test_ToSQLToken_converts_string_to_token_correctly() {
    let string = "SELECT";
    let result = string.tokenize();

    assert!(matches!(result[0], SQLToken::Keyword(_)));
}


#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_2() {
    let string = ";";
    let result = string.tokenize();

    assert!(matches!(result[0], SQLToken::Symbol(Symbol::Semicolon)));
}

#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_3() {
    let string = "age";
    let result = string.tokenize();

    assert!(matches!(result[0], SQLToken::Identifier(_)));
}

#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_4() {
    let string = "(name,";
    let result = string.tokenize();

    assert_eq!(result.len(), 3);
    assert!(matches!(result[0], SQLToken::Symbol(Symbol::LeftParenthesis)));
    assert!(matches!(result[1], SQLToken::Identifier(_)));
    assert!(matches!(result[2], SQLToken::Symbol(Symbol::Comma)));
}


#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_5() {
    let string = "SELECT (name, age, weight) FROM people;";
    let result = string.tokenize();

    assert_eq!(result.len(), 11);
    assert!(matches!(result[0], SQLToken::Keyword(_)));
    assert!(matches!(result[1], SQLToken::Symbol(Symbol::LeftParenthesis)));
    assert!(matches!(result[2], SQLToken::Identifier(_)));
    assert!(matches!(result[3], SQLToken::Symbol(Symbol::Comma)));
    assert!(matches!(result[4], SQLToken::Identifier(_)));
    assert!(matches!(result[5], SQLToken::Symbol(Symbol::Comma)));
    assert!(matches!(result[6], SQLToken::Identifier(_)));
    assert!(matches!(result[7], SQLToken::Symbol(Symbol::RightParenthesis)));
    assert!(matches!(result[8], SQLToken::Keyword(_)));
    assert!(matches!(result[9], SQLToken::Identifier(_)));
    assert!(matches!(result[10], SQLToken::Symbol(Symbol::Semicolon)));
}

#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_6() {
    let string = "(name,";
    let result = string.tokenize();

    assert_eq!(result.len(), 3);
    match &result[1] {
        SQLToken::Identifier(identifier) => assert_eq!(identifier, "name"),
        _ => panic!("Expected CreateTable statement"),

    }
}

#[test]
fn test_ToSQLToken_converts_string_to_token_correctly_7() {
    let string = "name TEXT);";
    let result = string.tokenize();

    assert_eq!(result.len(), 4);
    match &result[0] {
        SQLToken::Identifier(identifier) => assert_eq!(identifier, "name"),
        _ => panic!("Expected CreateTable statement"),
    }
}

#[test]
fn test_ToSQLStatement_converts_string_to_statement_correct_1() {
    let string = "SELECT (name, age, weight) FROM people;";
    let result = string.to_sql_statment().unwrap();

    assert!(matches!(result, SQLStatement::Select(_)))

}

#[test]
fn test_ToSQLStatement_converts_string_to_statement_correctly_2() {
    let string = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
    let result = string.to_sql_statment().unwrap();
    
    assert!(matches!(result, SQLStatement::CreateTable(_)))
}

#[test]
fn test_ToSQLStatement_extracts_tablename_from_CREATE_statement_correctly() {
    let string = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
    let result = string.to_sql_statment().unwrap();
    match result {
        SQLStatement::CreateTable(statement) => assert_eq!(statement.table_name, "users"),
        _ => panic!("Expected CreateTable statement"),
    }
}



#[test]
fn test_ToSQLStatement_extracts_columns_from_CREATE_statement_correctly() {
    let string = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
    let result = string.to_sql_statment().unwrap();
    match result {
        SQLStatement::CreateTable(statement) => assert_eq!(statement.columns, vec!["id", "name"]),
        _ => panic!("Expected CreateTable statement"),
    }
}

#[test]
fn test_ToSQLStatement_extracts_columns_from_CREATE_statement_correctly_2() {
    let string = "CREATE TABLE users ( id INTEGER PRIMARY KEY, name TEXT );";
    let result = string.to_sql_statment().unwrap();
    match result {
        SQLStatement::CreateTable(statement) => assert_eq!(statement.columns, vec!["id", "name"]),
        _ => panic!("Expected CreateTable statement"),
    }
}


#[test]
fn test_ToSQLStatement_extracts_columns_from_SELECT_statement_correctly() {
    let string = "SELECT (name, age, weight) FROM people;";
    let result = string.to_sql_statment().unwrap();
    match result {
        SQLStatement::Select(statement) => match statement.columns {
                                                Some(columns) => assert_eq!(columns, vec!["name", "age", "weight"]),
                                                None => panic!("Columns not properly extracted from SELECT statment")
                                            },
        _ => panic!("Expected CreateTable statement"),
    }
}

#[test]
fn test_ToSQLStatement_extracts_columns_from_SELECT_statement_correctly_2() {
    let string = "SELECT * FROM people;";
    let result = string.to_sql_statment().unwrap();
    match result {
        SQLStatement::Select(statement) => assert_eq!(statement.columns, None),
        _ => panic!("Expected CreateTable statement"),
    }
}
