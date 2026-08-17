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

use std::fs;
use std::io;
use std::mem;
use std::io::{Seek, Write};
use std::mem::MaybeUninit;
use std::path;
use super::{Error, Result, Sample, SampleFormat, WavSpec, WavSpecEx};
use ::read;

/// Extends the functionality of `io::Write` with additional methods.
///
/// The methods may be used on any type that implements `io::Write`.
pub trait WriteExt: io::Write {
    /// Writes an unsigned 8-bit integer.
    fn write_u8(&mut self, x: u8) -> io::Result<()>;

    /// Writes a signed 16-bit integer in little endian format.
    fn write_le_i16(&mut self, x: i16) -> io::Result<()>;

    /// Writes an unsigned 16-bit integer in little endian format.
    fn write_le_u16(&mut self, x: u16) -> io::Result<()>;

    /// Writes a signed 24-bit integer in little endian format.
    ///
    /// The most significant byte of the `i32` is ignored.
    fn write_le_i24(&mut self, x: i32) -> io::Result<()>;

    /// Writes a signed 24-bit integer in 4-byte little endian format.
    ///
    /// The most significant byte of the `i32` is replaced with zeroes.
    fn write_le_i24_4(&mut self, x: i32) -> io::Result<()>;

    /// Writes an unsigned 24-bit integer in little endian format.
    ///
    /// The most significant byte of the `u32` is ignored.
    fn write_le_u24(&mut self, x: u32) -> io::Result<()>;

    /// Writes a signed 32-bit integer in little endian format.
    fn write_le_i32(&mut self, x: i32) -> io::Result<()>;

    /// Writes an unsigned 32-bit integer in little endian format.
    fn write_le_u32(&mut self, x: u32) -> io::Result<()>;

    /// Writes an IEEE float in little endian format.
    fn write_le_f32(&mut self, x: f32) -> io::Result<()>;
}

impl<W> WriteExt for W
    where W: io::Write
{
    #[inline(always)]
    fn write_u8(&mut self, x: u8) -> io::Result<()> {
        let buf = [x];
        self.write_all(&buf)
    }

    #[inline(always)]
    fn write_le_i16(&mut self, x: i16) -> io::Result<()> {
        self.write_le_u16(x as u16)
    }

    #[inline(always)]
    fn write_le_u16(&mut self, x: u16) -> io::Result<()> {
        let mut buf = [0u8; 2];
        buf[0] = (x & 0xff) as u8;
        buf[1] = (x >> 8) as u8;
        self.write_all(&buf)
    }

    #[inline(always)]
    fn write_le_i24(&mut self, x: i32) -> io::Result<()> {
        self.write_le_u24(x as u32)
    }

    #[inline(always)]
    fn write_le_i24_4(&mut self, x: i32) -> io::Result<()> {
        self.write_le_u32((x as u32) & 0x00_ff_ff_ff)
    }

    #[inline(always)]
    fn write_le_u24(&mut self, x: u32) -> io::Result<()> {
        let mut buf = [0u8; 3];
        buf[0] = ((x >> 00) & 0xff) as u8;
        buf[1] = ((x >> 08) & 0xff) as u8;
        buf[2] = ((x >> 16) & 0xff) as u8;
        self.write_all(&buf)
    }

    #[inline(always)]
    fn write_le_i32(&mut self, x: i32) -> io::Result<()> {
        self.write_le_u32(x as u32)
    }

    #[inline(always)]
    fn write_le_u32(&mut self, x: u32) -> io::Result<()> {
        let mut buf = [0u8; 4];
        buf[0] = ((x >> 00) & 0xff) as u8;
        buf[1] = ((x >> 08) & 0xff) as u8;
        buf[2] = ((x >> 16) & 0xff) as u8;
        buf[3] = ((x >> 24) & 0xff) as u8;
        self.write_all(&buf)
    }

    #[inline(always)]
    fn write_le_f32(&mut self, x: f32) -> io::Result<()> {
        let u = unsafe { mem::transmute::<f32, u32>(x) };
        self.write_le_u32(u)
    }
}

/// Generates a bitmask with `channels` ones in the least significant bits.
///
/// According to the [spec](https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible#remarks),
/// if `channels` is greater than the number of bits in the channel mask, 18 non-reserved bits,
/// extra channels are not assigned to any physical speaker location.  In this scenario, this
/// function will return a filled channel mask.
fn channel_mask(channels: u16) -> u32 {
    // Clamp to 0-18 to stay within reserved bits.
    let channels = if channels > 18 { 18 } else { channels };
    (0..channels as u32).map(|c| 1 << c).fold(0, |a, c| a | c)
}

#[test]
fn verify_channel_mask() {
    assert_eq!(channel_mask(0), 0);
    assert_eq!(channel_mask(1), 1);
    assert_eq!(channel_mask(2), 3);
    assert_eq!(channel_mask(3), 7);
    assert_eq!(channel_mask(4), 0xF);
    assert_eq!(channel_mask(8), 0xFF);
    assert_eq!(channel_mask(16), 0xFFFF);
    // expect channels >= 18 to yield the same mask
    assert_eq!(channel_mask(18), 0x3FFFF);
    assert_eq!(channel_mask(32), 0x3FFFF);
    assert_eq!(channel_mask(64), 0x3FFFF);
    assert_eq!(channel_mask(129), 0x3FFFF);
}

/// A writer that accepts samples and writes the WAVE format.
///
/// The writer needs a `WavSpec` or `WavSpecEx` that describes the audio
/// properties. Then samples can be written with `write_sample`. Channel data is
/// interleaved. The number of samples written must be a multiple of the number
/// of channels. After all samples have been written, the file must be
/// finalized. This can be done by calling `finalize`. If `finalize` is not
/// called, the file will be finalized upon drop. However, finalization may
/// fail, and without calling `finalize`, such a failure cannot be observed.
pub struct WavWriter<W>
    where W: io::Write + io::Seek
{
    /// Specifies properties of the audio data.
    spec: WavSpec,

    /// The (container) bytes per sample. This is the bit rate / 8 rounded up.
    bytes_per_sample: u16,

    /// The writer that will be written to.
    writer: W,

    /// The number of bytes written to the data section.
    ///
    /// This is an `u32` because WAVE cannot accomodate more data.
    data_bytes_written: u32,

    /// Whether the header has been finalized.
    finalized: bool,

    /// The buffer for the sample writer, which is recycled throughout calls to
    /// avoid allocating frequently.
    sample_writer_buffer: Vec<MaybeUninit<u8>>,

    /// The offset of the length field of the data chunk.
    ///
    /// This field needs to be overwritten after all data has been written. To
    /// support different size fmt chunks, and other chunks interspersed, the
    /// offset is flexible.
    data_len_offset: u32,
}

enum FmtKind {
    PcmWaveFormat,
    WaveFormatExtensible,
}

impl<W> WavWriter<W>
    where W: io::Write + io::Seek
{
    /// Creates a writer that writes the WAVE format to the underlying writer.
    ///
    /// The underlying writer is assumed to be at offset 0. `WavWriter` employs
    /// *no* buffering internally. It is recommended to wrap the writer in a
    /// `BufWriter` to avoid too many `write` calls. The `create()` constructor
    /// does this automatically.
    ///
    /// This writes parts of the header immediately, hence a `Result` is
    /// returned.
    pub fn new(writer: W, spec: WavSpec) -> Result<WavWriter<W>> {
        let spec_ex = WavSpecEx {
            spec: spec,
            bytes_per_sample: (spec.bits_per_sample + 7) / 8,
        };
        WavWriter::new_with_spec_ex(writer, spec_ex)
    }


    /// Creates a writer that writes the WAVE format to the underlying writer.
    ///
    /// The underlying writer is assumed to be at offset 0. `WavWriter` employs
    /// *no* buffering internally. It is recommended to wrap the writer in a
    /// `BufWriter` to avoid too many `write` calls. The `create()` constructor
    /// does this automatically.
    ///
    /// This writes parts of the header immediately, hence a `Result` is
    /// returned.
    pub fn new_with_spec_ex(writer: W, spec_ex: WavSpecEx) -> Result<WavWriter<W>> {
        let spec = spec_ex.spec;

        // Write the older PCMWAVEFORMAT structure if possible, because it is
        // more widely supported. For more than two channels or more than 16
        // bits per sample, the newer WAVEFORMATEXTENSIBLE is required. See also
        // https://msdn.microsoft.com/en-us/library/ms713497.aspx.
        let fmt_kind = if spec.channels > 2 || spec.bits_per_sample > 16 {
            FmtKind::WaveFormatExtensible
        } else {
            FmtKind::PcmWaveFormat
        };

        let mut writer = WavWriter {
            spec: spec,
            bytes_per_sample: spec_ex.bytes_per_sample,
            writer: writer,
            data_bytes_written: 0,
            sample_writer_buffer: Vec::new(),
            finalized: false,
            data_len_offset: match fmt_kind {
                FmtKind::WaveFormatExtensible => 64,
                FmtKind::PcmWaveFormat => 40,
            },
        };

        // Hound can only write those bit depths. If something else was
        // requested, fail early, rather than writing a header but then failing
        // at the first sample.
        let supported = match spec.bits_per_sample {
            8 => true,
            16 => true,
            24 => true,
            32 => true,
            _ => false,
        };

        if !supported {
            return Err(Error::Unsupported)
        }

        // Write headers, up to the point where data should be written.
        try!(writer.write_headers(fmt_kind));

        Ok(writer)
    }

    /// Writes the RIFF WAVE header, fmt chunk, and data chunk header.
    fn write_headers(&mut self, fmt_kind: FmtKind) -> io::Result<()> {
        // Write to an in-memory buffer before writing to the underlying writer.
        let mut header = [0u8; 68];

        {
            let mut buffer = io::Cursor::new(&mut header[..]);

            // Write the headers for the RIFF WAVE format.
            try!(buffer.write_all("RIFF".as_bytes()));

            // Skip 4 bytes that will be filled with the file size afterwards.
            try!(buffer.write_le_u32(0));

            try!(buffer.write_all("WAVE".as_bytes()));
            try!(buffer.write_all("fmt ".as_bytes()));

            match fmt_kind {
                FmtKind::PcmWaveFormat => {
                    try!(self.write_pcmwaveformat(&mut buffer));
                }
                FmtKind::WaveFormatExtensible => {
                    try!(self.write_waveformatextensible(&mut buffer));
                }
            }

            // Finally the header of the "data" chunk. The number of bytes
            // that this will take is not known at this point. The 0 will
            // be overwritten later.
            try!(buffer.write_all("data".as_bytes()));
            try!(buffer.write_le_u32(0));
        }

        // The data length field are the last 4 bytes of the header.
        let header_len = self.data_len_offset as usize + 4;

        self.writer.write_all(&header[..header_len])
    }

    /// Writes the spec as a WAVEFORMAT structure.
    ///
    /// The `WAVEFORMAT` struct is a subset of both `WAVEFORMATEX` and
    /// `WAVEFORMATEXTENSIBLE`. This does not write the `wFormatTag` member.
    fn write_waveformat(&self, buffer: &mut io::Cursor<&mut [u8]>) -> io::Result<()> {
        let spec = &self.spec;
        // The field nChannels.
        try!(buffer.write_le_u16(spec.channels));

        // The field nSamplesPerSec.
        try!(buffer.write_le_u32(spec.sample_rate));
        let bytes_per_sec = spec.sample_rate
                          * self.bytes_per_sample as u32
                          * spec.channels as u32;

        // The field nAvgBytesPerSec;
        try!(buffer.write_le_u32(bytes_per_sec));

        // The field nBlockAlign. Block align * sample rate = bytes per sec.
        try!(buffer.write_le_u16((bytes_per_sec / spec.sample_rate) as u16));

        Ok(())
    }

    /// Writes the content of the fmt chunk as PCMWAVEFORMAT struct.
    fn write_pcmwaveformat(&mut self, buffer: &mut io::Cursor<&mut [u8]>) -> io::Result<()> {
        // Write the size of the WAVE header chunk.
        try!(buffer.write_le_u32(16));

        // The following is based on the PCMWAVEFORMAT struct as documented at
        // https://msdn.microsoft.com/en-us/library/ms712832.aspx. See also
        // http://soundfile.sapp.org/doc/WaveFormat/.

        // The field wFormatTag
        match self.spec.sample_format {
            // WAVE_FORMAT_PCM
            SampleFormat::Int => {
                try!(buffer.write_le_u16(1));
            },
            // WAVE_FORMAT_IEEE_FLOAT
            SampleFormat::Float => {
                if self.spec.bits_per_sample == 32 {
                    try!(buffer.write_le_u16(3));
                } else {
                    panic!("Invalid number of bits per sample. \
                           When writing SampleFormat::Float, \
                           bits_per_sample must be 32.");
                }
            },
        };

        try!(self.write_waveformat(buffer));

        // The field wBitsPerSample, the real number of bits per sample.
        try!(buffer.write_le_u16(self.spec.bits_per_sample));

        // Note: for WAVEFORMATEX, there would be another 16-byte field `cbSize`
        // here that should be set to zero. And the header size would be 18
        // rather than 16.

        Ok(())
    }

    /// Writes the contents of the fmt chunk as WAVEFORMATEXTENSIBLE struct.
    fn write_waveformatextensible(&mut self, buffer: &mut io::Cursor<&mut [u8]>) -> io::Result<()> {
        // Write the size of the WAVE header chunk.
        try!(buffer.write_le_u32(40));

        // The following is based on the WAVEFORMATEXTENSIBLE struct, documented
        // at https://msdn.microsoft.com/en-us/library/ms713496.aspx and
        // https://msdn.microsoft.com/en-us/library/ms713462.aspx.

        // The field wFormatTag, value 1 means WAVE_FORMAT_PCM, but we use
        // the slightly more sophisticated WAVE_FORMAT_EXTENSIBLE.
        try!(buffer.write_le_u16(0xfffe));

        try!(self.write_waveformat(buffer));

        // The field wBitsPerSample. This is actually the size of the
        // container, so this is a multiple of 8.
        try!(buffer.write_le_u16(self.bytes_per_sample as u16 * 8));
        // The field cbSize, the number of remaining bytes in the struct.
        try!(buffer.write_le_u16(22));
        // The field wValidBitsPerSample, the real number of bits per sample.
        try!(buffer.write_le_u16(self.spec.bits_per_sample));
        // The field dwChannelMask.
        // TODO: add the option to specify the channel mask. For now, use
        // the default assignment.
        try!(buffer.write_le_u32(channel_mask(self.spec.channels)));

        // The field SubFormat.
        let subformat_guid = match self.spec.sample_format {
            // PCM audio with integer samples.
            SampleFormat::Int => super::KSDATAFORMAT_SUBTYPE_PCM,
            // PCM audio with 32-bit IEEE float samples.
            SampleFormat::Float => {
                if self.spec.bits_per_sample == 32 {
                    super::KSDATAFORMAT_SUBTYPE_IEEE_FLOAT
                } else {
                    panic!("Invalid number of bits per sample. \
                           When writing SampleFormat::Float, \
                           bits_per_sample must be 32.");
                }
            }
        };
        try!(buffer.write_all(&subformat_guid));

        Ok(())
    }

    /// Writes a single sample for one channel.
    ///
    /// WAVE interleaves channel data, so the channel that this writes the
    /// sample to depends on previous writes. This will return an error if the
    /// sample does not fit in the number of bits specified in the `WavSpec`.
    #[inline]
    pub fn write_sample<S: Sample>(&mut self, sample: S) -> Result<()> {
        try!(sample.write_padded(
            &mut self.writer,
            self.spec.bits_per_sample,
            self.bytes_per_sample,
        ));
        self.data_bytes_written += self.bytes_per_sample as u32;
        Ok(())
    }

    /// Create an efficient writer that writes 16-bit integer samples only.
    ///
    /// When it is known what the kind of samples will be, many dynamic checks
    /// can be omitted. Furthermore, this writer employs buffering internally,
    /// which allows omitting return value checks except on flush. The internal
    /// buffer will be sized such that exactly `num_samples` samples can be
    /// written to it, and the buffer is recycled across calls to
    /// `get_i16_writer()` if the previous buffer was sufficiently large.
    ///
    /// # Panics
    ///
    /// Panics if the spec does not match a 16 bits per sample integer format.
    ///
    /// Attempting to write more than `num_samples` samples to the writer will
    /// panic too.
    pub fn get_i16_writer<'s>(&'s mut self,
                              num_samples: u32)
                              -> SampleWriter16<'s, W> {
        if self.spec.sample_format != SampleFormat::Int {
            panic!("When calling get_i16_writer, the sample format must be int.");
        }
        if self.spec.bits_per_sample != 16 {
            panic!("When calling get_i16_writer, the number of bits per sample must be 16.");
        }

        let num_bytes = num_samples as usize * 2;

        if self.sample_writer_buffer.len() < num_bytes {
            // We need a bigger buffer. There is no point in growing the old
            // one, as we are going to overwrite the samples anyway, so just
            // allocate a new one.
            let mut new_buffer = Vec::<MaybeUninit<u8>>::with_capacity(num_bytes);

            // The potentially garbage memory here will not be exposed: the
            // buffer is only exposed when flushing, but `flush()` asserts that
            // all samples have been written.
            unsafe { new_buffer.set_len(num_bytes); }

            self.sample_writer_buffer = new_buffer;
        }

        SampleWriter16 {
            writer: &mut self.writer,
            buffer: &mut self.sample_writer_buffer[..num_bytes],
            data_bytes_written: &mut self.data_bytes_written,
            index: 0,
        }
    }

    fn update_header(&mut self) -> Result<()> {
        // The header size minus magic and 32-bit filesize (8 bytes).
        // The data chunk length (4 bytes) is the last part of the header.
        let header_size = self.data_len_offset + 4 - 8;
        let file_size = self.data_bytes_written + header_size;

        try!(self.writer.seek(io::SeekFrom::Start(4)));
        try!(self.writer.write_le_u32(file_size));
        try!(self.writer.seek(io::SeekFrom::Start(self.data_len_offset as u64)));
        try!(self.writer.write_le_u32(self.data_bytes_written));

        // Signal error if the last sample was not finished, but do so after
        // everything has been written, so that no data is lost, even though
        // the file is now ill-formed.
        if (self.data_bytes_written / self.bytes_per_sample as u32)
            % self.spec.channels as u32 != 0 {
            Err(Error::UnfinishedSample)
        } else {
            Ok(())
        }
    }

    /// Updates the WAVE header and flushes the underlying writer.
    ///
    /// Flush writes the WAVE header to the underlying writer to make the
    /// written bytes a valid wav file, and then flushes the writer. It is still
    /// possible to write more samples after flushing.
    ///
    /// Flush can be used for “checkpointing”. Even if after the flush there is
    /// an IO error or the writing process dies, the file can still be read by a
    /// compliant decoder up to the last flush.
    ///
    /// Note that if the number of samples written is not a multiple of the
    /// channel count, the intermediate wav file will not be valid. In that case
    /// `flush()` will still flush the data and write the (invalid) wav file,
    /// but `Error::UnfinishedSample` will be returned afterwards.
    ///
    /// It is not necessary to call `finalize()` directly after `flush()`, if no
    /// samples have been written after flushing.
    pub fn flush(&mut self) -> Result<()> {
        let current_pos = try!(self.writer.seek(io::SeekFrom::Current(0)));
        try!(self.update_header());
        try!(self.writer.flush());
        try!(self.writer.seek(io::SeekFrom::Start(current_pos)));
        Ok(())
    }

    /// Updates the WAVE header (which requires knowing all samples).
    ///
    /// This method must be called after all samples have been written. If it
    /// is not called, the destructor will finalize the file, but any errors
    /// that occur in the process cannot be observed in that manner.
    pub fn finalize(mut self) -> Result<()> {
        self.finalized = true;
        try!(self.update_header());
        // We need to perform a flush here to truly capture all errors before
        // the writer is dropped: for a buffered writer, the write to the buffer
        // may succeed, but the write to the underlying writer may fail. So
        // flush explicitly.
        try!(self.writer.flush());
        Ok(())
    }

    /// Returns information about the WAVE file being written.
    ///
    /// This is the same spec that was passed to `WavWriter::new()`. For a
    /// writer constructed with `WavWriter::new_append()` or
    /// `WavWriter::append()`, this method returns the spec of the file being
    /// appended to.
    pub fn spec(&self) -> WavSpec {
        self.spec
    }

    /// Returns the duration of the file written so far, in samples.
    ///
    /// The duration is independent of the number of channels. It is expressed
    /// in units of samples. The duration in seconds can be obtained by
    /// dividing this number by the sample rate.
    pub fn duration(&self) -> u32 {
        self.data_bytes_written / (self.bytes_per_sample as u32 * self.spec.channels as u32)
    }

    /// Returns the number of samples in the file written so far.
    ///
    /// The length of the file is its duration (in samples) times the number of
    /// channels.
    pub fn len(&self) -> u32 {
        self.data_bytes_written / self.bytes_per_sample as u32
    }
}

impl<W> Drop for WavWriter<W>
    where W: io::Write + io::Seek
{
    fn drop(&mut self) {
        // If the file was not explicitly finalized (to update the headers), do
        // it in the drop. This can fail, but drop should not panic, so a
        // failure is ignored silently here.
        if !self.finalized {
            let _r = self.update_header();
        }
    }
}

/// Reads the relevant parts of the header required to support append.
///
/// Returns (spec_ex, data_len, data_len_offset).
fn read_append<W: io::Read + io::Seek>(mut reader: &mut W) -> Result<(WavSpecEx, u32, u32)> {
    let (spec_ex, data_len) = {
        try!(read::read_wave_header(&mut reader));
        try!(read::read_until_data(&mut reader))
    };

    // Record the position of the data chunk length, so we can overwrite it
    // later.
    let data_len_offset = try!(reader.seek(io::SeekFrom::Current(0))) as u32 - 4;

    let spec = spec_ex.spec;
    let num_samples = data_len / spec_ex.bytes_per_sample as u32;

    // There must not be trailing bytes in the data chunk, otherwise the
    // bytes we write will be off.
    if num_samples * spec_ex.bytes_per_sample as u32 != data_len {
        let msg = "data chunk length is not a multiple of sample size";
        return Err(Error::FormatError(msg));
    }

    // Hound cannot read or write other bit depths than those, so rather
    // than refusing to write later, fail early.
    let supported = match (spec_ex.bytes_per_sample, spec.bits_per_sample) {
        (1, 8) => true,
        (2, 16) => true,
        (3, 24) => true,
        (4, 32) => true,
        _ => false,
    };

    if !supported {
        return Err(Error::Unsupported);
    }

    // The number of samples must be a multiple of the number of channels,
    // otherwise the last inter-channel sample would not have data for all
    // channels.
    if num_samples % spec_ex.spec.channels as u32 != 0 {
        return Err(Error::FormatError("invalid data chunk length"));
    }

    Ok((spec_ex, data_len, data_len_offset))
}

impl WavWriter<io::BufWriter<fs::File>> {
    /// Creates a writer that writes the WAVE format to a file.
    ///
    /// This is a convenience constructor that creates the file, wraps it in a
    /// `BufWriter`, and then constructs a `WavWriter` from it. The file will
    /// be overwritten if it exists.
    pub fn create<P: AsRef<path::Path>>(filename: P,
                                        spec: WavSpec)
                                        -> Result<WavWriter<io::BufWriter<fs::File>>> {
        let file = try!(fs::File::create(filename));
        let buf_writer = io::BufWriter::new(file);
        WavWriter::new(buf_writer, spec)
    }

    /// Creates a writer that appends samples to an existing file.
    ///
    /// This is a convenience constructor that opens the file in append mode,
    /// reads its header using a buffered reader, and then constructs an
    /// appending `WavWriter` that writes to the file using a `BufWriter`.
    ///
    /// See `WavWriter::new_append()` for more details about append behavior.
    pub fn append<P: AsRef<path::Path>>(filename: P) -> Result<WavWriter<io::BufWriter<fs::File>>> {
        // Open the file in append mode, start reading from the start.
        let mut file = try!(fs::OpenOptions::new().read(true).write(true).open(filename));
        try!(file.seek(io::SeekFrom::Start(0)));

        // Read the header using a buffered reader.
        let mut buf_reader = io::BufReader::new(file);
        let (spec_ex, data_len, data_len_offset) = try!(read_append(&mut buf_reader));
        let mut file = buf_reader.into_inner();

        // Seek to the data position, and from now on, write using a buffered
        // writer.
        try!(file.seek(io::SeekFrom::Current(data_len as i64)));
        let buf_writer = io::BufWriter::new(file);

        let writer = WavWriter {
            spec: spec_ex.spec,
            bytes_per_sample: spec_ex.bytes_per_sample,
            writer: buf_writer,
            data_bytes_written: data_len,
            sample_writer_buffer: Vec::new(),
            finalized: false,
            data_len_offset: data_len_offset,
        };

        Ok(writer)
    }
}

impl<W> WavWriter<W> where W: io::Read + io::Write + io::Seek {
    /// Creates a writer that appends samples to an existing file stream.
    ///
    /// This first reads the existing header to obtain the spec, then seeks to
    /// the end of the writer. The writer then appends new samples to the end of
    /// the stream.
    ///
    /// The underlying writer is assumed to be at offset 0.
    ///
    /// If the existing file includes a fact chunk, it will not be updated after
    /// appending, and hence become outdated. For files produced by Hound this
    /// is not an issue, because Hound never writes a fact chunk. For all the
    /// formats that Hound can write, the fact chunk is redundant.
    pub fn new_append(mut writer: W) -> Result<WavWriter<W>> {
        let (spec_ex, data_len, data_len_offset) = try!(read_append(&mut writer));
        try!(writer.seek(io::SeekFrom::Current(data_len as i64)));
        let writer = WavWriter {
            spec: spec_ex.spec,
            bytes_per_sample: spec_ex.bytes_per_sample,
            writer: writer,
            data_bytes_written: data_len,
            sample_writer_buffer: Vec::new(),
            finalized: false,
            data_len_offset: data_len_offset,
        };

        Ok(writer)
    }
}


/// A writer that specifically only writes integer samples of 16 bits per sample.
///
/// The writer buffers written samples internally so they can be written in a
/// single batch later on. This has two advantages when performance is
/// important:
///
///  * There is no need for error handling during writing, only on flush. This
///    eliminates a lot of branches.
///  * The buffer can be written once, which reduces the overhead of the write
///    call. Because writing to an `io::BufWriter` is implemented with a
///    `memcpy` (even for single bytes), there is a large overhead to writing
///    small amounts of data such as a 16-bit sample. By writing large blocks
///    (or by not using `BufWriter`) this overhead can be avoided.
///
/// A `SampleWriter16` can be obtained by calling [`WavWriter::get_i16_writer`](
/// struct.WavWriter.html#method.get_i16_writer).
pub struct SampleWriter16<'parent, W> where W: io::Write + io::Seek + 'parent {
    /// The writer borrowed from the wrapped WavWriter.
    writer: &'parent mut W,

    /// The internal buffer that samples are written to before they are flushed.
    buffer: &'parent mut [MaybeUninit<u8>],

    /// Reference to the `data_bytes_written` field of the writer.
    data_bytes_written: &'parent mut u32,

    /// The index into the buffer where the next bytes will be written.
    index: u32,
}

impl<'parent, W: io::Write + io::Seek> SampleWriter16<'parent, W> {
    /// Writes a single sample for one channel.
    ///
    /// WAVE interleaves channel data, so the channel that this writes the
    /// sample to depends on previous writes.
    ///
    /// Unlike `WavWriter::write_sample()`, no range check is performed. Only
    /// the least significant 16 bits are considered, everything else is
    /// discarded.  Apart from that check, this method is more efficient than
    /// `WavWriter::write_sample()`, because it can avoid dispatching on the
    /// number of bits. That was done already when the `SampleWriter16` was
    /// constructed.
    ///
    /// Note that nothing is actually written until `flush()` is called.
    #[inline(always)]
    pub fn write_sample<S: Sample>(&mut self, sample: S) {
        assert!((self.index as usize) + 2 <= self.buffer.len(),
          "Trying to write more samples than reserved for the sample writer.");

        // SAFETY: We performed the bounds check in the above assertion.
        unsafe { self.write_sample_unchecked(sample) };
    }

    unsafe fn write_u16_le_unchecked(&mut self, value: u16) {
        // On little endian machines the compiler produces assembly code
        // that merges the following two lines into a single instruction.
        *self.buffer.get_unchecked_mut(self.index as usize) = MaybeUninit::new(value as u8);
        self.buffer.get_unchecked_mut(self.index as usize).assume_init();
        *self.buffer.get_unchecked_mut(self.index as usize + 1) = MaybeUninit::new((value >> 8) as u8);
        self.buffer.get_unchecked_mut(self.index as usize + 1).assume_init();
    }

    /// Like `write_sample()`, but does not perform a bounds check when writing
    /// to the internal buffer.
    ///
    /// It is the responsibility of the programmer to ensure that no more
    /// samples are written than allocated when the writer was created.
    #[inline(always)]
    pub unsafe fn write_sample_unchecked<S: Sample>(&mut self, sample: S) {
        self.write_u16_le_unchecked(sample.as_i16() as u16);
        self.index += 2;
    }

    /// Flush the internal buffer to the underlying writer.
    ///
    /// # Panics
    ///
    /// Panics if insufficient samples (less than specified when the writer was
    /// constructed) have been written with `write_sample()`.
    pub fn flush(self) -> Result<()> {
        if self.index as usize != self.buffer.len() {
            panic!("Insufficient samples written to the sample writer.");
        }

        // SAFETY: casting `self.buffer` to a `*const [MaybeUninit<u8>]` is safe
        // since the caller guarantees that `self.buffer` is initialized, and
        // `MaybeUninit<u8>` is guaranteed to have the same layout as `u8`. The
        // pointer obtained is valid since it refers to memory owned by
        // `self.buffer` which is a reference and thus guaranteed to be valid
        // for reads. This is copied from the nightly implementation for
        // slice_assume_init_ref.
        let slice = unsafe { &*(self.buffer as *const [MaybeUninit<u8>] as *const [u8]) };

        try!(self.writer.write_all(slice));

        *self.data_bytes_written += self.buffer.len() as u32;
        Ok(())
    }
}

#[test]
fn short_write_should_signal_error() {
    use SampleFormat;

    let mut buffer = io::Cursor::new(Vec::new());

    let write_spec = WavSpec {
        channels: 17,
        sample_rate: 48000,
        bits_per_sample: 8,
        sample_format: SampleFormat::Int,
    };

    // Deliberately write one sample less than 17 * 5.
    let mut writer = WavWriter::new(&mut buffer, write_spec).unwrap();
    for s in 0..17 * 5 - 1 {
        writer.write_sample(s as i16).unwrap();
    }
    let error = writer.finalize().err().unwrap();

    match error {
        Error::UnfinishedSample => {}
        _ => panic!("UnfinishedSample error should have been returned."),
    }
}

#[test]
fn wide_write_should_signal_error() {
    let mut buffer = io::Cursor::new(Vec::new());

    let spec8 = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 8,
        sample_format: SampleFormat::Int,
    };
    {
        let mut writer = WavWriter::new(&mut buffer, spec8).unwrap();
        assert!(writer.write_sample(127_i8).is_ok());
        assert!(writer.write_sample(127_i16).is_ok());
        assert!(writer.write_sample(127_i32).is_ok());
        assert!(writer.write_sample(128_i16).is_err());
        assert!(writer.write_sample(128_i32).is_err());
    }

    let spec16 = WavSpec { bits_per_sample: 16, ..spec8 };
    {
        let mut writer = WavWriter::new(&mut buffer, spec16).unwrap();
        assert!(writer.write_sample(32767_i16).is_ok());
        assert!(writer.write_sample(32767_i32).is_ok());
        assert!(writer.write_sample(32768_i32).is_err());
    }

    let spec24 = WavSpec { bits_per_sample: 24, ..spec8 };
    {
        let mut writer = WavWriter::new(&mut buffer, spec24).unwrap();
        assert!(writer.write_sample(8_388_607_i32).is_ok());
        assert!(writer.write_sample(8_388_608_i32).is_err());
    }
}

#[test]
fn s24_wav_write() {
    use std::fs::File;
    use std::io::Read;
    let mut buffer = io::Cursor::new(Vec::new());

    let spec = WavSpecEx {
        spec: WavSpec {
            channels: 2,
            sample_rate: 48000,
            bits_per_sample: 24,
            sample_format: SampleFormat::Int,
        },
        bytes_per_sample: 4,
    };
    {
        let mut writer = WavWriter::new_with_spec_ex(&mut buffer, spec).unwrap();
        assert!(writer.write_sample(-96_i32).is_ok());
        assert!(writer.write_sample(23_052_i32).is_ok());
        assert!(writer.write_sample(8_388_607_i32).is_ok());
        assert!(writer.write_sample(-8_360_672_i32).is_ok());
    }

    let mut expected = Vec::new();
    File::open("testsamples/waveformatextensible-24bit-4byte-48kHz-stereo.wav")
        .unwrap()
        .read_to_end(&mut expected)
        .unwrap();

    assert_eq!(buffer.into_inner(), expected);
}
