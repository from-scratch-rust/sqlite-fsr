pub fn parse_varint(bytes: &[u8]) -> (i64, usize) {
    let mut value: u64 = 0;  // Use u64 to avoid premature sign extension
    let mut len = 0;
    
    for &byte in bytes.iter().take(9) {
        len += 1;
        
        if len == 9 {
            // 9th byte uses all 8 bits
            value = (value << 8) | (byte as u64);
            return (value as i64, len);
        } else {
            // First 8 bytes use lower 7 bits
            value = (value << 7) | ((byte & 0x7F) as u64);
            
            if byte & 0x80 == 0 {
                // High bit is 0, we're done
                return (value as i64, len);
            }
        }
    }
    
    // Should not reach here if bytes.len() >= 9
    (value as i64, len)
}

pub fn encode_varint(mut value: i64) -> Vec<u8> {
    let mut varint_bytes = Vec::new();

    loop {
        let byte = (value & 0x7F) as u8; // take 7 bits
        value >>= 7;
        if value != 0 {
            varint_bytes.push(byte | 0x80); // set MSB = 1 (more bytes follow)
        } else {
            varint_bytes.push(byte); // last byte, MSB = 0
            break;
        }
    }

    return varint_bytes
}
