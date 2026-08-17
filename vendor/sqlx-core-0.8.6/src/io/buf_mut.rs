use bytes::BufMut;

pub trait BufMutExt: BufMut {
    fn put_str_nul(&mut self, s: &str);
}

impl BufMutExt for Vec<u8> {
    fn put_str_nul(&mut self, s: &str) {
        self.extend(s.as_bytes());
        self.push(0);
    }
}
