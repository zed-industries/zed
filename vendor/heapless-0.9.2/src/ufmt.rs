use crate::{
    c_string::{self, CString},
    len_type::LenType,
    string::{StringInner, StringStorage},
    vec::{VecInner, VecStorage},
    CapacityError,
};
use ufmt::uDisplay;
use ufmt_write::uWrite;

impl<LenT: LenType, S: StringStorage + ?Sized> uDisplay for StringInner<LenT, S> {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_str(self.as_str())
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> uWrite for StringInner<LenT, S> {
    type Error = CapacityError;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.push_str(s)
    }
}

impl<LenT: LenType, S: VecStorage<u8> + ?Sized> uWrite for VecInner<u8, LenT, S> {
    type Error = CapacityError;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.extend_from_slice(s.as_bytes())
    }
}

impl<const N: usize, LenT: LenType> uWrite for CString<N, LenT> {
    type Error = c_string::ExtendError;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.extend_from_bytes(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{String, Vec};

    use ufmt::{derive::uDebug, uwrite};

    #[derive(uDebug)]
    struct Pair {
        x: u32,
        y: u32,
    }

    #[test]
    fn test_udisplay_string() {
        let str_a = String::<32>::try_from("world").unwrap();
        let mut str_b = String::<32>::new();
        uwrite!(str_b, "Hello {}!", str_a).unwrap();

        assert_eq!(str_b, "Hello world!");
    }

    #[test]
    fn test_uwrite_string() {
        let a = 123;
        let b = Pair { x: 0, y: 1234 };

        let mut s = String::<32>::new();
        uwrite!(s, "{} -> {:?}", a, b).unwrap();

        assert_eq!(s, "123 -> Pair { x: 0, y: 1234 }");
    }

    #[test]
    fn test_uwrite_string_err() {
        let p = Pair { x: 0, y: 1234 };
        let mut s = String::<4>::new();
        assert!(uwrite!(s, "{:?}", p).is_err());
    }

    #[test]
    fn test_uwrite_vec() {
        let a = 123;
        let b = Pair { x: 0, y: 1234 };

        let mut v = Vec::<u8, 32>::new();
        uwrite!(v, "{} -> {:?}", a, b).unwrap();

        assert_eq!(v, b"123 -> Pair { x: 0, y: 1234 }");
    }
}
