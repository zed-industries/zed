//! Classifies file contents as text or binary and decodes text of any
//! supported encoding into UTF-8, both whole and as a stream.

use std::io::{self, Read};

use anyhow::Result;
use chardetng::EncodingDetector;
use encoding_rs::{CoderResult, Decoder, Encoding, UTF_8, UTF_16BE, UTF_16LE};

/// Number of leading bytes that [`analyze_byte_content`] inspects.
pub const FILE_ANALYSIS_BYTES: usize = 1024;

const ENCODING_DETECTION_BYTES: usize = 1024 * 1024;

const DECODING_BLOCK_BYTES: usize = 64 * 1024;

/// Text produced by [`decode_text`], with what is needed to write it back via [`encode_text`].
pub struct DecodedText {
    /// The decoded UTF-8 text.
    pub text: String,
    /// The encoding the bytes were decoded from.
    pub encoding: &'static Encoding,
    /// Whether the bytes started with a byte order mark.
    pub has_bom: bool,
}

/// Inspects the start of a file.
/// Returns the BOM encoding, if any, and otherwise the [`ByteContent`] classification of the prefix.
pub fn decode_byte_header(prefix: &[u8]) -> (Option<&'static Encoding>, ByteContent) {
    if let Some((encoding, _bom_len)) = Encoding::for_bom(prefix) {
        return (Some(encoding), ByteContent::Unknown);
    }
    (None, analyze_byte_content(prefix))
}

/// Decodes a whole file into UTF-8, detecting its encoding.
/// Fails when the bytes are classified as [`ByteContent::Binary`].
pub fn decode_text(bytes: Vec<u8>) -> Result<DecodedText> {
    let (bom_encoding, byte_content) = decode_byte_header(&bytes);
    if let Some(encoding) = bom_encoding {
        let (text, _) = encoding.decode_with_bom_removal(&bytes);
        return Ok(DecodedText {
            text: text.into_owned(),
            encoding,
            has_bom: true,
        });
    }

    if let Some(encoding) = byte_content.encoding() {
        let (text, _, _) = encoding.decode(&bytes);
        return Ok(DecodedText {
            text: text.into_owned(),
            encoding,
            has_bom: false,
        });
    }
    anyhow::ensure!(
        byte_content != ByteContent::Binary,
        "Binary files are not supported"
    );

    match String::from_utf8(bytes) {
        Ok(text) if !text.contains('\x1b') => Ok(DecodedText {
            text,
            encoding: UTF_8,
            has_bom: false,
        }),
        Ok(text) => Ok(decode_with_detected_encoding(text.into_bytes())),
        Err(error) => Ok(decode_with_detected_encoding(error.into_bytes())),
    }
}

/// Inverse of [`decode_text`]: encodes UTF-8 text into `encoding`, prepending a BOM when asked.
pub fn encode_text(text: String, encoding: &'static Encoding, has_bom: bool) -> Vec<u8> {
    if encoding == UTF_8 && !has_bom {
        return text.into_bytes();
    }

    // encoding_rs follows the WHATWG standard and encodes UTF-16 labels as UTF-8.
    if encoding == UTF_16BE {
        let mut bytes = Vec::with_capacity(text.len() * 2 + 2);
        if has_bom {
            bytes.extend_from_slice(&[0xFE, 0xFF]);
        }
        bytes.extend(text.encode_utf16().flat_map(u16::to_be_bytes));
        return bytes;
    }

    if encoding == UTF_16LE {
        let mut bytes = Vec::with_capacity(text.len() * 2 + 2);
        if has_bom {
            bytes.extend_from_slice(&[0xFF, 0xFE]);
        }
        bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        return bytes;
    }

    let (encoded, _, _) = encoding.encode(&text);
    if has_bom && encoding == UTF_8 {
        let mut bytes = Vec::with_capacity(encoded.len() + 3);
        bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        bytes.extend_from_slice(&encoded);
        bytes
    } else {
        encoded.into_owned()
    }
}

/// A [`Read`] adapter that transcodes its source into UTF-8 block by block, with bounded memory.
/// When constructed without an encoding, it passes leading ASCII through and guesses the encoding
/// from a bounded window of the following bytes.
pub struct DecodingReader<'a> {
    inner: &'a mut (dyn Read + Send),
    decoder: Option<Decoder>,
    input: Vec<u8>,
    pending: Vec<u8>,
    output: Vec<u8>,
    output_position: usize,
    input_exhausted: bool,
    finished: bool,
}

impl<'a> DecodingReader<'a> {
    /// Wraps `inner`, decoding with `encoding` or detecting one lazily when `None`.
    pub fn new(inner: &'a mut (dyn Read + Send), encoding: Option<&'static Encoding>) -> Self {
        Self {
            inner,
            decoder: encoding.map(Encoding::new_decoder),
            input: vec![0; DECODING_BLOCK_BYTES],
            pending: Vec::new(),
            output: Vec::new(),
            output_position: 0,
            input_exhausted: false,
            finished: false,
        }
    }

    /// The encoding in use, `None` until detection has happened.
    pub fn encoding(&self) -> Option<&'static Encoding> {
        self.decoder.as_ref().map(Decoder::encoding)
    }

    fn fill_output(&mut self) -> io::Result<()> {
        self.output.clear();
        self.output_position = 0;

        let read = if self.input_exhausted {
            0
        } else {
            self.inner.read(&mut self.input)?
        };
        if read == 0 {
            self.input_exhausted = true;
        }
        let block = &self.input[..read];

        self.finished = self.input_exhausted;
        match self.decoder.as_mut() {
            Some(decoder) => decode_block(decoder, block, self.input_exhausted, &mut self.output),
            None => {
                if self.pending.is_empty() {
                    let Some(ascii_len) = first_non_ascii_or_escape(block) else {
                        self.output.extend_from_slice(block);
                        return Ok(());
                    };
                    self.output.extend_from_slice(&block[..ascii_len]);
                    self.pending.extend_from_slice(&block[ascii_len..]);
                } else {
                    self.pending.extend_from_slice(block);
                }
                if self.pending.len() < ENCODING_DETECTION_BYTES && !self.input_exhausted {
                    return Ok(());
                }

                let mut decoder =
                    detect_encoding(&self.pending, self.input_exhausted).new_decoder();
                let pending = std::mem::take(&mut self.pending);
                decode_block(
                    &mut decoder,
                    &pending,
                    self.input_exhausted,
                    &mut self.output,
                )?;
                self.decoder = Some(decoder);
                Ok(())
            }
        }
    }
}

impl Read for DecodingReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        while self.output_position == self.output.len() {
            if self.finished {
                return Ok(0);
            }
            self.fill_output()?;
        }

        let available = &self.output[self.output_position..];
        let count = available.len().min(buffer.len());
        buffer[..count].copy_from_slice(&available[..count]);
        self.output_position += count;
        Ok(count)
    }
}

/// Verdict of [`analyze_byte_content`] on a file prefix.
#[derive(Debug, PartialEq)]
pub enum ByteContent {
    /// UTF-16 little-endian text without a BOM.
    Utf16Le,
    /// UTF-16 big-endian text without a BOM.
    Utf16Be,
    /// Not text; the file must not be decoded or searched.
    Binary,
    /// Text in UTF-8 or some yet unknown encoding.
    Unknown,
}

impl ByteContent {
    /// The encoding this classification pins down, if any.
    pub fn encoding(&self) -> Option<&'static Encoding> {
        match self {
            Self::Utf16Le => Some(UTF_16LE),
            Self::Utf16Be => Some(UTF_16BE),
            Self::Binary | Self::Unknown => None,
        }
    }
}

/// Classifies the first [`FILE_ANALYSIS_BYTES`] of a file.
///
/// Heuristic check using null byte distribution plus a generic text-likeness
/// heuristic. This prefers UTF-16 when many bytes are NUL and otherwise
/// distinguishes between text-like and binary-like content.
pub fn analyze_byte_content(bytes: &[u8]) -> ByteContent {
    if bytes.len() < 2 {
        return ByteContent::Unknown;
    }

    if is_known_binary_header(bytes) {
        return ByteContent::Binary;
    }

    let limit = bytes.len().min(FILE_ANALYSIS_BYTES);
    let mut even_null_count = 0usize;
    let mut odd_null_count = 0usize;
    let mut non_text_like_count = 0usize;
    let mut seen_control_bytes = 0u128;

    for (i, &byte) in bytes[..limit].iter().enumerate() {
        if byte == 0 {
            if i % 2 == 0 {
                even_null_count += 1;
            } else {
                odd_null_count += 1;
            }
            non_text_like_count += 1;
            continue;
        }

        let is_text_like = match byte {
            b'\t' | b'\n' | b'\r' | 0x0C | 0x1B => true,
            0x20..=0x7E => true,
            0x80..=0xFF => true,
            _ => false,
        };

        if !is_text_like {
            non_text_like_count += 1;
            seen_control_bytes |= 1u128 << byte;
        }
    }

    let total_null_count = even_null_count + odd_null_count;
    let has_significant_nulls = total_null_count > 0 && total_null_count >= limit / 16;
    let nulls_skew_to_even = even_null_count > odd_null_count * 4;
    let nulls_skew_to_odd = odd_null_count > even_null_count * 4;

    if has_significant_nulls {
        let sample = &bytes[..limit];

        // UTF-16BE ASCII: [0x00, char] — nulls at even positions (high byte first)
        // UTF-16LE ASCII: [char, 0x00] — nulls at odd positions (low byte first)

        if nulls_skew_to_even && is_plausible_utf16_text(sample, false) {
            return ByteContent::Utf16Be;
        }

        if nulls_skew_to_odd && is_plausible_utf16_text(sample, true) {
            return ByteContent::Utf16Le;
        }

        return ByteContent::Binary;
    }

    let few_distinct_control_bytes = seen_control_bytes.count_ones() < 4;
    if non_text_like_count * 100 < limit * 8
        || (total_null_count == 0 && few_distinct_control_bytes)
    {
        ByteContent::Unknown
    } else {
        ByteContent::Binary
    }
}

fn decode_block(
    decoder: &mut Decoder,
    block: &[u8],
    last: bool,
    output: &mut Vec<u8>,
) -> io::Result<()> {
    let capacity = decoder
        .max_utf8_buffer_length(block.len())
        .ok_or_else(|| io::Error::new(io::ErrorKind::OutOfMemory, "decoded block is too large"))?;
    let start = output.len();
    output.resize(start + capacity, 0);
    let (result, consumed, written, _) = decoder.decode_to_utf8(block, &mut output[start..], last);
    output.truncate(start + written);
    if result != CoderResult::InputEmpty || consumed != block.len() {
        return Err(io::Error::other("decoder did not consume the whole block"));
    }
    Ok(())
}

fn detect_encoding(bytes: &[u8], complete: bool) -> &'static Encoding {
    let start = first_non_ascii_or_escape(bytes).unwrap_or(0);
    let end = bytes
        .len()
        .min(start.saturating_add(ENCODING_DETECTION_BYTES));
    let mut detector = EncodingDetector::new();
    detector.feed(&bytes[start..end], complete && end == bytes.len());
    detector.guess(None, true)
}

fn decode_with_detected_encoding(bytes: Vec<u8>) -> DecodedText {
    let encoding = detect_encoding(&bytes, true);
    let (text, _, _) = encoding.decode(&bytes);
    DecodedText {
        text: text.into_owned(),
        encoding,
        has_bom: false,
    }
}

fn first_non_ascii_or_escape(bytes: &[u8]) -> Option<usize> {
    const CHUNK: usize = 4096;
    bytes
        .chunks(CHUNK)
        .enumerate()
        .find_map(|(chunk_index, chunk)| {
            if chunk.is_ascii() && !chunk.contains(&0x1B) {
                return None;
            }
            chunk
                .iter()
                .position(|&byte| !byte.is_ascii() || byte == 0x1B)
                .map(|offset| chunk_index * CHUNK + offset)
        })
}

const KNOWN_BINARY_HEADERS: &[&[u8]] = &[
    b"%PDF-",                            // PDF
    b"PK\x03\x04",                       // ZIP local header
    b"PK\x05\x06",                       // ZIP end of central directory
    b"PK\x07\x08",                       // ZIP spanning/splitting
    b"\x89PNG\r\n\x1a\n",                // PNG
    b"\xFF\xD8\xFF",                     // JPEG
    b"GIF87a",                           // GIF87a
    b"GIF89a",                           // GIF89a
    b"IWAD",                             // Doom IWAD archive
    b"PWAD",                             // Doom PWAD archive
    b"RIFF",                             // WAV, AVI, WebP
    b"OggS",                             // OGG (Vorbis, Opus, FLAC)
    b"fLaC",                             // FLAC
    b"ID3",                              // MP3 with ID3v2 tag
    b"\xFF\xFB",                         // MP3 frame sync (MPEG1 Layer3)
    b"\xFF\xFA",                         // MP3 frame sync (MPEG1 Layer3)
    b"\xFF\xF3",                         // MP3 frame sync (MPEG2 Layer3)
    b"\xFF\xF2",                         // MP3 frame sync (MPEG2 Layer3)
    b"\xFF\xF1",                         // AAC ADTS frame sync (MPEG-4)
    b"\xFF\xF9",                         // AAC ADTS frame sync (MPEG-2)
    b"II*\x00",                          // TIFF little-endian
    b"MM\x00*",                          // TIFF big-endian
    b"glTF\x01\x00\x00\x00",             // Binary glTF 1
    b"glTF\x02\x00\x00\x00",             // Binary glTF 2
    b"\x7FELF",                          // ELF
    b"\xFE\xED\xFA\xCE",                 // Mach-O 32-bit big-endian
    b"\xFE\xED\xFA\xCF",                 // Mach-O 64-bit big-endian
    b"\xCE\xFA\xED\xFE",                 // Mach-O 32-bit little-endian
    b"\xCF\xFA\xED\xFE",                 // Mach-O 64-bit little-endian
    b"\xCA\xFE\xBA\xBE",                 // Mach-O universal binary, Java class
    b"!<arch>\n",                        // ar archive (static library, deb)
    b"\x00asm",                          // WebAssembly
    b"\x1F\x8B\x08",                     // gzip
    b"\x1F\x9D",                         // compress (LZW)
    b"\xFD7zXZ\x00",                     // xz
    b"\x28\xB5\x2F\xFD",                 // Zstandard
    b"\x04\x22\x4D\x18",                 // LZ4 frame
    b"7z\xBC\xAF\x27\x1C",               // 7-Zip
    b"Rar!\x1A\x07",                     // RAR
    b"MSCF\x00\x00\x00\x00",             // Microsoft Cabinet
    b"\xED\xAB\xEE\xDB",                 // RPM package
    b"SQLite format 3\x00",              // SQLite database
    b"\x89HDF\r\n\x1a\n",                // HDF5
    b"\x93NUMPY",                        // NumPy array
    b"\x80\x02",                         // Python pickle protocol 2
    b"\x80\x03",                         // Python pickle protocol 3
    b"\x80\x04",                         // Python pickle protocol 4
    b"\x80\x05",                         // Python pickle protocol 5
    b"wOFF",                             // WOFF font
    b"wOF2",                             // WOFF2 font
    b"OTTO\x00",                         // OpenType font with CFF outlines
    b"\x00\x01\x00\x00\x00",             // TrueType font
    b"\x1A\x45\xDF\xA3",                 // Matroska, WebM
    b"FLV\x01",                          // Flash Video
    b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1", // OLE2 compound file (legacy MS Office)
    b"Kaydara FBX Binary  \x00",         // FBX binary
    b"\x76\x2F\x31\x01",                 // OpenEXR
    b"DDS |\x00\x00\x00",                // DirectDraw Surface
    b"8BPS\x00",                         // Photoshop PSD
    b"\xABKTX ",                         // KTX texture
    b"\xC1\x83\x2A\x9E",                 // Unreal Engine package
    b"\xDE\x12\x04\x95",                 // gettext MO little-endian
    b"\x95\x04\x12\xDE",                 // gettext MO big-endian
    b"\xD4\xC3\xB2\xA1",                 // pcap little-endian
    b"\xA1\xB2\xC3\xD4",                 // pcap big-endian
    b"\xC5\xD0\xD3\xC6",                 // DOS EPS binary
    b"\x00\x00\x00\x0CjP  ",             // JPEG 2000
    b"\x00\x00\x00\x0CJXL \r\n\x87\n",   // JPEG XL container
    b"\xFF\x0A",                         // JPEG XL codestream
    b"\x00\x00\x01\x00",                 // Windows ICO
    b"\x00\x00\x02\x00",                 // Windows CUR
];

fn is_known_binary_header(bytes: &[u8]) -> bool {
    KNOWN_BINARY_HEADERS
        .iter()
        .any(|header| bytes.starts_with(header))
        || is_iso_media_header(bytes)
        || is_bitmap_header(bytes)
        || is_gguf_header(bytes)
        || is_safetensors_header(bytes)
}

fn is_gguf_header(bytes: &[u8]) -> bool {
    bytes.starts_with(b"GGUF") && bytes.get(5..8) == Some(&[0, 0, 0])
}

fn is_iso_media_header(bytes: &[u8]) -> bool {
    bytes.first() == Some(&0) && bytes.get(4..8) == Some(b"ftyp")
}

fn is_bitmap_header(bytes: &[u8]) -> bool {
    bytes.starts_with(b"BM") && bytes.get(6..10) == Some(&[0, 0, 0, 0])
}

fn is_safetensors_header(bytes: &[u8]) -> bool {
    bytes.get(3..8) == Some(&[0, 0, 0, 0, 0]) && bytes.get(8) == Some(&b'{')
}

// Null byte skew alone is not enough to identify UTF-16 -- binary formats with
// small 16-bit values (like PCM audio) produce the same pattern. Decode the
// bytes as UTF-16 and reject if too many code units land in control character
// ranges or form unpaired surrogates, which real text almost never contains.
fn is_plausible_utf16_text(bytes: &[u8], little_endian: bool) -> bool {
    let mut suspicious_count = 0usize;
    let mut word_like_count = 0usize;
    let mut total = 0usize;

    let mut i = 0;
    while let Some(code_unit) = read_u16(bytes, i, little_endian) {
        total += 1;

        match code_unit {
            0x0009 | 0x000A | 0x000C | 0x000D => {}
            0x0020 | 0x0030..=0x0039 | 0x0041..=0x005A | 0x0061..=0x007A => {
                word_like_count += 1;
            }
            // C0/C1 control characters and non-characters
            0x0000..=0x001F | 0x007F..=0x009F | 0xFFFE | 0xFFFF => suspicious_count += 1,
            0xD800..=0xDBFF => {
                let next_offset = i + 2;
                let has_low_surrogate = read_u16(bytes, next_offset, little_endian)
                    .is_some_and(|next| (0xDC00..=0xDFFF).contains(&next));
                if has_low_surrogate {
                    total += 1;
                    word_like_count += 2;
                    i += 2;
                } else {
                    suspicious_count += 1;
                }
            }
            // Lone low surrogate without a preceding high surrogate
            0xDC00..=0xDFFF => suspicious_count += 1,
            0x0100.. => word_like_count += 1,
            _ => {}
        }

        i += 2;
    }

    if total == 0 {
        return false;
    }

    // Real UTF-16 text has near-zero control characters; binary data with
    // small 16-bit values typically exceeds 5%. 2% provides a safe margin.
    let low_control_ratio = suspicious_count * 100 < total * 2;

    // Binary formats that interleave short ASCII fragments with small
    // length/type fields (e.g. game asset formats) can dodge the control
    // character check above while barely containing any real words: their
    // code units land on ASCII punctuation and Latin-1 symbol values rather
    // than letters, digits, or spaces. Real text is overwhelmingly made of
    // word characters, so require a minimum share of them. Code units above
    // the Latin-1 range (and surrogate pairs) also count as word-like so
    // that scripts such as Cyrillic or Greek, whose letters are non-ASCII,
    // are still recognized -- tag bytes paired with a zero byte can never
    // land there.
    let enough_word_chars = word_like_count * 100 >= total * 30;

    low_control_ratio && enough_word_chars
}

fn read_u16(bytes: &[u8], offset: usize, little_endian: bool) -> Option<u16> {
    let pair = [*bytes.get(offset)?, *bytes.get(offset + 1)?];
    if little_endian {
        return Some(u16::from_le_bytes(pair));
    }
    Some(u16::from_be_bytes(pair))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_and_encodes_windows_1251() {
        let expected = "строка один\nстрока два\n";
        let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode(expected);

        let decoded = decode_text(bytes.clone().into_owned()).unwrap();

        assert_eq!(decoded.text, expected);
        assert_eq!(decoded.encoding, encoding_rs::WINDOWS_1251);
        assert!(!decoded.has_bom);
        assert_eq!(
            encode_text(decoded.text, decoded.encoding, decoded.has_bom),
            bytes.as_ref()
        );
    }

    #[test]
    fn preserves_unicode_boms() {
        let expected = "Hello, мир\n";
        for encoding in [UTF_8, UTF_16LE, UTF_16BE] {
            let bytes = encode_text(expected.to_owned(), encoding, true);
            let decoded = decode_text(bytes.clone()).unwrap();

            assert_eq!(decoded.text, expected);
            assert_eq!(decoded.encoding, encoding);
            assert!(decoded.has_bom);
            assert_eq!(
                encode_text(decoded.text, decoded.encoding, decoded.has_bom),
                bytes
            );
        }
    }

    #[test]
    fn detects_known_binary_headers() {
        let cases: &[(&[u8], &str)] = &[
            (b"II*\x00\x08\x00\x00\x00", "TIFF little endian"),
            (b"MM\x00*\x00\x00\x00\x08", "TIFF big endian"),
            (b"glTF\x02\x00\x00\x00\x10\x00\x00\x00", "glTF binary"),
            (b"\x7FELF\x02\x01\x01\x00", "ELF"),
            (b"\xCF\xFA\xED\xFE\x0C\x00\x00\x01", "Mach-O 64-bit"),
            (b"\x1F\x8B\x08\x00", "gzip"),
            (b"\x28\xB5\x2F\xFD\x04", "zstd"),
            (b"SQLite format 3\x00", "SQLite"),
            (b"\x93NUMPY\x01\x00", "NumPy"),
            (b"GGUF\x03\x00\x00\x00", "GGUF"),
            (b"\x00\x00\x00\x18ftypmp42", "MP4"),
            (b"BM\x36\x10\x0E\x00\x00\x00\x00\x00\x36\x00\x00\x00", "BMP"),
            (
                b"\x90\x01\x00\x00\x00\x00\x00\x00{\"__metadata__\"",
                "safetensors",
            ),
        ];
        for (header, label) in cases {
            let mut bytes = header.to_vec();
            bytes.resize(FILE_ANALYSIS_BYTES, b'A');
            assert_eq!(analyze_byte_content(&bytes), ByteContent::Binary, "{label}");
        }
        for text in ["glTF Sample Models\n", "OTTO GmbH\n", "GGUF is a format\n"] {
            assert_eq!(
                analyze_byte_content(text.as_bytes()),
                ByteContent::Unknown,
                "{text:?}"
            );
        }
    }

    #[test]
    fn classifies_nul_free_content() {
        let mut protobuf_like = Vec::new();
        while protobuf_like.len() < FILE_ANALYSIS_BYTES {
            protobuf_like.extend_from_slice(b"\x0a\x0c/conv1/Conv_0\x12\x02\x08\x01\x1a\x04Conv\x22\x06\x08\x03\x10\x05\x18\x07");
        }
        let random_like = pseudo_random_bytes(FILE_ANALYSIS_BYTES)
            .into_iter()
            .map(|byte| if byte == 0 { 1 } else { byte })
            .collect::<Vec<_>>();
        let russian_text = "Съешь же ещё этих мягких французских булок, да выпей чаю.\n".repeat(20);
        let (russian, _, _) = encoding_rs::WINDOWS_1251.encode(&russian_text);
        let chinese_text =
            "这是一个用于测试编码检测的中文句子，其中不包含任何西文字符。\n".repeat(20);
        let (chinese, _, _) = encoding_rs::GBK.encode(&chinese_text);
        let ansi_log =
            "\x1b[32mINFO\x1b[0m \x1b[1m[server]\x1b[0m ok id=\x1b[33m42\x1b[0m\n".repeat(40);
        let control_delimited = "alpha\x01beta\x01gamma\x01delta\x02\n".repeat(60);
        let overstrike =
            "N\x08NA\x08AM\x08ME\x08E\n       ls - list directory contents\n".repeat(30);

        let cases: [(&str, &[u8], ByteContent); 7] = [
            ("protobuf-like", &protobuf_like, ByteContent::Binary),
            ("random without NUL", &random_like, ByteContent::Binary),
            ("windows-1251", &russian, ByteContent::Unknown),
            ("gbk", &chinese, ByteContent::Unknown),
            ("ansi log", ansi_log.as_bytes(), ByteContent::Unknown),
            (
                "control delimited",
                control_delimited.as_bytes(),
                ByteContent::Unknown,
            ),
            (
                "nroff overstrike",
                overstrike.as_bytes(),
                ByteContent::Unknown,
            ),
        ];
        for (label, bytes, expected) in cases {
            assert_eq!(analyze_byte_content(bytes), expected, "{label}");
        }
    }

    #[test]
    fn streaming_decoder_matches_whole_file_decoding() {
        let chinese_text =
            "这是一个用于测试编码检测的中文句子，其中不包含任何西文字符。\n".repeat(20_000);
        let (chinese, _, _) = encoding_rs::GBK.encode(&chinese_text);
        assert!(chinese.len() > ENCODING_DETECTION_BYTES);
        let (russian, _, _) =
            encoding_rs::WINDOWS_1251.encode("Съешь же ещё этих мягких французских булок\n");
        let mut ascii_then_russian = "plain ascii line\n".repeat(5_000).into_bytes();
        ascii_then_russian.extend_from_slice(&russian);
        let mut utf16 = vec![0xFF, 0xFE];
        utf16.extend(
            "utf-16 text\n"
                .repeat(100)
                .encode_utf16()
                .flat_map(u16::to_le_bytes),
        );

        for (label, bytes) in [
            ("gbk beyond the detection window", chinese.into_owned()),
            ("ascii prefix then windows-1251", ascii_then_russian),
            ("utf-16 with bom", utf16),
            ("pure ascii", b"just ascii\n".repeat(3)),
        ] {
            let expected = decode_text(bytes.clone()).unwrap();
            for read_limit in [usize::MAX, 8 * 1024, 7] {
                let mut source = ShortReads {
                    inner: io::Cursor::new(bytes.clone()),
                    limit: read_limit,
                };
                let mut reader = DecodingReader::new(&mut source, None);
                let mut streamed = String::new();
                reader.read_to_string(&mut streamed).unwrap();
                assert_eq!(
                    streamed, expected.text,
                    "{label}, read_limit = {read_limit}"
                );
                assert_eq!(
                    reader.encoding().unwrap_or(UTF_8),
                    expected.encoding,
                    "{label}, read_limit = {read_limit}"
                );
            }
        }
    }

    /// reproduction of issue #50785
    fn build_pcm16_wav_bytes() -> Vec<u8> {
        let header: Vec<u8> = vec![
            /*  RIFF header  */
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0xc6, 0xcf, 0x00, 0x00, // file size: 8
            0x57, 0x41, 0x56, 0x45, // "WAVE"
            /*  fmt chunk  */
            0x66, 0x6d, 0x74, 0x20, // "fmt "
            0x10, 0x00, 0x00, 0x00, // chunk size: 16
            0x01, 0x00, // format: PCM (1)
            0x01, 0x00, // channels: 1 (mono)
            0x80, 0x3e, 0x00, 0x00, // sample rate: 16000
            0x00, 0x7d, 0x00, 0x00, // byte rate: 32000
            0x02, 0x00, // block align: 2
            0x10, 0x00, // bits per sample: 16
            /*  LIST chunk  */
            0x4c, 0x49, 0x53, 0x54, // "LIST"
            0x1a, 0x00, 0x00, 0x00, // chunk size: 26
            0x49, 0x4e, 0x46, 0x4f, // "INFO"
            0x49, 0x53, 0x46, 0x54, // "ISFT"
            0x0d, 0x00, 0x00, 0x00, // sub-chunk size: 13
            0x4c, 0x61, 0x76, 0x66, 0x36, 0x32, 0x2e, 0x33, // "Lavf62.3"
            0x2e, 0x31, 0x30, 0x30, 0x00, // ".100\0"
            /* padding byte for word alignment */
            0x00, // data chunk header
            0x64, 0x61, 0x74, 0x61, // "data"
            0x80, 0xcf, 0x00, 0x00, // chunk size
        ];

        let mut bytes = header;

        // fill remaining space up to `FILE_ANALYSIS_BYTES` with synthetic PCM
        let audio_bytes_needed = FILE_ANALYSIS_BYTES - bytes.len();
        for i in 0..(audio_bytes_needed / 2) {
            let sample = (i & 0xFF) as u8;
            bytes.push(sample); // low byte: varies
            bytes.push(0x00); // high byte: zero for small values
        }

        bytes
    }

    #[test]
    fn test_pcm16_wav_detected_as_binary() {
        let wav_bytes = build_pcm16_wav_bytes();
        assert_eq!(wav_bytes.len(), FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&wav_bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "PCM 16-bit WAV should be detected as Binary via RIFF header"
        );
    }

    #[test]
    fn test_le16_binary_not_misdetected_as_utf16le() {
        let mut bytes = b"FAKE".to_vec();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            let sample = (bytes.len() & 0xFF) as u8;
            bytes.push(sample);
            bytes.push(0x00);
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "LE 16-bit binary with control characters should be detected as Binary"
        );
    }

    #[test]
    fn test_be16_binary_not_misdetected_as_utf16be() {
        let mut bytes = b"FAKE".to_vec();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.push(0x00);
            let sample = (bytes.len() & 0xFF) as u8;
            bytes.push(sample);
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "BE 16-bit binary with control characters should be detected as Binary"
        );
    }

    // Mimics binary formats that interleave short ASCII fragments with small
    // length/type fields (as seen in some game/asset binary formats, e.g.
    // Tibia-style OTBM maps): most high bytes are zero, matching UTF-16LE's
    // null-byte pattern for ASCII, but the low bytes are mostly non-word
    // "tag" values rather than real letters/digits/spaces.
    fn build_tag_interleaved_binary_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        let tags: [u8; 6] = [0xFE, 0xFF, 0x25, 0x2B, 0xA3, 0xC5];
        let mut i = 0;
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.push(tags[i % tags.len()]);
            bytes.push(0x00);
            i += 1;
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);
        bytes
    }

    #[test]
    fn test_tag_interleaved_binary_not_misdetected_as_utf16le() {
        let bytes = build_tag_interleaved_binary_bytes();
        assert_eq!(bytes.len(), FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "binary data with sparse non-word low bytes and null high bytes \
             should not be misdetected as UTF-16LE text"
        );
    }

    #[test]
    fn test_utf16le_text_detected_as_utf16le() {
        let text = "Hello, world! This is a UTF-16 test string. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_le_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Le);
    }

    #[test]
    fn test_utf16be_text_detected_as_utf16be() {
        let text = "Hello, world! This is a UTF-16 test string. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_be_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Be);
    }

    #[test]
    fn test_utf16le_cyrillic_text_detected_as_utf16le() {
        let text = "Привет, мир! Это тестовая строка в UTF-16. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_le_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Le);
    }

    #[test]
    fn test_utf16be_greek_text_detected_as_utf16be() {
        let text = "Γεια σου κόσμε! Αυτή είναι μια δοκιμαστική συμβολοσειρά. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_be_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Be);
    }

    #[test]
    fn test_known_binary_headers() {
        let cases: &[(&[u8], &str)] = &[
            (b"RIFF\x00\x00\x00\x00WAVE", "WAV"),
            (b"RIFF\x00\x00\x00\x00AVI ", "AVI"),
            (b"OggS\x00\x02", "OGG"),
            (b"fLaC\x00\x00", "FLAC"),
            (b"ID3\x03\x00", "MP3 ID3v2"),
            (b"\xFF\xFB\x90\x00", "MP3 MPEG1 Layer3"),
            (b"\xFF\xF3\x90\x00", "MP3 MPEG2 Layer3"),
        ];

        for (header, label) in cases {
            let mut bytes = header.to_vec();
            bytes.resize(FILE_ANALYSIS_BYTES, 0x41); // pad with 'A'
            assert_eq!(
                analyze_byte_content(&bytes),
                ByteContent::Binary,
                "{label} should be detected as Binary"
            );
        }
    }

    struct ShortReads<R> {
        inner: R,
        limit: usize,
    }

    impl<R: Read> Read for ShortReads<R> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let len = buffer.len().min(self.limit);
            self.inner.read(&mut buffer[..len])
        }
    }

    fn pseudo_random_bytes(len: usize) -> Vec<u8> {
        let mut state = 0x2545_F491_4F6C_DD1Du64;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (state >> 56) as u8
            })
            .collect()
    }
}
