
use std::fs::File;
use std::io::{SeekFrom, Seek, Read};
use crate::command::sql::parser::sql_statement::SelectStatement;
use crate::models::{schema::*, DBFile};
use crate::models::tablepage::*;
use crate::models::record::Record;


pub fn select(statement: SelectStatement, entry: &SchemaRow, file: &mut DBFile) -> Vec<Record> {
    let page_size = file.schema.page_size;
    file.seek(SeekFrom::Start(page_size as u64 * (entry.rootpage-1)  as u64)).expect("seek failed");
    let mut table_page_buf = vec![0; file.schema.page_size as usize];
    file.read_exact(&mut table_page_buf).expect("failed to read row size for table");
    let mut table_page: TablePage = match table_page_buf[0] {
                                        0x0D => TablePage::Leaf(LeafTablePage::from_bytes(&table_page_buf)),
                                        0x05 => TablePage::Interior(InteriorTablePage::from_bytes(&table_page_buf, file)),
                                        _    => panic!("unsupported page type"),
                                    };
    let table_rows = table_page.to_table_rows(&statement, &entry.sql);
    return table_rows;
}