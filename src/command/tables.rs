use crate::models::schema::SchemaRAW;

pub fn get_table_names(schema_data: &SchemaRAW) -> Vec<String> {
    let mut table_names: Vec<String>  = Vec::from([]); 
    for schemarow_header in schema_data.to_schema_rows() {
        if !schemarow_header.name.starts_with("sqlite_") { 
            table_names.push(schemarow_header.table_name); 
        }
    }
    return table_names;
}
