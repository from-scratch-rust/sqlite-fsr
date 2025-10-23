
use std::iter::Peekable;

use crate::{command::sql::parser::{sql_token::{Symbol, Tokenize}, SQLToken}, models::error::SQLSyntaxError};

pub enum SQLStatement {
    Select(SelectStatement),
    CreateTable(CreateTableStatement),
}

#[derive(Debug)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub columns: Vec<String>
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
        let mut columns: Vec<String> = columns_defintions.iter()
                                        .map(|column_defintion| column_defintion[0].clone())
                                        .collect();
        

        Self { table_name, columns }
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
                        Some(SQLToken::Symbol(Symbol::Comma)) => { /* comma consumed, break to next column */ break; }
                        _ => break
                    }
                }
                column_definitions.push(column_defintion_components);
            }

        }else { panic!() }

        return column_definitions;
    }

}

pub struct SelectStatement {
    pub columns: Option<Vec<String>>,
    pub table_name: String,
    pub where_clause: Option<Condition>
}

impl SelectStatement {
    pub fn from_tokens(tokens: Vec<SQLToken>) -> Self {
        let mut tokens_cursor = tokens.into_iter().peekable();

        if let Some(SQLToken::Keyword(first_word)) = tokens_cursor.nth(0) { assert!(first_word == "SELECT") }
        else { panic!() }
                
        tokens_cursor.next_if(|t| matches!(t, SQLToken::Symbol(Symbol::LeftParenthesis)));
        let columns: Option<Vec<String>> = if let Some(SQLToken::Identifier(column)) = tokens_cursor.peek() {
                                                match column.as_str() {
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

        Self { columns, table_name, where_clause: None }
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

pub struct Condition {
    pub left: String,
    pub operator: String,
    pub right: String
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