use std::{
    convert::{AsRef, From, Into, TryFrom},
    fmt,
    result::Result as StdResult,
    str,
};

use super::frame::{CloseFrame, Frame};
use crate::error::{CapacityError, Error, Result};

mod string_collect {
    use utf8::DecodeError;

    use crate::error::{Error, Result};

    #[derive(Debug)]
    pub struct StringCollector {
        data: String,
        incomplete: Option<utf8::Incomplete>,
    }

    impl StringCollector {
        pub fn new() -> Self {
            StringCollector { data: String::new(), incomplete: None }
        }

        pub fn len(&self) -> usize {
            self.data
                .len()
                .saturating_add(self.incomplete.map(|i| i.buffer_len as usize).unwrap_or(0))
        }

        pub fn extend<T: AsRef<[u8]>>(&mut self, tail: T) -> Result<()> {
            let mut input: &[u8] = tail.as_ref();

            if let Some(mut incomplete) = self.incomplete.take() {
                if let Some((result, rest)) = incomplete.try_complete(input) {
                    input = rest;
                    if let Ok(text) = result {
                        self.data.push_str(text);
                    } else {
                        return Err(Error::Utf8);
                    }
                } else {
                    input = &[];
                    self.incomplete = Some(incomplete);
                }
            }

            if !input.is_empty() {
                match utf8::decode(input) {
                    Ok(text) => {
                        self.data.push_str(text);
                        Ok(())
                    }
                    Err(DecodeError::Incomplete { valid_prefix, incomplete_suffix }) => {
                        self.data.push_str(valid_prefix);
                        self.incomplete = Some(incomplete_suffix);
                        Ok(())
                    }
                    Err(DecodeError::Invalid { valid_prefix, .. }) => {
                        self.data.push_str(valid_prefix);
                        Err(Error::Utf8)
                    }
                }
            } else {
                Ok(())
            }
        }

        pub fn into_string(self) -> Result<String> {
            if self.incomplete.is_some() {
                Err(Error::Utf8)
            } else {
                Ok(self.data)
            }
        }
    }
}

use self::string_collect::StringCollector;

/// A struct representing the incomplete message.
#[derive(Debug)]
pub struct IncompleteMessage {
    collector: IncompleteMessageCollector,
}

#[derive(Debug)]
enum IncompleteMessageCollector {
    Text(StringCollector),
    Binary(Vec<u8>),
}

impl IncompleteMessage {
    /// Create new.
    pub fn new(message_type: IncompleteMessageType) -> Self {
        IncompleteMessage {
            collector: match message_type {
                IncompleteMessageType::Binary => IncompleteMessageCollector::Binary(Vec::new()),
                IncompleteMessageType::Text => {
                    IncompleteMessageCollector::Text(StringCollector::new())
                }
            },
        }
    }

    /// Get the current filled size of the buffer.
    pub fn len(&self) -> usize {
        match self.collector {
            IncompleteMessageCollector::Text(ref t) => t.len(),
            IncompleteMessageCollector::Binary(ref b) => b.len(),
        }
    }

    /// Add more data to an existing message.
    pub fn extend<T: AsRef<[u8]>>(&mut self, tail: T, size_limit: Option<usize>) -> Result<()> {
        // Always have a max size. This ensures an error in case of concatenating two buffers
        // of more than `usize::max_value()` bytes in total.
        let max_size = size_limit.unwrap_or_else(usize::max_value);
        let my_size = self.len();
        let portion_size = tail.as_ref().len();
        // Be careful about integer overflows here.
        if my_size > max_size || portion_size > max_size - my_size {
            return Err(Error::Capacity(CapacityError::MessageTooLong {
                size: my_size + portion_size,
                max_size,
            }));
        }

        match self.collector {
            IncompleteMessageCollector::Binary(ref mut v) => {
                v.extend(tail.as_ref());
                Ok(())
            }
            IncompleteMessageCollector::Text(ref mut t) => t.extend(tail),
        }
    }

    /// Convert an incomplete message into a complete one.
    pub fn complete(self) -> Result<Message> {
        match self.collector {
            IncompleteMessageCollector::Binary(v) => Ok(Message::Binary(v)),
            IncompleteMessageCollector::Text(t) => {
                let text = t.into_string()?;
                Ok(Message::Text(text))
            }
        }
    }
}

/// The type of incomplete message.
pub enum IncompleteMessageType {
    Text,
    Binary,
}

/// An enum representing the various forms of a WebSocket message.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    /// A text WebSocket message
    Text(String),
    /// A binary WebSocket message
    Binary(Vec<u8>),
    /// A ping message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Ping(Vec<u8>),
    /// A pong message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Pong(Vec<u8>),
    /// A close message with the optional close frame.
    Close(Option<CloseFrame<'static>>),
    /// Raw frame. Note, that you're not going to get this value while reading the message.
    Frame(Frame),
}

impl Message {
    /// Create a new text WebSocket message from a stringable.
    pub fn text<S>(string: S) -> Message
    where
        S: Into<String>,
    {
        Message::Text(string.into())
    }

    /// Create a new binary WebSocket message by converting to `Vec<u8>`.
    pub fn binary<B>(bin: B) -> Message
    where
        B: Into<Vec<u8>>,
    {
        Message::Binary(bin.into())
    }

    /// Indicates whether a message is a text message.
    pub fn is_text(&self) -> bool {
        matches!(*self, Message::Text(_))
    }

    /// Indicates whether a message is a binary message.
    pub fn is_binary(&self) -> bool {
        matches!(*self, Message::Binary(_))
    }

    /// Indicates whether a message is a ping message.
    pub fn is_ping(&self) -> bool {
        matches!(*self, Message::Ping(_))
    }

    /// Indicates whether a message is a pong message.
    pub fn is_pong(&self) -> bool {
        matches!(*self, Message::Pong(_))
    }

    /// Indicates whether a message is a close message.
    pub fn is_close(&self) -> bool {
        matches!(*self, Message::Close(_))
    }

    /// Get the length of the WebSocket message.
    pub fn len(&self) -> usize {
        match *self {
            Message::Text(ref string) => string.len(),
            Message::Binary(ref data) | Message::Ping(ref data) | Message::Pong(ref data) => {
                data.len()
            }
            Message::Close(ref data) => data.as_ref().map(|d| d.reason.len()).unwrap_or(0),
            Message::Frame(ref frame) => frame.len(),
        }
    }

    /// Returns true if the WebSocket message has no content.
    /// For example, if the other side of the connection sent an empty string.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Consume the WebSocket and return it as binary data.
    pub fn into_data(self) -> Vec<u8> {
        match self {
            Message::Text(string) => string.into_bytes(),
            Message::Binary(data) | Message::Ping(data) | Message::Pong(data) => data,
            Message::Close(None) => Vec::new(),
            Message::Close(Some(frame)) => frame.reason.into_owned().into_bytes(),
            Message::Frame(frame) => frame.into_data(),
        }
    }

    /// Attempt to consume the WebSocket message and convert it to a String.
    pub fn into_text(self) -> Result<String> {
        match self {
            Message::Text(string) => Ok(string),
            Message::Binary(data) | Message::Ping(data) | Message::Pong(data) => {
                Ok(String::from_utf8(data)?)
            }
            Message::Close(None) => Ok(String::new()),
            Message::Close(Some(frame)) => Ok(frame.reason.into_owned()),
            Message::Frame(frame) => Ok(frame.into_string()?),
        }
    }

    /// Attempt to get a &str from the WebSocket message,
    /// this will try to convert binary data to utf8.
    pub fn to_text(&self) -> Result<&str> {
        match *self {
            Message::Text(ref string) => Ok(string),
            Message::Binary(ref data) | Message::Ping(ref data) | Message::Pong(ref data) => {
                Ok(str::from_utf8(data)?)
            }
            Message::Close(None) => Ok(""),
            Message::Close(Some(ref frame)) => Ok(&frame.reason),
            Message::Frame(ref frame) => Ok(frame.to_text()?),
        }
    }
}

impl From<String> for Message {
    fn from(string: String) -> Self {
        Message::text(string)
    }
}

impl<'s> From<&'s str> for Message {
    fn from(string: &'s str) -> Self {
        Message::text(string)
    }
}

impl<'b> From<&'b [u8]> for Message {
    fn from(data: &'b [u8]) -> Self {
        Message::binary(data)
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        Message::binary(data)
    }
}

impl From<Message> for Vec<u8> {
    fn from(message: Message) -> Self {
        message.into_data()
    }
}

impl TryFrom<Message> for String {
    type Error = Error;

    fn try_from(value: Message) -> StdResult<Self, Self::Error> {
        value.into_text()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        if let Ok(string) = self.to_text() {
            write!(f, "{}", string)
        } else {
            write!(f, "Binary Data<length={}>", self.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let t = Message::text("test".to_owned());
        assert_eq!(t.to_string(), "test".to_owned());

        let bin = Message::binary(vec![0, 1, 3, 4, 241]);
        assert_eq!(bin.to_string(), "Binary Data<length=5>".to_owned());
    }

    #[test]
    fn binary_convert() {
        let bin = [6u8, 7, 8, 9, 10, 241];
        let msg = Message::from(&bin[..]);
        assert!(msg.is_binary());
        assert!(msg.into_text().is_err());
    }

    #[test]
    fn binary_convert_vec() {
        let bin = vec![6u8, 7, 8, 9, 10, 241];
        let msg = Message::from(bin);
        assert!(msg.is_binary());
        assert!(msg.into_text().is_err());
    }

    #[test]
    fn binary_convert_into_vec() {
        let bin = vec![6u8, 7, 8, 9, 10, 241];
        let bin_copy = bin.clone();
        let msg = Message::from(bin);
        let serialized: Vec<u8> = msg.into();
        assert_eq!(bin_copy, serialized);
    }

    #[test]
    fn text_convert() {
        let s = "kiwotsukete";
        let msg = Message::from(s);
        assert!(msg.is_text());
    }
}
