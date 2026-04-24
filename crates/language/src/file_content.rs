pub const FILE_ANALYSIS_BYTES: usize = 1024;

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
    let mut total = 0usize;

    let mut i = 0;
    while let Some(code_unit) = read_u16(bytes, i, little_endian) {
        total += 1;

        match code_unit {
            0x0009 | 0x000A | 0x000C | 0x000D => {}
            // C0/C1 control characters and non-characters
            0x0000..=0x001F | 0x007F..=0x009F | 0xFFFE | 0xFFFF => suspicious_count += 1,
            0xD800..=0xDBFF => {
                let next_offset = i + 2;
                let has_low_surrogate = read_u16(bytes, next_offset, little_endian)
                    .is_some_and(|next| (0xDC00..=0xDFFF).contains(&next));
                if has_low_surrogate {
                    total += 1;
                    i += 2;
                } else {
                    suspicious_count += 1;
                }
            }
            // Lone low surrogate without a preceding high surrogate
            0xDC00..=0xDFFF => suspicious_count += 1,
            _ => {}
        }

        i += 2;
    }

    if total == 0 {
        return false;
    }

    // Real UTF-16 text has near-zero control characters; binary data with
    // small 16-bit values typically exceeds 5%. 2% provides a safe margin.
    suspicious_count * 100 < total * 2
}

fn read_u16(bytes: &[u8], offset: usize, little_endian: bool) -> Option<u16> {
    let pair = [*bytes.get(offset)?, *bytes.get(offset + 1)?];
    if little_endian {
        return Some(u16::from_le_bytes(pair));
    }
    Some(u16::from_be_bytes(pair))
}
