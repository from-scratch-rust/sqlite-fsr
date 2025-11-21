use crate::command::sql::parser::sql_statement::CreateTableStatement;

#[derive(Debug)]
pub struct SchemaRow {
    pub object_type: String,        // "table", "index", etc.
    pub name: String,               // object name
    pub table_name: String,         // table the object belongs to
    pub rootpage: i8,               // root b-tree page number
    pub sql: CreateTableStatement   // CREATE statement
}
