pub fn escape_bytes_to(bytes: &[u8], buf: &mut String) {
    for &c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'\'' => buf.push_str("\\\'"),
            b'"' => buf.push_str("\\\""),
            b'\\' => buf.push_str(r"\\"),
            b'\x20'..=b'\x7e' => buf.push(c as char),
            _ => {
                buf.push('\\');
                buf.push((b'0' + (c >> 6)) as char);
                buf.push((b'0' + ((c >> 3) & 7)) as char);
                buf.push((b'0' + (c & 7)) as char);
            }
        }
    }
}

pub fn quote_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push('"');
    escape_bytes_to(bytes, buf);
    buf.push('"');
}

#[cfg(test)]
mod test {
    use crate::lexer::str_lit::StrLit;
    use crate::text_format::escape_bytes_to;

    fn escape(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 4);
        escape_bytes_to(data, &mut s);
        s
    }

    fn unescape_string(escaped: &str) -> Vec<u8> {
        StrLit {
            escaped: escaped.to_owned(),
        }
        .decode_bytes()
        .expect("decode_bytes")
    }

    fn test_escape_unescape(text: &str, escaped: &str) {
        assert_eq!(text.as_bytes(), &unescape_string(escaped)[..]);
        assert_eq!(escaped, &escape(text.as_bytes())[..]);
    }

    #[test]
    fn test_print_to_bytes() {
        assert_eq!("ab", escape(b"ab"));
        assert_eq!("a\\\\023", escape(b"a\\023"));
        assert_eq!("a\\r\\n\\t \\'\\\"\\\\", escape(b"a\r\n\t '\"\\"));
        assert_eq!("\\344\\275\\240\\345\\245\\275", escape("你好".as_bytes()));
    }

    #[test]
    fn test_unescape_string() {
        test_escape_unescape("", "");
        test_escape_unescape("aa", "aa");
        test_escape_unescape("\n", "\\n");
        test_escape_unescape("\r", "\\r");
        test_escape_unescape("\t", "\\t");
        test_escape_unescape("你好", "\\344\\275\\240\\345\\245\\275");
        // hex
        assert_eq!(b"aaa\x01bbb", &unescape_string("aaa\\x01bbb")[..]);
        assert_eq!(b"aaa\xcdbbb", &unescape_string("aaa\\xCDbbb")[..]);
        assert_eq!(b"aaa\xcdbbb", &unescape_string("aaa\\xCDbbb")[..]);
        // quotes
        assert_eq!(b"aaa\"bbb", &unescape_string("aaa\\\"bbb")[..]);
        assert_eq!(b"aaa\'bbb", &unescape_string("aaa\\\'bbb")[..]);
    }
}
