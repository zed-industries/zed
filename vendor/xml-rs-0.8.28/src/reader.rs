//! Contains high-level interface for a pull-based XML parser.
//!
//! The most important type in this module is `EventReader`, which provides an iterator
//! view for events in XML document.

use std::io::Read;
use std::iter::FusedIterator;
use std::result;

use crate::common::{Position, TextPosition};

pub use self::config::{ParserConfig, ParserConfig2};
pub use self::error::{Error, ErrorKind};
pub use self::events::XmlEvent;

use self::parser::PullParser;

mod config;
mod error;
mod events;
mod indexset;
mod lexer;
mod parser;

/// A result type yielded by `XmlReader`.
pub type Result<T, E = Error> = result::Result<T, E>;

/// A wrapper around an `std::io::Read` instance which provides pull-based XML parsing.
///
/// The reader should be wrapped in a `BufReader`, otherwise parsing may be very slow.
pub struct EventReader<R: Read> {
    source: R,
    parser: PullParser,
}

impl<R: Read> EventReader<R> {
    /// Creates a new reader, consuming the given stream. The reader should be wrapped in a `BufReader`, otherwise parsing may be very slow.
    #[inline]
    pub fn new(source: R) -> Self {
        Self::new_with_config(source, ParserConfig2::new())
    }

    /// Creates a new reader with the provded configuration, consuming the given stream. The reader should be wrapped in a `BufReader`, otherwise parsing may be very slow.
    #[inline]
    pub fn new_with_config(source: R, config: impl Into<ParserConfig2>) -> Self {
        Self { source, parser: PullParser::new(config) }
    }

    /// Pulls and returns next XML event from the stream.
    ///
    /// If this returns [Err] or [`XmlEvent::EndDocument`] then further calls to
    /// this method will return this event again.
    #[inline]
    pub fn next(&mut self) -> Result<XmlEvent> {
        self.parser.next(&mut self.source)
    }

    /// Skips all XML events until the next end tag at the current level.
    ///
    /// Convenience function that is useful for the case where you have
    /// encountered a start tag that is of no interest and want to
    /// skip the entire XML subtree until the corresponding end tag.
    #[inline]
    pub fn skip(&mut self) -> Result<()> {
        let mut depth = 1;

        while depth > 0 {
            match self.next()? {
                XmlEvent::StartElement { .. } => depth += 1,
                XmlEvent::EndElement { .. } => depth -= 1,
                XmlEvent::EndDocument => return Err(Error {
                    kind: ErrorKind::UnexpectedEof,
                    pos: self.parser.position(),
                }),
                _ => {},
            }
        }

        Ok(())
    }

    /// Access underlying reader
    ///
    /// Using it directly while the event reader is parsing is not recommended
    pub fn source(&self) -> &R { &self.source }

    /// Access underlying reader
    ///
    /// Using it directly while the event reader is parsing is not recommended
    pub fn source_mut(&mut self) -> &mut R { &mut self.source }

    /// Unwraps this `EventReader`, returning the underlying reader.
    ///
    /// Note that this operation is destructive; unwrapping the reader and wrapping it
    /// again with `EventReader::new()` will create a fresh reader which will attempt
    /// to parse an XML document from the beginning.
    pub fn into_inner(self) -> R {
        self.source
    }

    /// Returns the DOCTYPE of the document if it has already been seen
    ///
    /// Available only after the root `StartElement` event
    #[inline]
    pub fn doctype(&self) -> Option<&str> {
        self.parser.doctype()
    }
}

impl<B: Read> Position for EventReader<B> {
    /// Returns the position of the last event produced by the reader.
    #[inline]
    fn position(&self) -> TextPosition {
        self.parser.position()
    }
}

impl<R: Read> IntoIterator for EventReader<R> {
    type IntoIter = Events<R>;
    type Item = Result<XmlEvent>;

    fn into_iter(self) -> Events<R> {
        Events { reader: self, finished: false }
    }
}

/// An iterator over XML events created from some type implementing `Read`.
///
/// When the next event is `xml::event::Error` or `xml::event::EndDocument`, then
/// it will be returned by the iterator once, and then it will stop producing events.
pub struct Events<R: Read> {
    reader: EventReader<R>,
    finished: bool,
}

impl<R: Read> Events<R> {
    /// Unwraps the iterator, returning the internal `EventReader`.
    #[inline]
    pub fn into_inner(self) -> EventReader<R> {
        self.reader
    }

    /// Access the underlying reader
    ///
    /// It's not recommended to use it while the events are still being parsed
    pub fn source(&self) -> &R { &self.reader.source }

    /// Access the underlying reader
    ///
    /// It's not recommended to use it while the events are still being parsed
    pub fn source_mut(&mut self) -> &mut R { &mut self.reader.source }
}

impl<R: Read> FusedIterator for Events<R> {
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<XmlEvent>;

    #[inline]
    fn next(&mut self) -> Option<Result<XmlEvent>> {
        if self.finished && !self.reader.parser.is_ignoring_end_of_stream() {
            None
        } else {
            let ev = self.reader.next();
            if let Ok(XmlEvent::EndDocument) | Err(_) = ev {
                self.finished = true;
            }
            Some(ev)
        }
    }
}

impl<'r> EventReader<&'r [u8]> {
    /// A convenience method to create an `XmlReader` from a string slice.
    #[inline]
    #[must_use]
    pub fn from_str(source: &'r str) -> Self {
        EventReader::new(source.as_bytes())
    }
}
