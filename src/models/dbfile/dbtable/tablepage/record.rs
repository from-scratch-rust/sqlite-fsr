use std::fmt;

#[derive(Debug)]
pub struct Record {
    pub row_id: i64,
    pub column_values: Vec<Vec<u8>>
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.column_values.len() {
            let value = if self.column_values[i].is_empty() { 
                            "NULL".to_string()
                        } else {
                            self.column_values[i].iter().map(|&c| c as char).collect::<String>()
                        };
            write!(f, "{}", value)?;
            if i != self.column_values.len() - 1 { write!(f, " ")?; }
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
