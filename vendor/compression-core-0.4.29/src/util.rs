pub const fn _assert_send<T: Send>() {}
pub const fn _assert_sync<T: Sync>() {}

#[derive(Debug, Default)]
pub struct PartialBuffer<B> {
    buffer: B,
    index: usize,
}

impl<B: AsRef<[u8]>> PartialBuffer<B> {
    pub fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    pub fn written(&self) -> &[u8] {
        &self.buffer.as_ref()[..self.index]
    }

    pub fn unwritten(&self) -> &[u8] {
        &self.buffer.as_ref()[self.index..]
    }

    pub fn advance(&mut self, amount: usize) {
        self.index += amount;
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buffer
    }

    pub fn into_inner(self) -> B {
        self.buffer
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> PartialBuffer<B> {
    pub fn unwritten_mut(&mut self) -> &mut [u8] {
        &mut self.buffer.as_mut()[self.index..]
    }

    pub fn copy_unwritten_from<C: AsRef<[u8]>>(&mut self, other: &mut PartialBuffer<C>) -> usize {
        let len = self.unwritten().len().min(other.unwritten().len());

        self.unwritten_mut()[..len].copy_from_slice(&other.unwritten()[..len]);

        self.advance(len);
        other.advance(len);
        len
    }
}

impl<B: AsRef<[u8]> + Default> PartialBuffer<B> {
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> From<B> for PartialBuffer<B> {
    fn from(buffer: B) -> Self {
        Self::new(buffer)
    }
}
