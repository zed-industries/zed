use crate::ParseError;

use std::io;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Line<'a> {
    /// Either a comment or an empty line.
    Nothing,
    /// A section header, e.g. `[something.rs]`
    Section(&'a str),
    /// A propery/key-value pair, e.g. `indent_size = 2`
    Pair(&'a str, &'a str),
}

type LineReadResult<'a> = Result<Line<'a>, ParseError>;

/// Identifies the line type and extracts relevant slices.
/// Does not do any lowercasing or anything beyond basic validation.
///
/// It's usually not necessary to call this function directly.
///
/// If the `allow-empty-values` feature is enabled,
/// lines with a key but no value will be returned as a [`Line::Pair`].
/// Otherwise, they are considered invalid.
pub fn parse_line(line: &str) -> LineReadResult<'_> {
    let mut l = line.trim_start();
    if l.starts_with(is_comment) {
        return Ok(Line::Nothing);
    }

    // check for trailing comments after section headers
    let last_closing_bracket = l.rfind(']');
    let last_comment = l.rfind(is_comment);

    if let (Some(bracket), Some(comment)) = (last_closing_bracket, last_comment) {
        if comment > bracket {
            // there is a comment following a closing bracket, trim it.
            l = l[0..comment].as_ref();
        }
    }

    l = l.trim_end();
    if l.is_empty() {
        Ok(Line::Nothing)
    } else if let Some(s) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        if s.is_empty() {
            Err(ParseError::InvalidLine)
        } else {
            Ok(Line::Section(s))
        }
    } else if let Some((key_raw, val_raw)) = l.split_once('=') {
        let key = key_raw.trim_end();
        let val = val_raw.trim_start();
        match (key.is_empty(), val.is_empty()) {
            (true, _) => Err(ParseError::InvalidLine),
            (false, true) => {
                #[cfg(feature = "allow-empty-values")]
                {
                    Ok(Line::Pair(key.trim_end(), val))
                }
                #[cfg(not(feature = "allow-empty-values"))]
                {
                    Err(ParseError::InvalidLine)
                }
            }
            (false, false) => Ok(Line::Pair(key.trim_end(), val.trim_start())),
        }
    } else {
        Err(ParseError::InvalidLine)
    }
}

/// Struct for extracting valid INI-like lines from text,
/// suitable for initial parsing of individual .editorconfig files.
/// Does minimal validation and does not modify the input text in any way.
pub struct LineReader<R: io::BufRead> {
    ticker: usize,
    line: String,
    reader: R,
}

impl<R: io::BufRead> LineReader<R> {
    /// Constructs a new line reader.
    pub fn new(r: R) -> LineReader<R> {
        LineReader {
            ticker: 0,
            line: String::with_capacity(256),
            reader: r,
        }
    }

    /// Returns the line number of the contained line.
    pub fn line_no(&self) -> usize {
        self.ticker
    }

    /// Returns a reference to the contained line.
    pub fn line(&self) -> &str {
        self.line.as_str()
    }

    /// Parses the contained line using [`parse_line`].
    ///
    /// It's usually not necessary to call this method.
    /// See [`LineReader::next`].
    pub fn reparse(&self) -> LineReadResult<'_> {
        parse_line(self.line())
    }

    /// Reads and parses the next line from the stream.
    pub fn next_line(&mut self) -> LineReadResult<'_> {
        self.line.clear();
        match self.reader.read_line(&mut self.line) {
            Err(e) => Err(ParseError::Io(e)),
            Ok(0) => Err(ParseError::Eof),
            Ok(_) => {
                self.ticker += 1;
                if self.ticker == 1 {
                    parse_line(self.line.strip_prefix('\u{FEFF}').unwrap_or(&self.line))
                } else {
                    self.reparse()
                }
            }
        }
    }
}

fn is_comment(c: char) -> bool {
    c == ';' || c == '#'
}
