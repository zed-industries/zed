use std::ops::Range;

use anyhow::{Context as _, Result, ensure};

use super::{
    data_inspector::{Endian, NumberType},
    find_bytes, parse_hex,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SearchMode {
    Hex,
    Ascii,
    Utf8,
    Number,
}

impl SearchMode {
    pub const ALL: [Self; 4] = [Self::Hex, Self::Ascii, Self::Utf8, Self::Number];

    pub fn label(self) -> &'static str {
        match self {
            Self::Hex => "Hex / Wildcards",
            Self::Ascii => "ASCII",
            Self::Utf8 => "UTF-8",
            Self::Number => "Number",
        }
    }

    pub fn replacement(self, text: &str, kind: NumberType, endian: Endian) -> Result<Vec<u8>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }
        match self {
            Self::Hex => parse_hex(text),
            Self::Ascii => {
                ensure!(text.is_ascii(), "ASCII search accepts only ASCII text");
                Ok(text.as_bytes().to_vec())
            }
            Self::Utf8 => Ok(text.as_bytes().to_vec()),
            Self::Number => kind.encode(text, endian),
        }
    }

    pub fn pattern(self, text: &str, kind: NumberType, endian: Endian) -> Result<SearchPattern> {
        if self == Self::Hex {
            SearchPattern::hex(text)
        } else {
            SearchPattern::exact(self.replacement(text, kind, endian)?)
        }
    }
}

pub(super) struct SearchPattern {
    length: usize,
    exact: Option<Vec<u8>>,
    regex: regex::bytes::Regex,
    reversed_regex: Option<regex::bytes::Regex>,
}

impl SearchPattern {
    fn exact(bytes: Vec<u8>) -> Result<Self> {
        ensure!(!bytes.is_empty(), "Enter a search pattern");
        ensure!(
            bytes.len() <= 4096,
            "Search patterns support up to 4096 bytes"
        );
        let pattern = bytes
            .iter()
            .map(|byte| format!("\\x{byte:02X}"))
            .collect::<String>();
        Self::compile(bytes.len(), Some(bytes), &pattern, None)
    }

    fn hex(text: &str) -> Result<Self> {
        let digits: Vec<_> = text
            .chars()
            .filter(|character| !character.is_ascii_whitespace())
            .collect();
        ensure!(!digits.is_empty(), "Enter a search pattern");
        ensure!(
            digits.len() <= 8192,
            "Search patterns support up to 4096 bytes"
        );
        ensure!(
            digits.len() % 2 == 0,
            "Each byte needs two hexadecimal digits or ?"
        );
        if !digits.contains(&'?') {
            return Self::exact(parse_hex(text)?);
        }
        let mut tokens = Vec::new();
        for pair in digits.chunks_exact(2) {
            ensure!(
                pair.iter()
                    .all(|digit| *digit == '?' || digit.is_ascii_hexdigit()),
                "Use hex digits or ? wildcards"
            );
            if pair == ['?', '?'] {
                tokens.push("(?s:.)".to_string());
                continue;
            }
            let mut pattern = String::new();
            pattern.push('[');
            for byte in 0..=255u8 {
                let high = pair[0] == '?' || pair[0].to_digit(16) == Some((byte >> 4) as u32);
                let low = pair[1] == '?' || pair[1].to_digit(16) == Some((byte & 15) as u32);
                if high && low {
                    pattern.push_str(&format!("\\x{byte:02X}"));
                }
            }
            pattern.push(']');
            tokens.push(pattern);
        }
        let pattern = tokens.concat();
        let reversed = tokens.into_iter().rev().collect::<String>();
        Self::compile(digits.len() / 2, None, &pattern, Some(&reversed))
    }

    fn compile(
        length: usize,
        exact: Option<Vec<u8>>,
        pattern: &str,
        reversed: Option<&str>,
    ) -> Result<Self> {
        let compile = |pattern| {
            regex::bytes::RegexBuilder::new(pattern)
                .unicode(false)
                .build()
        };
        let regex = compile(pattern)?;
        let reversed_regex = reversed.map(compile).transpose()?;
        Ok(Self {
            length,
            exact,
            regex,
            reversed_regex,
        })
    }

    pub fn find(&self, bytes: &[u8], start: usize, backwards: bool) -> Result<Option<usize>> {
        if let Some(exact) = &self.exact {
            return find_bytes(bytes, exact, start, backwards);
        }
        if self.length > bytes.len() {
            return Ok(None);
        }
        let start = start.min(bytes.len() - self.length + 1);
        if !backwards {
            return Ok(self
                .regex
                .find_at(bytes, start)
                .or_else(|| self.regex.find(bytes))
                .map(|found| found.start()));
        }
        let regex = self
            .reversed_regex
            .as_ref()
            .context("Invalid wildcard search")?;
        let reversed: Vec<_> = bytes.iter().rev().copied().collect();
        let offset = bytes.len() - (start + self.length - 1);
        Ok(regex
            .find_at(&reversed, offset)
            .or_else(|| regex.find(&reversed))
            .map(|found| bytes.len() - found.end()))
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn matches(&self, bytes: &[u8]) -> bool {
        bytes.len() == self.length && self.regex.is_match(bytes)
    }

    pub fn matches_in<'a>(&'a self, bytes: &'a [u8]) -> impl Iterator<Item = Range<usize>> + 'a {
        self.regex.find_iter(bytes).map(|found| found.range())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_number_and_wildcard_search() -> Result<()> {
        let make =
            |mode: SearchMode, text: &str| mode.pattern(text, NumberType::U16, Endian::Little);
        assert_eq!(
            make(SearchMode::Ascii, "ab")?.find(b"abab", 1, false)?,
            Some(2)
        );
        assert!(make(SearchMode::Ascii, "é").is_err());
        assert_eq!(
            make(SearchMode::Utf8, "é")?.find("café".as_bytes(), 0, false)?,
            Some(3)
        );
        assert_eq!(
            make(SearchMode::Number, "4660")?.find(&[0x34, 0x12], 0, false)?,
            Some(0)
        );
        let pattern = make(SearchMode::Hex, "A? ?F ??")?;
        assert!(pattern.matches(&[0xab, 0x1f, 0]));
        assert!(!pattern.matches(&[0xbb, 0x1f, 0]));
        assert_eq!(
            pattern.find(&[0xab, 0x1f, 0, 0xac, 0x2f, 255], 1, false)?,
            Some(3)
        );
        let overlap = make(SearchMode::Hex, "?? ??")?;
        assert_eq!(overlap.find(&[1, 2, 3], 0, true)?, Some(1));
        assert_eq!(overlap.find(&[1, 2, 3], 1, true)?, Some(0));
        assert_eq!(
            overlap.matches_in(&[1, 2, 3, 4]).collect::<Vec<_>>(),
            [0..2, 2..4]
        );
        assert!(make(SearchMode::Hex, "?").is_err());
        assert!(make(SearchMode::Hex, "GG").is_err());
        assert!(make(SearchMode::Utf8, "").is_err());
        assert!(make(SearchMode::Utf8, &"x".repeat(4097)).is_err());
        assert_eq!(make(SearchMode::Hex, &"A?".repeat(4096))?.length(), 4096);
        assert!(make(SearchMode::Hex, &"??".repeat(4097)).is_err());
        assert!(
            SearchMode::Hex
                .replacement("??", NumberType::U8, Endian::Little)
                .is_err()
        );
        assert_eq!(
            SearchMode::Number.replacement("4660", NumberType::U16, Endian::Big)?,
            [0x12, 0x34]
        );
        assert!(
            SearchMode::Number
                .replacement("65536", NumberType::U16, Endian::Big)
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn test_wildcard_wrapping_and_overlaps_match_reference() -> Result<()> {
        let bytes = [0xaf, 0xaf, 0x1f, 0xff, 0xaa, 0x1f, 0];
        for text in ["??", "?? ??", "A?", "?F", "A? ?F", "?? ?? ?? ??"] {
            let pattern = SearchMode::Hex.pattern(text, NumberType::U8, Endian::Little)?;
            let matches: Vec<_> = (0..=bytes.len() - pattern.length())
                .filter(|offset| pattern.matches(&bytes[*offset..*offset + pattern.length()]))
                .collect();
            for start in 0..=bytes.len() {
                assert_eq!(
                    pattern.find(&bytes, start, true)?,
                    matches
                        .iter()
                        .rev()
                        .find(|offset| **offset < start)
                        .or_else(|| matches.last())
                        .copied(),
                    "{text}, previous from {start}"
                );
                assert_eq!(
                    pattern.find(&bytes, start, false)?,
                    matches
                        .iter()
                        .find(|offset| **offset >= start)
                        .or_else(|| matches.first())
                        .copied(),
                    "{text}, next from {start}"
                );
            }
        }
        let pattern = SearchMode::Hex.pattern("?? ??", NumberType::U8, Endian::Little)?;
        assert_eq!(pattern.find(&vec![0xff; 65_536], 0, true)?, Some(65_534));
        Ok(())
    }
}
