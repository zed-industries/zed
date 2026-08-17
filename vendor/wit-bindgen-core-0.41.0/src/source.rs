use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::{self, Write};
use std::ops::Deref;

#[derive(Default)]
pub struct Files {
    files: BTreeMap<String, Vec<u8>>,
}

impl Files {
    pub fn push(&mut self, name: &str, contents: &[u8]) {
        match self.files.entry(name.to_owned()) {
            Entry::Vacant(entry) => {
                entry.insert(contents.to_owned());
            }
            Entry::Occupied(ref mut entry) => {
                entry.get_mut().extend_from_slice(contents);
            }
        }
    }

    pub fn get_size(&mut self, name: &str) -> Option<usize> {
        self.files.get(name).map(|data| data.len())
    }

    pub fn remove(&mut self, name: &str) -> Option<Vec<u8>> {
        self.files.remove(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ [u8])> {
        self.files.iter().map(|p| (p.0.as_str(), p.1.as_slice()))
    }
}

#[derive(Default)]
pub struct Source {
    s: String,
    indent: usize,
    in_line_comment: bool,
    continuing_line: bool,
}

impl Source {
    pub fn append_src(&mut self, src: &Source) {
        self.s.push_str(&src.s);
        self.indent += src.indent;
        self.in_line_comment = src.in_line_comment;
    }

    pub fn push_str(&mut self, src: &str) {
        let lines = src.lines().collect::<Vec<_>>();
        for (i, line) in lines.iter().enumerate() {
            if !self.continuing_line {
                if !line.is_empty() {
                    for _ in 0..self.indent {
                        self.s.push_str("  ");
                    }
                }
                self.continuing_line = true;
            }

            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                self.in_line_comment = true;
            }

            if !self.in_line_comment {
                if trimmed.starts_with('}') && self.s.ends_with("  ") {
                    self.s.pop();
                    self.s.pop();
                }
            }
            self.s.push_str(if lines.len() == 1 {
                line
            } else {
                line.trim_start()
            });
            if !self.in_line_comment {
                if trimmed.ends_with('{') {
                    self.indent += 1;
                }
                if trimmed.starts_with('}') {
                    // Note that a `saturating_sub` is used here to prevent a panic
                    // here in the case of invalid code being generated in debug
                    // mode. It's typically easier to debug those issues through
                    // looking at the source code rather than getting a panic.
                    self.indent = self.indent.saturating_sub(1);
                }
            }
            if i != lines.len() - 1 || src.ends_with('\n') {
                self.newline();
            }
        }
    }

    pub fn indent(&mut self, amt: usize) {
        self.indent += amt;
    }

    pub fn deindent(&mut self, amt: usize) {
        self.indent -= amt;
    }

    /// Set the indentation level, and return the old level.
    pub fn set_indent(&mut self, amt: usize) -> usize {
        let old = self.indent;
        self.indent = amt;
        old
    }

    fn newline(&mut self) {
        self.in_line_comment = false;
        self.continuing_line = false;
        self.s.push('\n');
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        &mut self.s
    }

    pub fn as_str(&self) -> &str {
        &self.s
    }
}

impl Write for Source {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl Deref for Source {
    type Target = str;
    fn deref(&self) -> &str {
        &self.s
    }
}

impl From<Source> for String {
    fn from(s: Source) -> String {
        s.s
    }
}

/// Calls [`write!`] with the passed arguments and unwraps the result.
///
/// Useful for writing to things with infallible `Write` implementations like
/// `Source` and `String`.
///
/// [`write!`]: std::write
#[macro_export]
macro_rules! uwrite {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).unwrap()
    };
}

/// Calls [`writeln!`] with the passed arguments and unwraps the result.
///
/// Useful for writing to things with infallible `Write` implementations like
/// `Source` and `String`.
///
/// [`writeln!`]: std::writeln
#[macro_export]
macro_rules! uwriteln {
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::Source;

    #[test]
    fn simple_append() {
        let mut s = Source::default();
        s.push_str("x");
        assert_eq!(s.s, "x");
        s.push_str("y");
        assert_eq!(s.s, "xy");
        s.push_str("z ");
        assert_eq!(s.s, "xyz ");
        s.push_str(" a ");
        assert_eq!(s.s, "xyz  a ");
        s.push_str("\na");
        assert_eq!(s.s, "xyz  a \na");
    }

    #[test]
    fn newline_remap() {
        let mut s = Source::default();
        s.push_str("function() {\n");
        s.push_str("y\n");
        s.push_str("}\n");
        assert_eq!(s.s, "function() {\n  y\n}\n");
    }

    #[test]
    fn if_else() {
        let mut s = Source::default();
        s.push_str("if() {\n");
        s.push_str("y\n");
        s.push_str("} else if () {\n");
        s.push_str("z\n");
        s.push_str("}\n");
        assert_eq!(s.s, "if() {\n  y\n} else if () {\n  z\n}\n");
    }

    #[test]
    fn trim_ws() {
        let mut s = Source::default();
        s.push_str(
            "function() {
                x
        }",
        );
        assert_eq!(s.s, "function() {\n  x\n}");
    }
}
