use super::{tag_from_bytes, tag_from_str_lossy, Tag};
use core::fmt;

/// Setting combining a tag and a value for features and variations.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Setting<T> {
    /// The tag that identifies the setting.
    pub tag: Tag,
    /// The value for the setting.
    pub value: T,
}

impl<T: fmt::Display> fmt::Display for Setting<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.tag.to_be_bytes();
        let tag_name = core::str::from_utf8(&bytes).unwrap_or("");
        write!(f, "\"{}\" {}", tag_name, self.value)
    }
}

impl Setting<u16> {
    /// Parses a feature setting according to the CSS grammar.
    pub fn parse(s: &str) -> Option<Self> {
        Self::parse_list(s).next()
    }

    /// Parses a comma separated list of feature settings according to the CSS
    /// grammar.
    pub fn parse_list(s: &str) -> impl Iterator<Item = Self> + '_ + Clone {
        ParseList::new(s)
            .map(|(_, tag, value_str)| {
                let (ok, value) = match value_str {
                    "on" | "" => (true, 1),
                    "off" => (true, 0),
                    _ => match value_str.parse::<u16>() {
                        Ok(value) => (true, value),
                        _ => (false, 0),
                    },
                };
                (ok, tag, value)
            })
            .take_while(|(ok, _, _)| *ok)
            .map(|(_, tag, value)| Self { tag, value })
    }
}

impl Setting<f32> {
    /// Parses a variation setting according to the CSS grammar.    
    pub fn parse(s: &str) -> Option<Self> {
        Self::parse_list(s).next()
    }

    /// Parses a comma separated list of variation settings according to the
    /// CSS grammar.    
    pub fn parse_list(s: &str) -> impl Iterator<Item = Self> + '_ + Clone {
        ParseList::new(s)
            .map(|(_, tag, value_str)| {
                let (ok, value) = match value_str.parse::<f32>() {
                    Ok(value) => (true, value),
                    _ => (false, 0.),
                };
                (ok, tag, value)
            })
            .take_while(|(ok, _, _)| *ok)
            .map(|(_, tag, value)| Self { tag, value })
    }
}

impl<T> From<(Tag, T)> for Setting<T> {
    fn from(v: (Tag, T)) -> Self {
        Self {
            tag: v.0,
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(Tag, T)> for Setting<T> {
    fn from(v: &(Tag, T)) -> Self {
        Self {
            tag: v.0,
            value: v.1,
        }
    }
}

impl<T: Copy> From<&([u8; 4], T)> for Setting<T> {
    fn from(v: &([u8; 4], T)) -> Self {
        Self {
            tag: tag_from_bytes(&v.0),
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(&[u8; 4], T)> for Setting<T> {
    fn from(v: &(&[u8; 4], T)) -> Self {
        Self {
            tag: tag_from_bytes(v.0),
            value: v.1,
        }
    }
}

impl<T> From<(&str, T)> for Setting<T> {
    fn from(v: (&str, T)) -> Self {
        Self {
            tag: tag_from_str_lossy(v.0),
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(&str, T)> for Setting<T> {
    fn from(v: &(&str, T)) -> Self {
        Self {
            tag: tag_from_str_lossy(v.0),
            value: v.1,
        }
    }
}

#[derive(Clone)]
struct ParseList<'a> {
    source: &'a [u8],
    len: usize,
    pos: usize,
}

impl<'a> ParseList<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            len: source.len(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for ParseList<'a> {
    type Item = (usize, Tag, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = self.pos;
        while pos < self.len && {
            let ch = self.source[pos];
            ch.is_ascii_whitespace() || ch == b','
        } {
            pos += 1;
        }
        self.pos = pos;
        if pos >= self.len {
            return None;
        }
        let first = self.source[pos];
        let mut start = pos;
        let quote = match first {
            b'"' | b'\'' => {
                pos += 1;
                start += 1;
                first
            }
            _ => return None,
        };
        let mut tag_str = None;
        while pos < self.len {
            if self.source[pos] == quote {
                tag_str = core::str::from_utf8(self.source.get(start..pos)?).ok();
                pos += 1;
                break;
            }
            pos += 1;
        }
        self.pos = pos;
        let tag_str = tag_str?;
        if tag_str.len() != 4 || !tag_str.is_ascii() {
            return None;
        }
        let tag = tag_from_str_lossy(tag_str);
        while pos < self.len {
            if !self.source[pos].is_ascii_whitespace() {
                break;
            }
            pos += 1;
        }
        self.pos = pos;
        start = pos;
        let mut end = start;
        while pos < self.len {
            if self.source[pos] == b',' {
                pos += 1;
                break;
            }
            pos += 1;
            end += 1;
        }
        let value = core::str::from_utf8(self.source.get(start..end)?)
            .ok()?
            .trim();
        self.pos = pos;
        Some((pos, tag, value))
    }
}
