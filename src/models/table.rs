use crate::{command::sql::parser::sql_statement::{CreateTableStatement, SelectStatement}, models::{record::Record, DBFile}};


pub struct Table {
    pub description: CreateTableStatement,
    pub file: DBFile
}

impl Table {
    pub fn to_table_rows(&self, statement: SelectStatement) -> Vec<Record> {
        let records: Vec<Record> = Vec::new();
        
        return records;
    }
}