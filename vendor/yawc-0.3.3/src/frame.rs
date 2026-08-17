//! # Frame
//!
//! The `frame` module implements WebSocket frames as defined in [RFC 6455 Section 5.2](https://datatracker.ietf.org/doc/html/rfc6455#section-5.2),
//! providing the core building blocks for WebSocket communication. Each frame represents an atomic unit of data transmission,
//! containing both the payload and protocol-level metadata.
//!
//! ## Overview of WebSocket Frames
//!
//! WebSocket messages are transmitted as a sequence of frames. The module provides two main frame implementations:
//!
//! - [`Frame`]: Full frame with all protocol metadata and masking capabilities
//!
//! ### Frame Binary Format
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-------+-+-------------+-------------------------------+
//! |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
//! |I|S|S|S|  (4)  |A|     (7)     |         (16 or 64 bits)       |
//! |N|V|V|V|       |S|             |                               |
//! | |1|2|3|       |K|             |                               |
//! +-+-+-+-+-------+-+-------------+-------------------------------+
//! |        Extended payload length continued, if payload len == 127|
//! +---------------------------------------------------------------+
//! |                               |   Masking-key, if MASK set to 1|
//! +-------------------------------+-------------------------------+
//! |     Masking-key (continued)       |          Payload Data      |
//! +-----------------------------------+ - - - - - - - - - - - - - -+
//! :                     Payload Data continued ...                :
//! +---------------------------------------------------------------+
//! ```
//!
//! Frames come in two categories:
//!
//! - **Data Frames**: Carry application payload with:
//!   - `OpCode::Text`: UTF-8 text data
//!   - `OpCode::Binary`: Raw binary data
//!   - `OpCode::Continuation`: Continuation of a fragmented message
//! - **Control Frames**: Manage the connection with:
//!   - `OpCode::Close`: Initiates connection closure with optional status code and reason
//!   - `OpCode::Ping`: Checks connection liveness, requires a Pong response
//!   - `OpCode::Pong`: Responds to Ping frames
//!
//! ## Frame Structure
//!
//! The [`Frame`] struct implements the full WebSocket frame format with:
//!
//! - `fin`: Final fragment flag (1 bit)
//! - `opcode`: Frame type identifier (4 bits)
//! - `mask`: Optional 32-bit XOR masking key
//! - `payload`: Frame data as `BytesMut`
//! - `is_compressed`: Per-message compression flag (1 bit, RSV1)
//!
//! ### Frame Construction
//!
//! The module provides ergonomic constructors for common frame types:
//!
//! ```rust
//! use yawc::frame::Frame;
//! use bytes::Bytes;
//! use yawc::close::CloseCode;
//!
//! // Text frame with UTF-8 payload
//! let text_frame = Frame::text("Hello, WebSocket!");
//!
//! // Binary frame
//! let binary_frame = Frame::binary(vec![1, 2, 3, 4]);
//!
//! // Control frames
//! let ping = Frame::ping("Ping payload");
//! let pong = Frame::pong("Pong response");
//! let close = Frame::close(CloseCode::Normal, b"Normal closure");
//!
//! // Fragmented message
//! let first = Frame::text("Hello, ").with_fin(false);
//! let continuation = Frame::continuation("World!");
//! ```
//!
//! ## Frame Processing
//!
//! Frames support automatic masking (required for client-to-server messages) and optional
//! per-message compression via the WebSocket permessage-deflate extension. The module handles:
//!
//! - Frame header parsing and serialization
//! - Payload masking and unmasking
//! - Message fragmentation and reassembly
//! - UTF-8 validation for text frames
//! - Compression and decompression of payloads (if permessage-deflate is enabled)
//!
//! For more details on the WebSocket protocol and frame handling, see [RFC 6455 Section 5](https://datatracker.ietf.org/doc/html/rfc6455#section-5).
#![cfg_attr(target_arch = "wasm32", allow(dead_code))] // Silence dead code warning for WASM

use bytes::Bytes;

use crate::{close::CloseCode, WebSocketError};

/// WebSocket operation code (OpCode) that determines the semantic meaning and handling of a frame.
///
/// Each variant represents a distinct frame type in the WebSocket protocol:
///
/// # Data Frame OpCodes
/// - `Continuation`: Continues a fragmented message started by another data frame
/// - `Text`: Contains UTF-8 encoded text data
/// - `Binary`: Contains raw binary data
///
/// # Control Frame OpCodes
/// - `Close`: Initiates or confirms connection closure
/// - `Ping`: Tests connection liveness, requiring a `Pong` response
/// - `Pong`: Responds to a `Ping` frame
///
/// # Reserved OpCodes
/// The ranges 0x3-0x7 and 0xB-0xF are reserved for future protocol extensions.
/// Frames with these opcodes will be rejected as invalid per RFC 6455.
///
/// The numeric values for each OpCode are defined in [RFC 6455, Section 11.8](https://datatracker.ietf.org/doc/html/rfc6455#section-11.8):
/// - Continuation = 0x0
/// - Text = 0x1
/// - Binary = 0x2
/// - Close = 0x8
/// - Ping = 0x9
/// - Pong = 0xA
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    Continuation,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl OpCode {
    /// Returns `true` if the `OpCode` represents a control frame (`Close`, `Ping`, or `Pong`).
    ///
    /// Control frames are used to manage the connection state and have special constraints:
    /// - Cannot be fragmented (the FIN bit must be set)
    /// - Payload must not exceed 125 bytes
    /// - Must be processed immediately rather than queued with data frames
    pub fn is_control(&self) -> bool {
        matches!(*self, OpCode::Close | OpCode::Ping | OpCode::Pong)
    }
}

impl TryFrom<u8> for OpCode {
    type Error = WebSocketError;

    /// Attempts to convert a byte value into an `OpCode`, returning an error if the byte does not match any valid `OpCode`.
    ///
    /// This conversion is typically used during frame parsing to interpret the opcode field from the frame header.
    /// Invalid opcodes (0x3-0x7 and 0xB-0xF) will result in a `WebSocketError::InvalidOpCode` error.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Continuation),
            0x1 => Ok(Self::Text),
            0x2 => Ok(Self::Binary),
            0x8 => Ok(Self::Close),
            0x9 => Ok(Self::Ping),
            0xA => Ok(Self::Pong),
            _ => Err(WebSocketError::InvalidOpCode(value)),
        }
    }
}

impl From<OpCode> for u8 {
    /// Converts an `OpCode` into its corresponding byte representation.
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::Continuation => 0x0,
            OpCode::Text => 0x1,
            OpCode::Binary => 0x2,
            OpCode::Close => 0x8,
            OpCode::Ping => 0x9,
            OpCode::Pong => 0xA,
        }
    }
}

/// Converts a `Frame` into a tuple of `(OpCode, Bytes)`.
///
/// This allows destructuring a frame into its component parts while
/// maintaining ownership of both the opcode and payload.
impl From<Frame> for (OpCode, Bytes) {
    fn from(val: Frame) -> Self {
        (val.opcode, val.payload)
    }
}

/// Converts a tuple of `(OpCode, Bytes)` to a `Frame`.
///
/// This allows constructing a `Frame` from an opcode and immutable bytes payload.
/// The frame will have `fin` set to `true` by default.
impl<T> From<(OpCode, T)> for Frame
where
    T: Into<Bytes>,
{
    fn from((opcode, payload): (OpCode, T)) -> Self {
        Self {
            fin: true,
            opcode,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }
}

/// Represents a WebSocket frame, encapsulating the data and metadata for message transmission.
///
/// **This is the primary type for creating and working with WebSocket frames.**
///
/// A WebSocket frame carries both the payload data and essential metadata. Use the builder
/// methods to create frames and accessor methods to inspect them safely
///
/// A WebSocket frame is the fundamental unit of communication in the WebSocket protocol, carrying both
/// the payload data and essential metadata. Frames can be categorized into two types:
///
/// 1. **Data Frames**
///    - Text frames containing UTF-8 encoded text
///    - Binary frames containing raw data
///    - Continuation frames for message fragmentation
///
/// 2. **Control Frames**
///    - Close frames for connection termination
///    - Ping frames for connection liveness checks
///    - Pong frames for responding to pings
///
/// # Creating Frames
///
/// While frames can be constructed directly, it's recommended to use the provided factory methods:
/// ```rust
/// use yawc::frame::Frame;
/// use yawc::close::CloseCode;
///
/// let text_frame = Frame::text("Hello");
/// let binary_frame = Frame::binary(vec![1, 2, 3]);
/// let ping_frame = Frame::ping(vec![]);
/// let close_frame = Frame::close(CloseCode::Normal, b"Goodbye");
/// ```
///
/// # Fields
/// - `fin`: Final fragment flag. When `true`, indicates this frame completes a message.
/// - `opcode`: Defines the frame type and interpretation (text, binary, control, etc).
/// - `mask`: Optional 32-bit XOR masking key required for client-to-server messages.
/// - `payload`: Frame payload data stored as dynamically sized bytes.
#[derive(Clone)]
pub struct Frame {
    /// Indicates if this is the final frame in a message.
    pub(crate) fin: bool,
    /// The opcode of the frame, defining its type.
    pub(crate) opcode: OpCode,
    /// Flag indicating whether the payload is compressed.
    pub(super) is_compressed: bool,
    /// The masking key for the frame, if any, used for security in client-to-server frames.
    pub(super) mask: Option<[u8; 4]>,
    /// The payload of the frame, containing the actual data.
    pub(crate) payload: Bytes,
}

pub(crate) const MAX_HEAD_SIZE: usize = 16;

impl Frame {
    /// Creates a text frame with the given payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Hello, WebSocket!");
    /// ```
    pub fn text(payload: impl Into<Bytes>) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Text,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Creates a binary frame with the given payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::binary(vec![1, 2, 3, 4]);
    /// ```
    pub fn binary(payload: impl Into<Bytes>) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Binary,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Creates a ping frame with the given payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::ping("optional payload");
    /// ```
    pub fn ping(payload: impl Into<Bytes>) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Ping,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Creates a pong frame with the given payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::pong("");
    /// ```
    pub fn pong(payload: impl Into<Bytes>) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Pong,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Creates a continuation frame with the given payload.
    ///
    /// Continuation frames are used for message fragmentation. The first fragment
    /// should be a Text or Binary frame with `fin` set to false, followed by
    /// zero or more Continuation frames, with the final frame having `fin` set to true.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// // First fragment (non-final text frame)
    /// let first = Frame::text("Hello, ").with_fin(false);
    ///
    /// // Middle fragment (continuation)
    /// let middle = Frame::continuation("World").with_fin(false);
    ///
    /// // Final fragment
    /// let last = Frame::continuation("!");
    /// ```
    pub fn continuation(payload: impl Into<Bytes>) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Continuation,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Sets the fin flag and returns self for method chaining.
    ///
    /// Use this to create fragmented messages. Set `fin` to `false` for
    /// non-final fragments.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let fragment = Frame::text("partial data").with_fin(false);
    /// assert!(!fragment.is_fin());
    /// ```
    pub fn with_fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    /// Creates a close frame with a close code and reason.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    /// use yawc::close::CloseCode;
    ///
    /// let frame = Frame::close(CloseCode::Normal, b"Goodbye");
    /// ```
    pub fn close(code: CloseCode, reason: impl AsRef<[u8]>) -> Self {
        let code16 = u16::from(code);
        let reason: &[u8] = reason.as_ref();
        let mut payload = Vec::with_capacity(2 + reason.len());
        payload.extend_from_slice(&code16.to_be_bytes());
        payload.extend_from_slice(reason);

        Self {
            fin: true,
            opcode: OpCode::Close,
            mask: None,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    pub(crate) fn into_fragments(self, partition: usize) -> impl Iterator<Item = Frame> {
        struct Split {
            index: usize,
            max_size: usize,
            frame: Option<Frame>,
        }

        impl Iterator for Split {
            type Item = Frame;

            fn next(&mut self) -> Option<Self::Item> {
                let mut frame = self.frame.take()?;
                if frame.payload.len() <= self.max_size {
                    if self.index != 0 {
                        // prepare the last frame
                        frame.set_fin(true);
                        frame.opcode = OpCode::Continuation;
                    }
                    // just return the remaining frame
                    Some(frame)
                } else {
                    let is_first = self.index == 0;
                    self.index += 1;
                    let chunk = frame.payload.split_to(self.max_size);
                    let opcode = if is_first {
                        frame.opcode
                    } else {
                        OpCode::Continuation
                    };
                    let mask = frame.mask;
                    self.frame = Some(frame);
                    Some(Frame::new(false, opcode, mask, chunk))
                }
            }
        }

        Split {
            index: 0,
            max_size: partition,
            frame: Some(self),
        }
    }

    /// Creates a close frame with a raw payload (for internal use).
    pub(crate) fn close_raw<T: Into<Bytes>>(payload: T) -> Self {
        Self {
            fin: true,
            opcode: OpCode::Close,
            mask: None,
            is_compressed: false,
            payload: payload.into(),
        }
    }

    /// Low-level constructor for creating frames with full control.
    ///
    /// Most users should use the helper methods like `Frame::text()`, `Frame::binary()`, etc.
    /// This constructor is for advanced use cases requiring precise control over frame flags.
    ///
    /// # Parameters
    /// - `fin`: Indicates if this frame is the final fragment in a message.
    /// - `opcode`: The operation code of the frame, defining its type (e.g., Text, Binary, Close).
    /// - `mask`: Optional 4-byte masking key, typically used in client-to-server frames.
    /// - `payload`: The frame payload data.
    ///
    /// # Returns
    /// A new instance of `Frame` with the specified parameters.
    pub(super) fn new(
        fin: bool,
        opcode: OpCode,
        mask: Option<[u8; 4]>,
        payload: impl Into<Bytes>,
    ) -> Self {
        Self {
            fin,
            opcode,
            mask,
            payload: payload.into(),
            is_compressed: false,
        }
    }

    /// Returns the frame's opcode.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::{Frame, OpCode};
    ///
    /// let frame = Frame::text("Hello");
    /// assert_eq!(frame.opcode(), OpCode::Text);
    /// ```
    #[inline(always)]
    pub fn opcode(&self) -> OpCode {
        self.opcode
    }

    /// Returns a reference to the frame's payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::binary(vec![1, 2, 3]);
    /// assert_eq!(frame.payload().as_ref(), &[1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn payload(&self) -> &Bytes {
        &self.payload
    }

    /// Returns a mutable reference to the frame's payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    /// use bytes::BufMut;
    ///
    /// let mut frame = Frame::binary(vec![1, 2]);
    /// // Modify payload if needed
    /// ```
    #[inline(always)]
    pub fn payload_mut(&mut self) -> &mut Bytes {
        &mut self.payload
    }

    /// Consumes the frame and returns its payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Hello");
    /// let payload = frame.into_payload();
    /// ```
    #[inline(always)]
    pub fn into_payload(self) -> Bytes {
        self.payload
    }

    /// Consumes the frame and returns its opcode, whether it is final or not and payload.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::{Frame, OpCode};
    ///
    /// let frame = Frame::text("Hello");
    /// let (opcode, is_fin, payload) = frame.into_parts();
    /// assert_eq!(opcode, OpCode::Text);
    /// ```
    #[inline(always)]
    pub fn into_parts(self) -> (OpCode, bool, Bytes) {
        (self.opcode, self.fin, self.payload)
    }

    /// Returns the opcode and payload as a string slice.
    ///
    /// # Errors
    /// Returns `Utf8Error` if the payload is not valid UTF-8.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::{Frame, OpCode};
    ///
    /// let frame = Frame::text("Hello");
    /// let (opcode, text) = frame.into_parts_str().unwrap();
    /// assert_eq!(opcode, OpCode::Text);
    /// assert_eq!(text, "Hello");
    /// ```
    #[inline]
    pub fn into_parts_str(&self) -> Result<(OpCode, &str), std::str::Utf8Error> {
        let text = std::str::from_utf8(&self.payload)?;
        Ok((self.opcode, text))
    }

    /// Returns whether this is the final frame in a message.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Complete message");
    /// assert!(frame.is_fin());
    /// ```
    #[inline(always)]
    pub fn is_fin(&self) -> bool {
        self.fin
    }

    /// Sets whether this is the final frame in a message.
    ///
    /// This is used for message fragmentation. Most users should not need this.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let mut frame = Frame::text("Fragment");
    /// frame.set_fin(false); // Mark as non-final for fragmentation
    /// ```
    #[inline(always)]
    pub fn set_fin(&mut self, fin: bool) {
        self.fin = fin;
    }

    /// Sets a custom masking key for this frame.
    ///
    /// According to RFC 6455, all frames sent from a client to a server must be masked.
    /// Normally, the library generates a random mask automatically. This method allows
    /// you to specify your own mask for testing or special use cases.
    ///
    /// # Parameters
    /// - `mask`: A 4-byte masking key, or `None` to disable masking
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let mut frame = Frame::text("Hello");
    /// frame.set_mask(Some([0x12, 0x34, 0x56, 0x78]));
    /// ```
    #[inline(always)]
    pub fn set_mask(&mut self, mask: Option<[u8; 4]>) {
        self.mask = mask;
    }

    /// Sets a custom masking key for this frame (builder pattern).
    ///
    /// This is the builder-style version of [`set_mask`](Self::set_mask).
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Hello")
    ///     .with_mask([0x12, 0x34, 0x56, 0x78]);
    /// ```
    #[inline(always)]
    pub fn with_mask(mut self, mask: [u8; 4]) -> Self {
        self.mask = Some(mask);
        self
    }

    /// Sets a randomly generated masking key for this frame.
    ///
    /// This is useful when you want to ensure a frame is masked but don't need
    /// to specify the exact mask value.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let mut frame = Frame::text("Hello");
    /// frame.set_random_mask();
    /// ```
    #[inline(always)]
    pub fn set_random_mask(&mut self) {
        self.mask = Some(rand::random());
    }

    /// Sets a randomly generated masking key for this frame (builder pattern).
    ///
    /// This is the builder-style version of [`set_random_mask`](Self::set_random_mask).
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Hello").with_random_mask();
    /// ```
    #[inline(always)]
    pub fn with_random_mask(mut self) -> Self {
        self.mask = Some(rand::random());
        self
    }

    /// Returns the payload as a string slice, expecting valid UTF-8.
    ///
    /// # Panics
    /// Panics if the payload is not valid UTF-8. Use `is_utf8()` to check first,
    /// or use this only with frames you know contain text.
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    ///
    /// let frame = Frame::text("Hello");
    /// assert_eq!(frame.as_str(), "Hello");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.payload).expect("frame payload is not valid UTF-8")
    }

    /// Extracts the close code from a Close frame's payload.
    ///
    /// # Returns
    /// - `Some(CloseCode)` if the payload contains a valid close code
    /// - `None` if the payload is empty or too short to contain a close code
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    /// use yawc::close::CloseCode;
    ///
    /// let frame = Frame::close(CloseCode::Normal, b"Goodbye");
    /// assert_eq!(frame.close_code(), Some(CloseCode::Normal));
    /// ```
    pub fn close_code(&self) -> Option<CloseCode> {
        let code = CloseCode::from(u16::from_be_bytes(self.payload.get(0..2)?.try_into().ok()?));
        Some(code)
    }

    /// Extracts the close reason from a Close frame's payload.
    ///
    /// # Returns
    /// - `Ok(Some(&str))` containing the reason string if present and valid UTF-8
    /// - `Ok(None)` if no close reason was given
    /// - `Err(WebSocketError::InvalidUTF8)` if the reason is not valid UTF-8
    ///
    /// # Example
    /// ```rust
    /// use yawc::frame::Frame;
    /// use yawc::close::CloseCode;
    ///
    /// let frame = Frame::close(CloseCode::Normal, b"Goodbye");
    /// assert_eq!(frame.close_reason().unwrap(), Some("Goodbye"));
    /// ```
    pub fn close_reason(&self) -> Result<Option<&str>, WebSocketError> {
        if self.payload.is_empty() {
            return Ok(None);
        }

        let reason = self.payload.get(2..).ok_or(WebSocketError::InvalidUTF8)?;

        std::str::from_utf8(reason)
            .map(Some)
            .map_err(|_| WebSocketError::InvalidUTF8)
    }

    /// Checks if the frame payload is valid UTF-8.
    ///
    /// # Returns
    /// - `true` if the payload is valid UTF-8.
    /// - `false` otherwise.
    #[inline(always)]
    pub fn is_utf8(&self) -> bool {
        std::str::from_utf8(&self.payload).is_ok()
    }

    /// Generates and sets a random mask if none is already set.
    #[inline]
    pub(super) fn set_random_mask_if_not_set(&mut self) {
        if self.mask.is_none() {
            let mask: [u8; 4] = rand::random();
            self.mask = Some(mask);
        }
    }

    /// Write frame header directly into BytesMut without intermediate buffer.
    /// This is faster than fmt_head as it eliminates an extra copy.
    #[inline]
    pub(super) fn write_head(&self, dst: &mut bytes::BytesMut) {
        use bytes::BufMut;

        let compression = u8::from(self.is_compressed);
        let first_byte = (self.fin as u8) << 7 | compression << 6 | u8::from(self.opcode);

        let len = self.payload.len();

        if len < 126 {
            dst.put_u8(first_byte);
            dst.put_u8(len as u8 | if self.mask.is_some() { 0x80 } else { 0 });
        } else if len < 65536 {
            dst.put_u8(first_byte);
            dst.put_u8(126 | if self.mask.is_some() { 0x80 } else { 0 });
            dst.put_u16(len as u16);
        } else {
            dst.put_u8(first_byte);
            dst.put_u8(127 | if self.mask.is_some() { 0x80 } else { 0 });
            dst.put_u64(len as u64);
        }

        if let Some(mask) = self.mask {
            dst.put_slice(&mask);
        }
    }
}

/// Unit tests for the `yawc::frame` module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::close::CloseCode;
    use bytes::{Bytes, BytesMut};
    use wasm_bindgen_test::wasm_bindgen_test;

    /// Tests for the `OpCode` enum.
    mod opcode_tests {
        use super::*;

        #[test]
        #[wasm_bindgen_test]
        fn test_is_control() {
            // Control frames
            assert!(OpCode::Close.is_control());
            assert!(OpCode::Ping.is_control());
            assert!(OpCode::Pong.is_control());

            // Data frames
            assert!(!OpCode::Continuation.is_control());
            assert!(!OpCode::Text.is_control());
            assert!(!OpCode::Binary.is_control());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_try_from_u8_valid() {
            assert_eq!(OpCode::try_from(0x0).unwrap(), OpCode::Continuation);
            assert_eq!(OpCode::try_from(0x1).unwrap(), OpCode::Text);
            assert_eq!(OpCode::try_from(0x2).unwrap(), OpCode::Binary);
            assert_eq!(OpCode::try_from(0x8).unwrap(), OpCode::Close);
            assert_eq!(OpCode::try_from(0x9).unwrap(), OpCode::Ping);
            assert_eq!(OpCode::try_from(0xA).unwrap(), OpCode::Pong);
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_try_from_u8_invalid() {
            // Invalid opcodes should return an error
            for &code in &[0x3, 0x4, 0x5, 0x6, 0x7, 0xB, 0xC, 0xD, 0xE, 0xF] {
                assert!(OpCode::try_from(code).is_err());
            }
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_from_opcode_to_u8() {
            assert_eq!(u8::from(OpCode::Continuation), 0x0);
            assert_eq!(u8::from(OpCode::Text), 0x1);
            assert_eq!(u8::from(OpCode::Binary), 0x2);
            assert_eq!(u8::from(OpCode::Close), 0x8);
            assert_eq!(u8::from(OpCode::Ping), 0x9);
            assert_eq!(u8::from(OpCode::Pong), 0xA);
        }
    }

    /// Tests for the `Frame` struct.
    mod frame_tests {
        use super::*;

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_text() {
            let text = "Hello, WebSocket!";
            let frame = Frame::text(text);

            assert_eq!(frame.opcode(), OpCode::Text);
            assert_eq!(frame.payload().as_ref(), text.as_bytes());
            assert!(frame.is_fin());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_binary() {
            let data = vec![0x01, 0x02, 0x03];
            let frame = Frame::binary(data.clone());

            assert_eq!(frame.opcode(), OpCode::Binary);
            assert_eq!(frame.payload().as_ref(), &data[..]);
            assert!(frame.is_fin());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_close() {
            let reason = "Normal closure";
            let frame = Frame::close(CloseCode::Normal, reason);

            assert_eq!(frame.opcode(), OpCode::Close);
            assert!(frame.is_fin());

            // The payload should contain the close code (1000) and the reason
            let mut expected_payload = Vec::new();
            expected_payload.extend_from_slice(&1000u16.to_be_bytes());
            expected_payload.extend_from_slice(reason.as_bytes());

            assert_eq!(frame.payload().as_ref(), &expected_payload[..]);
            assert_eq!(frame.close_code(), Some(CloseCode::Normal));
            assert_eq!(frame.close_reason().unwrap(), Some(reason));
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_close_raw() {
            let payload = vec![0x03, 0xE8]; // Close code 1000 without reason
            let frame = Frame::close_raw(payload.clone());

            assert_eq!(frame.opcode(), OpCode::Close);
            assert_eq!(frame.payload().as_ref(), &payload[..]);
            assert!(frame
                .close_reason()
                .is_ok_and(|reason| reason.is_some_and(|reason| reason.is_empty())));
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_empty_close() {
            let frame = Frame::close_raw(vec![]);

            assert_eq!(frame.opcode(), OpCode::Close);
            assert!(frame.payload().is_empty());
            assert!(frame.close_code().is_none());
            assert!(frame.close_reason().is_ok_and(|reason| reason.is_none()));
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_ping() {
            let payload = b"Ping payload";
            let frame = Frame::ping(&payload[..]);

            assert_eq!(frame.opcode(), OpCode::Ping);
            assert_eq!(frame.payload().as_ref(), &payload[..]);
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_pong() {
            let payload = b"Pong payload";
            let frame = Frame::pong(&payload[..]);

            assert_eq!(frame.opcode(), OpCode::Pong);
            assert_eq!(frame.payload().as_ref(), &payload[..]);
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_continuation() {
            let payload = b"continuation data";
            let frame = Frame::continuation(&payload[..]);

            assert_eq!(frame.opcode(), OpCode::Continuation);
            assert_eq!(frame.payload().as_ref(), &payload[..]);
            assert!(frame.is_fin());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_with_fin() {
            let frame = Frame::text("fragment").with_fin(false);

            assert!(!frame.is_fin());
            assert_eq!(frame.opcode(), OpCode::Text);
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_fragmentation() {
            // Test creating a fragmented message
            let first = Frame::text("Hello, ").with_fin(false);
            let middle = Frame::continuation("World").with_fin(false);
            let last = Frame::continuation("!");

            assert!(!first.is_fin());
            assert_eq!(first.opcode(), OpCode::Text);

            assert!(!middle.is_fin());
            assert_eq!(middle.opcode(), OpCode::Continuation);

            assert!(last.is_fin());
            assert_eq!(last.opcode(), OpCode::Continuation);
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_from_tuple() {
            let frame = Frame::from((OpCode::Text, Bytes::from("Test")));
            let (opcode, payload): (OpCode, Bytes) = frame.into();

            assert_eq!(opcode, OpCode::Text);
            assert_eq!(payload, Bytes::from("Test"));
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_from_tuple_bytes() {
            let opcode = OpCode::Binary;
            let payload = Bytes::from_static(b"\xDE\xAD\xBE\xEF");

            let frame = Frame::from((opcode, payload.clone()));

            assert_eq!(frame.opcode(), OpCode::Binary);
            assert_eq!(frame.payload().as_ref(), payload.as_ref());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_new() {
            let payload = BytesMut::from("Test payload");
            let frame = Frame::new(true, OpCode::Text, None, payload.clone());

            assert!(frame.is_fin());
            assert_eq!(frame.opcode(), OpCode::Text);
            assert_eq!(frame.payload().as_ref(), payload.as_ref());
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_as_str() {
            let frame = Frame::text("Hello, World!");
            assert_eq!(frame.as_str(), "Hello, World!");
        }

        #[test]
        #[wasm_bindgen_test]
        fn test_frame_is_utf8() {
            let valid_utf8 = Frame::text("Hello, 世界");
            assert!(valid_utf8.is_utf8());

            let invalid_utf8 = Frame::binary(vec![0xFF, 0xFE, 0xFD]);
            assert!(!invalid_utf8.is_utf8());
        }
    }
}
