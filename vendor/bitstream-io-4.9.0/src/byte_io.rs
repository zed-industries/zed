pub trait ByteSource: Sized {
    type Error: Sized;

    #[inline(always)]
    fn read_byte(&mut self) -> Result<u8, Self::Error> {
        let mut byte = 0;
        self.read_bytes(core::slice::from_mut(&mut byte))
            .map(|()| byte)
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    fn skip_bytes(&mut self, bytes: u32) -> Result<(), Self::Error> {
        fn skip_chunks<const SIZE: usize, R>(
            reader: &mut R,
            mut bytes: usize,
        ) -> Result<(), R::Error>
        where
            R: ByteSource,
        {
            let mut buf = [0; SIZE];
            while bytes > 0 {
                let to_read = bytes.min(SIZE);
                reader.read_bytes(&mut buf[0..to_read])?;
                bytes -= to_read;
            }
            Ok(())
        }

        match bytes {
            0..256 => skip_chunks::<8, Self>(self, bytes as usize),
            256..1024 => skip_chunks::<256, Self>(self, bytes as usize),
            1024..4096 => skip_chunks::<1024, Self>(self, bytes as usize),
            _ => skip_chunks::<4096, Self>(self, bytes as usize),
        }
    }
}
