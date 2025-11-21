use crate::{command::sql::parser::sql_statement::{SelectStatement}, models::dbfile::dbtable::record::Record, models::dbfile::schema::SchemaRow, DBFile};
use crate::models::dbfile::dbtable::tablepage::*;
use crate::models::dbfile::dbtable::tablepage::Table;
use std::io::{Seek, SeekFrom, Read};

pub struct DBTable <'a>{
    pub description: SchemaRow,
    dbfile: &'a mut DBFile
}


impl DBTable <'_> {
    pub fn new(description: SchemaRow, dbfile: &mut DBFile) -> DBTable {
        DBTable { description, dbfile }
    }

    pub fn to_table_rows(&mut self, statement: SelectStatement) -> Vec<Record> {
        
        let table_rootpage_offset = self.dbfile.schema.page_size as u64 * (self.description.rootpage-1) as u64;
        self.dbfile.seek(SeekFrom::Start(table_rootpage_offset)).expect("seek failed");

        let mut table_page_buffer = vec![0; self.dbfile.schema.page_size as usize];
        self.dbfile.read_exact(&mut table_page_buffer).expect("failed to read row size for table");

        let mut table_page: TablePage = match table_page_buffer[0] {
                                    0x0D => TablePage::Leaf(LeafTablePage::from_bytes(&table_page_buffer)),
                                    0x05 => TablePage::Interior(InteriorTablePage::from_bytes(&table_page_buffer, self.dbfile)),
                                    _    => panic!("unsupported page type"),
                                };

        
        let records: Vec<Record> = table_page.to_table_rows(&statement, &self.description.sql);

        return records;
    }
}