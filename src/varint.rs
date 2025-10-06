pub fn parse_varint(bytes: &[u8]) -> (i64, usize) {
    let mut value: i64 = 0;
    let mut len = 0;

    for &byte in bytes.iter().take(9) {
        value = (value << 7) | (byte & 0x7F) as i64;
        len += 1;

        if byte & 0x80 == 0 {
            return (value, len);
        }
    }

    // If it's at least 9 bytes, SQLite says it's a full 64-bit integer
    if bytes.len() >= 9 {
        let mut v: i64 = 0;
        for &b in &bytes[..9] {
            v = (v << 8) | (b as i64);
        }
        return (v, 9);
    }

    // Fallback: if slice < 9 and all bytes had high bit set,
    // just return what we have
    (value, len)
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
