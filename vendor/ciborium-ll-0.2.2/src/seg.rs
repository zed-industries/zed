use super::*;

use ciborium_io::Read;

use core::marker::PhantomData;

/// A parser for incoming segments
pub trait Parser: Default {
    /// The type of item that is parsed
    type Item: ?Sized;

    /// The parsing error that may occur
    type Error;

    /// The main parsing function
    ///
    /// This function processes the incoming bytes and returns the item.
    ///
    /// One important detail that **MUST NOT** be overlooked is that the
    /// parser may save data from a previous parsing attempt. The number of
    /// bytes saved is indicated by the `Parser::saved()` function. The saved
    /// bytes will be copied into the beginning of the `bytes` array before
    /// processing. Therefore, two requirements should be met.
    ///
    /// First, the incoming byte slice should be larger than the saved bytes.
    ///
    /// Second, the incoming byte slice should contain new bytes only after
    /// the saved byte prefix.
    ///
    /// If both criteria are met, this allows the parser to prepend its saved
    /// bytes without any additional allocation.
    fn parse<'a>(&mut self, bytes: &'a mut [u8]) -> Result<&'a Self::Item, Self::Error>;

    /// Indicates the number of saved bytes in the parser
    fn saved(&self) -> usize {
        0
    }
}

/// A bytes parser
///
/// No actual processing is performed and the input bytes are directly
/// returned. This implies that this parser never saves any bytes internally.
#[derive(Default)]
pub struct Bytes(());

impl Parser for Bytes {
    type Item = [u8];
    type Error = core::convert::Infallible;

    fn parse<'a>(&mut self, bytes: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        Ok(bytes)
    }
}

/// A text parser
///
/// This parser converts the input bytes to a `str`. This parser preserves
/// trailing invalid UTF-8 sequences in the case that chunking fell in the
/// middle of a valid UTF-8 character.
#[derive(Default)]
pub struct Text {
    stored: usize,
    buffer: [u8; 3],
}

impl Parser for Text {
    type Item = str;
    type Error = core::str::Utf8Error;

    fn parse<'a>(&mut self, bytes: &'a mut [u8]) -> Result<&'a str, Self::Error> {
        // If we cannot advance, return nothing.
        if bytes.len() <= self.stored {
            return Ok("");
        }

        // Copy previously invalid data into place.
        bytes[..self.stored].clone_from_slice(&self.buffer[..self.stored]);

        Ok(match core::str::from_utf8(bytes) {
            Ok(s) => {
                self.stored = 0;
                s
            }
            Err(e) => {
                let valid_len = e.valid_up_to();
                let invalid_len = bytes.len() - valid_len;

                // If the size of the invalid UTF-8 is large enough to hold
                // all valid UTF-8 characters, we have a syntax error.
                if invalid_len > self.buffer.len() {
                    return Err(e);
                }

                // Otherwise, store the invalid bytes for the next read cycle.
                self.buffer[..invalid_len].clone_from_slice(&bytes[valid_len..]);
                self.stored = invalid_len;

                // Decode the valid part of the string.
                core::str::from_utf8(&bytes[..valid_len]).unwrap()
            }
        })
    }

    fn saved(&self) -> usize {
        self.stored
    }
}

/// A CBOR segment
///
/// This type represents a single bytes or text segment on the wire. It can be
/// read out in parsed chunks based on the size of the input scratch buffer.
pub struct Segment<'r, R: Read, P: Parser> {
    reader: &'r mut Decoder<R>,
    unread: usize,
    offset: usize,
    parser: P,
}

impl<'r, R: Read, P: Parser> Segment<'r, R, P> {
    /// Gets the number of unprocessed bytes
    #[inline]
    pub fn left(&self) -> usize {
        self.unread + self.parser.saved()
    }

    /// Gets the next parsed chunk within the segment
    ///
    /// Returns `Ok(None)` when all chunks have been read.
    #[inline]
    pub fn pull<'a>(
        &mut self,
        buffer: &'a mut [u8],
    ) -> Result<Option<&'a P::Item>, Error<R::Error>> {
        use core::cmp::min;

        let prev = self.parser.saved();
        match self.unread {
            0 if prev == 0 => return Ok(None),
            0 => return Err(Error::Syntax(self.offset)),
            _ => (),
        }

        // Determine how many bytes to read.
        let size = min(buffer.len(), prev + self.unread);
        let full = &mut buffer[..size];
        let next = &mut full[min(size, prev)..];

        // Read additional bytes.
        self.reader.read_exact(next)?;
        self.unread -= next.len();

        self.parser
            .parse(full)
            .or(Err(Error::Syntax(self.offset)))
            .map(Some)
    }
}

/// A sequence of CBOR segments
///
/// CBOR allows for bytes or text items to be segmented. This type represents
/// the state of that segmented input stream.
pub struct Segments<'r, R: Read, P: Parser> {
    reader: &'r mut Decoder<R>,
    finish: bool,
    nested: usize,
    parser: PhantomData<P>,
    unwrap: fn(Header) -> Result<Option<usize>, ()>,
}

impl<'r, R: Read, P: Parser> Segments<'r, R, P> {
    #[inline]
    pub(crate) fn new(
        decoder: &'r mut Decoder<R>,
        unwrap: fn(Header) -> Result<Option<usize>, ()>,
    ) -> Self {
        Self {
            reader: decoder,
            finish: false,
            nested: 0,
            parser: PhantomData,
            unwrap,
        }
    }

    /// Gets the next segment in the stream
    ///
    /// Returns `Ok(None)` at the conclusion of the stream.
    #[inline]
    pub fn pull(&mut self) -> Result<Option<Segment<R, P>>, Error<R::Error>> {
        while !self.finish {
            let offset = self.reader.offset();
            match self.reader.pull()? {
                Header::Break if self.nested == 1 => return Ok(None),
                Header::Break if self.nested > 1 => self.nested -= 1,
                header => match (self.unwrap)(header) {
                    Err(..) => return Err(Error::Syntax(offset)),
                    Ok(None) => self.nested += 1,
                    Ok(Some(len)) => {
                        self.finish = self.nested == 0;
                        return Ok(Some(Segment {
                            reader: self.reader,
                            unread: len,
                            offset,
                            parser: P::default(),
                        }));
                    }
                },
            }
        }

        Ok(None)
    }
}
