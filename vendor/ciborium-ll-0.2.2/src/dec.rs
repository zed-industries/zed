use super::*;

use ciborium_io::Read;

/// An error that occurred while decoding
#[derive(Debug)]
pub enum Error<T> {
    /// An error occurred while reading bytes
    ///
    /// Contains the underlying error returned while reading.
    Io(T),

    /// An error occurred while parsing bytes
    ///
    /// Contains the offset into the stream where the syntax error occurred.
    Syntax(usize),
}

impl<T> From<T> for Error<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Io(value)
    }
}

/// A decoder for deserializing CBOR items
///
/// This decoder manages the low-level decoding of CBOR items into `Header`
/// objects. It also contains utility functions for parsing segmented bytes
/// and text inputs.
pub struct Decoder<R: Read> {
    reader: R,
    offset: usize,
    buffer: Option<Title>,
}

impl<R: Read> From<R> for Decoder<R> {
    #[inline]
    fn from(value: R) -> Self {
        Self {
            reader: value,
            offset: 0,
            buffer: None,
        }
    }
}

impl<R: Read> Read for Decoder<R> {
    type Error = R::Error;

    #[inline]
    fn read_exact(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        assert!(self.buffer.is_none());
        self.reader.read_exact(data)?;
        self.offset += data.len();
        Ok(())
    }
}

impl<R: Read> Decoder<R> {
    #[inline]
    fn pull_title(&mut self) -> Result<Title, Error<R::Error>> {
        if let Some(title) = self.buffer.take() {
            self.offset += title.1.as_ref().len() + 1;
            return Ok(title);
        }

        let mut prefix = [0u8; 1];
        self.read_exact(&mut prefix[..])?;

        let major = match prefix[0] >> 5 {
            0 => Major::Positive,
            1 => Major::Negative,
            2 => Major::Bytes,
            3 => Major::Text,
            4 => Major::Array,
            5 => Major::Map,
            6 => Major::Tag,
            7 => Major::Other,
            _ => unreachable!(),
        };

        let mut minor = match prefix[0] & 0b00011111 {
            x if x < 24 => Minor::This(x),
            24 => Minor::Next1([0; 1]),
            25 => Minor::Next2([0; 2]),
            26 => Minor::Next4([0; 4]),
            27 => Minor::Next8([0; 8]),
            31 => Minor::More,
            _ => return Err(Error::Syntax(self.offset - 1)),
        };

        self.read_exact(minor.as_mut())?;
        Ok(Title(major, minor))
    }

    #[inline]
    fn push_title(&mut self, item: Title) {
        assert!(self.buffer.is_none());
        self.buffer = Some(item);
        self.offset -= item.1.as_ref().len() + 1;
    }

    /// Pulls the next header from the input
    #[inline]
    pub fn pull(&mut self) -> Result<Header, Error<R::Error>> {
        let offset = self.offset;
        self.pull_title()?
            .try_into()
            .map_err(|_| Error::Syntax(offset))
    }

    /// Push a single header into the input buffer
    ///
    /// # Panics
    ///
    /// This function panics if called while there is already a header in the
    /// input buffer. You should take care to call this function only after
    /// pulling a header to ensure there is nothing in the input buffer.
    #[inline]
    pub fn push(&mut self, item: Header) {
        self.push_title(Title::from(item))
    }

    /// Gets the current byte offset into the stream
    ///
    /// The offset starts at zero when the decoder is created. Therefore, if
    /// bytes were already read from the reader before the decoder was created,
    /// you must account for this.
    #[inline]
    pub fn offset(&mut self) -> usize {
        self.offset
    }

    /// Process an incoming bytes item
    ///
    /// In CBOR, bytes can be segmented. The logic for this can be a bit tricky,
    /// so we encapsulate that logic using this function. This function **MUST**
    /// be called immediately after first pulling a `Header::Bytes(len)` from
    /// the wire and `len` must be provided to this function from that value.
    ///
    /// The `buf` parameter provides a buffer used when reading in the segmented
    /// bytes. A large buffer will result in fewer calls to read incoming bytes
    /// at the cost of memory usage. You should consider this trade off when
    /// deciding the size of your buffer.
    #[inline]
    pub fn bytes(&mut self, len: Option<usize>) -> Segments<R, crate::seg::Bytes> {
        self.push(Header::Bytes(len));
        Segments::new(self, |header| match header {
            Header::Bytes(len) => Ok(len),
            _ => Err(()),
        })
    }

    /// Process an incoming text item
    ///
    /// In CBOR, text can be segmented. The logic for this can be a bit tricky,
    /// so we encapsulate that logic using this function. This function **MUST**
    /// be called immediately after first pulling a `Header::Text(len)` from
    /// the wire and `len` must be provided to this function from that value.
    ///
    /// The `buf` parameter provides a buffer used when reading in the segmented
    /// text. A large buffer will result in fewer calls to read incoming bytes
    /// at the cost of memory usage. You should consider this trade off when
    /// deciding the size of your buffer.
    #[inline]
    pub fn text(&mut self, len: Option<usize>) -> Segments<R, crate::seg::Text> {
        self.push(Header::Text(len));
        Segments::new(self, |header| match header {
            Header::Text(len) => Ok(len),
            _ => Err(()),
        })
    }
}
