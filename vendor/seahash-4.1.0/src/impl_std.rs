use crate::SeaHasher;
use std::hash::Hasher;
use std::io;

impl io::Write for SeaHasher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Hasher::write(self, buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_write_trait() {
        let reader: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let mut hasher = SeaHasher::new();
        // io::copy consumes the mutable reader -> cloning the buffer
        let _ = io::copy(&mut reader.clone(), &mut hasher).unwrap();
        let hash = hasher.finish();
        let control = crate::hash(&reader);
        assert_eq!(control, hash);
    }
}
