/// Count up to `(2^32)-1` occurrences of a byte in a slice
/// of bytes, simple
///
/// # Example
///
/// ```
/// let s = b"This is yet another Text with spaces";
/// let number_of_spaces = bytecount::naive_count_32(s, b' ');
/// assert_eq!(number_of_spaces, 6);
/// ```
pub fn naive_count_32(haystack: &[u8], needle: u8) -> usize {
    haystack.iter().fold(0, |n, c| n + (*c == needle) as u32) as usize
}

/// Count occurrences of a byte in a slice of bytes, simple
///
/// # Example
///
/// ```
/// let s = b"This is yet another Text with spaces";
/// let number_of_spaces = bytecount::naive_count(s, b' ');
/// assert_eq!(number_of_spaces, 6);
/// ```
pub fn naive_count(utf8_chars: &[u8], needle: u8) -> usize {
    utf8_chars
        .iter()
        .fold(0, |n, c| n + (*c == needle) as usize)
}

/// Count the number of UTF-8 encoded Unicode codepoints in a slice of bytes, simple
///
/// This function is safe to use on any byte array, valid UTF-8 or not,
/// but the output is only meaningful for well-formed UTF-8.
///
/// # Example
///
/// ```
/// let swordfish = "メカジキ";
/// let char_count = bytecount::naive_num_chars(swordfish.as_bytes());
/// assert_eq!(char_count, 4);
/// ```
pub fn naive_num_chars(utf8_chars: &[u8]) -> usize {
    utf8_chars
        .iter()
        .filter(|&&byte| (byte >> 6) != 0b10)
        .count()
}
