#![cfg_attr(feature="bench", feature(test))]

#![deny(warnings)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::needless_lifetimes)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#[cfg(all(feature="bench", test))]
extern crate test;

#[doc=include_str!("../README.md")]
type _DocTestReadme = ();

use std::fmt::{self};
use std::char::{self};
use std::error::{Error};
use std::io::{self, BufRead};
use arrayvec::{ArrayVec};

/// A structure, containing read bytes, and an [`io::Error`].
///
/// The `io::Error` is an actual I/O error if some occurred,
/// or a synthetic error with either the [`UnexpectedEof`](std::io::ErrorKind::UnexpectedEof)
/// kind if a multi-byte char was unexpectedly terminated,
/// either the [`InvalidData`](std::io::ErrorKind::InvalidData)
/// kind if no actual I/O error occurred, but read byte sequence was not recognised as a valid UTF-8.  
#[derive(Debug)]
pub struct ReadCharError {
    bytes: ArrayVec<u8, { CHAR_MAX_LEN as usize }>,
    io_error: io::Error,
}

impl ReadCharError {
    /// A byte sequence, representing an invalid or incomplete UTF-8-encoded char.
    pub fn as_bytes(&self) -> &[u8] { &self.bytes }
    /// Returns a reference to the I/O error.
    pub fn as_io_error(&self) -> &io::Error { &self.io_error }
    /// Consumes the `ReadCharError`, returning the I/O error.
    pub fn into_io_error(self) -> io::Error { self.io_error }
}

impl Error for ReadCharError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.io_error) }
}

impl From<ReadCharError> for io::Error {
    fn from(e: ReadCharError) -> io::Error { e.into_io_error() }
}

impl fmt::Display for ReadCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid UTF-8 byte sequence")?;
        for b in self.as_bytes() {
            write!(f, " {b:02X}")?;
        }
        write!(f, " read")?;
        match self.as_io_error().kind() {
            io::ErrorKind::InvalidData => { },
            io::ErrorKind::UnexpectedEof => { write!(f, " (unexpected EOF)")?; }
            _ => { write!(f, " ({})", self.as_io_error())?; }
        }
        Ok(())
    }
}

/// An iterator over the chars of an instance of [`BufRead`].
///
/// In contrast to [`CharsRaw`], the error type is
/// [`io::Error`], and therefore more likely to be drop-in
/// compatible, at the price of losing the UTF-8 context bytes in the error
/// message.
///
/// This struct is generally created by calling
/// [`chars`](BufReadCharsExt::chars) on a [`BufRead`].
#[derive(Debug)]
pub struct Chars<'a, T: BufRead + ?Sized>(&'a mut T);

impl<'a, T: BufRead + ?Sized> Iterator for Chars<'a, T> {
    type Item = io::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.read_char_raw().map_err(|x| x.into_io_error()).transpose()
    }
}

/// An iterator over the chars of an instance of [`BufRead`].
///
/// This struct is generally created by calling [`chars_raw`](BufReadCharsExt::chars_raw)
/// on a [`BufRead`].
#[derive(Debug)]
pub struct CharsRaw<'a, T: BufRead + ?Sized>(&'a mut T);

impl<'a, T: BufRead + ?Sized> Iterator for CharsRaw<'a, T> {
    type Item = Result<char, ReadCharError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.read_char_raw().transpose()
    }
}

const CHAR_MAX_LEN: u8 = 4;
const LEAD_BYTE_MASK: [u8; CHAR_MAX_LEN as usize] = [0x7F, 0x1F, 0x0F, 0x07];
const TAIL_BYTE_MASK: u8 = 0x3F;
const TAIL_BYTE_SIGNATURE: u8 = 0x80;
const TAIL_BYTE_BITS_COUNT: u8 = 6;
const CHAR_MIN_VALUE: [u32; CHAR_MAX_LEN as usize] = [0, 0x80, 0x800, 0x10000];

fn read_byte_and_ignore_interrupts(reader: &mut (impl BufRead + ?Sized)) -> io::Result<Option<u8>> {
    loop {
        match reader.fill_buf() {
            Ok(buf) => return Ok(buf.first().copied()),
            Err(e) => {
                if e.kind() != io::ErrorKind::Interrupted {
                    return Err(e)
                }
            }
        }
    };
}

/// Extends [`BufRead`] with methods for reading chars.
pub trait BufReadCharsExt : BufRead {
    /// Returns an iterator over the chars of this reader.
    ///
    /// In contrast to [`chars_raw`](BufReadCharsExt::chars_raw), the error type is
    /// [`io::Error`], and therefore more likely to be drop-in
    /// compatible, at the price of losing the UTF-8 context bytes in the error
    /// message.
    ///
    /// The iterator returned from this function will yield instances of
    /// [`io::Result`]`<char>`.
    fn chars(&mut self) -> Chars<'_, Self> { Chars(self) }

    /// Returns an iterator over the chars of this reader.
    ///
    /// The iterator returned from this function will yield instances of
    /// [`Result`]`<char, `[`ReadCharError`]`>`.
    fn chars_raw(&mut self) -> CharsRaw<'_, Self> { CharsRaw(self) }

    /// Reads a char from the underlying reader.
    ///
    /// In contrast to [`read_char_raw`](BufReadCharsExt::read_char_raw), the error type is
    /// [`io::Error`], and therefore more likely to be drop-in
    /// compatible, at the price of losing the UTF-8 context bytes in the error
    /// message.
    ///
    /// Returns
    /// - `Ok(Some(char))` if a char has successfully read,
    /// - `Ok(None)` if the stream has reached EOF before any byte was read,
    /// - `Err(err)` if an I/O error occurred, or read byte sequence was not recognised as a valid UTF-8.
    ///
    /// If this function encounters an error of the kind
    /// [`io::ErrorKind::Interrupted`]
    /// then the error is ignored and the operation will continue.
    fn read_char(&mut self) -> io::Result<Option<char>> {
        self.read_char_raw().map_err(|x| x.into_io_error())
    }

    /// Reads a char from the underlying reader.
    ///
    /// Returns
    /// - `Ok(Some(char))` if a char has successfully read,
    /// - `Ok(None)` if the stream has reached EOF before any byte was read,
    /// - `Err(err)` if an I/O error occurred, or read byte sequence was not recognised as a valid UTF-8.
    ///
    /// If this function encounters an error of the kind
    /// [`io::ErrorKind::Interrupted`]
    /// then the error is ignored and the operation will continue.
    fn read_char_raw(&mut self) -> Result<Option<char>, ReadCharError> {
        match read_byte_and_ignore_interrupts(self) {
            Err(e) => Err(ReadCharError { bytes: ArrayVec::new(), io_error: e }),
            Ok(None) => Ok(None),
            Ok(Some(lead_byte)) => {
                self.consume(1);
                let leading_ones = lead_byte.leading_ones();
                if leading_ones == 0 { return Ok(Some(char::from(lead_byte))); }
                if leading_ones == 1 || leading_ones > 4 {
                    let mut bytes = ArrayVec::new();
                    bytes.push(lead_byte);
                    return Err(ReadCharError { bytes, io_error: io::Error::from(io::ErrorKind::InvalidData) });
                }
                let mut bytes = ArrayVec::new();
                bytes.push(lead_byte);
                let tail_bytes_count = (leading_ones - 1) as u8;
                let mut item =
                    ((lead_byte & LEAD_BYTE_MASK[tail_bytes_count as usize]) as u32)
                    << (TAIL_BYTE_BITS_COUNT * tail_bytes_count)
                ;
                for tail_byte_index in (0 .. tail_bytes_count).rev() {
                    match read_byte_and_ignore_interrupts(self) {
                        Err(e) => return Err(ReadCharError { bytes, io_error: e }),
                        Ok(None) => return Err(ReadCharError { bytes, io_error: io::Error::from(io::ErrorKind::UnexpectedEof) }),
                        Ok(Some(tail_byte)) => {
                            if tail_byte & !TAIL_BYTE_MASK != TAIL_BYTE_SIGNATURE {
                                return Err(ReadCharError { bytes, io_error: io::Error::from(io::ErrorKind::InvalidData) });
                            }
                            bytes.push(tail_byte);
                            item |=
                                ((tail_byte & TAIL_BYTE_MASK) as u32)
                                << (tail_byte_index * TAIL_BYTE_BITS_COUNT)
                            ;
                            self.consume(1);
                        }
                    }
                }
                if item < CHAR_MIN_VALUE[tail_bytes_count as usize] {
                    return Err(ReadCharError { bytes, io_error: io::Error::from(io::ErrorKind::InvalidData) });
                }
                match char::from_u32(item) {
                    None => Err(ReadCharError { bytes, io_error: io::Error::from(io::ErrorKind::InvalidData) }),
                    Some(item) => Ok(Some(item))
                }
            }
        }
    }
}

impl<T: BufRead + ?Sized> BufReadCharsExt for T { }

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use std::io::{BufRead, BufReader, ErrorKind};
    use crate::{BufReadCharsExt};

    #[test]
    fn read_valid_unicode() {
        assert_eq!(vec!['A', 'B', 'c', 'd', ' ', 'А', 'Б', 'в', 'г', 'д', ' ', 'U', '\0'],
                    BufReader::new("ABcd АБвгд U\0".as_bytes()).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
    }

    #[test]
    fn edgecase_one_two_bytes() {
        assert_eq!(vec!['\x7F'],
                    BufReader::new(&[ 0x7F ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
        assert_eq!(vec!['\u{0080}'],
                    BufReader::new(&[ 0xC2, 0x80 ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());

        let mut bytes = BufReader::new(&[ 0xC2 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xC2][..], err.as_bytes());
        assert_eq!(ErrorKind::UnexpectedEof, err.as_io_error().kind());

        let mut bytes = BufReader::new(&[ 0xC1, 0xBF ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xC1, 0xBF][..], err.as_bytes());
        assert_eq!(ErrorKind::InvalidData, err.as_io_error().kind());
    }

    #[test]
    fn edgecase_two_three_bytes() {
        assert_eq!(vec!['\u{07FF}'],
                    BufReader::new(&[ 0xDF, 0xBF ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
        assert_eq!(vec!['\u{0800}'],
                    BufReader::new(&[ 0xE0, 0xA0, 0x80 ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());

        let mut bytes = BufReader::new(&[ 0xE0, 0xA0 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xE0, 0xA0][..], err.as_bytes());
        assert_eq!(ErrorKind::UnexpectedEof, err.as_io_error().kind());

        let mut bytes = BufReader::new(&[ 0xE0, 0x9F, 0xBF ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xE0, 0x9F, 0xBF][..], err.as_bytes());
        assert_eq!(ErrorKind::InvalidData, err.as_io_error().kind());
    }

    #[test]
    fn edgecase_three_four_bytes() {
        assert_eq!(vec!['\u{00FFFF}'],
                    BufReader::new(&[ 0xEF, 0xBF, 0xBF ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
        assert_eq!(vec!['\u{010000}'],
                    BufReader::new(&[ 0xF0, 0x90, 0x80, 0x80 ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());

        let mut bytes = BufReader::new(&[ 0xF0, 0x90, 0x80 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xF0, 0x90, 0x80][..], err.as_bytes());
        assert_eq!(ErrorKind::UnexpectedEof, err.as_io_error().kind());

        let mut bytes = BufReader::new(&[ 0xF0, 0x8F, 0xBF, 0xBF ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xF0, 0x8F, 0xBF, 0xBF][..], err.as_bytes());
        assert_eq!(ErrorKind::InvalidData, err.as_io_error().kind());
    }

    #[test]
    fn edgecase_four_bytes_max() {
        assert_eq!(vec!['\u{10FFFF}'],
                    BufReader::new(&[ 0xF4, 0x8F, 0xBF, 0xBF ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
        //            BufReader::new(&[ 0xF7, 0xBF, 0xBF, 0xBF ][..]).chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());

        let mut bytes = BufReader::new(&[ 0xF8, 0x41 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(2, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xF8][..], err.as_bytes());
        assert_eq!(ErrorKind::InvalidData, err.as_io_error().kind());

        let normal_char = res[1].as_ref().unwrap();
        assert_eq!(&'A', normal_char);

        // Now we want to force `read_char` to make this call:
        assert_eq!(None, std::char::from_u32(0x00110000));
        // Sadly, there is no more specific way to test this.
        let mut bytes = BufReader::new(&[ 0xF4, 0x90, 0x80, 0x80 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xF4, 0x90, 0x80, 0x80][..], err.as_bytes());
        assert_eq!(ErrorKind::InvalidData, err.as_io_error().kind());
    }

    #[test]
    fn read_io_valid_unicode() {
        assert_eq!(vec!['A', 'B', 'c', 'd', ' ', 'А', 'Б', 'в', 'г', 'д', ' ', 'U', '\0'],
                    BufReader::new("ABcd АБвгд U\0".as_bytes()).chars().map(|x| x.unwrap()).collect::<Vec<_>>());
    }

    #[test]
    fn read_valid_unicode_from_dyn_read() {
        let bytes: &mut dyn BufRead = &mut BufReader::new("ABcd АБвгд UV".as_bytes());
        assert_eq!(
            vec!['A', 'B', 'c', 'd', ' ', 'А', 'Б', 'в', 'г', 'д', ' ', 'U', 'V'],
            bytes.chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn do_not_take_extra_bytes() {
        let mut bytes = BufReader::new("ABcd АБвгд UV".as_bytes());
        assert_eq!(vec!['A', 'B', 'c', 'd'], bytes.chars_raw().take(4).map(|x| x.unwrap()).collect::<Vec<_>>());
        assert_eq!(vec![' ', 'А', 'Б', 'в', 'г', 'д', ' ', 'U', 'V'], bytes.chars_raw().map(|x| x.unwrap()).collect::<Vec<_>>());
    }

    #[test]
    fn read_value_out_of_range() {
        let mut bytes = BufReader::new(&[ 0xF5, 0x8F, 0xBF, 0xBF ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xF5, 0x8F, 0xBF, 0xBF][..], err.as_bytes());
    }

    #[test]
    fn read_io_value_out_of_range() {
        let mut bytes = BufReader::new(&[ 0xF5, 0x8F, 0xBF, 0xBF ][..]);
        let res = bytes.chars().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(ErrorKind::InvalidData, err.kind());
    }

    #[test]
    fn read_io_incomplete_twobyte() {
        let mut bytes = BufReader::new(&[ 0xC3 ][..]);  // 0xC3 0xA4 = 'ä'
        let res = bytes.chars().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(ErrorKind::UnexpectedEof, err.kind());
    }

    #[test]
    fn read_io_incomplete_threebyte() {
        let mut bytes = BufReader::new(&[ 0xE1, 0xBA ][..]);  // 0xE1 0xBA 0xB9 = 'ẹ'
        let res = bytes.chars().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(ErrorKind::UnexpectedEof, err.kind());
    }

    #[test]
    fn read_surrogate() {
        let mut bytes = BufReader::new(&[ 0xED, 0xA0, 0x80 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(1, res.len());
        let err = res[0].as_ref().err().unwrap();
        assert_eq!(&[0xED, 0xA0, 0x80][..], err.as_bytes());
    }

    #[test]
    fn read_invalid_sequences() {
        let mut bytes = BufReader::new(&[ 0x81, 0x82, 0xC1, 0x07, 0xC1, 0x87, 0xC2, 0xC2, 0x82, 0xF7, 0x88, 0x89, 0x07 ][..]);
        let res = bytes.chars_raw().collect::<Vec<_>>();
        assert_eq!(9, res.len());
        assert_eq!(&[0x81][..], res[0].as_ref().err().unwrap().as_bytes());
        assert_eq!(&[0x82][..], res[1].as_ref().err().unwrap().as_bytes());
        assert_eq!(&[0xC1][..], res[2].as_ref().err().unwrap().as_bytes());
        assert_eq!('\x07', *res[3].as_ref().unwrap());
        assert_eq!(&[0xC1, 0x87][..], res[4].as_ref().err().unwrap().as_bytes());
        assert_eq!(&[0xC2][..], res[5].as_ref().err().unwrap().as_bytes());
        assert_eq!('\u{82}', *res[6].as_ref().unwrap());
        assert_eq!(&[0xF7, 0x88, 0x89][..], res[7].as_ref().err().unwrap().as_bytes());
        assert_eq!('\x07', *res[8].as_ref().unwrap());
    }

    #[quickcheck]
    fn read_string(s: String) -> bool {
        let mut t = String::new();
        BufReader::new(s.as_bytes()).chars_raw().for_each(|c| t.push(c.unwrap()));
        s == t
    }

    #[quickcheck]
    fn read_array(b: Vec<u8>) -> bool {
        let mut t = Vec::new();
        BufReader::new(&b[..]).chars_raw().for_each(|c|
            t.append(&mut c.map_or_else(|e| e.as_bytes().to_vec(), |s| s.to_string().as_bytes().to_vec()))
        );
        b == t
    }
}

#[cfg(all(feature="bench", test))]
mod benchs {
    use rand::distributions::{Distribution, Uniform};
    use rand::thread_rng;
    use std::hint::black_box;
    use std::io::BufReader;
    use std::vec::{Vec};
    use test::Bencher;
    use crate::{BufReadCharsExt};

    #[bench]
    fn read_array_bench(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut bytes: Vec<u8> = Uniform::new_inclusive(0u8, 255u8).sample_iter(&mut rng).take(10000).collect();
        b.iter(move || {
            black_box(&mut bytes);
            black_box(BufReader::new(&bytes[..]).chars_raw().last());
        });
    }
}
