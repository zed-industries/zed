use encoding_rs;
use std::{borrow::Cow, fmt::Debug};

pub use encoding_rs::{
    BIG5, EUC_JP, EUC_KR, GB18030, GBK, IBM866, ISO_2022_JP, ISO_8859_2, ISO_8859_3, ISO_8859_4,
    ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_8_I, ISO_8859_10, ISO_8859_13,
    ISO_8859_14, ISO_8859_15, ISO_8859_16, KOI8_R, KOI8_U, MACINTOSH, SHIFT_JIS, UTF_8, UTF_16BE,
    UTF_16LE, WINDOWS_874, WINDOWS_1250, WINDOWS_1251, WINDOWS_1252, WINDOWS_1253, WINDOWS_1254,
    WINDOWS_1255, WINDOWS_1256, WINDOWS_1257, WINDOWS_1258, X_MAC_CYRILLIC,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encoding {
    pub encoding: &'static encoding_rs::Encoding,
    pub with_bom: bool,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding {
            encoding: UTF_8,
            with_bom: false,
        }
    }
}

impl Encoding {
    pub fn decode(&self, input: Vec<u8>) -> anyhow::Result<String> {
        if self.encoding == UTF_8 && !self.with_bom {
            return Ok(String::from_utf8(input)?);
        }
        let Some(result) = self
            .encoding
            .decode_without_bom_handling_and_without_replacement(&input)
        else {
            return Err(anyhow::anyhow!(
                "input is not valid {}",
                self.encoding.name()
            ));
        };

        if self.with_bom && result.starts_with("\u{FEFF}") {
            Ok(result[3..].to_string())
        } else {
            Ok(result.into_owned())
        }
    }

    pub fn bom(&self) -> Option<&'static [u8]> {
        if !self.with_bom {
            return None;
        }
        if self.encoding == UTF_8 {
            Some(&[0xEF, 0xBB, 0xBF])
        } else if self.encoding == UTF_16BE {
            Some(&[0xFE, 0xFF])
        } else if self.encoding == UTF_16LE {
            Some(&[0xFF, 0xFE])
        } else {
            None
        }
    }

    pub fn encode_chunk<'a>(&self, input: &'a str) -> anyhow::Result<Cow<'a, [u8]>> {
        if self.encoding == UTF_8 {
            Ok(Cow::Borrowed(input.as_bytes()))
        } else if self.encoding == UTF_16BE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16BE bytes
            let utf16be_bytes = input.encode_utf16().flat_map(|u| u.to_be_bytes());

            data.extend(utf16be_bytes);
            Ok(Cow::Owned(data))
        } else if self.encoding == UTF_16LE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16LE bytes
            let utf16le_bytes = input.encode_utf16().flat_map(|u| u.to_le_bytes());

            data.extend(utf16le_bytes);
            Ok(Cow::Owned(data))
        } else {
            // todo: should we error on invalid content when encoding?
            let (cow, _encoding_used, _had_errors) = self.encoding.encode(&input);

            Ok(cow)
        }
    }

    pub fn name(&self) -> &'static str {
        let name = self.encoding.name();

        match name {
            "UTF-8" => "UTF-8",
            "UTF-16LE" => "UTF-16 LE",
            "UTF-16BE" => "UTF-16 BE",
            "windows-1252" => "Windows-1252",
            "windows-1251" => "Windows-1251",
            "windows-1250" => "Windows-1250",
            "ISO-8859-2" => "ISO 8859-2",
            "ISO-8859-3" => "ISO 8859-3",
            "ISO-8859-4" => "ISO 8859-4",
            "ISO-8859-5" => "ISO 8859-5",
            "ISO-8859-6" => "ISO 8859-6",
            "ISO-8859-7" => "ISO 8859-7",
            "ISO-8859-8" => "ISO 8859-8",
            "ISO-8859-13" => "ISO 8859-13",
            "ISO-8859-15" => "ISO 8859-15",
            "KOI8-R" => "KOI8-R",
            "KOI8-U" => "KOI8-U",
            "macintosh" => "MacRoman",
            "x-mac-cyrillic" => "Mac Cyrillic",
            "windows-874" => "Windows-874",
            "windows-1253" => "Windows-1253",
            "windows-1254" => "Windows-1254",
            "windows-1255" => "Windows-1255",
            "windows-1256" => "Windows-1256",
            "windows-1257" => "Windows-1257",
            "windows-1258" => "Windows-1258",
            "EUC-KR" => "Windows-949",
            "EUC-JP" => "EUC-JP",
            "ISO-2022-JP" => "ISO 2022-JP",
            "GBK" => "GBK",
            "gb18030" => "GB18030",
            "Big5" => "Big5",
            _ => name,
        }
    }

    pub fn from_name(name: &str) -> Self {
        let encoding = match name {
            "UTF-8" => encoding_rs::UTF_8,
            "UTF-16 LE" => encoding_rs::UTF_16LE,
            "UTF-16 BE" => encoding_rs::UTF_16BE,
            "Windows-1252" => encoding_rs::WINDOWS_1252,
            "Windows-1251" => encoding_rs::WINDOWS_1251,
            "Windows-1250" => encoding_rs::WINDOWS_1250,
            "ISO 8859-2" => encoding_rs::ISO_8859_2,
            "ISO 8859-3" => encoding_rs::ISO_8859_3,
            "ISO 8859-4" => encoding_rs::ISO_8859_4,
            "ISO 8859-5" => encoding_rs::ISO_8859_5,
            "ISO 8859-6" => encoding_rs::ISO_8859_6,
            "ISO 8859-7" => encoding_rs::ISO_8859_7,
            "ISO 8859-8" => encoding_rs::ISO_8859_8,
            "ISO 8859-13" => encoding_rs::ISO_8859_13,
            "ISO 8859-15" => encoding_rs::ISO_8859_15,
            "KOI8-R" => encoding_rs::KOI8_R,
            "KOI8-U" => encoding_rs::KOI8_U,
            "MacRoman" => encoding_rs::MACINTOSH,
            "Mac Cyrillic" => encoding_rs::X_MAC_CYRILLIC,
            "Windows-874" => encoding_rs::WINDOWS_874,
            "Windows-1253" => encoding_rs::WINDOWS_1253,
            "Windows-1254" => encoding_rs::WINDOWS_1254,
            "Windows-1255" => encoding_rs::WINDOWS_1255,
            "Windows-1256" => encoding_rs::WINDOWS_1256,
            "Windows-1257" => encoding_rs::WINDOWS_1257,
            "Windows-1258" => encoding_rs::WINDOWS_1258,
            "Windows-949" => encoding_rs::EUC_KR,
            "EUC-JP" => encoding_rs::EUC_JP,
            "ISO 2022-JP" => encoding_rs::ISO_2022_JP,
            "GBK" => encoding_rs::GBK,
            "GB18030" => encoding_rs::GB18030,
            "Big5" => encoding_rs::BIG5,
            _ => encoding_rs::UTF_8, // Default to UTF-8 for unknown names
        };

        Encoding {
            encoding,
            with_bom: false,
        }
    }
}

#[derive(Default, Clone)]
pub struct EncodingOptions {
    pub expected: Encoding,
    pub auto_detect: bool,
}

impl EncodingOptions {
    pub fn process(&self, bytes: Vec<u8>) -> anyhow::Result<(Encoding, String)> {
        let encoding = if self.auto_detect
            && let Some(encoding) = Self::detect(&bytes)
        {
            encoding
        } else {
            self.expected
        };

        Ok((encoding, encoding.decode(bytes)?))
    }

    fn detect(bytes: &[u8]) -> Option<Encoding> {
        if bytes.starts_with(&[0xFE, 0xFF]) {
            Some(Encoding {
                encoding: UTF_16BE,
                with_bom: true,
            })
        } else if bytes.starts_with(&[0xFF, 0xFE]) {
            Some(Encoding {
                encoding: UTF_16LE,
                with_bom: true,
            })
        } else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            Some(Encoding {
                encoding: UTF_8,
                with_bom: true,
            })
        } else {
            None
        }
    }
}
