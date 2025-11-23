use sqlite_fsr::models::DBFile;

#[test]
fn test_tablerow_index_operator_access() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT name, color FROM apples";
    let results = file.execute(sql_statement).unwrap();
    
    // Test direct index access using [] operator
    let first_row = &results[0];
    assert_eq!(first_row[0], "Granny Smith");
    assert_eq!(first_row[1], "Light Green");
    
    let second_row = &results[1];
    assert_eq!(second_row[0], "Fuji");
    assert_eq!(second_row[1], "Red");
}

#[test]
fn test_tablerow_with_all_columns() {
    let mut file = DBFile::open("./tests/assets/sample.db").unwrap();
    let sql_statement = "SELECT * FROM apples";
    let results = file.execute(sql_statement).unwrap();
    
    // Test accessing id column (first column)
    let first_row = &results[0];
    assert_eq!(first_row[0], "1");
    assert_eq!(first_row[1], "Granny Smith");
    assert_eq!(first_row[2], "Light Green");
}
