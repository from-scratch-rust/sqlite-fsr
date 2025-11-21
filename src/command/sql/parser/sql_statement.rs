
use std::iter::Peekable;

use crate::{command::sql::parser::{sql_token::{Symbol, Tokenize}, SQLToken}, models::error::SQLSyntaxError};

pub enum SQLStatement {
    Select(SelectStatement),
    CreateTable(CreateTableStatement),
}

#[derive(Debug)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub columns: Vec<String>,
    pub integer_primary_key_column: Option<usize>
}

impl CreateTableStatement {
    
    pub fn from_tokens(tokens: Vec<SQLToken>) -> Self {
        
        let mut tokens_cursor = tokens.into_iter().peekable();

        if let Some(SQLToken::Identifier(second_word)) = tokens_cursor.nth(1) { assert!(second_word == "TABLE"); } 
        else { panic!(); }

        let mut table_name = String::new();
        if let Some(SQLToken::Identifier(third_word)) = tokens_cursor.next() { table_name = third_word }
        else { panic!(); }

        let columns_defintions = Self::extract_column_definitions(&mut tokens_cursor);
        let integer_primary_key_column = columns_defintions.iter().position(|column_definition| {column_definition.len() >= 4 && (column_definition[1].to_uppercase() == "INTEGER" && column_definition[2].to_uppercase() == "PRIMARY" && column_definition[3].to_uppercase() == "KEY") } );
        let mut columns: Vec<String> = columns_defintions.iter()
                                        .map(|column_defintion| column_defintion[0].clone())
                                        .collect();
        

        Self { table_name, columns, integer_primary_key_column }
    }

    fn extract_column_definitions(tokens_iterator: &mut Peekable<std::vec::IntoIter<SQLToken>>) -> Vec<Vec<String>> {
        let mut column_definitions: Vec<Vec<String>> = Vec::new();
        if let Some(SQLToken::Symbol(Symbol::LeftParenthesis)) = tokens_iterator.next() {

            while !matches!(tokens_iterator.peek(), Some(&SQLToken::Symbol(Symbol::RightParenthesis))) & !matches!(tokens_iterator.peek(), None){
                let mut column_defintion_components: Vec<String> = Vec::new();

                loop {
                    let token = tokens_iterator.next_if(|t| !matches!(t, SQLToken::Symbol(Symbol::RightParenthesis)));
                    match token {
                        Some(SQLToken::Identifier(column_defintion_component)) => column_defintion_components.push(column_defintion_component.to_string()),
                        Some(SQLToken::Symbol(Symbol::Comma)) => { break; }
                        _ => break
                    }
                }
                column_definitions.push(column_defintion_components);
            }

        } else { panic!() }

        return column_definitions;
    }

}

pub struct SelectStatement {
    pub table_name: String,
    pub columns: Option<Vec<String>>,
    pub where_clause: Option<Condition>,
    pub aggregator_function: Option<AggregatorFunction>
}
pub struct Condition {
    pub left: String,
    pub operator: String,
    pub right: String
}

#[derive(Debug, PartialEq)]
pub enum AggregatorFunction {
    COUNT,
    SUM
}

impl SelectStatement {
    pub fn from_tokens(tokens: Vec<SQLToken>) -> Self {
        let mut tokens_cursor = tokens.into_iter().peekable();

        if let Some(SQLToken::Keyword(first_word)) = tokens_cursor.nth(0) { assert!(first_word == "SELECT") }
        else { panic!() }
                
        tokens_cursor.next_if(|t| matches!(t, SQLToken::Symbol(Symbol::LeftParenthesis)));

        let aggregator_function = if let Some(SQLToken::Identifier(token)) = tokens_cursor.peek() {
                                        match token.as_str() {
                                            "COUNT" => {
                                                tokens_cursor.next();
                                                Some(AggregatorFunction::COUNT)
                                            },
                                            "SUM" => {
                                                tokens_cursor.next();
                                                Some(AggregatorFunction::SUM)
                                            },
                                            _ => None
                                        }                
                                  } else { panic!("Was expecting Indetifier token.") };

        tokens_cursor.next_if(|t| matches!(t, SQLToken::Symbol(Symbol::LeftParenthesis)));

        let columns: Option<Vec<String>> = if let Some(SQLToken::Identifier(token)) = tokens_cursor.peek() {
                                                match token.as_str() {
                                                    "*" => {
                                                        tokens_cursor.next();
                                                        None
                                                    },
                                                    _ => Self::extract_columns(&mut tokens_cursor)
                                                }
                                            } else { panic!() };

        tokens_cursor.next_if(|t| matches!(t, SQLToken::Symbol(Symbol::RightParenthesis)));

        if let Some(SQLToken::Keyword(first_word_after_columns)) = tokens_cursor.next() { assert!(first_word_after_columns == "FROM") }
        else { panic!() }

        let table_name = match tokens_cursor.next() {
                            Some(SQLToken::Identifier(tablename)) => tablename,
                            _ => panic!()
                         };

        Self { columns, table_name, where_clause: None, aggregator_function }
    }

    fn extract_columns(tokens_iterator: &mut Peekable<std::vec::IntoIter<SQLToken>>) -> Option<Vec<String>> {
        let mut columns: Vec<String> = Vec::new();

        while !matches!(tokens_iterator.peek(), Some(&SQLToken::Symbol(Symbol::RightParenthesis)))
        & !matches!(tokens_iterator.peek(), Some(&SQLToken::Keyword(_))) 
        & !matches!(tokens_iterator.peek(), None) {

            let token = tokens_iterator.next();
            match token {
                Some(SQLToken::Identifier(column)) => columns.push(column),
                Some(SQLToken::Symbol(Symbol::Comma)) => continue,
                Some(SQLToken::Symbol(Symbol::RightParenthesis)) => break,
                _ => panic!()
            }
            
        }

        match columns.len() {
            0 => return None,
            _ => return Some(columns)
        }
    }
}



pub trait ToSQLStatement {
    fn to_sql_statment(&self) -> Result<SQLStatement, SQLSyntaxError>;
}

impl ToSQLStatement for &str {
    fn to_sql_statment(&self) -> Result<SQLStatement, SQLSyntaxError> {
        let tokens: Vec<SQLToken> = self.tokenize();
                            
        match &tokens[0] {
            SQLToken::Keyword(s) if s == "CREATE" => Ok(SQLStatement::CreateTable(CreateTableStatement::from_tokens(tokens))),
            SQLToken::Keyword(s) if s == "SELECT" => Ok(SQLStatement::Select(SelectStatement::from_tokens(tokens))),            
            _ => Err(SQLSyntaxError::UnexpectedToken(String::new()))
        }
    }
}

impl ToSQLStatement for Vec<&str> {
    fn to_sql_statment(&self) -> Result<SQLStatement, SQLSyntaxError> {
        let joined = self.join(" ");
        joined.as_str().to_sql_statment()
    }
}



pub struct CreateIndexStatement {
    tablename: String,
    columns: Vec<String>
}

impl CreateIndexStatement {
    pub fn from_tokens(tokens: Vec<SQLToken>) -> Self {
        let tablename = String::new();
        let columns = Vec::new();

        CreateIndexStatement { tablename, columns }
    }
}