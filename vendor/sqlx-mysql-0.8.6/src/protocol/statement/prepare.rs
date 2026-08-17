use crate::io::ProtocolEncode;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/internals/en/com-stmt-prepare.html#packet-COM_STMT_PREPARE

pub struct Prepare<'a> {
    pub query: &'a str,
}

impl ProtocolEncode<'_, Capabilities> for Prepare<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) -> Result<(), crate::Error> {
        buf.push(0x16); // COM_STMT_PREPARE
        buf.extend(self.query.as_bytes());
        Ok(())
    }
}
