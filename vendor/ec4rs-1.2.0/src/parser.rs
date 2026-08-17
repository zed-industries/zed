use crate::linereader::LineReader;
use crate::ParseError;
use crate::Section;
use std::io;
use std::path::Path;

/// Parser for the text of an EditorConfig file.
///
/// This struct wraps any [`BufRead`][std::io::BufRead].
/// It eagerly parses the preamble on construction.
/// [`Section`]s may then be parsed by calling [`ConfigParser::read_section`].
pub struct ConfigParser<R: io::BufRead> {
    /// Incidates if a `root = true` line was found in the preamble.
    pub is_root: bool,
    eof: bool,
    reader: LineReader<R>,
    #[cfg(feature = "track-source")]
    path: Option<std::sync::Arc<Path>>,
}

impl<R: io::Read> ConfigParser<io::BufReader<R>> {
    /// Convenience function for construction using an unbuffered [`io::Read`].
    ///
    /// See [`ConfigParser::new`].
    pub fn new_buffered(source: R) -> Result<ConfigParser<io::BufReader<R>>, ParseError> {
        Self::new(io::BufReader::new(source))
    }
    /// Convenience function for construction using an unbuffered [`io::Read`]
    /// which is assumed to be a file at `path`.
    ///
    /// See [`ConfigParser::new_with_path`].
    pub fn new_buffered_with_path(
        source: R,
        path: Option<impl Into<std::sync::Arc<Path>>>,
    ) -> Result<ConfigParser<io::BufReader<R>>, ParseError> {
        Self::new_with_path(io::BufReader::new(source), path)
    }
}

impl<R: io::BufRead> ConfigParser<R> {
    /// Constructs a new [`ConfigParser`] and reads the preamble from the provided source,
    /// which is assumed to be a file at `path`.
    ///
    /// Returns `Ok` if the preamble was parsed successfully,
    /// otherwise returns `Err` with the error that occurred during reading.
    ///
    /// If the `track-source` feature is enabled and `path` is `Some`,
    /// [`RawValue`][crate::rawvalue::RawValue]s produced by this parser will
    /// have their sources set appropriately.
    /// Otherwise, `path` is unused.
    pub fn new_with_path(
        buf_source: R,
        #[allow(unused)] path: Option<impl Into<std::sync::Arc<Path>>>,
    ) -> Result<ConfigParser<R>, ParseError> {
        let mut reader = LineReader::new(buf_source);
        let mut is_root = false;
        let eof = loop {
            use crate::linereader::Line;
            match reader.next_line() {
                Err(ParseError::Eof) => break true,
                Err(e) => return Err(e),
                Ok(Line::Nothing) => (),
                Ok(Line::Section(_)) => break false,
                Ok(Line::Pair(k, v)) => {
                    if "root".eq_ignore_ascii_case(k) {
                        if let Ok(b) = v.to_ascii_lowercase().parse::<bool>() {
                            is_root = b;
                        }
                    }
                    // Quietly ignore unknown properties.
                }
            }
        };
        #[cfg(feature = "track-source")]
        let path = path.map(Into::into);
        Ok(ConfigParser {
            is_root,
            eof,
            reader,
            #[cfg(feature = "track-source")]
            path,
        })
    }
    /// Constructs a new [`ConfigParser`] and reads the preamble from the provided source.
    ///
    /// Returns `Ok` if the preamble was parsed successfully,
    /// otherwise returns `Err` with the error that occurred during reading.
    pub fn new(buf_source: R) -> Result<ConfigParser<R>, ParseError> {
        Self::new_with_path(buf_source, Option::<std::sync::Arc<Path>>::None)
    }

    /// Returns `true` if there may be another section to read.
    pub fn has_more(&self) -> bool {
        self.eof
    }

    /// Returns the current line number.
    pub fn line_no(&self) -> usize {
        self.reader.line_no()
    }

    /// Parses a [`Section`], reading more if needed.
    pub fn read_section(&mut self) -> Result<Section, ParseError> {
        use crate::linereader::Line;
        if self.eof {
            return Err(ParseError::Eof);
        }
        if let Ok(Line::Section(header)) = self.reader.reparse() {
            let mut section = Section::new(header);
            loop {
                // Get line_no here to avoid borrowing issues, increment for 1-based indices.
                #[cfg(feature = "track-source")]
                let line_no = self.reader.line_no() + 1;
                match self.reader.next_line() {
                    Err(e) => {
                        self.eof = true;
                        break if matches!(e, ParseError::Eof) {
                            Ok(section)
                        } else {
                            Err(e)
                        };
                    }
                    Ok(Line::Section(_)) => break Ok(section),
                    Ok(Line::Nothing) => (),
                    Ok(Line::Pair(k, v)) => {
                        #[allow(unused_mut)]
                        let mut v = crate::rawvalue::RawValue::from(v.to_owned());
                        #[cfg(feature = "track-source")]
                        if let Some(path) = self.path.as_ref() {
                            v.set_source(path.clone(), line_no);
                        }
                        section.insert(k, v);
                    }
                }
            }
        } else {
            Err(ParseError::InvalidLine)
        }
    }
}

impl<R: io::BufRead> Iterator for ConfigParser<R> {
    type Item = Result<Section, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.read_section() {
            Ok(r) => Some(Ok(r)),
            Err(ParseError::Eof) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<R: io::BufRead> std::iter::FusedIterator for ConfigParser<R> {}

impl<R: io::BufRead> crate::PropertiesSource for &mut ConfigParser<R> {
    fn apply_to(
        self,
        props: &mut crate::Properties,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        let path = path.as_ref();
        for section_result in self {
            match section_result {
                Ok(section) => {
                    let _ = section.apply_to(props, path);
                }
                Err(error) => return Err(crate::Error::Parse(error)),
            }
        }
        Ok(())
    }
}
