use core::char;
use core::fmt::{self, Write as _};
use core::str;

pub(crate) fn display(mut bytes: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    loop {
        match str::from_utf8(bytes) {
            Ok(valid) => return f.write_str(valid),
            Err(utf8_error) => {
                let valid_up_to = utf8_error.valid_up_to();
                let valid = unsafe { str::from_utf8_unchecked(&bytes[..valid_up_to]) };
                f.write_str(valid)?;
                f.write_char(char::REPLACEMENT_CHARACTER)?;
                if let Some(error_len) = utf8_error.error_len() {
                    bytes = &bytes[valid_up_to + error_len..];
                } else {
                    return Ok(());
                }
            }
        }
    }
}

pub(crate) fn debug(mut bytes: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('"')?;

    while !bytes.is_empty() {
        let from_utf8_result = str::from_utf8(bytes);
        let valid = match from_utf8_result {
            Ok(valid) => valid,
            Err(utf8_error) => {
                let valid_up_to = utf8_error.valid_up_to();
                unsafe { str::from_utf8_unchecked(&bytes[..valid_up_to]) }
            }
        };

        let mut written = 0;
        for (i, ch) in valid.char_indices() {
            let esc = ch.escape_debug();
            if esc.len() != 1 && ch != '\'' {
                f.write_str(&valid[written..i])?;
                for ch in esc {
                    f.write_char(ch)?;
                }
                written = i + ch.len_utf8();
            }
        }
        f.write_str(&valid[written..])?;

        match from_utf8_result {
            Ok(_valid) => break,
            Err(utf8_error) => {
                let end_of_broken = if let Some(error_len) = utf8_error.error_len() {
                    valid.len() + error_len
                } else {
                    bytes.len()
                };
                for b in &bytes[valid.len()..end_of_broken] {
                    write!(f, "\\x{:02x}", b)?;
                }
                bytes = &bytes[end_of_broken..];
            }
        }
    }

    f.write_char('"')
}
