/// Four byte tag value.
pub type Tag = u32;

/// Creates a tag from four bytes.
pub const fn tag_from_bytes(bytes: &[u8; 4]) -> Tag {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}

/// Creates a tag from the first four bytes of a string, inserting
/// spaces for any missing bytes.
pub fn tag_from_str_lossy(s: &str) -> Tag {
    let mut bytes = [b' '; 4];
    for (i, b) in s.as_bytes().iter().enumerate().take(4) {
        bytes[i] = *b;
    }
    tag_from_bytes(&bytes)
}
