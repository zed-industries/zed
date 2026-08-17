use std::fmt;
use std::sync::mpsc::SendError;
use std::error::Error as StdError;
use x11rb::errors::{ConnectError, ConnectionError, ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::Atom;

#[must_use]
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Set(SendError<Atom>),
    XcbConnect(ConnectError),
    XcbConnection(ConnectionError),
    XcbReplyOrId(ReplyOrIdError),
    XcbReply(ReplyError),
    Lock,
    Timeout,
    Owner,
    UnexpectedType(Atom),
    // Could change name on next major, since this uses pipes now.
    EventFdCreate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            Set(e) => write!(f, "XCB - couldn't set atom: {:?}", e),
            XcbConnect(e) => write!(f, "XCB - couldn't establish conection: {:?}", e),
            XcbConnection(e) => write!(f, "XCB connection error: {:?}", e),
            XcbReplyOrId(e) => write!(f, "XCB reply error: {:?}", e),
            XcbReply(e) => write!(f, "XCB reply error: {:?}", e),
            Lock => write!(f, "XCB: Lock is poisoned"),
            Timeout => write!(f, "Selection timed out"),
            Owner => write!(f, "Failed to set new owner of XCB selection"),
            UnexpectedType(target) => write!(f, "Unexpected Reply type: {:?}", target),
            EventFdCreate => write!(f, "Failed to create eventfd"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Set(e) => Some(e),
            XcbConnection(e) => Some(e),
            XcbReply(e) => Some(e),
            XcbReplyOrId(e) => Some(e),
            XcbConnect(e) => Some(e),
            Lock | Timeout | Owner | UnexpectedType(_) | EventFdCreate => None,
        }
    }
}

macro_rules! define_from {
    ( $item:ident from $err:ty ) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::$item(err)
            }
        }
    }
}

define_from!(Set from SendError<Atom>);
define_from!(XcbConnect from ConnectError);
define_from!(XcbConnection from ConnectionError);
define_from!(XcbReply from ReplyError);
define_from!(XcbReplyOrId from ReplyOrIdError);
