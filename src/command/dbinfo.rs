use crate::models::schema::SchemaRAW;

pub fn get_dbinfo(schema_data: &SchemaRAW) -> (u16, usize) {
    return (schema_data.page_size, schema_data.cells.len());
}
