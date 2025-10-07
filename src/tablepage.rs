use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::varint::parse_varint;
use crate::record::Record;


pub trait Table {
    fn to_table_rows(&mut self) -> Vec<Record>;
}


pub enum TablePage<'a> {
    Leaf(LeafTablePage),
    Interior(InteriorTablePage<'a>),
}

impl Table for TablePage<'_> {
    fn to_table_rows(&mut self) -> Vec<Record> {
        match self {
            TablePage::Leaf(p) => p.to_table_rows(),
            TablePage::Interior(p) => p.to_table_rows(),
        }
    }
}

#[derive(Debug)]
pub struct LeafTablePage {
    pub header: [u8; 8],
    pub cells: Vec<Vec<u8>>
}

impl LeafTablePage {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header: [u8; 8] = data[0..8].try_into().unwrap();
        let cell_count = u16::from_be_bytes([header[3], header[4]]);

        let cell_pointer_array: Vec<u16> = data[8..(8 + (cell_count * 2) as usize)]
                                            .chunks_exact(2)
                                            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                                            .collect();
        
        let mut cells: Vec<Vec<u8>> = Vec::new();
        for index in 0..cell_pointer_array.len() {
            let cell_pointer = cell_pointer_array[index] as usize;
            let mut offset = cell_pointer;

            // 1) parse payload size
            let (payload_size, payload_varint_len) = parse_varint(&data[offset..]);
            offset += payload_varint_len;

            // 2) parse rowid
            let (_rowid, rowid_varint_len) = parse_varint(&data[offset..]);
            offset += rowid_varint_len;

            // 3) now offset points to start of payload
            let cell_end = offset + payload_size as usize;

            // 4) slice the whole cell (from cell_pointer, not just payload)
            let cell = data[cell_pointer .. cell_end].to_vec();
            cells.push(cell);
        }
        Self { header, cells }
    }
}

impl Table for LeafTablePage {

    fn to_table_rows(&mut self) -> Vec<Record> {
        let mut table_rows: Vec<Record> = Vec::new();
        for cells_index in 0..self.cells.len() {
            let cell = &self.cells[cells_index];
            let (_, cell_size_varint_size) = parse_varint(&cell[0..9]);
            let (row_id, row_id_varint_size) = parse_varint(&cell[cell_size_varint_size..cell_size_varint_size+9]);
            
            let cell_content_offset = cell_size_varint_size + row_id_varint_size;
            let record_data = &cell[cell_content_offset..];

            let (record_header_size, record_body_size, record_column_value_sizes): (usize, usize, Vec<i64>)  = {
                let mut record_header_values: Vec<i64> = Vec::new();
                let (record_header_size, record_header_varint_size) = parse_varint(&record_data[0..9]);
                let mut record_body_size = 0;
                let mut record_header_value_index = record_header_varint_size;
                while record_header_value_index < record_header_size as usize {
                    let end = (record_header_value_index+9).min(record_data.len());
                    let (mut record_header_value_size, record_header_value_varint_size)  = parse_varint(&record_data[record_header_value_index..end]);
                    if [0, 8, 9, 12, 13].contains(&record_header_value_size) {
                        record_header_value_size = 0;
                    } else if (record_header_value_size >= 12) & (record_header_value_size % 2 == 0) {
                        record_header_value_size = (record_header_value_size-12)/2
                    } else if (record_header_value_size >= 13) & (record_header_value_size % 2 != 0) {
                        record_header_value_size = (record_header_value_size-13)/2
                    }
                    record_header_values.push(record_header_value_size);
                    record_body_size += record_header_value_size;
                    record_header_value_index += record_header_value_varint_size;
                }

                (record_header_size as usize, record_body_size as usize, record_header_values)
            };
            
            let record_body: &[u8] = &record_data[record_header_size..];

            let mut column_values = Vec::new();
            let mut record_body_offset = 0; 
            let mut index = 0;
            while record_body_offset < record_body_size {
                let column_value_size = record_column_value_sizes[index] as usize;
                let column_value = record_body[record_body_offset..record_body_offset+column_value_size].to_vec();
                column_values.push(column_value);
                record_body_offset += column_value_size;
                index += 1;
            }

            let table_row = Record { row_id, column_values, column_headers: record_column_value_sizes };
            table_rows.push(table_row);        
        }
        return table_rows;
    }
}


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
        println!("sibling_page_number: {}",sibling_page_number.unwrap());
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
    fn to_table_rows(&mut self) -> Vec<Record> {
        let mut result: Vec<Record> = Vec::new();
        let mut tblrowcount = 0;
        for index in 0..self.cells.len() {
            let cell = self.cells[index];
            // print!("page_number: {} ", cell.0);
            // print!("rowid: {} \n", cell.1);

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
            let table_rows = table_page.to_table_rows();
            tblrowcount += table_rows.len();
            // println!("tblrowcount: {}", tblrowcount);
            table_rows.iter().for_each(|r| println!("{}", r));
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
                    let sibling_tablepage_rows = sibling_tablepage.unwrap().to_table_rows();
                    result.extend(sibling_tablepage_rows);
                }

            }
            None => ()
        } 

        return result;
    }
}