use std::fs::File;
use crate::{command::sql::parser::sql_statement::SelectStatement, utils::varint::parse_varint};
use crate::models::record::Record;
use std::io::{Seek, SeekFrom, Read};
use crate::models::tablepage::{Table, TablePage, LeafTablePage};


#[derive(Debug)]
pub struct InteriorTablePage<'a> {
    pub header: [u8; 12],
    pub cells: Vec<(u32, i64)>,
    pub file: &'a mut File,
    pub sibling_page_number: Option<u32>
}

impl<'a> InteriorTablePage<'a> {
    pub fn from_bytes(data: &[u8], file: &'a mut File) -> Self {
        let header: [u8; 12] = data[0..12].try_into().unwrap();
        let cell_count = u16::from_be_bytes([header[3], header[4]]);
        let sibling_page_number = match u32::from_be_bytes([header[8], header[9], header[10], header[11]]){
                                        0 => None,
                                        n => Some(n)
                                    };
        let cell_pointer_array: Vec<u16> = data[12..(12 + (cell_count * 2) as usize)]
                                            .chunks_exact(2)
                                            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                                            .collect();
        
        let mut cells: Vec<(u32, i64)> = Vec::new();
        for index in 0..cell_pointer_array.len() {
            let cell_pointer = cell_pointer_array[index] as usize;
            let mut offset = cell_pointer;

            let page_number = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
            offset += 4;

            let (key, key_varint_size) = parse_varint(&data[offset..]);
            offset += key_varint_size;


            cells.push((page_number, key));
        }
        Self { header, cells, file, sibling_page_number }
    }
}

impl Table for InteriorTablePage<'_> {
    fn to_table_rows(&mut self, statement: &SelectStatement) -> Vec<Record> {
        let mut result: Vec<Record> = Vec::new();
        for index in 0..self.cells.len() {
            let cell = self.cells[index];

            self.file.seek(SeekFrom::Start(0));

            let page_size = 4096;
            let mut page_buffer = vec![0u8; page_size as usize];
            let start = page_size * (cell.0 - 1) as u64;
            self.file.seek(SeekFrom::Start(start));
            self.file.read_exact(&mut page_buffer);

            let mut table_page: TablePage = match page_buffer[0] {
                                                0x0D => TablePage::Leaf(LeafTablePage::from_bytes(&page_buffer)),
                                                0x05 => TablePage::Interior(InteriorTablePage::from_bytes(&page_buffer, self.file)),
                                                0x0a => {
                                                    println!("index b-tree leaf page type not supported");
                                                    continue;
                                                },
                                                0x02 => {
                                                    println!("index b-tree interior page type not supported");
                                                    continue;
                                                },
                                                e => panic!("unsupported page type {}", e),
                                            };
            let table_rows = table_page.to_table_rows(statement);

            result.extend(table_rows);
        }

        match self.sibling_page_number {
            Some(n) => {
                let page_size = 4096;
                let mut sibling_page_buffer = vec![0u8; page_size as usize];
                let sibling_page_number = self.sibling_page_number.unwrap();
                let start = page_size * (sibling_page_number - 1) as u64;
                self.file.seek(SeekFrom::Start(start));
                self.file.read_exact(&mut sibling_page_buffer);
                let mut sibling_tablepage: Option<TablePage> = match sibling_page_buffer[0] {
                                                                    0x0D => Some(TablePage::Leaf(LeafTablePage::from_bytes(&sibling_page_buffer))),
                                                                    0x05 => Some(TablePage::Interior(InteriorTablePage::from_bytes(&sibling_page_buffer, self.file))),
                                                                    0x0a => None,
                                                                    0x02 => None,
                                                                    _    => panic!("unsupported page type"),
                                                                };
                if sibling_tablepage.is_some() {
                    let sibling_tablepage_rows = sibling_tablepage.unwrap().to_table_rows(statement);
                    result.extend(sibling_tablepage_rows);
                }

            }
            None => ()
        } 

        return result;
    }
}