
use crate::{command::sql::parser::{sql_token::Tokenize, SQLToken}, models::error::SQLSyntaxError};

pub enum SQLStatement {
    Select(SelectStatement),
    CreateTable(CreateTableStatement),
}

pub struct CreateTableStatement {
    pub columns: Vec<String>,
    pub table_name: String
}

impl CreateTableStatement {
    // pub fn from_tokens(&self, tokens: Vec<SQLToken>) -> Self {
    //     let columns = Vec::new();
    //     let index = 1;
    //     while let SQLToken::Identifier(token) = tokens[index] {
    //         index += 1;
    //     };



        
    // }
}

pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table_name: String,
    pub where_clause: Option<Condition>
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
            SQLToken::Keyword(s) if s == "CREATE" => Ok(SQLStatement::CreateTable(CreateTableStatement {columns: Vec::new(), table_name: String::new()})),
            SQLToken::Keyword(s) if s == "SELECT" => Ok(SQLStatement::Select(SelectStatement {columns: Vec::new(), table_name: String::new(), where_clause: None})),            
            _ => Err(SQLSyntaxError::UexpectedValue(String::new()))
        }
    }
}