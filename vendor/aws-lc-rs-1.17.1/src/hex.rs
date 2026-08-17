// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

/// Converts bytes to a lower-case hex string
#[allow(clippy::missing_panics_doc)]
pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut encoding = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        let upper_val = byte >> 4u8;
        let lower_val = byte & 0x0f;
        // DON'T PANIC: it shouldn't be possible to panic because only bottom 4 bits can be set.
        encoding.push(char::from_digit(u32::from(upper_val), 16).unwrap());
        encoding.push(char::from_digit(u32::from(lower_val), 16).unwrap());
    }
    encoding
}

/// Converts bytes to an upper-case hex string
pub fn encode_upper<T: AsRef<[u8]>>(bytes: T) -> String {
    encode(bytes).to_ascii_uppercase()
}

/// Converts a hex string to a vector of bytes
/// # Errors
/// Returns an error if `hex_str` contains a non-hex digit.
#[allow(clippy::missing_panics_doc)]
pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::<u8>::with_capacity(hex_str.len() / 2 + 1);
    let mut current_byte = b'\0';
    let mut index: u32 = 0;
    for ch in hex_str.chars() {
        if !ch.is_ascii_hexdigit() {
            return Err("Invalid hex string".to_string());
        }
        #[allow(clippy::cast_possible_truncation)]
        // DON'T PANIC: it should not be possible to panic because we verify above that the character is a
        // hex digit.
        let value = ch.to_digit(16).unwrap() as u8;
        if index % 2 == 0 {
            current_byte = value << 4;
        } else {
            current_byte |= value;
            bytes.push(current_byte);
        }

        if let Some(idx) = index.checked_add(1) {
            index = idx;
        } else {
            break;
        }
    }
    if index % 2 == 1 {
        bytes.push(current_byte);
    }
    Ok(bytes)
}

/// Converts a hex string to a vector of bytes.
/// It ignores any characters that are not valid hex digits.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn decode_dirty(hex_str: &str) -> Vec<u8> {
    let clean: String = hex_str.chars().filter(char::is_ascii_hexdigit).collect();
    // DON'T PANIC: it should not be possible to panic because we filter out all non-hex digits.
    decode(clean.as_str()).unwrap()
}
