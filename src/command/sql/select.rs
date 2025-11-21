
use crate::command::sql::parser::sql_statement::{SelectStatement, AggregatorFunction};
use crate::models::record::Record;
use crate::models::table::DBTable;

pub fn select(table: &mut DBTable, mut statement: SelectStatement) -> Vec<Record> {
    // take aggregator out of the statement so we can pass the (now non-aggregating) statement
    let aggregator = statement.aggregator_function.take();

    let table_rows = table.to_table_rows(statement);

    if let Some(aggregator_function) = aggregator {
        return aggregate_table_rows(table_rows, aggregator_function);
    }

    table_rows
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