use  std::fs::File;
use sqlite_fsr::utils::varint::*;
use sqlite_fsr::models::schema::*;
use sqlite_fsr::command::tables;

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
