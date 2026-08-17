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

use std::cmp;
use std::fs;
use std::io;
use std::marker;
use std::path;
use super::{Error, Result, Sample, SampleFormat, WavSpec, WavSpecEx};

/// Extends the functionality of `io::Read` with additional methods.
///
/// The methods may be used on any type that implements `io::Read`.
pub trait ReadExt: io::Read {
    /// Reads as many bytes as `buf` is long.
    ///
    /// This may issue multiple `read` calls internally. An error is returned
    /// if `read` read 0 bytes before the buffer is full.
    //  TODO: There is an RFC proposing a method like this for the standard library.
    fn read_into(&mut self, buf: &mut [u8]) -> io::Result<()>;

    /// Reads 4 bytes and returns them in an array.
    fn read_4_bytes(&mut self) -> io::Result<[u8; 4]>;

    /// Skip over `n` bytes.
    fn skip_bytes(&mut self, n: usize) -> io::Result<()>;

    /// Reads a single byte and interprets it as an 8-bit signed integer.
    fn read_i8(&mut self) -> io::Result<i8>;

    /// Reads a single byte and interprets it as an 8-bit unsigned integer.
    fn read_u8(&mut self) -> io::Result<u8>;

    /// Reads two bytes and interprets them as a little-endian 16-bit signed integer.
    fn read_le_i16(&mut self) -> io::Result<i16>;

    /// Reads two bytes and interprets them as a little-endian 16-bit unsigned integer.
    fn read_le_u16(&mut self) -> io::Result<u16>;

    /// Reads three bytes and interprets them as a little-endian 24-bit signed integer.
    ///
    /// The sign bit will be extended into the most significant byte.
    fn read_le_i24(&mut self) -> io::Result<i32>;

    /// Reads four bytes and interprets them as a little-endian 24-bit signed integer.
    ///
    /// The sign bit will be extended into the most significant byte.
    fn read_le_i24_4(&mut self) -> io::Result<i32>;

    /// Reads three bytes and interprets them as a little-endian 24-bit unsigned integer.
    ///
    /// The most significant byte will be 0.
    fn read_le_u24(&mut self) -> io::Result<u32>;

    /// Reads four bytes and interprets them as a little-endian 32-bit signed integer.
    fn read_le_i32(&mut self) -> io::Result<i32>;

    /// Reads four bytes and interprets them as a little-endian 32-bit unsigned integer.
    fn read_le_u32(&mut self) -> io::Result<u32>;

    /// Reads four bytes and interprets them as a little-endian 32-bit IEEE float.
    fn read_le_f32(&mut self) -> io::Result<f32>;
}

impl<R> ReadExt for R
    where R: io::Read
{
    #[inline(always)]
    fn read_into(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let mut n = 0;
        while n < buf.len() {
            let progress = try!(self.read(&mut buf[n..]));
            if progress > 0 {
                n += progress;
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to read enough bytes."));
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn skip_bytes(&mut self, n: usize) -> io::Result<()> {
        // Read from the input in chunks of 1024 bytes at a time, and discard
        // the result. 1024 is a tradeoff between doing a lot of calls, and
        // using too much stack space. This method is not in a hot path, so it
        // can afford to do this.
        let mut n_read = 0;
        let mut buf = [0u8; 1024];
        while n_read < n {
            let end = cmp::min(n - n_read, 1024);
            let progress = try!(self.read(&mut buf[0..end]));
            if progress > 0 {
                n_read += progress;
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to read enough bytes."));
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn read_4_bytes(&mut self) -> io::Result<[u8; 4]> {
        let mut buf = [0_u8; 4];
        try!(self.read_into(&mut buf[..]));
        Ok(buf)
    }

    #[inline(always)]
    fn read_i8(&mut self) -> io::Result<i8> {
        self.read_u8().map(|x| x as i8)
    }

    #[inline(always)]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        try!(self.read_into(&mut buf));
        Ok(buf[0])
    }

    #[inline(always)]
    fn read_le_i16(&mut self) -> io::Result<i16> {
        self.read_le_u16().map(|x| x as i16)
    }

    #[inline(always)]
    fn read_le_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        try!(self.read_into(&mut buf));
        Ok((buf[1] as u16) << 8 | (buf[0] as u16))
    }

    #[inline(always)]
    fn read_le_i24(&mut self) -> io::Result<i32> {
        self.read_le_u24().map(|x|
            // Test the sign bit, if it is set, extend the sign bit into the
            // most significant byte.
            if x & (1 << 23) == 0 {
                x as i32
            } else {
                (x | 0xff_00_00_00) as i32
            }
        )
    }

    #[inline(always)]
    fn read_le_i24_4(&mut self) -> io::Result<i32> {
        self.read_le_u32().map(|x|
            // Test the sign bit, if it is set, extend the sign bit into the
            // most significant byte. Otherwise, mask out the top byte.
            if x & (1 << 23) == 0 {
                (x & 0x00_ff_ff_ff) as i32
            } else {
                (x | 0xff_00_00_00) as i32
            }
        )
    }

    #[inline(always)]
    fn read_le_u24(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 3];
        try!(self.read_into(&mut buf));
        Ok((buf[2] as u32) << 16 | (buf[1] as u32) << 8 | (buf[0] as u32))
    }

    #[inline(always)]
    fn read_le_i32(&mut self) -> io::Result<i32> {
        self.read_le_u32().map(|x| x as i32)
    }

    #[inline(always)]
    fn read_le_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        try!(self.read_into(&mut buf));
        Ok((buf[3] as u32) << 24 | (buf[2] as u32) << 16 |
           (buf[1] as u32) << 8  | (buf[0] as u32) << 0)
    }

    #[inline(always)]
    fn read_le_f32(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        try!(self.read_into(&mut buf));
        Ok(f32::from_le_bytes(buf))
    }
}

/// The different chunks that a WAVE file can contain.
enum ChunkKind {
    Fmt,
    Fact,
    Data,
    Unknown,
}

/// Describes the structure of a chunk in the WAVE file.
struct ChunkHeader {
    pub kind: ChunkKind,
    pub len: u32,
}

/// A reader that reads the WAVE format from the underlying reader.
///
/// A `WavReader` is a streaming reader. It reads data from the underlying
/// reader on demand, and it reads no more than strictly necessary. No internal
/// buffering is performed on the underlying reader, but this can easily be
/// added by wrapping the reader in an `io::BufReader`. The `open` constructor
/// takes care of this for you.
pub struct WavReader<R> {
    /// Specification of the file as found in the fmt chunk.
    spec: WavSpec,

    /// The number of bytes used to store a sample in the stream.
    bytes_per_sample: u16,

    /// The number of samples in the data chunk.
    ///
    /// The data chunk is limited to a 4 GiB length because its header has a
    /// 32-bit length field. A sample takes at least one byte to store, so the
    /// number of samples is always less than 2^32.
    num_samples: u32,

    /// The number of samples read so far.
    samples_read: u32,

    /// The reader from which the WAVE format is read.
    reader: R,
}

/// An iterator that yields samples of type `S` read from a `WavReader`.
///
/// The type `S` must have at least as many bits as the bits per sample of the
/// file, otherwise every iteration will return an error.
pub struct WavSamples<'wr, R, S>
    where R: 'wr
{
    reader: &'wr mut WavReader<R>,
    phantom_sample: marker::PhantomData<S>,
}

/// An iterator that yields samples of type `S` read from a `WavReader`.
///
/// The type `S` must have at least as many bits as the bits per sample of the
/// file, otherwise every iteration will return an error.
pub struct WavIntoSamples<R, S> {
    reader: WavReader<R>,
    phantom_sample: marker::PhantomData<S>,
}

/// Reads the RIFF WAVE header, returns the supposed file size.
///
/// This function can be used to quickly check if the file could be a wav file
/// by reading 12 bytes of the header. If an `Ok` is returned, the file is
/// likely a wav file. If an `Err` is returned, it is definitely not a wav
/// file.
///
/// The returned file size cannot be larger than 2<sup>32</sup> + 7 bytes.
pub fn read_wave_header<R: io::Read>(reader: &mut R) -> Result<u64> {
    // Every WAVE file starts with the four bytes 'RIFF' and a file length.
    // TODO: the old approach of having a slice on the stack and reading
    // into it is more cumbersome, but also avoids a heap allocation. Is
    // the compiler smart enough to avoid the heap allocation anyway? I
    // would not expect it to be.
    if b"RIFF" != &try!(reader.read_4_bytes())[..] {
        return Err(Error::FormatError("no RIFF tag found"));
    }

    let file_len = try!(reader.read_le_u32());

    // Next four bytes indicate the file type, which should be WAVE.
    if b"WAVE" != &try!(reader.read_4_bytes())[..] {
        return Err(Error::FormatError("no WAVE tag found"));
    }

    // The stored file length does not include the "RIFF" magic and 4-byte
    // length field, so the total size is 8 bytes more than what is stored.
    Ok(file_len as u64 + 8)
}

/// Reads chunks until a data chunk is encountered.
///
/// Returns the information from the fmt chunk and the length of the data
/// chunk in bytes. Afterwards, the reader will be positioned at the first
/// content byte of the data chunk.
pub fn read_until_data<R: io::Read>(mut reader: R) -> Result<(WavSpecEx, u32)> {
    let mut spec_opt = None;

    loop {
        let header = try!(WavReader::read_chunk_header(&mut reader));
        match header.kind {
            ChunkKind::Fmt => {
                let spec = try!(WavReader::read_fmt_chunk(&mut reader, header.len));
                spec_opt = Some(spec);
            }
            ChunkKind::Fact => {
                // All (compressed) non-PCM formats must have a fact chunk
                // (Rev. 3 documentation). The chunk contains at least one
                // value, the number of samples in the file.
                //
                // The number of samples field is redundant for sampled
                // data, since the Data chunk indicates the length of the
                // data. The number of samples can be determined from the
                // length of the data and the container size as determined
                // from the Format chunk.
                // http://www-mmsp.ece.mcgill.ca/documents/audioformats/wave/wave.html
                let _samples_per_channel = reader.read_le_u32();
            }
            ChunkKind::Data => {
                // The "fmt" chunk must precede the "data" chunk. Any
                // chunks that come after the data chunk will be ignored.
                if let Some(spec) = spec_opt {
                    return Ok((spec, header.len));
                } else {
                    return Err(Error::FormatError("missing fmt chunk"));
                }
            }
            ChunkKind::Unknown => {
                // Ignore the chunk; skip all of its bytes.
                try!(reader.skip_bytes(header.len as usize));
            }
        }
        // If no data chunk is ever encountered, the function will return
        // via one of the try! macros that return an Err on end of file.
    }
}

impl<R> WavReader<R>
    where R: io::Read
{
    /// Attempts to read an 8-byte chunk header.
    fn read_chunk_header(reader: &mut R) -> Result<ChunkHeader> {
        let mut kind_str = [0; 4];
        try!(reader.read_into(&mut kind_str));
        let len = try!(reader.read_le_u32());

        let kind = match &kind_str[..] {
            b"fmt " => ChunkKind::Fmt,
            b"fact" => ChunkKind::Fact,
            b"data" => ChunkKind::Data,
            _ => ChunkKind::Unknown,
        };

        Ok(ChunkHeader { kind: kind, len: len })
    }

    /// Reads the fmt chunk of the file, returns the information it provides.
    fn read_fmt_chunk(reader: &mut R, chunk_len: u32) -> Result<WavSpecEx> {
        // A minimum chunk length of at least 16 is assumed. Note: actually,
        // the first 14 bytes contain enough information to fully specify the
        // file. I have not encountered a file with a 14-byte fmt section
        // though. If you ever encounter such file, please contact me.
        if chunk_len < 16 {
            return Err(Error::FormatError("invalid fmt chunk size"));
        }

        // Read the WAVEFORMAT struct, as defined at
        // https://msdn.microsoft.com/en-us/library/ms713498.aspx.
        // ```
        // typedef struct {
        //     WORD  wFormatTag;
        //     WORD  nChannels;
        //     DWORD nSamplesPerSec;
        //     DWORD nAvgBytesPerSec;
        //     WORD  nBlockAlign;
        // } WAVEFORMAT;
        // ```
        // The WAVEFORMATEX struct has two more members, as defined at
        // https://msdn.microsoft.com/en-us/library/ms713497.aspx
        // ```
        // typedef struct {
        //     WORD  wFormatTag;
        //     WORD  nChannels;
        //     DWORD nSamplesPerSec;
        //     DWORD nAvgBytesPerSec;
        //     WORD  nBlockAlign;
        //     WORD  wBitsPerSample;
        //     WORD  cbSize;
        // } WAVEFORMATEX;
        // ```
        // There is also PCMWAVEFORMAT as defined at
        // https://msdn.microsoft.com/en-us/library/dd743663.aspx.
        // ```
        // typedef struct {
        //   WAVEFORMAT wf;
        //   WORD       wBitsPerSample;
        // } PCMWAVEFORMAT;
        // ```
        // In either case, the minimal length of the fmt section is 16 bytes,
        // meaning that it does include the `wBitsPerSample` field. (The name
        // is misleading though, because it is the number of bits used to store
        // a sample, not all of the bits need to be valid for all versions of
        // the WAVE format.)
        let format_tag = try!(reader.read_le_u16());
        let n_channels = try!(reader.read_le_u16());
        let n_samples_per_sec = try!(reader.read_le_u32());
        let n_bytes_per_sec = try!(reader.read_le_u32());
        let block_align = try!(reader.read_le_u16());
        let bits_per_sample = try!(reader.read_le_u16());

        if n_channels == 0 {
            return Err(Error::FormatError("file contains zero channels"));
        }

        let bytes_per_sample = block_align / n_channels;
        // We allow bits_per_sample to be less than bytes_per_sample so that
        // we can support things such as 24 bit samples in 4 byte containers.
        if Some(bits_per_sample) > bytes_per_sample.checked_mul(8) {
            return Err(Error::FormatError("sample bits exceeds size of sample"));
        }

        // This field is redundant, and may be ignored. We do validate it to
        // fail early for ill-formed files.
        if Some(n_bytes_per_sec) != (block_align as u32).checked_mul(n_samples_per_sec) {
            return Err(Error::FormatError("inconsistent fmt chunk"));
        }

        // The bits per sample for a WAVEFORMAT struct is the number of bits
        // used to store a sample. Therefore, it must be a multiple of 8.
        if bits_per_sample % 8 != 0 {
            return Err(Error::FormatError("bits per sample is not a multiple of 8"));
        }

        if bits_per_sample == 0 {
            return Err(Error::FormatError("bits per sample is 0"));
        }

        let mut spec = WavSpec {
            channels: n_channels,
            sample_rate: n_samples_per_sec,
            bits_per_sample: bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        // The different format tag definitions can be found in mmreg.h that is
        // part of the Windows SDK. The vast majority are esoteric vendor-
        // specific formats. We handle only a few. The following values could
        // be of interest:
        const PCM: u16 = 0x0001;
        const ADPCM: u16 = 0x0002;
        const IEEE_FLOAT: u16 = 0x0003;
        const EXTENSIBLE: u16 = 0xfffe;
        // We may update our WavSpec based on more data we read from the header.
        match format_tag {
            PCM => try!(WavReader::read_wave_format_pcm(reader, chunk_len, &spec)),
            ADPCM => return Err(Error::Unsupported),
            IEEE_FLOAT => try!(WavReader::read_wave_format_ieee_float(reader, chunk_len, &mut spec)),
            EXTENSIBLE => try!(WavReader::read_wave_format_extensible(reader, chunk_len, &mut spec)),
            _ => return Err(Error::Unsupported),
        };

        Ok(WavSpecEx {
            spec: spec,
            bytes_per_sample: bytes_per_sample,
        })
    }

    fn read_wave_format_pcm(mut reader: R, chunk_len: u32, spec: &WavSpec) -> Result<()> {
        // When there is a PCMWAVEFORMAT struct, the chunk is 16 bytes long.
        // The WAVEFORMATEX structs includes two extra bytes, `cbSize`.
        let is_wave_format_ex = match chunk_len {
            16 => false,
            18 => true,
            // Other sizes are unexpected, but such files do occur in the wild,
            // and reading these files is still possible, so we allow this.
            40 => true,
            _ => return Err(Error::FormatError("unexpected fmt chunk size")),
        };

        if is_wave_format_ex {
            // `cbSize` can be used for non-PCM formats to specify the size of
            // additional data. However, for WAVE_FORMAT_PCM, the member should
            // be ignored, see https://msdn.microsoft.com/en-us/library/ms713497.aspx.
            // Nonzero values do in fact occur in practice.
            let _cb_size = try!(reader.read_le_u16());

            // For WAVE_FORMAT_PCM in WAVEFORMATEX, only 8 or 16 bits per
            // sample are valid according to
            // https://msdn.microsoft.com/en-us/library/ms713497.aspx.
            // 24 bits per sample is explicitly not valid inside a WAVEFORMATEX
            // structure, but such files do occur in the wild nonetheless, and
            // there is no good reason why we couldn't read them.
            match spec.bits_per_sample {
                8 => {}
                16 => {}
                24 => {}
                _ => return Err(Error::FormatError("bits per sample is not 8 or 16")),
            }
        }

        // If the chunk len was longer than expected, ignore the additional bytes.
        if chunk_len == 40 {
            try!(reader.skip_bytes(22));
        }
        Ok(())
    }

    fn read_wave_format_ieee_float(mut reader: R, chunk_len: u32, spec: &mut WavSpec) -> Result<()> {
        // When there is a PCMWAVEFORMAT struct, the chunk is 16 bytes long.
        // The WAVEFORMATEX structs includes two extra bytes, `cbSize`.
        let is_wave_format_ex = chunk_len == 18;

        if !is_wave_format_ex && chunk_len != 16 {
            return Err(Error::FormatError("unexpected fmt chunk size"));
        }

        if is_wave_format_ex {
            // For WAVE_FORMAT_IEEE_FLOAT which we are reading, there should
            // be no extra data, so `cbSize` should be 0.
            let cb_size = try!(reader.read_le_u16());
            if cb_size != 0 {
                return Err(Error::FormatError("unexpected WAVEFORMATEX size"));
            }
        }

        // For WAVE_FORMAT_IEEE_FLOAT, the bits_per_sample field should be
        // set to `32` according to
        // https://msdn.microsoft.com/en-us/library/windows/hardware/ff538799(v=vs.85).aspx.
        //
        // Note that some applications support 64 bits per sample. This is
        // not yet supported by hound.
        if spec.bits_per_sample != 32 {
            return Err(Error::FormatError("bits per sample is not 32"));
        }

        spec.sample_format = SampleFormat::Float;
        Ok(())
    }

    fn read_wave_format_extensible(mut reader: R, chunk_len: u32, spec: &mut WavSpec) -> Result<()> {
        // 16 bytes were read already, there must be two more for the `cbSize`
        // field, and `cbSize` itself must be at least 22, so the chunk length
        // must be at least 40.
        if chunk_len < 40 {
            return Err(Error::FormatError("unexpected fmt chunk size"));
        }

        // `cbSize` is the last field of the WAVEFORMATEX struct.
        let cb_size = try!(reader.read_le_u16());

        // `cbSize` must be at least 22, but in this case we assume that it is
        // 22, because we would not know how to handle extra data anyway.
        if cb_size != 22 {
            return Err(Error::FormatError("unexpected WAVEFORMATEXTENSIBLE size"));
        }

        // What follows is the rest of the `WAVEFORMATEXTENSIBLE` struct, as
        // defined at https://msdn.microsoft.com/en-us/library/ms713496.aspx.
        // ```
        // typedef struct {
        //   WAVEFORMATEX  Format;
        //   union {
        //     WORD  wValidBitsPerSample;
        //     WORD  wSamplesPerBlock;
        //     WORD  wReserved;
        //   } Samples;
        //   DWORD   dwChannelMask;
        //   GUID    SubFormat;
        // } WAVEFORMATEXTENSIBLE, *PWAVEFORMATEXTENSIBLE;
        // ```
        let valid_bits_per_sample = try!(reader.read_le_u16());
        let _channel_mask = try!(reader.read_le_u32()); // Not used for now.
        let mut subformat = [0u8; 16];
        try!(reader.read_into(&mut subformat));

        // Several GUIDS are defined. At the moment, only the following are supported:
        //
        // * KSDATAFORMAT_SUBTYPE_PCM (PCM audio with integer samples).
        // * KSDATAFORMAT_SUBTYPE_IEEE_FLOAT (PCM audio with floating point samples).
        let sample_format = match subformat {
            super::KSDATAFORMAT_SUBTYPE_PCM => SampleFormat::Int,
            super::KSDATAFORMAT_SUBTYPE_IEEE_FLOAT => SampleFormat::Float,
            _ => return Err(Error::Unsupported),
        };

        // Fallback to bits_per_sample if the valid_bits_per_sample is obviously wrong to support non standard headers found in the wild.
        if valid_bits_per_sample > 0 {
            spec.bits_per_sample = valid_bits_per_sample;
        }

        spec.sample_format = sample_format;
        Ok(())
    }

    /// Attempts to create a reader that reads the WAVE format.
    ///
    /// The header is read immediately. Reading the data will be done on
    /// demand.
    pub fn new(mut reader: R) -> Result<WavReader<R>> {
        try!(read_wave_header(&mut reader));
        let (spec_ex, data_len) = try!(read_until_data(&mut reader));

        let num_samples = data_len / spec_ex.bytes_per_sample as u32;

        // It could be that num_samples * bytes_per_sample < data_len.
        // If data_len is not a multiple of bytes_per_sample, there is some
        // trailing data. Either somebody is playing some steganography game,
        // but more likely something is very wrong, and we should refuse to
        // decode the file, as it is invalid.
        if num_samples * spec_ex.bytes_per_sample as u32 != data_len {
            let msg = "data chunk length is not a multiple of sample size";
            return Err(Error::FormatError(msg));
        }

        // The number of samples must be a multiple of the number of channels,
        // otherwise the last inter-channel sample would not have data for all
        // channels.
        if num_samples % spec_ex.spec.channels as u32 != 0 {
            return Err(Error::FormatError("invalid data chunk length"));
        }

        let wav_reader = WavReader {
            spec: spec_ex.spec,
            bytes_per_sample: spec_ex.bytes_per_sample,
            num_samples: num_samples,
            samples_read: 0,
            reader: reader,
        };

        Ok(wav_reader)
    }

    /// Returns information about the WAVE file.
    pub fn spec(&self) -> WavSpec {
        self.spec
    }

    /// Returns an iterator over all samples.
    ///
    /// The channel data is interleaved. The iterator is streaming. That is,
    /// if you call this method once, read a few samples, and call this method
    /// again, the second iterator will not start again from the beginning of
    /// the file, it will continue where the first iterator stopped.
    ///
    /// The type `S` must have at least `spec().bits_per_sample` bits,
    /// otherwise every iteration will return an error. All bit depths up to
    /// 32 bits per sample can be decoded into an `i32`, but if you know
    /// beforehand that you will be reading a file with 16 bits per sample, you
    /// can save memory by decoding into an `i16`.
    ///
    /// The type of `S` (int or float) must match `spec().sample_format`,
    /// otherwise every iteration will return an error.
    pub fn samples<'wr, S: Sample>(&'wr mut self) -> WavSamples<'wr, R, S> {
        WavSamples {
            reader: self,
            phantom_sample: marker::PhantomData,
        }
    }

    /// Same as `samples`, but takes ownership of the `WavReader`.
    ///
    /// See `samples()` for more info.
    pub fn into_samples<S: Sample>(self) -> WavIntoSamples<R, S> {
        WavIntoSamples {
            reader: self,
            phantom_sample: marker::PhantomData,
        }
    }

    /// Returns the duration of the file in samples.
    ///
    /// The duration is independent of the number of channels. It is expressed
    /// in units of samples. The duration in seconds can be obtained by
    /// dividing this number by the sample rate. The duration is independent of
    /// how many samples have been read already.
    pub fn duration(&self) -> u32 {
        self.num_samples / self.spec.channels as u32
    }

    /// Returns the number of values that the sample iterator will yield.
    ///
    /// The length of the file is its duration (in samples) times the number of
    /// channels. The length is independent of how many samples have been read
    /// already. To get the number of samples left, use `len()` on the
    /// `samples()` iterator.
    pub fn len(&self) -> u32 {
        self.num_samples
    }

    /// Destroys the `WavReader` and returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Seek to the given time within the file.
    ///
    /// The given time is measured in number of samples (independent of the
    /// number of channels) since the beginning of the audio data. To seek to
    /// a particular time in seconds, multiply the number of seconds with
    /// `WavSpec::sample_rate`. The given time should not exceed the duration of
    /// the file (returned by `duration()`). The behavior when seeking beyond
    /// `duration()` depends on the reader's `Seek` implementation.
    ///
    /// This method requires that the inner reader `R` implements `Seek`.
    pub fn seek(&mut self, time: u32) -> io::Result<()>
        where R: io::Seek,
    {
        let bytes_per_sample = self.spec.bits_per_sample / 8;
        let sample_position = time * self.spec.channels as u32;
        let offset_samples = sample_position as i64 - self.samples_read as i64;
        let offset_bytes = offset_samples * bytes_per_sample as i64;
        try!(self.reader.seek(io::SeekFrom::Current(offset_bytes)));
        self.samples_read = sample_position;
        Ok(())
    }
}

impl WavReader<io::BufReader<fs::File>> {
    /// Attempts to create a reader that reads from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a
    /// `BufReader` and then constructs a `WavReader` from it.
    pub fn open<P: AsRef<path::Path>>(filename: P) -> Result<WavReader<io::BufReader<fs::File>>> {
        let file = try!(fs::File::open(filename));
        let buf_reader = io::BufReader::new(file);
        WavReader::new(buf_reader)
    }
}

fn iter_next<R, S>(reader: &mut WavReader<R>) -> Option<Result<S>>
    where R: io::Read,
          S: Sample
{
    if reader.samples_read < reader.num_samples {
        reader.samples_read += 1;
        let sample = Sample::read(&mut reader.reader,
                                  reader.spec.sample_format,
                                  reader.bytes_per_sample,
                                  reader.spec.bits_per_sample);
        Some(sample.map_err(Error::from))
    } else {
        None
    }
}

fn iter_size_hint<R>(reader: &WavReader<R>) -> (usize, Option<usize>) {
    let samples_left = reader.num_samples - reader.samples_read;
    (samples_left as usize, Some(samples_left as usize))
}

impl<'wr, R, S> Iterator for WavSamples<'wr, R, S>
    where R: io::Read,
          S: Sample
{
    type Item = Result<S>;

    fn next(&mut self) -> Option<Result<S>> {
        iter_next(&mut self.reader)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        iter_size_hint(&self.reader)
    }
}

impl<'wr, R, S> ExactSizeIterator for WavSamples<'wr, R, S>
    where R: io::Read,
          S: Sample
{
}

impl<R, S> Iterator for WavIntoSamples<R, S>
    where R: io::Read,
          S: Sample
{
    type Item = Result<S>;

    fn next(&mut self) -> Option<Result<S>> {
        iter_next(&mut self.reader)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        iter_size_hint(&self.reader)
    }
}

impl<R, S> ExactSizeIterator for WavIntoSamples<R, S>
    where R: io::Read,
          S: Sample
{
}

#[test]
fn duration_and_len_agree() {
    let files = &["testsamples/pcmwaveformat-16bit-44100Hz-mono.wav",
                  "testsamples/waveformatex-16bit-44100Hz-stereo.wav",
                  "testsamples/waveformatextensible-32bit-48kHz-stereo.wav"];

    for fname in files {
        let reader = WavReader::open(fname).unwrap();
        assert_eq!(reader.spec().channels as u32 * reader.duration(),
                   reader.len());
    }
}

/// Tests reading a wave file with the PCMWAVEFORMAT struct.
#[test]
fn read_wav_pcm_wave_format_pcm() {
    let mut wav_reader = WavReader::open("testsamples/pcmwaveformat-16bit-44100Hz-mono.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().sample_rate, 44100);
    assert_eq!(wav_reader.spec().bits_per_sample, 16);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i16> = wav_reader.samples()
        .map(|r| r.unwrap())
        .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[2, -3, 5, -7]);
}

#[test]
fn read_wav_skips_unknown_chunks() {
    // The test samples are the same as without the -extra suffix, but ffmpeg
    // has kindly added some useless chunks in between the fmt and data chunk.
    let files = ["testsamples/pcmwaveformat-16bit-44100Hz-mono-extra.wav",
                 "testsamples/waveformatex-16bit-44100Hz-mono-extra.wav"];

    for file in &files {
        let mut wav_reader = WavReader::open(file).unwrap();

        assert_eq!(wav_reader.spec().channels, 1);
        assert_eq!(wav_reader.spec().sample_rate, 44100);
        assert_eq!(wav_reader.spec().bits_per_sample, 16);
        assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

        let sample = wav_reader.samples::<i16>().next().unwrap().unwrap();
        assert_eq!(sample, 2);
    }
}

#[test]
fn read_wav_0_valid_bits_fallback() {
    let mut wav_reader = WavReader::open("testsamples/nonstandard-02.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 2);
    assert_eq!(wav_reader.spec().sample_rate, 48000);
    assert_eq!(wav_reader.spec().bits_per_sample, 32);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
        .map(|r| r.unwrap())
        .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[19, -229373, 33587161, -2147483497]);
}

#[test]
fn len_and_size_hint_are_correct() {
    let mut wav_reader = WavReader::open("testsamples/pcmwaveformat-16bit-44100Hz-mono.wav")
        .unwrap();

    assert_eq!(wav_reader.len(), 4);

    {
        let mut samples = wav_reader.samples::<i16>();

        assert_eq!(samples.size_hint(), (4, Some(4)));
        samples.next();
        assert_eq!(samples.size_hint(), (3, Some(3)));
    }

    // Reading should not affect the initial length.
    assert_eq!(wav_reader.len(), 4);

    // Creating a new iterator resumes where the previous iterator stopped.
    {
        let mut samples = wav_reader.samples::<i16>();

        assert_eq!(samples.size_hint(), (3, Some(3)));
        samples.next();
        assert_eq!(samples.size_hint(), (2, Some(2)));
    }
}

#[test]
fn size_hint_is_exact() {
    let files = &["testsamples/pcmwaveformat-16bit-44100Hz-mono.wav",
                  "testsamples/waveformatex-16bit-44100Hz-stereo.wav",
                  "testsamples/waveformatextensible-32bit-48kHz-stereo.wav"];

    for fname in files {
        let mut reader = WavReader::open(fname).unwrap();
        let len = reader.len();
        let mut iter = reader.samples::<i32>();
        for i in 0..len {
            let remaining = (len - i) as usize;
            assert_eq!(iter.size_hint(), (remaining, Some(remaining)));
            assert!(iter.next().is_some());
        }
        assert!(iter.next().is_none());
    }
}

#[test]
fn samples_equals_into_samples() {
    let wav_reader_val = WavReader::open("testsamples/pcmwaveformat-8bit-44100Hz-mono.wav").unwrap();
    let mut wav_reader_ref = WavReader::open("testsamples/pcmwaveformat-8bit-44100Hz-mono.wav").unwrap();

    let samples_val: Vec<i16> = wav_reader_val.into_samples()
                                              .map(|r| r.unwrap())
                                              .collect();

    let samples_ref: Vec<i16> = wav_reader_ref.samples()
                                              .map(|r| r.unwrap())
                                              .collect();

    assert_eq!(samples_val, samples_ref);
}

/// Tests reading a wave file with the WAVEFORMATEX struct.
#[test]
fn read_wav_wave_format_ex_pcm() {
    let mut wav_reader = WavReader::open("testsamples/waveformatex-16bit-44100Hz-mono.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().sample_rate, 44100);
    assert_eq!(wav_reader.spec().bits_per_sample, 16);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i16> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[2, -3, 5, -7]);
}

#[test]
fn read_wav_wave_format_ex_ieee_float() {
    let mut wav_reader = WavReader::open("testsamples/waveformatex-ieeefloat-44100Hz-mono.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().sample_rate, 44100);
    assert_eq!(wav_reader.spec().bits_per_sample, 32);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Float);

    let samples: Vec<f32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[2.0, 3.0, -16411.0, 1019.0]);
}

#[test]
fn read_wav_stereo() {
    let mut wav_reader = WavReader::open("testsamples/waveformatex-16bit-44100Hz-stereo.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 2);
    assert_eq!(wav_reader.spec().sample_rate, 44100);
    assert_eq!(wav_reader.spec().bits_per_sample, 16);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i16> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact eight samples.
    assert_eq!(&samples[..], &[2, -3, 5, -7, 11, -13, 17, -19]);

}

#[test]
fn read_wav_pcm_wave_format_8bit() {
    let mut wav_reader = WavReader::open("testsamples/pcmwaveformat-8bit-44100Hz-mono.wav")
                                   .unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().bits_per_sample, 8);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i16> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[19, -53, 89, -127]);
}

/// Test reading 24 bit samples in a 4 byte container using the pcmwaveformat header. This is
/// technically a non-compliant wave file, but it is the sort of file generated by
/// 'arecord -f S24_LE -r 48000 -c 2 input.wav' so it should be supported.
#[test]
fn read_wav_pcm_wave_format_24bit_4byte() {
    let mut wav_reader = WavReader::open("testsamples/pcmwaveformat-24bit-4byte-48kHz-stereo.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 2);
    assert_eq!(wav_reader.spec().sample_rate, 48_000);
    assert_eq!(wav_reader.spec().bits_per_sample, 24);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[-96, 23_052, 8_388_607, -8_360_672]);
}

/// Regression test for a real-world wav file encountered in Quake.
#[test]
fn read_wav_wave_format_ex_8bit() {
    let mut wav_reader = WavReader::open("testsamples/waveformatex-8bit-11025Hz-mono.wav").unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().bits_per_sample, 8);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The audio data has been zeroed out, but for 8-bit files, a zero means a
    // sample value of 128.
    assert_eq!(&samples[..], &[-128, -128, -128, -128]);
}

/// This test sample tests both reading the WAVEFORMATEXTENSIBLE header, and 24-bit samples.
#[test]
fn read_wav_wave_format_extensible_pcm_24bit() {
    let mut wav_reader = WavReader::open("testsamples/waveformatextensible-24bit-192kHz-mono.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().sample_rate, 192_000);
    assert_eq!(wav_reader.spec().bits_per_sample, 24);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[-17, 4_194_319, -6_291_437, 8_355_817]);
}

/// This test sample tests both reading the WAVEFORMATEXTENSIBLE header, and 24-bit samples with a
/// 4 byte container size.
#[test]
fn read_wav_wave_format_extensible_pcm_24bit_4byte() {
    let mut wav_reader = WavReader::open("testsamples/waveformatextensible-24bit-4byte-48kHz-stereo.wav")
        .unwrap();

    assert_eq!(wav_reader.spec().channels, 2);
    assert_eq!(wav_reader.spec().sample_rate, 48_000);
    assert_eq!(wav_reader.spec().bits_per_sample, 24);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[-96, 23_052, 8_388_607, -8_360_672]);
}

#[test]
fn read_wav_32bit() {
    let mut wav_reader = WavReader::open("testsamples/waveformatextensible-32bit-48kHz-stereo.wav")
                                   .unwrap();

    assert_eq!(wav_reader.spec().bits_per_sample, 32);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[19, -229_373, 33_587_161, -2_147_483_497]);
}

#[test]
fn read_wav_wave_format_extensible_ieee_float() {
    let mut wav_reader =
        WavReader::open("testsamples/waveformatextensible-ieeefloat-44100Hz-mono.wav").unwrap();

    assert_eq!(wav_reader.spec().channels, 1);
    assert_eq!(wav_reader.spec().sample_rate, 44100);
    assert_eq!(wav_reader.spec().bits_per_sample, 32);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Float);

    let samples: Vec<f32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    // The test file has been prepared with these exact four samples.
    assert_eq!(&samples[..], &[2.0, 3.0, -16411.0, 1019.0]);
}

#[test]
fn read_wav_nonstandard_01() {
    // The test sample here is adapted from a file encountered in the wild (data
    // chunk replaced with two zero samples, some metadata dropped, and the file
    // length in the header fixed). It is not a valid file according to the
    // standard, but many players can deal with it nonetheless. (The file even
    // contains some metadata; open it in a hex editor if you would like to know
    // which program created it.) The file contains a regular PCM format tag,
    // but the size of the fmt chunk is one that would be expected of a
    // WAVEFORMATEXTENSIBLE chunk. The bits per sample is 24, which is invalid
    // for WAVEFORMATEX, but we can read it nonetheless.
    let mut wav_reader = WavReader::open("testsamples/nonstandard-01.wav").unwrap();

    assert_eq!(wav_reader.spec().bits_per_sample, 24);
    assert_eq!(wav_reader.spec().sample_format, SampleFormat::Int);

    let samples: Vec<i32> = wav_reader.samples()
                                      .map(|r| r.unwrap())
                                      .collect();

    assert_eq!(&samples[..], &[0, 0]);
}

#[test]
fn wide_read_should_signal_error() {
    let mut reader24 = WavReader::open("testsamples/waveformatextensible-24bit-192kHz-mono.wav")
        .unwrap();

    // Even though we know the first value is 17, and it should fit in an `i8`,
    // a general 24-bit sample will not fit in an `i8`, so this should fail.
    // 16-bit is still not wide enough, but 32-bit should do the trick.
    assert!(reader24.samples::<i8>().next().unwrap().is_err());
    assert!(reader24.samples::<i16>().next().unwrap().is_err());
    assert!(reader24.samples::<i32>().next().unwrap().is_ok());

    let mut reader32 = WavReader::open("testsamples/waveformatextensible-32bit-48kHz-stereo.wav")
        .unwrap();

    // In general, 32-bit samples will not fit in anything but an `i32`.
    assert!(reader32.samples::<i8>().next().unwrap().is_err());
    assert!(reader32.samples::<i16>().next().unwrap().is_err());
    assert!(reader32.samples::<i32>().next().unwrap().is_ok());
}

#[test]
fn sample_format_mismatch_should_signal_error() {
    let mut reader_f32 = WavReader::open("testsamples/waveformatex-ieeefloat-44100Hz-mono.wav")
        .unwrap();

    assert!(reader_f32.samples::<i8>().next().unwrap().is_err());
    assert!(reader_f32.samples::<i16>().next().unwrap().is_err());
    assert!(reader_f32.samples::<i32>().next().unwrap().is_err());
    assert!(reader_f32.samples::<f32>().next().unwrap().is_ok());

    let mut reader_i8 = WavReader::open("testsamples/pcmwaveformat-8bit-44100Hz-mono.wav").unwrap();

    assert!(reader_i8.samples::<i8>().next().unwrap().is_ok());
    assert!(reader_i8.samples::<i16>().next().unwrap().is_ok());
    assert!(reader_i8.samples::<i32>().next().unwrap().is_ok());
    assert!(reader_i8.samples::<f32>().next().unwrap().is_err());
}

#[test]
fn fuzz_crashes_should_be_fixed() {
    use std::fs;
    use std::ffi::OsStr;

    // This is a regression test: all crashes and other issues found through
    // fuzzing should not cause a crash.
    let dir = fs::read_dir("testsamples/fuzz").ok()
                 .expect("failed to enumerate fuzz test corpus");
    for path in dir {
        let path = path.ok().expect("failed to obtain path info").path();
        let is_file = fs::metadata(&path).unwrap().file_type().is_file();
        if is_file && path.extension() == Some(OsStr::new("wav")) {
            println!("    testing {} ...", path.to_str()
                                               .expect("unsupported filename"));
            let mut reader = match WavReader::open(path) {
                Ok(r) => r,
                Err(..) => continue,
            };
            match reader.spec().sample_format {
                SampleFormat::Int => {
                    for sample in reader.samples::<i32>() {
                        match sample {
                            Ok(..) => { }
                            Err(..) => break,
                        }
                    }
                }
                SampleFormat::Float => {
                    for sample in reader.samples::<f32>() {
                        match sample {
                            Ok(..) => { }
                            Err(..) => break,
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn seek_is_consistent() {
    let files = &["testsamples/pcmwaveformat-16bit-44100Hz-mono.wav",
                  "testsamples/waveformatex-16bit-44100Hz-stereo.wav",
                  "testsamples/waveformatextensible-32bit-48kHz-stereo.wav"];
    for fname in files {
        let mut reader = WavReader::open(fname).unwrap();

        // Seeking back to the start should "reset" the reader.
        let count = reader.samples::<i32>().count();
        reader.seek(0).unwrap();
        assert_eq!(reader.samples_read, 0);
        assert_eq!(count, reader.samples::<i32>().count());

        // Seek to the last sample.
        let last_time = reader.duration() - 1;
        let channels = reader.spec.channels;
        reader.seek(last_time).unwrap();
        {
            let mut samples = reader.samples::<i32>();
            for _ in 0..channels {
                assert!(samples.next().is_some());
            }
            assert!(samples.next().is_none());
        }

        // Seeking beyond the audio data produces no samples.
        let num_samples = reader.len();
        reader.seek(num_samples).unwrap();
        assert!(reader.samples::<i32>().next().is_none());
        reader.seek(::std::u32::MAX / channels as u32).unwrap();
        assert!(reader.samples::<i32>().next().is_none());
    }
}
