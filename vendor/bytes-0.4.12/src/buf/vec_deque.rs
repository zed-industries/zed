use std::collections::VecDeque;

use super::Buf;

impl Buf for VecDeque<u8> {
    fn remaining(&self) -> usize {
        self.len()
    }

    fn bytes(&self) -> &[u8] {
        let (s1, s2) = self.as_slices();
        if s1.is_empty() {
            s2
        } else {
            s1
        }
    }

    fn advance(&mut self, cnt: usize) {
        self.drain(..cnt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(b"hello world");
        assert_eq!(11, buffer.remaining());
        assert_eq!(b"hello world", buffer.bytes());
        buffer.advance(6);
        assert_eq!(b"world", buffer.bytes());
        buffer.extend(b" piece");
        assert_eq!(b"world piece" as &[u8], &buffer.collect::<Vec<u8>>()[..]);
    }
}
