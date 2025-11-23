use std::path::PathBuf;

use sqlite_fsr::models::dbfile::{DBFile, schema::*};

#[test]
fn test_dbinfo_command_reads_pagesize_correctly() {
    let dbfile = DBFile::open("./tests/assets/sample.db").unwrap();
    let (result, _) = dbfile.get_dbinfo();
    assert_eq!(result, 4096);
}

#[test]
fn test_dbinfo_command_reads_table_count_correctly() {
    let dbfile = DBFile::open("./tests/assets/sample.db").unwrap();
    let result = dbfile.get_table_names();
    assert_eq!(result.len(), 2);
}
