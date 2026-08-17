// Hound -- A wav encoding and decoding library in Rust
// Copyright (C) 2015 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Hound, a wav encoding and decoding library.
//!
//! Examples
//! ========
//!
//! The following example renders a 440 Hz sine wave, and stores it as as a
//! mono wav file with a sample rate of 44.1 kHz and 16 bits per sample.
//!
//! ```
//! use std::f32::consts::PI;
//! use std::i16;
//! use hound;
//!
//! let spec = hound::WavSpec {
//!     channels: 1,
//!     sample_rate: 44100,
//!     bits_per_sample: 16,
//!     sample_format: hound::SampleFormat::Int,
//! };
//! let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
//! for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
//!     let sample = (t * 440.0 * 2.0 * PI).sin();
//!     let amplitude = i16::MAX as f32;
//!     writer.write_sample((sample * amplitude) as i16).unwrap();
//! }
//! writer.finalize().unwrap();
//! ```
//!
//! The following example computes the root mean square (RMS) of an audio file
//! with at most 16 bits per sample.
//!
//! ```
//! use hound;
//!
//! let mut reader = hound::WavReader::open("testsamples/pop.wav").unwrap();
//! let sqr_sum = reader.samples::<i16>()
//!                     .fold(0.0, |sqr_sum, s| {
//!     let sample = s.unwrap() as f64;
//!     sqr_sum + sample * sample
//! });
//! println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
//! ```

#![warn(missing_docs)]

use std::error;
use std::fmt;
use std::io;
use std::result;
use read::ReadExt;
use write::WriteExt;

mod read;
mod write;

pub use read::{WavReader, WavIntoSamples, WavSamples, read_wave_header};
pub use write::{SampleWriter16, WavWriter};

/// A type that can be used to represent audio samples.
///
/// Via this trait, decoding can be generic over `i8`, `i16`, `i32` and `f32`.
///
/// All integer formats with bit depths up to 32 bits per sample can be decoded
/// into `i32`, but it takes up more memory. If you know beforehand that you
/// will be reading a file with 16 bits per sample, then decoding into an `i16`
/// will be sufficient.
pub trait Sample: Sized {
    /// Writes the audio sample to the WAVE data chunk.
    fn write<W: io::Write>(self, writer: &mut W, bits: u16) -> Result<()>;

    /// Writes the audio sample to the WAVE data chunk, zero padding the size of
    /// the written sample out to `byte_width`.
    fn write_padded<W: io::Write>(self, writer: &mut W, bits: u16, byte_width: u16) -> Result<()>;

    /// Reads the audio sample from the WAVE data chunk.
    fn read<R: io::Read>(reader: &mut R, SampleFormat, bytes: u16, bits: u16) -> Result<Self>;

    /// Cast the sample to a 16-bit sample.
    ///
    /// This does not change the value of the sample, it only casts it. The
    /// value is assumed to fit within the range. This is not verified,
    /// truncation may occur.
    fn as_i16(self) -> i16;
}

/// Converts an unsigned integer in the range 0-255 to a signed one in the range -128-127.
///
/// Presumably, the designers of the WAVE format did not like consistency. For
/// all bit depths except 8, samples are stored as little-endian _signed_
/// integers. However, an 8-bit sample is instead stored as an _unsigned_
/// integer. Hound abstracts away this idiosyncrasy by providing only signed
/// sample types.
fn signed_from_u8(x: u8) -> i8 {
    (x as i16 - 128) as i8
}

/// Converts a signed integer in the range -128-127 to an unsigned one in the range 0-255.
fn u8_from_signed(x: i8) -> u8 {
    (x as i16 + 128) as u8
}

#[test]
fn u8_sign_conversion_is_bijective() {
    for x in 0..255 {
        assert_eq!(x, u8_from_signed(signed_from_u8(x)));
    }
    for x in -128..127 {
        assert_eq!(x, signed_from_u8(u8_from_signed(x)));
    }
}

/// Tries to cast the sample to an 8-bit signed integer, returning an error on overflow.
#[inline(always)]
fn narrow_to_i8(x: i32) -> Result<i8> {
    use std::i8;
    if x < i8::MIN as i32 || x > i8::MAX as i32 {
        Err(Error::TooWide)
    } else {
        Ok(x as i8)
    }
}

#[test]
fn verify_narrow_to_i8() {
    assert!(narrow_to_i8(127).is_ok());
    assert!(narrow_to_i8(128).is_err());
    assert!(narrow_to_i8(-128).is_ok());
    assert!(narrow_to_i8(-129).is_err());
}

/// Tries to cast the sample to a 16-bit signed integer, returning an error on overflow.
#[inline(always)]
fn narrow_to_i16(x: i32) -> Result<i16> {
    use std::i16;
    if x < i16::MIN as i32 || x > i16::MAX as i32 {
        Err(Error::TooWide)
    } else {
        Ok(x as i16)
    }
}

#[test]
fn verify_narrow_to_i16() {
    assert!(narrow_to_i16(32767).is_ok());
    assert!(narrow_to_i16(32768).is_err());
    assert!(narrow_to_i16(-32768).is_ok());
    assert!(narrow_to_i16(-32769).is_err());
}

/// Tries to cast the sample to a 24-bit signed integer, returning an error on overflow.
#[inline(always)]
fn narrow_to_i24(x: i32) -> Result<i32> {
    if x < -(1 << 23) || x > (1 << 23) - 1 {
        Err(Error::TooWide)
    } else {
        Ok(x)
    }
}

#[test]
fn verify_narrow_to_i24() {
    assert!(narrow_to_i24(8_388_607).is_ok());
    assert!(narrow_to_i24(8_388_608).is_err());
    assert!(narrow_to_i24(-8_388_608).is_ok());
    assert!(narrow_to_i24(-8_388_609).is_err());
}

impl Sample for i8 {
    fn write<W: io::Write>(self, writer: &mut W, bits: u16) -> Result<()> {
        self.write_padded(writer, bits, bits / 8)
    }

    fn write_padded<W: io::Write>(self, writer: &mut W, bits: u16, byte_width: u16) -> Result<()> {
        match (bits, byte_width) {
            (8, 1) => Ok(try!(writer.write_u8(u8_from_signed(self)))),
            (16, 2) => Ok(try!(writer.write_le_i16(self as i16))),
            (24, 3) => Ok(try!(writer.write_le_i24(self as i32))),
            (24, 4) => Ok(try!(writer.write_le_i24_4(self as i32))),
            (32, 4) => Ok(try!(writer.write_le_i32(self as i32))),
            _ => Err(Error::Unsupported),
        }
    }

    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as i16
    }

    fn read<R: io::Read>(reader: &mut R, fmt: SampleFormat, bytes: u16, bits: u16) -> Result<i8> {
        if fmt != SampleFormat::Int {
            return Err(Error::InvalidSampleFormat);
        }
        match (bytes, bits) {
            (1, 8) => Ok(try!(reader.read_u8().map(signed_from_u8))),
            (n, _) if n > 1 => Err(Error::TooWide),
            // TODO: add a genric decoder for any bit depth.
            _ => Err(Error::Unsupported),
        }
    }
}

impl Sample for i16 {
    fn write<W: io::Write>(self, writer: &mut W, bits: u16) -> Result<()> {
        self.write_padded(writer, bits, bits / 8)
    }

    fn write_padded<W: io::Write>(self, writer: &mut W, bits: u16, byte_width: u16) -> Result<()> {
        match (bits, byte_width) {
            (8, 1) => Ok(try!(
                writer.write_u8(u8_from_signed(try!(narrow_to_i8(self as i32))))
            )),
            (16, 2) => Ok(try!(writer.write_le_i16(self))),
            (24, 3) => Ok(try!(writer.write_le_i24(self as i32))),
            (24, 4) => Ok(try!(writer.write_le_i24_4(self as i32))),
            (32, 4) => Ok(try!(writer.write_le_i32(self as i32))),
            _ => Err(Error::Unsupported),
        }
    }

    #[inline(always)]
    fn as_i16(self) -> i16 {
        self
    }

    fn read<R: io::Read>(reader: &mut R, fmt: SampleFormat, bytes: u16, bits: u16) -> Result<i16> {
        if fmt != SampleFormat::Int {
            return Err(Error::InvalidSampleFormat);
        }
        match (bytes, bits) {
            (1, 8) => Ok(try!(reader.read_u8().map(signed_from_u8).map(|x| x as i16))),
            (2, 16) => Ok(try!(reader.read_le_i16())),
            (n, _) if n > 2 => Err(Error::TooWide),
            // TODO: add a generic decoder for any bit depth.
            _ => Err(Error::Unsupported),
        }
    }
}

impl Sample for i32 {
    fn write<W: io::Write>(self, writer: &mut W, bits: u16) -> Result<()> {
        self.write_padded(writer, bits, bits / 8)
    }

    fn write_padded<W: io::Write>(self, writer: &mut W, bits: u16, byte_width: u16) -> Result<()> {
        match (bits, byte_width) {
            (8, 1) => Ok(try!(
                writer.write_u8(u8_from_signed(try!(narrow_to_i8(self))))
            )),
            (16, 2) => Ok(try!(writer.write_le_i16(try!(narrow_to_i16(self))))),
            (24, 3) => Ok(try!(writer.write_le_i24(try!(narrow_to_i24(self))))),
            (24, 4) => Ok(try!(writer.write_le_i24_4(try!(narrow_to_i24(self))))),
            (32, 4) => Ok(try!(writer.write_le_i32(self))),
            _ => Err(Error::Unsupported),
        }
    }

    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as i16
    }

    fn read<R: io::Read>(reader: &mut R, fmt: SampleFormat, bytes: u16, bits: u16) -> Result<i32> {
        if fmt != SampleFormat::Int {
            return Err(Error::InvalidSampleFormat);
        }
        match (bytes, bits) {
            (1, 8) => Ok(try!(reader.read_u8().map(signed_from_u8).map(|x| x as i32))),
            (2, 16) => Ok(try!(reader.read_le_i16().map(|x| x as i32))),
            (3, 24) => Ok(try!(reader.read_le_i24())),
            (4, 24) => Ok(try!(reader.read_le_i24_4())),
            (4, 32) => Ok(try!(reader.read_le_i32())),
            (n, _) if n > 4 => Err(Error::TooWide),
            // TODO: add a generic decoder for any bit depth.
            _ => Err(Error::Unsupported),
        }
    }
}

impl Sample for f32 {
    fn write<W: io::Write>(self, writer: &mut W, bits: u16) -> Result<()> {
        self.write_padded(writer, bits, bits / 8)
    }

    fn write_padded<W: io::Write>(self, writer: &mut W, bits: u16, byte_width: u16) -> Result<()> {
        match (bits, byte_width) {
            (32, 4) => Ok(try!(writer.write_le_f32(self))),
            _ => Err(Error::Unsupported),
        }
    }

    fn as_i16(self) -> i16 {
        panic!("Calling as_i16 with an f32 is invalid.");
    }

    fn read<R: io::Read>(reader: &mut R, fmt: SampleFormat, bytes: u16, bits: u16) -> Result<Self> {
        if fmt != SampleFormat::Float {
            return Err(Error::InvalidSampleFormat);
        }
        match (bytes, bits) {
            (4, 32) => Ok(try!(reader.read_le_f32())),
            (n, _) if n > 4 => Err(Error::TooWide),
            _ => Err(Error::Unsupported),
        }
    }
}

/// Specifies whether a sample is stored as an "IEEE Float" or an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// Wave files with the `WAVE_FORMAT_IEEE_FLOAT` format tag store samples as floating point
    /// values.
    ///
    /// Values are normally in the range [-1.0, 1.0].
    Float,
    /// Wave files with the `WAVE_FORMAT_PCM` format tag store samples as integer values.
    Int,
}

/// Specifies properties of the audio data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WavSpec {
    /// The number of channels.
    pub channels: u16,

    /// The number of samples per second.
    ///
    /// A common value is 44100, this is 44.1 kHz which is used for CD audio.
    pub sample_rate: u32,

    /// The number of bits per sample.
    ///
    /// A common value is 16 bits per sample, which is used for CD audio.
    pub bits_per_sample: u16,

    /// Whether the wav's samples are float or integer values.
    pub sample_format: SampleFormat,
}

/// Specifies properties of the audio data, as well as the layout of the stream.
#[derive(Clone, Copy)]
pub struct WavSpecEx {
    /// The normal information about the audio data.
    ///
    /// Bits per sample here is the number of _used_ bits per sample, not the
    /// number of bits used to _store_ a sample.
    pub spec: WavSpec,

    /// The number of bytes used to store a sample.
    pub bytes_per_sample: u16,
}

/// The error type for operations on `WavReader` and `WavWriter`.
#[derive(Debug)]
pub enum Error {
    /// An IO error occured in the underlying reader or writer.
    IoError(io::Error),
    /// Ill-formed WAVE data was encountered.
    FormatError(&'static str),
    /// The sample has more bits than the destination type.
    ///
    /// When iterating using the `samples` iterator, this means that the
    /// destination type (produced by the iterator) is not wide enough to hold
    /// the sample. When writing, this means that the sample cannot be written,
    /// because it requires more bits than the bits per sample specified.
    TooWide,
    /// The number of samples written is not a multiple of the number of channels.
    UnfinishedSample,
    /// The format is not supported.
    Unsupported,
    /// The sample format is different than the destination format.
    ///
    /// When iterating using the `samples` iterator, this means the destination
    /// type (produced by the iterator) has a different sample format than the
    /// samples in the wav file.
    ///
    /// For example, this will occur if the user attempts to produce `i32`
    /// samples (which have a `SampleFormat::Int`) from a wav file that
    /// contains floating point data (`SampleFormat::Float`).
    InvalidSampleFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::IoError(ref err) => err.fmt(formatter),
            Error::FormatError(reason) => {
                try!(formatter.write_str("Ill-formed WAVE file: "));
                formatter.write_str(reason)
            }
            Error::TooWide => {
                formatter.write_str("The sample has more bits than the destination type.")
            }
            Error::UnfinishedSample => {
                formatter.write_str(
                    "The number of samples written is not a multiple of the number of channels.")
            }
            Error::Unsupported => {
                formatter.write_str("The wave format of the file is not supported.")
            }
            Error::InvalidSampleFormat => {
                formatter.write_str("The sample format differs from the destination format.")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::FormatError(reason) => reason,
            Error::TooWide => "the sample has more bits than the destination type",
            Error::UnfinishedSample => "the number of samples written is not a multiple of the number of channels",
            Error::Unsupported => "the wave format of the file is not supported",
            Error::InvalidSampleFormat => "the sample format differs from the destination format",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::FormatError(_) => None,
            Error::TooWide => None,
            Error::UnfinishedSample => None,
            Error::Unsupported => None,
            Error::InvalidSampleFormat => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

/// A type for results generated by Hound where the error type is hard-wired.
pub type Result<T> = result::Result<T, Error>;

// The WAVEFORMATEXTENSIBLE struct can contain several subformats.
// These are identified by a GUID. The various GUIDS can be found in the file
// mmreg.h that is part of the Windows SDK. The following GUIDS are defined:
// - PCM:        00000001-0000-0010-8000-00aa00389b71
// - IEEE_FLOAT: 00000003-0000-0010-8000-00aa00389b71
// When written to a wav file, the byte order of a GUID is native for the first
// three sections, which is assumed to be little endian, and big endian for the
// last 8-byte section (which does contain a hyphen, for reasons unknown to me).

/// Subformat type for PCM audio with integer samples.
const KSDATAFORMAT_SUBTYPE_PCM: [u8; 16] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80,
                                            0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71];

/// Subformat type for IEEE_FLOAT audio with float samples.
const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: [u8; 16] = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
                                                   0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71];


impl WavSpec {
    /// Get "stand-alone" wav header representing infinite or unknown size wav file.
    /// Use this if you need to write audio data to non-seekable sinks (like stdout).
    ///
    /// Actual samples are supposed to be written using low-level [`Sample::write`] call.
    ///
    /// Such wav files are produced e.g. by FFmpeg and have `0xFFFFFFFF` instead of chunk sizes.
    ///
    /// Note that such files may be non-standard. Consider using [`WavWriter`] for better API.
    ///
    /// Example:
    ///
    /// ```no_run
    /// extern crate hound;
    /// use std::io::Write;
    /// 
    /// let spec = hound::WavSpec {
    ///     bits_per_sample: 16,
    ///     channels: 1,
    ///     sample_format: hound::SampleFormat::Int,
    ///     sample_rate: 16000,
    /// };
    /// 
    /// let v = spec.into_header_for_infinite_file();
    /// 
    /// let so = std::io::stdout();
    /// let mut so = so.lock();
    /// so.write_all(&v[..]).unwrap();
    /// 
    /// loop {
    ///    for i in 0..126 {
    ///       let x : i16 = (i * 256) as i16;
    ///       hound::Sample::write(x, &mut so, 16).unwrap();
    ///    }
    /// }
    /// ```
    pub fn into_header_for_infinite_file(self) -> Vec<u8> {
        let mut c = std::io::Cursor::new(Vec::with_capacity(0x44));
        {
            let w = WavWriter::new(&mut c, self);
            drop(w);
        }
        let mut v = c.into_inner();

        // Set WAVE chunk size to a special signal value
        v[4] = 0xFF; v[5] = 0xFF; v[6] = 0xFF; v[7] = 0xFF;

        // Detect fmt size, get offset of data chunk's size and set it to signal value
        if v[16] == 0x10 {
            // pcm wave
            v[0x28] = 0xFF; v[0x29] = 0xFF; v[0x2A] = 0xFF; v[0x2B] = 0xFF; 
        } else if v[16] == 0x28 {
            // extensible
            v[0x40] = 0xFF; v[0x41] = 0xFF; v[0x42] = 0xFF; v[0x43] = 0xFF; 
        } else {
            unreachable!()
        }

        v
    }
}

#[test]
fn write_read_i16_is_lossless() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    {
        let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
        for s in -1024_i16..1024 {
            writer.write_sample(s).unwrap();
        }
        writer.finalize().unwrap();
    }

    {
        buffer.set_position(0);
        let mut reader = WavReader::new(&mut buffer).unwrap();
        assert_eq!(write_spec, reader.spec());
        assert_eq!(reader.len(), 2048);
        for (expected, read) in (-1024_i16..1024).zip(reader.samples()) {
            assert_eq!(expected, read.unwrap());
        }
    }
}

#[test]
fn write_read_i16_via_sample_writer_is_lossless() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    {
        let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
        {
            {
                let mut sample_writer = writer.get_i16_writer(1024);
                for s in -1024_i16..0 {
                    sample_writer.write_sample(s);
                }
                sample_writer.flush().unwrap();
            }

            {
                let mut sample_writer = writer.get_i16_writer(1024);
                for s in 0i16..1024 {
                    unsafe { sample_writer.write_sample_unchecked(s); }
                }
                sample_writer.flush().unwrap();
            }
        }
        writer.finalize().unwrap();
    }

    {
        buffer.set_position(0);
        let mut reader = WavReader::new(&mut buffer).unwrap();
        assert_eq!(write_spec, reader.spec());
        assert_eq!(reader.len(), 2048);
        for (expected, read) in (-1024_i16..1024).zip(reader.samples()) {
            assert_eq!(expected, read.unwrap());
        }
    }
}

#[test]
fn write_read_i8_is_lossless() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 16,
        sample_rate: 48000,
        bits_per_sample: 8,
        sample_format: SampleFormat::Int,
    };

    // Write `i8` samples.
    {
        let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
        // Iterate over i16 because we cannot specify the upper bound otherwise.
        for s in -128_i16..127 + 1 {
            writer.write_sample(s as i8).unwrap();
        }
        writer.finalize().unwrap();
    }

    // Then read them into `i16`.
    {
        buffer.set_position(0);
        let mut reader = WavReader::new(&mut buffer).unwrap();
        assert_eq!(write_spec, reader.spec());
        assert_eq!(reader.len(), 256);
        for (expected, read) in (-128_i16..127 + 1).zip(reader.samples()) {
            assert_eq!(expected, read.unwrap());
        }
    }
}

#[test]
fn write_read_i24_is_lossless() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 16,
        sample_rate: 96000,
        bits_per_sample: 24,
        sample_format: SampleFormat::Int,
    };

    // Write `i32` samples, but with at most 24 bits per sample.
    {
        let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
        for s in -128_i32..127 + 1 {
            writer.write_sample(s * 256 * 256).unwrap();
        }
        writer.finalize().unwrap();
    }

    // Then read them into `i32`. This should extend the sign in the correct
    // manner.
    {
        buffer.set_position(0);
        let mut reader = WavReader::new(&mut buffer).unwrap();
        assert_eq!(write_spec, reader.spec());
        assert_eq!(reader.len(), 256);
        for (expected, read) in (-128_i32..127 + 1)
            .map(|x| x * 256 * 256)
            .zip(reader.samples()) {
            assert_eq!(expected, read.unwrap());
        }
    }
}
#[test]
fn write_read_f32_is_lossless() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    {
        let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
        for s in 1_u32..257 {
            writer.write_sample(1.0f32 / s as f32).unwrap();
        }
        writer.finalize().unwrap();
    }

    {
        buffer.set_position(0);
        let mut reader = WavReader::new(&mut buffer).unwrap();
        assert_eq!(write_spec, reader.spec());
        assert_eq!(reader.len(), 256);
        for (expected, read) in (1..257)
            .map(|x| 1.0_f32 / x as f32)
            .zip(reader.samples()) {
            assert_eq!(expected, read.unwrap());
        }
    }
}

#[test]
#[should_panic]
fn no_32_bps_for_float_sample_format_panics() {
    let mut buffer = io::Cursor::new(Vec::new());
    let write_spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16, // will panic, because value must be 32 for floating point
        sample_format: SampleFormat::Float,
    };

    WavWriter::new(&mut buffer, write_spec).unwrap();
}

#[test]
fn flush_should_produce_valid_file() {
    use std::mem;
    use std::io::Seek;

    let mut buffer = io::Cursor::new(Vec::new());
    let samples = &[2, 4, 5, 7, 11, 13];

    {
        let spec = WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut writer = WavWriter::new(&mut buffer, spec).unwrap();

        for &x in samples {
            writer.write_sample(x).unwrap();
        }

        // We should be able to see everything up to the flush later.
        writer.flush().unwrap();

        // Write more samples. These should be in the buffer, but not read by the
        // reader if we don't finalize the writer.
        writer.write_sample(17).unwrap();
        writer.write_sample(19).unwrap();

        mem::forget(writer);
    }

    buffer.seek(io::SeekFrom::Start(0)).unwrap();

    let mut reader = WavReader::new(&mut buffer).unwrap();
    let read_samples: Vec<i16> = reader.samples()
        .map(|r| r.unwrap())
        .collect();

    // We expect to see all samples up to the flush, but not the later ones.
    assert_eq!(&read_samples[..], &samples[..]);
}

#[test]
fn new_append_should_append() {
    use std::io::Seek;

    let mut buffer = io::Cursor::new(Vec::new());
    let samples = &[2, 5, 7, 11];
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // Write initial file.
    {
        let mut writer = WavWriter::new(&mut buffer, spec).unwrap();
        for s in samples { writer.write_sample(*s).unwrap(); }
    }

    buffer.seek(io::SeekFrom::Start(0)).unwrap();

    // Append samples (the same ones a second time).
    {
        let mut writer = WavWriter::new_append(&mut buffer).unwrap();
        assert_eq!(writer.spec(), spec);
        for s in samples { writer.write_sample(*s).unwrap(); }
    }

    buffer.seek(io::SeekFrom::Start(0)).unwrap();

    let mut reader = WavReader::new(&mut buffer).unwrap();
    let read_samples: Vec<i16> = reader.samples()
        .map(|r| r.unwrap())
        .collect();

    // We expect to see all samples up to the flush, but not the later ones.
    assert_eq!(&read_samples[..], &[2, 5, 7, 11, 2, 5, 7, 11]);
}

#[test]
fn new_append_does_not_corrupt_files() {
    use std::io::Read;
    use std::fs;

    let sample_files = [
        "testsamples/pcmwaveformat-16bit-44100Hz-mono-extra.wav",
        "testsamples/pcmwaveformat-16bit-44100Hz-mono.wav",
        "testsamples/pcmwaveformat-8bit-44100Hz-mono.wav",
        "testsamples/pop.wav",
        "testsamples/waveformatex-16bit-44100Hz-mono-extra.wav",
        "testsamples/waveformatex-16bit-44100Hz-mono.wav",
        "testsamples/waveformatex-16bit-44100Hz-stereo.wav",
        "testsamples/waveformatextensible-24bit-192kHz-mono.wav",
        "testsamples/waveformatextensible-32bit-48kHz-stereo.wav",
        "testsamples/nonstandard-01.wav",
        "testsamples/nonstandard-02.wav",
        "testsamples/waveformatex-8bit-11025Hz-mono.wav",
    ];

    for fname in &sample_files {
        print!("testing {} ... ", fname);

        let mut buffer = Vec::new();
        let mut f = fs::File::open(fname).unwrap();
        f.read_to_end(&mut buffer).unwrap();

        let samples_orig: Vec<i32>;
        let samples_after: Vec<i32>;

        // Read samples first.
        let mut cursor = io::Cursor::new(buffer);
        {
            let mut reader = WavReader::new(&mut cursor).unwrap();
            samples_orig = reader.samples().map(|r| r.unwrap()).collect();
        }
        buffer = cursor.into_inner();

        // Open in append mode and append one sample.
        let mut cursor = io::Cursor::new(buffer);
        {
            let mut writer = WavWriter::new_append(&mut cursor).unwrap();
            writer.write_sample(41_i8).unwrap();
            writer.write_sample(43_i8).unwrap();
        }
        buffer = cursor.into_inner();

        {
            let cursor = io::Cursor::new(buffer);
            let mut reader = WavReader::new(cursor)
                .expect("Reading wav failed after append.");
            samples_after = reader.samples().map(|r| r.unwrap()).collect();
        }

        assert_eq!(&samples_orig[..], &samples_after[..samples_orig.len()]);
        assert_eq!(samples_after[samples_after.len() - 2], 41_i32);
        assert_eq!(samples_after[samples_after.len() - 1], 43_i32);

        println!("ok");
    }
}

#[cfg(test)]
fn assert_contents(fname: &str, expected: &[i16]) {
    let mut reader = WavReader::open(fname).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    assert_eq!(&samples[..], expected);
}

#[test]
fn append_works_on_files() {
    use std::fs;

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create("append.wav", spec).unwrap();
    writer.write_sample(11_i16).unwrap();
    writer.write_sample(13_i16).unwrap();
    writer.write_sample(17_i16).unwrap();
    writer.finalize().unwrap();

    assert_contents("append.wav", &[11, 13, 17]);

    let len = fs::metadata("append.wav").unwrap().len();

    let mut appender = WavWriter::append("append.wav").unwrap();

    appender.write_sample(19_i16).unwrap();
    appender.write_sample(23_i16).unwrap();
    appender.finalize().unwrap();

    // We appended four bytes of audio data (2 16-bit samples), so the file
    // should have grown by 4 bytes.
    assert_eq!(fs::metadata("append.wav").unwrap().len(), len + 4);

    assert_contents("append.wav", &[11, 13, 17, 19, 23]);
}

#[cfg(test)]
#[test]
fn test_into_header_for_infinite_file() {
    let spec = WavSpec {
        bits_per_sample: 16,
        channels: 1,
        sample_format: SampleFormat::Int,
        sample_rate: 16000,
    };
    let v = spec.into_header_for_infinite_file();
    assert_eq!(&v[..], &b"RIFF\xFF\xFF\xFF\xFFWAVE\
fmt \x10\x00\x00\x00\x01\x00\x01\x00\x80\x3e\x00\x00\x00\x7d\x00\x00\x02\x00\x10\x00\
data\xFF\xFF\xFF\xFF"[..]);

    let spec = WavSpec {
        bits_per_sample: 16,
        channels: 10,
        sample_format: SampleFormat::Int,
        sample_rate: 16000,
    };
    let v = spec.into_header_for_infinite_file();
    assert_eq!(&v[..], &b"RIFF\xFF\xFF\xFF\xFFWAVE\
fmt \x28\x00\x00\x00\xfe\xff\x0a\x00\x80\x3e\x00\x00\x00\xe2\x04\x00\
\x14\x00\x10\x00\x16\x00\x10\x00\xff\x03\x00\x00\x01\x00\x00\x00\
\x00\x00\x10\x00\x80\x00\x00\xaa\x00\x38\x9b\x71\
data\xFF\xFF\xFF\xFF"[..]);
}
