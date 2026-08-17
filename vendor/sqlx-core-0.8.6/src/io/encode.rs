pub trait ProtocolEncode<'en, Context = ()> {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), crate::Error>
    where
        Self: ProtocolEncode<'en, ()>,
    {
        self.encode_with(buf, ())
    }

    fn encode_with(&self, buf: &mut Vec<u8>, context: Context) -> Result<(), crate::Error>;
}

impl<'en, C> ProtocolEncode<'en, C> for &'_ [u8] {
    fn encode_with(&self, buf: &mut Vec<u8>, _context: C) -> Result<(), crate::Error> {
        buf.extend_from_slice(self);
        Ok(())
    }
}
