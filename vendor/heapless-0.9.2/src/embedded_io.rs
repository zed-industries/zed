use embedded_io::{Error, ErrorKind, ErrorType, Write};

use crate::{
    len_type::LenType,
    vec::{VecInner, VecStorage},
    CapacityError,
};

impl Error for CapacityError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::OutOfMemory
    }
}

impl<LenT: LenType, S: VecStorage<u8> + ?Sized> ErrorType for VecInner<u8, LenT, S> {
    type Error = CapacityError;
}

impl<LenT: LenType, S: VecStorage<u8> + ?Sized> Write for VecInner<u8, LenT, S> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(buf)?;
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec;
    use embedded_io::{Error, ErrorKind, Write};

    fn write(w: &mut impl Write, data: &[u8]) -> Result<(), ErrorKind> {
        w.write_all(data).map_err(|e| e.kind())
    }

    #[test]
    fn test_write() {
        let mut v: Vec<u8, 4> = Vec::new();
        assert_eq!(v.len(), 0);
        write(&mut v, &[1, 2]).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v.as_slice(), &[1, 2]);
        write(&mut v, &[3]).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);
        assert!(write(&mut v, &[4, 5]).is_err());
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);
    }
}
