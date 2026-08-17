pub mod str {
    use std::io::{Result, Write};

    #[derive(Debug)]
    pub struct StringWriter {
        buf: Vec<u8>,
    }

    impl Default for StringWriter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl StringWriter {
        pub fn new() -> StringWriter {
            StringWriter {
                buf: Vec::with_capacity(8 * 1024),
            }
        }

        pub fn into_string(self) -> String {
            if let Ok(s) = String::from_utf8(self.buf) {
                s
            } else {
                String::new()
            }
        }
    }

    impl Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    /// See https://github.com/handlebars-lang/handlebars.js/blob/37411901da42200ced8e1a7fc2f67bf83526b497/lib/handlebars/utils.js#L1
    pub fn escape_html(s: &str) -> String {
        let mut output = String::new();
        for c in s.chars() {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '"' => output.push_str("&quot;"),
                '&' => output.push_str("&amp;"),
                '\'' => output.push_str("&#x27;"),
                '`' => output.push_str("&#x60;"),
                '=' => output.push_str("&#x3D;"),
                _ => output.push(c),
            }
        }
        output
    }

    /// add indent for lines but last
    pub fn with_indent(s: &str, indent: &str) -> String {
        let mut output = String::new();

        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            output.push(c);
            // check if c is not the last character, we don't append
            // indent for last line break
            if c == '\n' && it.peek().is_some() {
                output.push_str(indent);
            }
        }

        output
    }

    #[inline]
    pub(crate) fn whitespace_matcher(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    #[inline]
    pub(crate) fn newline_matcher(c: char) -> bool {
        c == '\n' || c == '\r'
    }

    #[inline]
    pub(crate) fn strip_first_newline(s: &str) -> &str {
        if let Some(s) = s.strip_prefix("\r\n") {
            s
        } else if let Some(s) = s.strip_prefix('\n') {
            s
        } else {
            s
        }
    }

    pub(crate) fn find_trailing_whitespace_chars(s: &str) -> Option<&str> {
        let trimmed = s.trim_end_matches(whitespace_matcher);
        if trimmed.len() == s.len() {
            None
        } else {
            Some(&s[trimmed.len()..])
        }
    }

    pub(crate) fn ends_with_empty_line(text: &str) -> bool {
        let s = text.trim_end_matches(whitespace_matcher);
        // also matches when text is just whitespaces
        s.ends_with(newline_matcher) || s.is_empty()
    }

    pub(crate) fn starts_with_empty_line(text: &str) -> bool {
        text.trim_start_matches(whitespace_matcher)
            .starts_with(newline_matcher)
    }

    #[cfg(test)]
    mod test {
        use crate::support::str::StringWriter;
        use std::io::Write;

        #[test]
        fn test_string_writer() {
            let mut sw = StringWriter::new();

            let _ = sw.write("hello".to_owned().into_bytes().as_ref());
            let _ = sw.write("world".to_owned().into_bytes().as_ref());

            let s = sw.into_string();
            assert_eq!(s, "helloworld".to_string());
        }
    }
}
