use core::fmt;

/// `core::fmt` presenter for binary data encoded as hexadecimal (Base16).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HexDisplay<'a>(pub &'a [u8]);

impl fmt::Display for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl fmt::UpperHex for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hex = [0u8; 2];

        for &byte in self.0 {
            f.write_str(crate::upper::encode_str(&[byte], &mut hex)?)?;
        }

        Ok(())
    }
}

impl fmt::LowerHex for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hex = [0u8; 2];

        for &byte in self.0 {
            f.write_str(crate::lower::encode_str(&[byte], &mut hex)?)?;
        }

        Ok(())
    }
}
