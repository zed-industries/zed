use std::io::Read;

pub fn read_char(reader: &mut impl Read) -> std::io::Result<char> {
    let mut byte = [0u8; 1];
    let n = reader.read(&mut byte)?;
    if n == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ));
    }

    match byte[0] {
        // ASCII
        0x00..=0x7F => Ok(byte[0] as char),
        // UTF-8 lead byte
        0xC2..=0xF4 => {
            let width = match byte[0] {
                0xC2..=0xDF => 2,
                0xE0..=0xEF => 3,
                0xF0..=0xF4 => 4,
                _ => {
                    return Ok('\u{FFFD}');
                }
            };
            let mut utf8_buf = vec![byte[0]];
            for _ in 1..width {
                let n = reader.read(&mut byte)?;
                if n == 0 {
                    return Ok('\u{FFFD}');
                }
                utf8_buf.push(byte[0]);
            }
            if let Ok(s) = std::str::from_utf8(&utf8_buf) {
                if let Some(c) = s.chars().next() {
                    Ok(c)
                } else {
                    Ok('\u{FFFD}')
                }
            } else {
                Ok('\u{FFFD}')
            }
        }
        // Invalid byte
        _ => Ok('\u{FFFD}'),
    }
}
