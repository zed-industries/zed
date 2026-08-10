/// Splits `chunk` into complete lines, carrying any trailing partial line
/// over in `carry` for the next call. Recognizes `\n` as the line
/// terminator and strips a trailing `\r` (so both `\n`- and
/// `\r\n`-terminated sketches work). Invalid UTF-8 is replaced per
/// `String::from_utf8_lossy` rather than erroring -- a serial device can
/// send anything, and one garbled line shouldn't stop the monitor.
pub fn split_lines(carry: &mut Vec<u8>, chunk: &[u8]) -> Vec<String> {
    carry.extend_from_slice(chunk);
    let mut lines = Vec::new();
    while let Some(newline_index) = carry.iter().position(|&byte| byte == b'\n') {
        let mut line_bytes: Vec<u8> = carry.drain(..=newline_index).collect();
        line_bytes.pop(); // remove the '\n' itself
        if line_bytes.last() == Some(&b'\r') {
            line_bytes.pop();
        }
        lines.push(String::from_utf8_lossy(&line_bytes).into_owned());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_single_complete_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\n");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_multiple_lines_one_chunk() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"x\ny\nz\n");
        assert_eq!(lines, vec!["x".to_string(), "y".to_string(), "z".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_partial_line_carried_over() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\nworl");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert_eq!(carry, b"worl");

        let lines = split_lines(&mut carry, b"d\n");
        assert_eq!(lines, vec!["world".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_strips_carriage_return() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"a\r\nb\r\n");
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_split_lines_no_newline_yet() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"abc");
        assert!(lines.is_empty());
        assert_eq!(carry, b"abc");
    }

    #[test]
    fn test_split_lines_invalid_utf8_does_not_panic() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, &[0xFF, b'\n']);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_split_lines_empty_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"\n");
        assert_eq!(lines, vec!["".to_string()]);
    }
}
