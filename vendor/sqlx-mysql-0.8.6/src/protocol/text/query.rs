use crate::io::ProtocolEncode;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/internals/en/com-query.html

#[derive(Debug)]
pub(crate) struct Query<'q>(pub(crate) &'q str);

impl ProtocolEncode<'_, Capabilities> for Query<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) -> Result<(), crate::Error> {
        buf.push(0x03); // COM_QUERY
        buf.extend(self.0.as_bytes());
        Ok(())
    }
}
