use std::fmt;
use std::ops::{Index, IndexMut};
use crate::models::dbfile::dbtable::tablepage::Record;

#[derive(Debug)]
pub struct TableRow {
    pub row_id: i64,
    pub column_values: Vec<String>
}

impl From<Record> for TableRow {
    fn from(record: Record) -> Self {
        let column_values: Vec<String> = record.column_values.iter()
                                                             .map(|bytes| {
                                                                 if bytes.is_empty() {
                                                                     "NULL".to_string()
                                                                 } else {
                                                                     String::from_utf8_lossy(&bytes).to_string()
                                                                 }
                                                             })
                                                             .collect();
        let tablerow = TableRow { row_id: record.row_id, column_values };
        return tablerow;
    }
}

impl Index<usize> for TableRow {
    type Output = String;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.column_values[index]
    }
}

impl IndexMut<usize> for TableRow {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.column_values[index]
    }
}

impl fmt::Display for TableRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.column_values.len() {
            write!(f, "{}", self.column_values[i])?;
            if i != self.column_values.len() - 1 { 
                write!(f, " ")?; 
            }
        }
        Ok(())
    } 
}

// Wrapper so a vector of `TableRow` can be printed to the console.
// You cannot implement `Display` directly for `Vec<TableRow>` due to Rust's
// orphan rules, so we provide a small newtype wrapper instead.
pub struct TableRows(pub Vec<TableRow>);

impl From<Vec<TableRow>> for TableRows {
    fn from(v: Vec<TableRow>) -> Self {
        TableRows(v)
    }
}

impl From<Vec<Record>> for TableRows {
    fn from(records: Vec<Record>) -> Self {
        let table_rows: Vec<TableRow> = records.into_iter()
            .map(|record| TableRow::from(record))
            .collect();
        TableRows(table_rows)
    }
}

impl fmt::Display for TableRows {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rec in self.0.iter() {
            writeln!(f, "{}", rec)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for TableRows {
    type Target = [TableRow];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TableRows {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
