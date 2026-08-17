use super::*;

use ciborium_io::Write;

/// An encoder for serializing CBOR items
///
/// This structure wraps a writer and provides convenience functions for
/// writing `Header` objects to the wire.
pub struct Encoder<W: Write>(W);

impl<W: Write> From<W> for Encoder<W> {
    #[inline]
    fn from(value: W) -> Self {
        Self(value)
    }
}

impl<W: Write> Write for Encoder<W> {
    type Error = W::Error;

    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(data)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush()
    }
}

impl<W: Write> Encoder<W> {
    /// Push a `Header` to the wire
    #[inline]
    pub fn push(&mut self, header: Header) -> Result<(), W::Error> {
        let title = Title::from(header);

        let major = match title.0 {
            Major::Positive => 0,
            Major::Negative => 1,
            Major::Bytes => 2,
            Major::Text => 3,
            Major::Array => 4,
            Major::Map => 5,
            Major::Tag => 6,
            Major::Other => 7,
        };

        let minor = match title.1 {
            Minor::This(x) => x,
            Minor::Next1(..) => 24,
            Minor::Next2(..) => 25,
            Minor::Next4(..) => 26,
            Minor::Next8(..) => 27,
            Minor::More => 31,
        };

        self.0.write_all(&[major << 5 | minor])?;
        self.0.write_all(title.1.as_ref())
    }

    /// Serialize a byte slice as CBOR
    ///
    /// Optionally, segment the output into `segment` size segments. Note that
    /// if `segment == Some(0)` it will be silently upgraded to `Some(1)`. This
    /// minimum value is highly inefficient and should not be relied upon.
    #[inline]
    pub fn bytes(
        &mut self,
        value: &[u8],
        segment: impl Into<Option<usize>>,
    ) -> Result<(), W::Error> {
        let max = segment.into().unwrap_or(value.len());
        let max = core::cmp::max(max, 1);

        if max >= value.len() {
            self.push(Header::Bytes(Some(value.len())))?;
            self.write_all(value)?;
        } else {
            self.push(Header::Bytes(None))?;

            for chunk in value.chunks(max) {
                self.push(Header::Bytes(Some(chunk.len())))?;
                self.write_all(chunk)?;
            }

            self.push(Header::Break)?;
        }

        Ok(())
    }

    /// Serialize a string slice as CBOR
    ///
    /// Optionally, segment the output into `segment` size segments. Note that
    /// since care is taken to ensure that each segment is itself a valid UTF-8
    /// string, if `segment` contains a value of less than 4, it will be
    /// silently upgraded to 4. This minimum value is highly inefficient and
    /// should not be relied upon.
    #[inline]
    pub fn text(&mut self, value: &str, segment: impl Into<Option<usize>>) -> Result<(), W::Error> {
        let max = segment.into().unwrap_or(value.len());
        let max = core::cmp::max(max, 4);

        if max >= value.len() {
            self.push(Header::Text(Some(value.len())))?;
            self.write_all(value.as_bytes())?;
        } else {
            self.push(Header::Text(None))?;

            let mut bytes = value.as_bytes();
            while !bytes.is_empty() {
                let mut len = core::cmp::min(bytes.len(), max);
                while len > 0 && core::str::from_utf8(&bytes[..len]).is_err() {
                    len -= 1
                }

                let (prefix, suffix) = bytes.split_at(len);
                self.push(Header::Text(Some(prefix.len())))?;
                self.write_all(prefix)?;
                bytes = suffix;
            }

            self.push(Header::Break)?;
        }

        Ok(())
    }
}
