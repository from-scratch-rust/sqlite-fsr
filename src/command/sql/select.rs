
use crate::command::sql::parser::sql_statement::{SelectStatement, AggregatorFunction};
use crate::models::dbfile::dbtable::TableRow;
use crate::models::dbfile::table::DBTable;

pub fn select(table: &mut DBTable, mut statement: SelectStatement) -> Vec<TableRow> {

    let records = table.to_table_rows(&statement);
    let mut table_rows: Vec<TableRow> = records.into_iter()
                                               .map(|record| TableRow::from(record))
                                               .collect();
                                            
    let aggregator = statement.aggregator_function.take();
    if let Some(aggregator_function) = aggregator {
        table_rows = aggregate_table_rows(table_rows, aggregator_function);
    }

    table_rows
}


pub fn aggregate_table_rows(table_rows: Vec<TableRow>, aggregator_function: AggregatorFunction) -> Vec<TableRow>{
    let mut aggregated_rows: Vec<TableRow> = Vec::new();
    match aggregator_function {
        AggregatorFunction::COUNT => {
            let aggegated_row_count = table_rows.len();
            let row_id = 1;
            let column_values = vec![aggegated_row_count.to_string()];
            let table_row = TableRow { row_id, column_values };
            aggregated_rows.push(table_row);
        }
        _ => panic!()
    }
    return aggregated_rows;
}