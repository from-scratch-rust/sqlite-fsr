use std::fmt;
pub struct Record {
    pub row_id: i64,
    pub column_headers: Vec<i64>,
    pub column_values: Vec<Vec<u8>>
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.column_headers.len() {
            let value_size = self.column_headers[i] as usize;
            let value = if value_size == 0 { 
                            " NULL".to_string()
                        } else {
                            self.column_values[i].iter().map(|&c| c as char).collect::<String>()
                        };
            write!(f, " {}", value);
        }
        write!(f, "")
    } 
}