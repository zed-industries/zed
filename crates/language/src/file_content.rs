use anyhow::{Result, bail};
use chardetng::EncodingDetector;
use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};

pub const FILE_ANALYSIS_BYTES: usize = 1024;

pub struct DecodedText {
    pub text: String,
    pub encoding: &'static Encoding,
    pub has_bom: bool,
}

pub fn decode_text(bytes: Vec<u8>) -> Result<DecodedText> {
    if let Some((encoding, _bom_len)) = Encoding::for_bom(&bytes) {
        let (text, _) = encoding.decode_with_bom_removal(&bytes);
        return Ok(DecodedText {
            text: text.into_owned(),
            encoding,
            has_bom: true,
        });
    }

    let encoding = match analyze_byte_content(&bytes) {
        ByteContent::Utf16Le => UTF_16LE,
        ByteContent::Utf16Be => UTF_16BE,
        ByteContent::Binary => bail!("Binary files are not supported"),
        ByteContent::Unknown => {
            return match String::from_utf8(bytes) {
                Ok(text) if !text.contains('\x1b') => Ok(DecodedText {
                    text,
                    encoding: UTF_8,
                    has_bom: false,
                }),
                Ok(text) => Ok(decode_with_detected_encoding(text.into_bytes())),
                Err(error) => Ok(decode_with_detected_encoding(error.into_bytes())),
            };
        }
    };

    let (text, _, _) = encoding.decode(&bytes);
    Ok(DecodedText {
        text: text.into_owned(),
        encoding,
        has_bom: false,
    })
}

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

fn decode_with_detected_encoding(bytes: Vec<u8>) -> DecodedText {
    let mut detector = EncodingDetector::new();
    detector.feed(&bytes, true);
    let encoding = detector.guess(None, true);
    let (text, _, _) = encoding.decode(&bytes);
    DecodedText {
        text: text.into_owned(),
        encoding,
        has_bom: false,
    }
}

#[derive(Debug, PartialEq)]
pub enum ByteContent {
    Utf16Le,
    Utf16Be,
    Binary,
    Unknown,
}

// Heuristic check using null byte distribution plus a generic text-likeness
// heuristic. This prefers UTF-16 when many bytes are NUL and otherwise
// distinguishes between text-like and binary-like content.
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
            b'\t' | b'\n' | b'\r' | 0x0C => true,
            0x20..=0x7E => true,
            // Treat bytes that are likely part of UTF-8 or single-byte encodings as text-like.
            0x80..=0xBF | 0xC2..=0xF4 => true,
            _ => false,
        };

        if !is_text_like {
            non_text_like_count += 1;
        }
    }

    let total_null_count = even_null_count + odd_null_count;

    // If there are no NUL bytes at all, this is overwhelmingly likely to be text.
    if total_null_count == 0 {
        return ByteContent::Unknown;
    }

    let has_significant_nulls = total_null_count >= limit / 16;
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

    if non_text_like_count * 100 < limit * 8 {
        ByteContent::Unknown
    } else {
        ByteContent::Binary
    }
}

fn is_known_binary_header(bytes: &[u8]) -> bool {
    bytes.starts_with(b"%PDF-") // PDF
        || bytes.starts_with(b"PK\x03\x04") // ZIP local header
        || bytes.starts_with(b"PK\x05\x06") // ZIP end of central directory
        || bytes.starts_with(b"PK\x07\x08") // ZIP spanning/splitting
        || bytes.starts_with(b"\x89PNG\r\n\x1a\n") // PNG
        || bytes.starts_with(b"\xFF\xD8\xFF") // JPEG
        || bytes.starts_with(b"GIF87a") // GIF87a
        || bytes.starts_with(b"GIF89a") // GIF89a
        || bytes.starts_with(b"IWAD") // Doom IWAD archive
        || bytes.starts_with(b"PWAD") // Doom PWAD archive
        || bytes.starts_with(b"RIFF") // WAV, AVI, WebP
        || bytes.starts_with(b"OggS") // OGG (Vorbis, Opus, FLAC)
        || bytes.starts_with(b"fLaC") // FLAC
        || bytes.starts_with(b"ID3") // MP3 with ID3v2 tag
        || bytes.starts_with(b"\xFF\xFB") // MP3 frame sync (MPEG1 Layer3)
        || bytes.starts_with(b"\xFF\xFA") // MP3 frame sync (MPEG1 Layer3)
        || bytes.starts_with(b"\xFF\xF3") // MP3 frame sync (MPEG2 Layer3)
        || bytes.starts_with(b"\xFF\xF2") // MP3 frame sync (MPEG2 Layer3)
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
}
