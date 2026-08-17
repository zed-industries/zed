use byteorder::{NetworkEndian, ReadBytesExt};
use log::*;
use std::{
    borrow::Cow,
    default::Default,
    fmt,
    io::{Cursor, ErrorKind, Read, Write},
    result::Result as StdResult,
    str::Utf8Error,
    string::{FromUtf8Error, String},
};

use super::{
    coding::{CloseCode, Control, Data, OpCode},
    mask::{apply_mask, generate_mask},
};
use crate::error::{Error, ProtocolError, Result};

/// A struct representing the close command.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CloseFrame<'t> {
    /// The reason as a code.
    pub code: CloseCode,
    /// The reason as text string.
    pub reason: Cow<'t, str>,
}

impl<'t> CloseFrame<'t> {
    /// Convert into a owned string.
    pub fn into_owned(self) -> CloseFrame<'static> {
        CloseFrame { code: self.code, reason: self.reason.into_owned().into() }
    }
}

impl<'t> fmt::Display for CloseFrame<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.reason, self.code)
    }
}

/// A struct representing a WebSocket frame header.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FrameHeader {
    /// Indicates that the frame is the last one of a possibly fragmented message.
    pub is_final: bool,
    /// Reserved for protocol extensions.
    pub rsv1: bool,
    /// Reserved for protocol extensions.
    pub rsv2: bool,
    /// Reserved for protocol extensions.
    pub rsv3: bool,
    /// WebSocket protocol opcode.
    pub opcode: OpCode,
    /// A frame mask, if any.
    pub mask: Option<[u8; 4]>,
}

impl Default for FrameHeader {
    fn default() -> Self {
        FrameHeader {
            is_final: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode: OpCode::Control(Control::Close),
            mask: None,
        }
    }
}

impl FrameHeader {
    /// Parse a header from an input stream.
    /// Returns `None` if insufficient data and does not consume anything in this case.
    /// Payload size is returned along with the header.
    pub fn parse(cursor: &mut Cursor<impl AsRef<[u8]>>) -> Result<Option<(Self, u64)>> {
        let initial = cursor.position();
        match Self::parse_internal(cursor) {
            ret @ Ok(None) => {
                cursor.set_position(initial);
                ret
            }
            ret => ret,
        }
    }

    /// Get the size of the header formatted with given payload length.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self, length: u64) -> usize {
        2 + LengthFormat::for_length(length).extra_bytes() + if self.mask.is_some() { 4 } else { 0 }
    }

    /// Format a header for given payload size.
    pub fn format(&self, length: u64, output: &mut impl Write) -> Result<()> {
        let code: u8 = self.opcode.into();

        let one = {
            code | if self.is_final { 0x80 } else { 0 }
                | if self.rsv1 { 0x40 } else { 0 }
                | if self.rsv2 { 0x20 } else { 0 }
                | if self.rsv3 { 0x10 } else { 0 }
        };

        let lenfmt = LengthFormat::for_length(length);

        let two = { lenfmt.length_byte() | if self.mask.is_some() { 0x80 } else { 0 } };

        output.write_all(&[one, two])?;
        match lenfmt {
            LengthFormat::U8(_) => (),
            LengthFormat::U16 => {
                output.write_all(&(length as u16).to_be_bytes())?;
            }
            LengthFormat::U64 => {
                output.write_all(&length.to_be_bytes())?;
            }
        }

        if let Some(ref mask) = self.mask {
            output.write_all(mask)?
        }

        Ok(())
    }

    /// Generate a random frame mask and store this in the header.
    ///
    /// Of course this does not change frame contents. It just generates a mask.
    pub(crate) fn set_random_mask(&mut self) {
        self.mask = Some(generate_mask())
    }
}

impl FrameHeader {
    /// Internal parse engine.
    /// Returns `None` if insufficient data.
    /// Payload size is returned along with the header.
    fn parse_internal(cursor: &mut impl Read) -> Result<Option<(Self, u64)>> {
        let (first, second) = {
            let mut head = [0u8; 2];
            if cursor.read(&mut head)? != 2 {
                return Ok(None);
            }
            trace!("Parsed headers {:?}", head);
            (head[0], head[1])
        };

        trace!("First: {:b}", first);
        trace!("Second: {:b}", second);

        let is_final = first & 0x80 != 0;

        let rsv1 = first & 0x40 != 0;
        let rsv2 = first & 0x20 != 0;
        let rsv3 = first & 0x10 != 0;

        let opcode = OpCode::from(first & 0x0F);
        trace!("Opcode: {:?}", opcode);

        let masked = second & 0x80 != 0;
        trace!("Masked: {:?}", masked);

        let length = {
            let length_byte = second & 0x7F;
            let length_length = LengthFormat::for_byte(length_byte).extra_bytes();
            if length_length > 0 {
                match cursor.read_uint::<NetworkEndian>(length_length) {
                    Err(ref err) if err.kind() == ErrorKind::UnexpectedEof => {
                        return Ok(None);
                    }
                    Err(err) => {
                        return Err(err.into());
                    }
                    Ok(read) => read,
                }
            } else {
                u64::from(length_byte)
            }
        };

        let mask = if masked {
            let mut mask_bytes = [0u8; 4];
            if cursor.read(&mut mask_bytes)? != 4 {
                return Ok(None);
            } else {
                Some(mask_bytes)
            }
        } else {
            None
        };

        // Disallow bad opcode
        match opcode {
            OpCode::Control(Control::Reserved(_)) | OpCode::Data(Data::Reserved(_)) => {
                return Err(Error::Protocol(ProtocolError::InvalidOpcode(first & 0x0F)))
            }
            _ => (),
        }

        let hdr = FrameHeader { is_final, rsv1, rsv2, rsv3, opcode, mask };

        Ok(Some((hdr, length)))
    }
}

/// A struct representing a WebSocket frame.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame {
    header: FrameHeader,
    payload: Vec<u8>,
}

impl Frame {
    /// Get the length of the frame.
    /// This is the length of the header + the length of the payload.
    #[inline]
    pub fn len(&self) -> usize {
        let length = self.payload.len();
        self.header.len(length as u64) + length
    }

    /// Check if the frame is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a reference to the frame's header.
    #[inline]
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }

    /// Get a mutable reference to the frame's header.
    #[inline]
    pub fn header_mut(&mut self) -> &mut FrameHeader {
        &mut self.header
    }

    /// Get a reference to the frame's payload.
    #[inline]
    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    /// Get a mutable reference to the frame's payload.
    #[inline]
    pub fn payload_mut(&mut self) -> &mut Vec<u8> {
        &mut self.payload
    }

    /// Test whether the frame is masked.
    #[inline]
    pub(crate) fn is_masked(&self) -> bool {
        self.header.mask.is_some()
    }

    /// Generate a random mask for the frame.
    ///
    /// This just generates a mask, payload is not changed. The actual masking is performed
    /// either on `format()` or on `apply_mask()` call.
    #[inline]
    pub(crate) fn set_random_mask(&mut self) {
        self.header.set_random_mask()
    }

    /// This method unmasks the payload and should only be called on frames that are actually
    /// masked. In other words, those frames that have just been received from a client endpoint.
    #[inline]
    pub(crate) fn apply_mask(&mut self) {
        if let Some(mask) = self.header.mask.take() {
            apply_mask(&mut self.payload, mask)
        }
    }

    /// Consume the frame into its payload as binary.
    #[inline]
    pub fn into_data(self) -> Vec<u8> {
        self.payload
    }

    /// Consume the frame into its payload as string.
    #[inline]
    pub fn into_string(self) -> StdResult<String, FromUtf8Error> {
        String::from_utf8(self.payload)
    }

    /// Get frame payload as `&str`.
    #[inline]
    pub fn to_text(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.payload)
    }

    /// Consume the frame into a closing frame.
    #[inline]
    pub(crate) fn into_close(self) -> Result<Option<CloseFrame<'static>>> {
        match self.payload.len() {
            0 => Ok(None),
            1 => Err(Error::Protocol(ProtocolError::InvalidCloseSequence)),
            _ => {
                let mut data = self.payload;
                let code = u16::from_be_bytes([data[0], data[1]]).into();
                data.drain(0..2);
                let text = String::from_utf8(data)?;
                Ok(Some(CloseFrame { code, reason: text.into() }))
            }
        }
    }

    /// Create a new data frame.
    #[inline]
    pub fn message(data: Vec<u8>, opcode: OpCode, is_final: bool) -> Frame {
        debug_assert!(matches!(opcode, OpCode::Data(_)), "Invalid opcode for data frame.");

        Frame { header: FrameHeader { is_final, opcode, ..FrameHeader::default() }, payload: data }
    }

    /// Create a new Pong control frame.
    #[inline]
    pub fn pong(data: Vec<u8>) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Control(Control::Pong),
                ..FrameHeader::default()
            },
            payload: data,
        }
    }

    /// Create a new Ping control frame.
    #[inline]
    pub fn ping(data: Vec<u8>) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Control(Control::Ping),
                ..FrameHeader::default()
            },
            payload: data,
        }
    }

    /// Create a new Close control frame.
    #[inline]
    pub fn close(msg: Option<CloseFrame>) -> Frame {
        let payload = if let Some(CloseFrame { code, reason }) = msg {
            let mut p = Vec::with_capacity(reason.as_bytes().len() + 2);
            p.extend(u16::from(code).to_be_bytes());
            p.extend_from_slice(reason.as_bytes());
            p
        } else {
            Vec::new()
        };

        Frame { header: FrameHeader::default(), payload }
    }

    /// Create a frame from given header and data.
    pub fn from_payload(header: FrameHeader, payload: Vec<u8>) -> Self {
        Frame { header, payload }
    }

    /// Write a frame out to a buffer
    pub fn format(mut self, output: &mut impl Write) -> Result<()> {
        self.header.format(self.payload.len() as u64, output)?;
        self.apply_mask();
        output.write_all(self.payload())?;
        Ok(())
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        write!(
            f,
            "
<FRAME>
final: {}
reserved: {} {} {}
opcode: {}
length: {}
payload length: {}
payload: 0x{}
            ",
            self.header.is_final,
            self.header.rsv1,
            self.header.rsv2,
            self.header.rsv3,
            self.header.opcode,
            // self.mask.map(|mask| format!("{:?}", mask)).unwrap_or("NONE".into()),
            self.len(),
            self.payload.len(),
            self.payload.iter().fold(String::new(), |mut output, byte| {
                _ = write!(output, "{byte:02x}");
                output
            })
        )
    }
}

/// Handling of the length format.
enum LengthFormat {
    U8(u8),
    U16,
    U64,
}

impl LengthFormat {
    /// Get the length format for a given data size.
    #[inline]
    fn for_length(length: u64) -> Self {
        if length < 126 {
            LengthFormat::U8(length as u8)
        } else if length < 65536 {
            LengthFormat::U16
        } else {
            LengthFormat::U64
        }
    }

    /// Get the size of the length encoding.
    #[inline]
    fn extra_bytes(&self) -> usize {
        match *self {
            LengthFormat::U8(_) => 0,
            LengthFormat::U16 => 2,
            LengthFormat::U64 => 8,
        }
    }

    /// Encode the given length.
    #[inline]
    fn length_byte(&self) -> u8 {
        match *self {
            LengthFormat::U8(b) => b,
            LengthFormat::U16 => 126,
            LengthFormat::U64 => 127,
        }
    }

    /// Get the length format for a given length byte.
    #[inline]
    fn for_byte(byte: u8) -> Self {
        match byte & 0x7F {
            126 => LengthFormat::U16,
            127 => LengthFormat::U64,
            b => LengthFormat::U8(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::coding::{Data, OpCode};
    use std::io::Cursor;

    #[test]
    fn parse() {
        let mut raw: Cursor<Vec<u8>> =
            Cursor::new(vec![0x82, 0x07, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        let (header, length) = FrameHeader::parse(&mut raw).unwrap().unwrap();
        assert_eq!(length, 7);
        let mut payload = Vec::new();
        raw.read_to_end(&mut payload).unwrap();
        let frame = Frame::from_payload(header, payload);
        assert_eq!(frame.into_data(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    }

    #[test]
    fn format() {
        let frame = Frame::ping(vec![0x01, 0x02]);
        let mut buf = Vec::with_capacity(frame.len());
        frame.format(&mut buf).unwrap();
        assert_eq!(buf, vec![0x89, 0x02, 0x01, 0x02]);
    }

    #[test]
    fn display() {
        let f = Frame::message("hi there".into(), OpCode::Data(Data::Text), true);
        let view = format!("{}", f);
        assert!(view.contains("payload:"));
    }
}
