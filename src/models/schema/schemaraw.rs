use crate::command::sql::parser::sql_statement::CreateTableStatement;
use crate::command::sql::parser::sql_token::Tokenize;
use crate::models::schema::SchemaRow;
use crate::utils::varint::parse_varint;

pub struct SchemaRAW {
    pub page_size: u16,
    pub file_header: [u8; 100],
    pub page_header: [u8; 8],
    pub cell_pointer_array: Vec<u16>,
    pub cells: Vec<Vec<u8>>
}


impl SchemaRAW {
    pub fn from_bytes(data: &[u8]) -> Self {
        let page_size: u16 =  u16::from_be_bytes([data[16], data[17]]);
        let file_header = data[0..100].try_into().unwrap();
        let page_header: [u8; 8] =  match data[100] {
                                        0x0D => data[100..108].try_into().unwrap(),
                                        0x05 => data[100..112].try_into().unwrap(),
                                        _ => panic!("Unrecognized page type id for schema page")
                                    };
        let cell_count = u16::from_be_bytes([page_header[3], page_header[4]]);

        let cell_pointer_array: Vec<u16> = data[108..(108 + (cell_count * 2)) as usize]
                                            .chunks_exact(2)
                                            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                                            .collect();

        let mut cells: Vec<Vec<u8>> = Vec::new();
        for index in 0..cell_pointer_array.len() {
            let cell_pointer = cell_pointer_array[index] as usize;
            let mut offset = cell_pointer;
            // Parse payload size varint
            let (payload_size, len1) = parse_varint(&data[offset..]);
            offset += len1;

            // Parse rowid varint
            let (_rowid, len2) = parse_varint(&data[offset..]);
            offset += len2;

            // Now offset points to start of payload
            let cell_end = offset + payload_size as usize;

            let cell = data[cell_pointer..cell_end].to_vec();
            cells.push(cell);
        }
        return Self { page_size, file_header, page_header, cell_pointer_array, cells }
    }

    pub fn to_schema_rows(&self) -> Vec<SchemaRow> {
        let mut header_entries = Vec::new();
        for cells_index in 0..self.cells.len() {
            let cell = &self.cells[cells_index];
            let (_, cell_size_varint_size) = parse_varint(&cell[0..9]);
            let (_, row_id_varint_size) = parse_varint(&cell[cell_size_varint_size..cell_size_varint_size+9]);
            
            let cell_content_offset = cell_size_varint_size + row_id_varint_size;
            let record_data = &cell[cell_content_offset..];

            let record_header_value_sizes: Vec<i64>  = {
                let mut record_header_values: Vec<i64> = Vec::new();
                // let mut record_header_values: Vec<Vec<u8>> = Vec::new();

                let (record_header_size, record_header_varint_size) = parse_varint(&record_data[0..9]);
                record_header_values.push(record_header_size);
                
                let mut record_header_value_index = record_header_varint_size;
                while record_header_value_index < record_header_size as usize {
                    let (mut record_header_value_size, record_header_value_varint_size)  = parse_varint(&record_data[record_header_value_index..record_header_value_index+9]);
                    if [0, 8, 9, 12, 13].contains(&record_header_value_size) {
                        record_header_value_size = 0;
                    } else if (record_header_value_size >= 12) & (record_header_value_size % 2 == 0) {
                        record_header_value_size = (record_header_value_size-12)/2
                    } else if (record_header_value_size >= 13) & (record_header_value_size % 2 != 0) {
                        record_header_value_size = (record_header_value_size-13)/2

                    }
                    record_header_values.push(record_header_value_size);
                    record_header_value_index += record_header_value_varint_size;
                }

                record_header_values
            };
            
            let mut record_body_offset = record_header_value_sizes[0] as usize; 
            let record_body = &record_data[record_body_offset..];

            let object_type_bytes = record_body[..record_header_value_sizes[1] as usize].to_vec();
            let object_type = String::from_utf8(object_type_bytes).unwrap();
            
            record_body_offset = record_header_value_sizes[1] as usize;
            let name_bytes = record_body[record_body_offset..record_body_offset+record_header_value_sizes[2] as usize].to_vec();
            let name = String::from_utf8(name_bytes).unwrap();
            
            record_body_offset = record_body_offset+record_header_value_sizes[2] as usize;
           
            let table_name_bytes = record_body[record_body_offset..(record_body_offset+record_header_value_sizes[3] as usize).min(record_body.len())].to_vec();
            let table_name = String::from_utf8(table_name_bytes).unwrap();

            record_body_offset = (record_body_offset+record_header_value_sizes[3] as usize).min(record_body.len());
            let rootpage_byte = record_body[record_body_offset];
            let rootpage = rootpage_byte as i8;

            record_body_offset = record_body_offset+record_header_value_sizes[4] as usize;
            let sql_bytes = record_body[record_body_offset..].to_vec();
            let sql_string = String::from_utf8(sql_bytes).unwrap();
            print!("sql_string: {}", sql_string);
            let sql = CreateTableStatement::from_tokens(sql_string.tokenize());

            let schemarow_header = SchemaRow { object_type, name, table_name, rootpage, sql };
            header_entries.push(schemarow_header);        
        }
        return header_entries;
    }
}
