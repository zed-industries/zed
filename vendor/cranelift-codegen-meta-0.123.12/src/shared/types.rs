//! This module predefines all the Cranelift scalar types.

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Int {
    /// 8-bit int.
    I8 = 8,
    /// 16-bit int.
    I16 = 16,
    /// 32-bit int.
    I32 = 32,
    /// 64-bit int.
    I64 = 64,
    /// 128-bit int.
    I128 = 128,
}

/// This provides an iterator through all of the supported int variants.
pub(crate) struct IntIterator {
    index: u8,
}

impl IntIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for IntIterator {
    type Item = Int;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Int::I8),
            1 => Some(Int::I16),
            2 => Some(Int::I32),
            3 => Some(Int::I64),
            4 => Some(Int::I128),
            _ => return None,
        };
        self.index += 1;
        res
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Float {
    F16 = 16,
    F32 = 32,
    F64 = 64,
    F128 = 128,
}

/// Iterator through the variants of the Float enum.
pub(crate) struct FloatIterator {
    index: u8,
}

impl FloatIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

/// This provides an iterator through all of the supported float variants.
impl Iterator for FloatIterator {
    type Item = Float;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Float::F16),
            1 => Some(Float::F32),
            2 => Some(Float::F64),
            3 => Some(Float::F128),
            _ => return None,
        };
        self.index += 1;
        res
    }
}

#[cfg(test)]
mod iter_tests {
    use super::*;

    #[test]
    fn int_iter_works() {
        let mut int_iter = IntIterator::new();
        assert_eq!(int_iter.next(), Some(Int::I8));
        assert_eq!(int_iter.next(), Some(Int::I16));
        assert_eq!(int_iter.next(), Some(Int::I32));
        assert_eq!(int_iter.next(), Some(Int::I64));
        assert_eq!(int_iter.next(), Some(Int::I128));
        assert_eq!(int_iter.next(), None);
    }

    #[test]
    fn float_iter_works() {
        let mut float_iter = FloatIterator::new();
        assert_eq!(float_iter.next(), Some(Float::F16));
        assert_eq!(float_iter.next(), Some(Float::F32));
        assert_eq!(float_iter.next(), Some(Float::F64));
        assert_eq!(float_iter.next(), Some(Float::F128));
        assert_eq!(float_iter.next(), None);
    }
}
