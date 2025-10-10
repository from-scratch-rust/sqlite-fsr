use crate::utils::varint::parse_varint;
use crate::models::record::Record;
use crate::models::tablepage::Table;

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
