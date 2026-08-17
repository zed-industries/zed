//! Various codes defined in RFC 6455.

use std::{
    convert::{From, Into},
    fmt,
};

/// WebSocket message opcode as in RFC 6455.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    /// Data (text or binary).
    Data(Data),
    /// Control message (close, ping, pong).
    Control(Control),
}

/// Data opcodes as in RFC 6455
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Data {
    /// 0x0 denotes a continuation frame
    Continue,
    /// 0x1 denotes a text frame
    Text,
    /// 0x2 denotes a binary frame
    Binary,
    /// 0x3-7 are reserved for further non-control frames
    Reserved(u8),
}

/// Control opcodes as in RFC 6455
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Control {
    /// 0x8 denotes a connection close
    Close,
    /// 0x9 denotes a ping
    Ping,
    /// 0xa denotes a pong
    Pong,
    /// 0xb-f are reserved for further control frames
    Reserved(u8),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Data::Continue => write!(f, "CONTINUE"),
            Data::Text => write!(f, "TEXT"),
            Data::Binary => write!(f, "BINARY"),
            Data::Reserved(x) => write!(f, "RESERVED_DATA_{x}"),
        }
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Control::Close => write!(f, "CLOSE"),
            Control::Ping => write!(f, "PING"),
            Control::Pong => write!(f, "PONG"),
            Control::Reserved(x) => write!(f, "RESERVED_CONTROL_{x}"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCode::Data(d) => d.fmt(f),
            OpCode::Control(c) => c.fmt(f),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        use self::{
            Control::{Close, Ping, Pong},
            Data::{Binary, Continue, Text},
            OpCode::*,
        };
        match code {
            Data(Continue) => 0,
            Data(Text) => 1,
            Data(Binary) => 2,
            Data(self::Data::Reserved(i)) => i,

            Control(Close) => 8,
            Control(Ping) => 9,
            Control(Pong) => 10,
            Control(self::Control::Reserved(i)) => i,
        }
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        use self::{
            Control::{Close, Ping, Pong},
            Data::{Binary, Continue, Text},
            OpCode::*,
        };
        match byte {
            0 => Data(Continue),
            1 => Data(Text),
            2 => Data(Binary),
            i @ 3..=7 => Data(self::Data::Reserved(i)),
            8 => Control(Close),
            9 => Control(Ping),
            10 => Control(Pong),
            i @ 11..=15 => Control(self::Control::Reserved(i)),
            _ => panic!("Bug: OpCode out of range"),
        }
    }
}

use self::CloseCode::*;
/// Status code used to indicate why an endpoint is closing the WebSocket connection.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CloseCode {
    /// Indicates a normal closure, meaning that the purpose for
    /// which the connection was established has been fulfilled.
    Normal,
    /// Indicates that an endpoint is "going away", such as a server
    /// going down or a browser having navigated away from a page.
    Away,
    /// Indicates that an endpoint is terminating the connection due
    /// to a protocol error.
    Protocol,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a type of data it cannot accept (e.g., an
    /// endpoint that understands only text data MAY send this if it
    /// receives a binary message).
    Unsupported,
    /// Indicates that no status code was included in a closing frame. This
    /// close code makes it possible to use a single method, `on_close` to
    /// handle even cases where no close code was provided.
    Status,
    /// Indicates an abnormal closure. If the abnormal closure was due to an
    /// error, this close code will not be used. Instead, the `on_error` method
    /// of the handler will be called with the error. However, if the connection
    /// is simply dropped, without an error, this close code will be sent to the
    /// handler.
    Abnormal,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received data within a message that was not
    /// consistent with the type of the message (e.g., non-UTF-8 \[RFC3629\]
    /// data within a text message).
    Invalid,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a message that violates its policy.  This
    /// is a generic status code that can be returned when there is no
    /// other more suitable status code (e.g., Unsupported or Size) or if there
    /// is a need to hide specific details about the policy.
    Policy,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a message that is too big for it to
    /// process.
    Size,
    /// Indicates that an endpoint (client) is terminating the
    /// connection because it has expected the server to negotiate one or
    /// more extension, but the server didn't return them in the response
    /// message of the WebSocket handshake.  The list of extensions that
    /// are needed should be given as the reason for closing.
    /// Note that this status code is not used by the server, because it
    /// can fail the WebSocket handshake instead.
    Extension,
    /// Indicates that a server is terminating the connection because
    /// it encountered an unexpected condition that prevented it from
    /// fulfilling the request.
    Error,
    /// Indicates that the server is restarting. A client may choose to reconnect,
    /// and if it does, it should use a randomized delay of 5-30 seconds between attempts.
    Restart,
    /// Indicates that the server is overloaded and the client should either connect
    /// to a different IP (when multiple targets exist), or reconnect to the same IP
    /// when a user has performed an action.
    Again,
    #[doc(hidden)]
    Tls,
    #[doc(hidden)]
    Reserved(u16),
    #[doc(hidden)]
    Iana(u16),
    #[doc(hidden)]
    Library(u16),
    #[doc(hidden)]
    Bad(u16),
}

impl CloseCode {
    /// Check if this CloseCode is allowed.
    pub fn is_allowed(self) -> bool {
        !matches!(self, Bad(_) | Reserved(_) | Status | Abnormal | Tls)
    }
}

impl fmt::Display for CloseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code: u16 = self.into();
        write!(f, "{code}")
    }
}

impl From<CloseCode> for u16 {
    fn from(code: CloseCode) -> u16 {
        match code {
            Normal => 1000,
            Away => 1001,
            Protocol => 1002,
            Unsupported => 1003,
            Status => 1005,
            Abnormal => 1006,
            Invalid => 1007,
            Policy => 1008,
            Size => 1009,
            Extension => 1010,
            Error => 1011,
            Restart => 1012,
            Again => 1013,
            Tls => 1015,
            Reserved(code) => code,
            Iana(code) => code,
            Library(code) => code,
            Bad(code) => code,
        }
    }
}

impl<'t> From<&'t CloseCode> for u16 {
    fn from(code: &'t CloseCode) -> u16 {
        (*code).into()
    }
}

impl From<u16> for CloseCode {
    fn from(code: u16) -> CloseCode {
        match code {
            1000 => Normal,
            1001 => Away,
            1002 => Protocol,
            1003 => Unsupported,
            1005 => Status,
            1006 => Abnormal,
            1007 => Invalid,
            1008 => Policy,
            1009 => Size,
            1010 => Extension,
            1011 => Error,
            1012 => Restart,
            1013 => Again,
            1015 => Tls,
            1..=999 => Bad(code),
            1016..=2999 => Reserved(code),
            3000..=3999 => Iana(code),
            4000..=4999 => Library(code),
            _ => Bad(code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_from_u8() {
        let byte = 2u8;
        assert_eq!(OpCode::from(byte), OpCode::Data(Data::Binary));
    }

    #[test]
    fn opcode_into_u8() {
        let text = OpCode::Data(Data::Text);
        let byte: u8 = text.into();
        assert_eq!(byte, 1u8);
    }

    #[test]
    fn closecode_from_u16() {
        let byte = 1008u16;
        assert_eq!(CloseCode::from(byte), CloseCode::Policy);
    }

    #[test]
    fn closecode_into_u16() {
        let text = CloseCode::Away;
        let byte: u16 = text.into();
        assert_eq!(byte, 1001u16);
        assert_eq!(u16::from(text), 1001u16);
    }
}
