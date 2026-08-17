#![cfg(feature = "with-num-bigint")]

use std::fmt;
use std::io;
use std::ops::Neg;

use num_bigint::{BigInt, BigUint, Sign};

use crate::sealed::Sealed;
use crate::{Format, Grouping, ToFormattedString};

impl ToFormattedString for BigInt {
    #[inline(always)]
    fn read_to_io_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: io::Write,
    {
        match self.sign() {
            Sign::Minus => {
                let minus_sign = format.minus_sign().into_str();
                w.write_all(minus_sign.as_bytes())?;
                let s = self.neg().to_string();
                let c = io_algorithm(s, w, format)?;
                Ok(c + minus_sign.len())
            }
            Sign::NoSign | Sign::Plus => {
                let s = self.to_string();
                let c = io_algorithm(s, w, format)?;
                Ok(c)
            }
        }
    }

    #[inline(always)]
    fn read_to_fmt_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, fmt::Error>
    where
        F: Format,
        W: fmt::Write,
    {
        match self.sign() {
            Sign::Minus => {
                let minus_sign = format.minus_sign().into_str();
                w.write_str(minus_sign)?;
                let s = self.neg().to_string();
                let c = fmt_algorithm(s, w, format)?;
                Ok(c + minus_sign.len())
            }
            Sign::NoSign | Sign::Plus => {
                let s = self.to_string();
                let c = fmt_algorithm(s, w, format)?;
                Ok(c)
            }
        }
    }
}

impl ToFormattedString for BigUint {
    #[inline(always)]
    fn read_to_io_writer<F, W>(&self, w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: io::Write,
    {
        let s = self.to_string();
        let c = io_algorithm(s, w, format)?;
        Ok(c)
    }

    #[inline(always)]
    fn read_to_fmt_writer<F, W>(&self, w: W, format: &F) -> Result<usize, fmt::Error>
    where
        F: Format,
        W: fmt::Write,
    {
        let s = self.to_string();
        let c = fmt_algorithm(s, w, format)?;
        Ok(c)
    }
}

impl Sealed for BigInt {}
impl Sealed for BigUint {}

#[inline(always)]
fn io_algorithm<F, W>(s: String, mut w: W, format: &F) -> Result<usize, io::Error>
where
    W: io::Write,
    F: Format,
{
    let separator = format.separator().into_str();
    let grouping = format.grouping();

    if separator.is_empty() || grouping == Grouping::Posix {
        w.write_all(s.as_bytes())?;
        return Ok(s.len());
    }

    let mut sep_next = match s.len() {
        s_len if s_len < 4 => {
            w.write_all(s.as_bytes())?;
            return Ok(s_len);
        }
        s_len => match grouping {
            Grouping::Standard => match s_len % 3 {
                0 => 3,
                2 => 2,
                1 => 1,
                _ => unreachable!(),
            },
            Grouping::Indian => match s_len % 2 {
                0 => 1,
                1 => 2,
                _ => unreachable!(),
            },
            Grouping::Posix => unreachable!(),
        },
    };
    let sep_bytes = separator.as_bytes();
    let sep_len = separator.len();
    let sep_step = match grouping {
        Grouping::Standard => 4,
        Grouping::Indian => 3,
        Grouping::Posix => unreachable!(),
    };

    let mut bytes_written = 0;
    let mut digits_remaining = s.len();
    for digit in s.as_bytes().iter() {
        if bytes_written == sep_next {
            w.write_all(sep_bytes)?;
            bytes_written += sep_len;
            if digits_remaining > 3 {
                sep_next += sep_len + sep_step - 1;
            }
        }
        w.write_all(&[*digit])?;
        bytes_written += 1;
        digits_remaining -= 1;
    }

    Ok(bytes_written)
}

#[inline(always)]
fn fmt_algorithm<F, W>(s: String, mut w: W, format: &F) -> Result<usize, fmt::Error>
where
    W: fmt::Write,
    F: Format,
{
    let separator = format.separator().into_str();
    let grouping = format.grouping();

    if separator.is_empty() || grouping == Grouping::Posix {
        w.write_str(&s)?;
        return Ok(s.len());
    }

    let mut sep_next = match s.len() {
        s_len if s_len < 4 => {
            w.write_str(&s)?;
            return Ok(s_len);
        }
        s_len => match grouping {
            Grouping::Standard => match s_len % 3 {
                0 => 3,
                2 => 2,
                1 => 1,
                _ => unreachable!(),
            },
            Grouping::Indian => match s_len % 2 {
                0 => 1,
                1 => 2,
                _ => unreachable!(),
            },
            Grouping::Posix => unreachable!(),
        },
    };
    let sep_len = separator.len();
    let sep_no_chars = separator.chars().count();
    let sep_step = match grouping {
        Grouping::Standard => 4,
        Grouping::Indian => 3,
        Grouping::Posix => unreachable!(),
    };

    let mut bytes_written = 0;
    let mut chars_written = 0;
    let mut digits_remaining = s.len();
    for c in s.chars() {
        if chars_written == sep_next {
            w.write_str(separator)?;
            bytes_written += sep_len;
            chars_written += sep_no_chars;
            if digits_remaining > 3 {
                sep_next += sep_step + sep_no_chars - 1;
            }
        }
        w.write_char(c)?;
        bytes_written += 1;
        chars_written += 1;
        digits_remaining -= 1;
    }

    Ok(bytes_written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_format::CustomFormat;
    use crate::write_formatted::WriteFormatted;

    #[test]
    fn test_num_bigint_count() {
        let n = BigInt::new(Sign::Minus, vec![1_000_000_000]);

        let format = CustomFormat::builder()
            .minus_sign("𠜱")
            .separator("𠜱")
            .build()
            .unwrap();
        {
            let mut vec = Vec::new();
            let c = vec.write_formatted(&n, &format).unwrap();
            let s = String::from_utf8(vec).unwrap();
            assert_eq!(&s, "𠜱1𠜱000𠜱000𠜱000");
            assert_eq!(c, 26);
        }
        {
            let mut s = String::new();
            let c = s.write_formatted(&n, &format).unwrap();
            assert_eq!(&s, "𠜱1𠜱000𠜱000𠜱000");
            assert_eq!(c, 26);
        }

        let format = format
            .into_builder()
            .grouping(Grouping::Indian)
            .build()
            .unwrap();
        {
            let mut vec = Vec::new();
            let c = vec.write_formatted(&n, &format).unwrap();
            let s = String::from_utf8(vec).unwrap();
            assert_eq!(&s, "𠜱1𠜱00𠜱00𠜱00𠜱000");
            assert_eq!(c, 30);
        }
        {
            let mut s = String::new();
            let c = s.write_formatted(&n, &format).unwrap();
            assert_eq!(&s, "𠜱1𠜱00𠜱00𠜱00𠜱000");
            assert_eq!(c, 30);
        }
    }
}
