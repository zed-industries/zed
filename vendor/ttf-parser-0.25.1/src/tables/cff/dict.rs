use core::convert::TryFrom;
use core::ops::Range;

use crate::Stream;

// Limits according to the Adobe Technical Note #5176, chapter 4 DICT Data.
const TWO_BYTE_OPERATOR_MARK: u8 = 12;
const FLOAT_STACK_LEN: usize = 64;
const END_OF_FLOAT_FLAG: u8 = 0xf;

#[derive(Clone, Copy, Debug)]
pub struct Operator(pub u16);

impl Operator {
    #[inline]
    pub fn get(self) -> u16 {
        self.0
    }
}

pub struct DictionaryParser<'a> {
    data: &'a [u8],
    // The current offset.
    offset: usize,
    // Offset to the last operands start.
    operands_offset: usize,
    // Actual operands.
    //
    // While CFF can contain only i32 and f32 values, we have to store operands as f64
    // since f32 cannot represent the whole i32 range.
    // Meaning we have a choice of storing operands as f64 or as enum of i32/f32.
    // In both cases the type size would be 8 bytes, so it's easier to simply use f64.
    operands: &'a mut [f64],
    // An amount of operands in the `operands` array.
    operands_len: u16,
}

impl<'a> DictionaryParser<'a> {
    #[inline]
    pub fn new(data: &'a [u8], operands_buffer: &'a mut [f64]) -> Self {
        DictionaryParser {
            data,
            offset: 0,
            operands_offset: 0,
            operands: operands_buffer,
            operands_len: 0,
        }
    }

    #[inline(never)]
    pub fn parse_next(&mut self) -> Option<Operator> {
        let mut s = Stream::new_at(self.data, self.offset)?;
        self.operands_offset = self.offset;
        while !s.at_end() {
            let b = s.read::<u8>()?;
            // 0..=21 bytes are operators.
            if is_dict_one_byte_op(b) {
                let mut operator = u16::from(b);

                // Check that operator is two byte long.
                if b == TWO_BYTE_OPERATOR_MARK {
                    // Use a 1200 'prefix' to make two byte operators more readable.
                    // 12 3 => 1203
                    operator = 1200 + u16::from(s.read::<u8>()?);
                }

                self.offset = s.offset();
                return Some(Operator(operator));
            } else {
                skip_number(b, &mut s)?;
            }
        }

        None
    }

    /// Parses operands of the current operator.
    ///
    /// In the DICT structure, operands are defined before an operator.
    /// So we are trying to find an operator first and the we can actually parse the operands.
    ///
    /// Since this methods is pretty expensive and we do not care about most of the operators,
    /// we can speed up parsing by parsing operands only for required operators.
    ///
    /// We still have to "skip" operands during operators search (see `skip_number()`),
    /// but it's still faster that a naive method.
    pub fn parse_operands(&mut self) -> Option<()> {
        let mut s = Stream::new_at(self.data, self.operands_offset)?;
        self.operands_len = 0;
        while !s.at_end() {
            let b = s.read::<u8>()?;
            // 0..=21 bytes are operators.
            if is_dict_one_byte_op(b) {
                break;
            } else {
                let op = parse_number(b, &mut s)?;
                self.operands[usize::from(self.operands_len)] = op;
                self.operands_len += 1;

                if usize::from(self.operands_len) >= self.operands.len() {
                    break;
                }
            }
        }

        Some(())
    }

    #[inline]
    pub fn operands(&self) -> &[f64] {
        &self.operands[..usize::from(self.operands_len)]
    }

    #[inline]
    pub fn parse_number(&mut self) -> Option<f64> {
        self.parse_operands()?;
        self.operands().get(0).cloned()
    }

    #[inline]
    pub fn parse_offset(&mut self) -> Option<usize> {
        self.parse_operands()?;
        let operands = self.operands();
        if operands.len() == 1 {
            usize::try_from(operands[0] as i32).ok()
        } else {
            None
        }
    }

    #[inline]
    pub fn parse_range(&mut self) -> Option<Range<usize>> {
        self.parse_operands()?;
        let operands = self.operands();
        if operands.len() == 2 {
            let len = usize::try_from(operands[0] as i32).ok()?;
            let start = usize::try_from(operands[1] as i32).ok()?;
            let end = start.checked_add(len)?;
            Some(start..end)
        } else {
            None
        }
    }
}

// One-byte CFF DICT Operators according to the
// Adobe Technical Note #5176, Appendix H CFF DICT Encoding.
pub fn is_dict_one_byte_op(b: u8) -> bool {
    match b {
        0..=27 => true,
        28..=30 => false,  // numbers
        31 => true,        // Reserved
        32..=254 => false, // numbers
        255 => true,       // Reserved
    }
}

// Adobe Technical Note #5177, Table 3 Operand Encoding
pub fn parse_number(b0: u8, s: &mut Stream) -> Option<f64> {
    match b0 {
        28 => {
            let n = i32::from(s.read::<i16>()?);
            Some(f64::from(n))
        }
        29 => {
            let n = s.read::<i32>()?;
            Some(f64::from(n))
        }
        30 => parse_float(s),
        32..=246 => {
            let n = i32::from(b0) - 139;
            Some(f64::from(n))
        }
        247..=250 => {
            let b1 = i32::from(s.read::<u8>()?);
            let n = (i32::from(b0) - 247) * 256 + b1 + 108;
            Some(f64::from(n))
        }
        251..=254 => {
            let b1 = i32::from(s.read::<u8>()?);
            let n = -(i32::from(b0) - 251) * 256 - b1 - 108;
            Some(f64::from(n))
        }
        _ => None,
    }
}

fn parse_float(s: &mut Stream) -> Option<f64> {
    let mut data = [0u8; FLOAT_STACK_LEN];
    let mut idx = 0;

    loop {
        let b1: u8 = s.read()?;
        let nibble1 = b1 >> 4;
        let nibble2 = b1 & 15;

        if nibble1 == END_OF_FLOAT_FLAG {
            break;
        }

        idx = parse_float_nibble(nibble1, idx, &mut data)?;

        if nibble2 == END_OF_FLOAT_FLAG {
            break;
        }

        idx = parse_float_nibble(nibble2, idx, &mut data)?;
    }

    let s = core::str::from_utf8(&data[..idx]).ok()?;
    let n = s.parse().ok()?;
    Some(n)
}

// Adobe Technical Note #5176, Table 5 Nibble Definitions
fn parse_float_nibble(nibble: u8, mut idx: usize, data: &mut [u8]) -> Option<usize> {
    if idx == FLOAT_STACK_LEN {
        return None;
    }

    match nibble {
        0..=9 => {
            data[idx] = b'0' + nibble;
        }
        10 => {
            data[idx] = b'.';
        }
        11 => {
            data[idx] = b'E';
        }
        12 => {
            if idx + 1 == FLOAT_STACK_LEN {
                return None;
            }

            data[idx] = b'E';
            idx += 1;
            data[idx] = b'-';
        }
        13 => {
            return None;
        }
        14 => {
            data[idx] = b'-';
        }
        _ => {
            return None;
        }
    }

    idx += 1;
    Some(idx)
}

// Just like `parse_number`, but doesn't actually parses the data.
pub fn skip_number(b0: u8, s: &mut Stream) -> Option<()> {
    match b0 {
        28 => s.skip::<u16>(),
        29 => s.skip::<u32>(),
        30 => {
            while !s.at_end() {
                let b1 = s.read::<u8>()?;
                let nibble1 = b1 >> 4;
                let nibble2 = b1 & 15;
                if nibble1 == END_OF_FLOAT_FLAG || nibble2 == END_OF_FLOAT_FLAG {
                    break;
                }
            }
        }
        32..=246 => {}
        247..=250 => s.skip::<u8>(),
        251..=254 => s.skip::<u8>(),
        _ => return None,
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dict_number() {
        assert_eq!(
            parse_number(0xFA, &mut Stream::new(&[0x7C])).unwrap(),
            1000.0
        );
        assert_eq!(
            parse_number(0xFE, &mut Stream::new(&[0x7C])).unwrap(),
            -1000.0
        );
        assert_eq!(
            parse_number(0x1C, &mut Stream::new(&[0x27, 0x10])).unwrap(),
            10000.0
        );
        assert_eq!(
            parse_number(0x1C, &mut Stream::new(&[0xD8, 0xF0])).unwrap(),
            -10000.0
        );
        assert_eq!(
            parse_number(0x1D, &mut Stream::new(&[0x00, 0x01, 0x86, 0xA0])).unwrap(),
            100000.0
        );
        assert_eq!(
            parse_number(0x1D, &mut Stream::new(&[0xFF, 0xFE, 0x79, 0x60])).unwrap(),
            -100000.0
        );
    }
}
