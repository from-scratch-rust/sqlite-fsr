use std::fs::File;
use sqlite_fsr::models::schema::extract_raw_schema_data;

// #[test]
// fn test_SchemaRAW_from_bytes_extracts_CREATE_TABLE_statement_correctly() {
//     let apples_table_sql = "CREATE TABLE apples
//                             (
//                                     id integer primary key autoincrement,
//                                     name text,
//                                     color text
//                             )";
//     let mut file = File::open("./sample.db").unwrap();
//     let raw_schema = extract_raw_schema_data(&mut file);
//     let result = raw_schema.cells[0].iter()
//                     .map(|c| *c as char)
//                     .collect::<String>();


//     assert_eq!(result, apples_table_sql)
// }