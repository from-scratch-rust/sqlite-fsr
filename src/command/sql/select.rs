
use std::fs::File;
use std::io::{SeekFrom, Seek, Read};
use crate::command::sql::parser::sql_statement::{SelectStatement, AggregatorFunction};
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
    let mut table_rows = table_page.to_table_rows(&statement, &entry.sql);
    if let Some(aggregator_function) = statement.aggregator_function {
        table_rows = aggregate_table_rows(table_rows, aggregator_function)
    }
    return table_rows;
}



pub fn aggregate_table_rows(table_rows: Vec<Record>, aggregator_function: AggregatorFunction) -> Vec<Record>{
    let mut aggregated_rows: Vec<Record> = Vec::new();
    match aggregator_function {
        AggregatorFunction::COUNT => {
            let aggegated_row_count = table_rows.len();
            let row_id = 1;
            let column_headers = vec![aggegated_row_count.to_be_bytes().len() as i64];
            let column_values = vec![aggegated_row_count.to_string().as_bytes().to_vec()];
            let record = Record { row_id, column_headers, column_values };
            aggregated_rows.push(record);
        }
        _ => panic!()
    }
    return aggregated_rows;
}