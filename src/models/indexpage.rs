struct LeafIndexPage {
    pub header: [u8; 8],
    pub cells: Vec<Vec<u8>>
}

impl LeafIndexPage {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header: [u8; 8] = data[0..8].try_into().unwrap();
        let cell_count = u16::from_be_bytes([header[3], header[4]]);

        let mut cells: Vec<Vec<u8>> = Vec::new();


        Self { header, cells }
    }
}