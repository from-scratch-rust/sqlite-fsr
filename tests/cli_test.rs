use sqlite_fsr::{run};

#[cfg(test)]

use std::{fs::File, io::Cursor, result};

use sqlite_fsr::command::{dbinfo, tables, sql};
use sqlite_fsr::utils::varint::*;
use sqlite_fsr::models::schema::*;
use sqlite_fsr::models::error::*;


#[test]
fn test_run_fails_when_missing_all_args() {
    let args = vec![String::from("/path/to/binary")];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::MissingArgs)));
}

#[test]
fn test_run_fails_when_missing_one_args() {
    let args = vec![String::from("/path/to/binary"), String::from("./sample.db")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::MissingCommand)));
}

#[test]
fn test_run_fails_when_invalid_command() {
    let args = vec![String::from("/path/to/binary"), String::from("./sample.db"), String::from(".dbpictures")];
    let result = run(&args).unwrap_err();
    assert!( matches!(result, RunError::Args(CommandArgsError::InvalidCommand(_))));
}

#[test]
fn test_run_succeeds_when_valid_command() {
    let args = vec![String::from("/path/to/binary"), String::from("./sample.db"), String::from(".dbinfo")];
    let result = run(&args);
    assert!(result.is_ok());
}

#[test]
fn test_run_fails_when_invalid_filepath() {
    let args = vec![String::from("/path/to/binary"), String::from("../fake/index.db"), String::from(".dbinfo")];
    let result = run(&args).unwrap_err();
    assert!(matches!(result, RunError::Args(CommandArgsError::Io(_))));
}


#[test]
fn test_dbinfo_command_reads_pagesize_correctly() {
    let DB_PAGE_SIZE: u16 = 4096;

    let schema = SchemaRAW {
        page_size: DB_PAGE_SIZE,
        file_header: [0; 100],
        page_header: [0; 8],
        cell_pointer_array: Vec::new(),
        cells: Vec::new(),
    };

    let (result, _) = dbinfo::get_dbinfo(&schema);
    assert_eq!(result, 4096);
}

#[test]
fn test_dbinfo_command_reads_table_count_correctly() {
    let schema = SchemaRAW {
        page_size: 4096,
        file_header: [0; 100],
        page_header: {
            let mut header = [0u8; 8];
            // set cell count at bytes 3–4
            header[3..5].copy_from_slice(&(3u16).to_be_bytes());
            header
        },
        cell_pointer_array: vec![3983, 3901, 3779],
        cells: vec![
            vec![0; 111], // fake cell payload
            vec![0; 80],
            vec![0; 120],
        ],
    };

    let (_, result) = dbinfo::get_dbinfo(&schema);
    assert_eq!(result, 3);
}


#[test]
fn test_tables_command_reads_table_names_correctly() {
    let mut file = File::open("./sample.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = tables::get_table_names(&raw_schema);
    assert_eq!(result, ["apples", "oranges"]);
}

#[test]
fn test_from_bytes_extracts_schemaraw_data_correctly() {
    let DB_PAGE_SIZE = 4096 as u16;
    let SCHEMA_PAGE_TYPE = 13 as u8;
    let SCHEMA_PAGE_CELL_COUNT = 3 as u16;

    let mut data = [0; 4096];
    data[16..18].copy_from_slice(&DB_PAGE_SIZE.to_be_bytes());
    data[100] = SCHEMA_PAGE_TYPE;
    data[103..105].copy_from_slice(&SCHEMA_PAGE_CELL_COUNT.to_be_bytes());

    let CELL1_OFFSET: u16 = 3983;
    let CELL1_SIZE = encode_varint(111);
    data[108..110].copy_from_slice(&(CELL1_OFFSET).to_be_bytes());
    data[CELL1_OFFSET as usize..(CELL1_OFFSET as usize + CELL1_SIZE.len())].copy_from_slice(&CELL1_SIZE);

    let CELL2_OFFSET: u16 = 3901;
    let CELL2_SIZE = encode_varint(80);
    data[110..112].copy_from_slice(&(CELL2_OFFSET).to_be_bytes());
    data[CELL2_OFFSET as usize..(CELL2_OFFSET as usize + CELL2_SIZE.len())].copy_from_slice(&CELL2_SIZE);

    let CELL3_OFFSET: u16 = 3779;
    let CELL3_SIZE = encode_varint(120);     
    data[112..114].copy_from_slice(&(CELL3_OFFSET).to_be_bytes());
    data[CELL3_OFFSET as usize..(CELL3_OFFSET as usize + CELL3_SIZE.len())].copy_from_slice(&CELL3_SIZE);

    let raw_schema_data = SchemaRAW::from_bytes(&data);
    assert_eq!(raw_schema_data.file_header[16..18], DB_PAGE_SIZE.to_be_bytes());
    assert_eq!(raw_schema_data.page_header[3..5], SCHEMA_PAGE_CELL_COUNT.to_be_bytes());

    let cell_pointers: [u16; 3] = raw_schema_data.cell_pointer_array.clone().try_into().unwrap();
    assert_eq!(cell_pointers, [3983, 3901, 3779])
}


#[test]
fn test_COUNT_sql_command_executes_without_error() {
    let args = vec![String::from("/path/to/binary"), String::from("./sample.db"), String::from("\"SELECT COUNT(*) FROM apples\"")];
    let result = run(&args);
    assert!(result.is_ok());
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows() {
    let mut file = File::open("./sample.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = sql::execute(["SELECT", "COUNT(*)", "FROM", "apples"].to_vec(), &raw_schema, &mut file).unwrap();
    assert_eq!(result.len(), 4);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_2() {
    let mut file = File::open("./sample.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = sql::execute(["SELECT", "COUNT(*)", "FROM", "oranges"].to_vec(), &raw_schema, &mut file).unwrap();
    assert_eq!(result.len(), 6);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_3() {
    let mut file = File::open("./superheroes.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = sql::execute(["SELECT", "COUNT(*)", "FROM", "superheroes"].to_vec(), &raw_schema, &mut file).unwrap();
    assert_eq!(result.len(), 6895);
}

#[test]
fn test_COUNT_sql_command_returns_correct_number_of_rows_4() {
    let mut file = File::open("./companies.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = sql::execute(["SELECT", "COUNT(*)", "FROM", "companies"].to_vec(), &raw_schema, &mut file).unwrap();
    assert_eq!(result.len(), 55991);
}

#[test]
fn test_SELECT_sql_command_returns_correct_values() {
    let mut file = File::open("./sample.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result: Vec<String> = sql::execute(["SELECT", "name", "FROM", "apples"].to_vec(), &raw_schema, &mut file)
                    .unwrap()
                    .iter()
                    .map(|record| record.to_string())
                    .collect();
                
    assert_eq!(result, vec!["Granny Smith", "Fuji", "Honeycrisp", "Golden Delicious"]);
}


#[test]
fn test_COUNT_sql_command_returns_error_when_table_not_found() {
    let mut file = File::open("./sample.db").unwrap();
    let raw_schema = extract_raw_schema_data(&mut file);
    let result = sql::execute(["SELECT", "COUNT(*)", "FROM", "seafood"].to_vec(), &raw_schema, &mut file);
    assert!(result.is_err());
}