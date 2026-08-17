//! Generic Response Packets
//!
//! <https://dev.mysql.com/doc/internals/en/generic-response-packets.html>
//! <https://mariadb.com/kb/en/4-server-response-packets/>

mod eof;
mod err;
mod ok;
mod status;

pub use eof::EofPacket;
pub use err::ErrPacket;
pub use ok::OkPacket;
pub use status::Status;
