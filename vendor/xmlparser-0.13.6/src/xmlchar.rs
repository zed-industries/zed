/// Extension methods for XML-subset only operations.
pub trait XmlCharExt {
    /// Checks if the value is within the
    /// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar) range.
    fn is_xml_name_start(&self) -> bool;

    /// Checks if the value is within the
    /// [NameChar](https://www.w3.org/TR/xml/#NT-NameChar) range.
    fn is_xml_name(&self) -> bool;

    /// Checks if the value is within the
    /// [Char](https://www.w3.org/TR/xml/#NT-Char) range.
    fn is_xml_char(&self) -> bool;
}

impl XmlCharExt for char {
    #[inline]
    fn is_xml_name_start(&self) -> bool {
        // Check for ASCII first.
        if *self as u32 <= 128 {
            return match *self as u8 {
                  b'A'...b'Z'
                | b'a'...b'z'
                | b':'
                | b'_' => true,
                _ => false,
            };
        }

        match *self as u32 {
              0x0000C0...0x0000D6
            | 0x0000D8...0x0000F6
            | 0x0000F8...0x0002FF
            | 0x000370...0x00037D
            | 0x00037F...0x001FFF
            | 0x00200C...0x00200D
            | 0x002070...0x00218F
            | 0x002C00...0x002FEF
            | 0x003001...0x00D7FF
            | 0x00F900...0x00FDCF
            | 0x00FDF0...0x00FFFD
            | 0x010000...0x0EFFFF => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_name(&self) -> bool {
        // Check for ASCII first.
        if *self as u32 <= 128 {
            return (*self as u8).is_xml_name();
        }

        match *self as u32 {
              0x0000B7
            | 0x0000C0...0x0000D6
            | 0x0000D8...0x0000F6
            | 0x0000F8...0x0002FF
            | 0x000300...0x00036F
            | 0x000370...0x00037D
            | 0x00037F...0x001FFF
            | 0x00200C...0x00200D
            | 0x00203F...0x002040
            | 0x002070...0x00218F
            | 0x002C00...0x002FEF
            | 0x003001...0x00D7FF
            | 0x00F900...0x00FDCF
            | 0x00FDF0...0x00FFFD
            | 0x010000...0x0EFFFF => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_char(&self) -> bool {
        // Does not check for surrogate code points U+D800-U+DFFF,
        // since that check was performed by Rust when the `&str` was constructed.
        if (*self as u32) < 0x20 {
            return (*self as u8).is_xml_space()
        }
        match *self as u32 {
            0xFFFF | 0xFFFE => false,
            _ => true,
        }
    }
}


/// Extension methods for XML-subset only operations.
pub trait XmlByteExt {
    /// Checks if byte is a digit.
    ///
    /// `[0-9]`
    fn is_xml_digit(&self) -> bool;

    /// Checks if byte is a hex digit.
    ///
    /// `[0-9A-Fa-f]`
    fn is_xml_hex_digit(&self) -> bool;

    /// Checks if byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_xml_space(&self) -> bool;

    /// Checks if byte is an ASCII char.
    ///
    /// `[A-Za-z]`
    fn is_xml_letter(&self) -> bool;

    /// Checks if byte is within the ASCII
    /// [Char](https://www.w3.org/TR/xml/#NT-Char) range.
    fn is_xml_name(&self) -> bool;
}

impl XmlByteExt for u8 {
    #[inline]
    fn is_xml_digit(&self) -> bool {
        matches!(*self, b'0'...b'9')
    }

    #[inline]
    fn is_xml_hex_digit(&self) -> bool {
        matches!(*self, b'0'...b'9' | b'A'...b'F' | b'a'...b'f')
    }

    #[inline]
    fn is_xml_space(&self) -> bool {
        matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
    }

    #[inline]
    fn is_xml_letter(&self) -> bool {
        matches!(*self, b'A'...b'Z' | b'a'...b'z')
    }

    #[inline]
    fn is_xml_name(&self) -> bool {
        matches!(*self, b'A'...b'Z' | b'a'...b'z'| b'0'...b'9'| b':' | b'_' | b'-' | b'.')
    }
}
