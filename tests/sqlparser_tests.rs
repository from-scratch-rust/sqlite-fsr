use sqlite_fsr::command::sql::parser::{sql_statement::{SQLStatement, ToSQLStatement}, sql_token::{Symbol, ToSQLToken, Tokenize}, SQLToken};

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