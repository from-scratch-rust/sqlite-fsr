use std::fmt;

#[derive(Debug)]
pub struct Record {
    pub row_id: i64,
    pub column_headers: Vec<i64>,
    pub column_values: Vec<Vec<u8>>
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.row_id);
        for i in 0..self.column_headers.len() {
            let value_size = self.column_headers[i] as usize;
            let value = if value_size == 0 { 
                            "NULL".to_string()
                        } else {
                            self.column_values[i].iter().map(|&c| c as char).collect::<String>()
                        };
            write!(f, "{}", value)?;
            if i != self.column_headers.len() - 1 { write!(f, " ")?; }
        }
        Ok(())
    } 
}

// Wrapper so a vector of `Record` can be printed to the console.
// You cannot implement `Display` directly for `Vec<Record>` due to Rust's
// orphan rules, so we provide a small newtype wrapper instead.
pub struct Records(pub Vec<Record>);

impl From<Vec<Record>> for Records {
    fn from(v: Vec<Record>) -> Self {
        Records(v)
    }
}

impl fmt::Display for Records {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, rec) in self.0.iter().enumerate() {
            writeln!(f, "{}", rec)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Records {
    type Target = [Record];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Records {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
