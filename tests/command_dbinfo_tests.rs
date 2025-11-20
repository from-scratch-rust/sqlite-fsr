use sqlite_fsr::models::schema::*;

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
