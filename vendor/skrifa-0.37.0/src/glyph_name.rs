//! Support for accessing glyph names.

use core::ops::Range;
use raw::{
    tables::{
        cff::Cff,
        post::Post,
        postscript::{Charset, CharsetIter, StringId as Sid},
    },
    types::GlyphId,
    FontRef, TableProvider,
};

/// "Names must be no longer than 63 characters; some older implementations
/// can assume a length limit of 31 characters."
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/post#version-20>
const MAX_GLYPH_NAME_LEN: usize = 63;

/// Mapping from glyph identifiers to names.
///
/// This sources glyph names from the `post` and `CFF` tables in that order.
/// If glyph names are not available in either, then they are synthesized
/// as `gidDDD` where `DDD` is the glyph identifier in decimal. Use the
/// [`source`](Self::source) to determine which source was chosen.
#[derive(Clone)]
pub struct GlyphNames<'a> {
    inner: Inner<'a>,
}

#[derive(Clone)]
enum Inner<'a> {
    // Second field is num_glyphs
    Post(Post<'a>, u32),
    Cff(Cff<'a>, Charset<'a>),
    Synthesized(u32),
}

impl<'a> GlyphNames<'a> {
    /// Creates a new object for accessing glyph names from the given font.
    pub fn new(font: &FontRef<'a>) -> Self {
        let num_glyphs = font
            .maxp()
            .map(|maxp| maxp.num_glyphs() as u32)
            .unwrap_or_default();
        if let Ok(post) = font.post() {
            if post.num_names() != 0 {
                return Self {
                    inner: Inner::Post(post, num_glyphs),
                };
            }
        }
        if let Some((cff, charset)) = font
            .cff()
            .ok()
            .and_then(|cff| Some((cff.clone(), cff.charset(0).ok()??)))
        {
            return Self {
                inner: Inner::Cff(cff, charset),
            };
        }
        Self {
            inner: Inner::Synthesized(num_glyphs),
        }
    }

    /// Returns the chosen source for glyph names.
    pub fn source(&self) -> GlyphNameSource {
        match &self.inner {
            Inner::Post(..) => GlyphNameSource::Post,
            Inner::Cff(..) => GlyphNameSource::Cff,
            Inner::Synthesized(..) => GlyphNameSource::Synthesized,
        }
    }

    /// Returns the number of glyphs in the font.
    pub fn num_glyphs(&self) -> u32 {
        match &self.inner {
            Inner::Post(_, n) | Inner::Synthesized(n) => *n,
            Inner::Cff(_, charset) => charset.num_glyphs(),
        }
    }

    /// Returns the name for the given glyph identifier.
    pub fn get(&self, glyph_id: GlyphId) -> Option<GlyphName> {
        if glyph_id.to_u32() >= self.num_glyphs() {
            return None;
        }
        let name = match &self.inner {
            Inner::Post(post, _) => GlyphName::from_post(post, glyph_id),
            Inner::Cff(cff, charset) => charset
                .string_id(glyph_id)
                .ok()
                .and_then(|sid| GlyphName::from_cff_sid(cff, sid)),
            _ => None,
        };
        // If name is empty string, synthesize it
        if !name.as_ref().is_some_and(|s| !s.is_empty()) {
            return Some(GlyphName::synthesize(glyph_id));
        }
        Some(name.unwrap_or_else(|| GlyphName::synthesize(glyph_id)))
    }

    /// Returns an iterator yielding the identifier and name for all glyphs in
    /// the font.
    pub fn iter(&self) -> impl Iterator<Item = (GlyphId, GlyphName)> + 'a + Clone {
        match &self.inner {
            Inner::Post(post, n) => Iter::Post(0..*n, post.clone()),
            Inner::Cff(cff, charset) => Iter::Cff(cff.clone(), charset.iter()),
            Inner::Synthesized(n) => Iter::Synthesized(0..*n),
        }
    }
}

/// Specifies the chosen source for glyph names.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GlyphNameSource {
    /// Glyph names are sourced from the `post` table.
    Post,
    /// Glyph names are sourced from the `CFF` table.
    Cff,
    /// Glyph names are synthesized in the format `gidDDD` where `DDD` is
    /// the glyph identifier in decimal.
    Synthesized,
}

/// The name of a glyph.
#[derive(Clone)]
pub struct GlyphName {
    name: [u8; MAX_GLYPH_NAME_LEN],
    len: u8,
    is_synthesized: bool,
}

impl GlyphName {
    /// Returns the underlying name as a string.
    pub fn as_str(&self) -> &str {
        let bytes = &self.name[..self.len as usize];
        core::str::from_utf8(bytes).unwrap_or_default()
    }

    /// Returns true if the glyph name was synthesized, i.e. not found in any
    /// source.
    pub fn is_synthesized(&self) -> bool {
        self.is_synthesized
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut name = Self::default();
        name.append(bytes);
        name
    }

    fn from_post(post: &Post, glyph_id: GlyphId) -> Option<Self> {
        glyph_id
            .try_into()
            .ok()
            .and_then(|id| post.glyph_name(id))
            .map(|s| s.as_bytes())
            .map(Self::from_bytes)
    }

    fn from_cff_sid(cff: &Cff, sid: Sid) -> Option<Self> {
        cff.string(sid)
            .and_then(|s| core::str::from_utf8(s.bytes()).ok())
            .map(|s| s.as_bytes())
            .map(Self::from_bytes)
    }

    fn synthesize(glyph_id: GlyphId) -> Self {
        use core::fmt::Write;
        let mut name = Self {
            is_synthesized: true,
            ..Self::default()
        };
        let _ = write!(GlyphNameWrite(&mut name), "gid{}", glyph_id.to_u32());
        name
    }

    /// Appends the given bytes to `self` while keeping the maximum length
    /// at 63 bytes.
    ///
    /// This exists primarily to support the [`core::fmt::Write`] impl
    /// (which is used for generating synthesized glyph names) because
    /// we have no guarantee of how many times `write_str` might be called
    /// for a given format.
    fn append(&mut self, bytes: &[u8]) {
        // We simply truncate when length exceeds the max since glyph names
        // are expected to be <= 63 chars
        let start = self.len as usize;
        let available = MAX_GLYPH_NAME_LEN - start;
        let copy_len = available.min(bytes.len());
        self.name[start..start + copy_len].copy_from_slice(&bytes[..copy_len]);
        self.len = (start + copy_len) as u8;
    }
}

impl Default for GlyphName {
    fn default() -> Self {
        Self {
            name: [0; MAX_GLYPH_NAME_LEN],
            len: 0,
            is_synthesized: false,
        }
    }
}

impl core::fmt::Debug for GlyphName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GlyphName")
            .field("name", &self.as_str())
            .field("is_synthesized", &self.is_synthesized)
            .finish()
    }
}

impl core::fmt::Display for GlyphName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::ops::Deref for GlyphName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<&str> for GlyphName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

struct GlyphNameWrite<'a>(&'a mut GlyphName);

impl core::fmt::Write for GlyphNameWrite<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.append(s.as_bytes());
        Ok(())
    }
}

#[derive(Clone)]
enum Iter<'a> {
    Post(Range<u32>, Post<'a>),
    Cff(Cff<'a>, CharsetIter<'a>),
    Synthesized(Range<u32>),
}

impl Iter<'_> {
    fn next_name(&mut self) -> Option<Result<(GlyphId, GlyphName), GlyphId>> {
        match self {
            Self::Post(range, post) => {
                let gid = GlyphId::new(range.next()?);
                Some(
                    GlyphName::from_post(post, gid)
                        .map(|name| (gid, name))
                        .ok_or(gid),
                )
            }
            Self::Cff(cff, iter) => {
                let (gid, sid) = iter.next()?;
                Some(
                    GlyphName::from_cff_sid(cff, sid)
                        .map(|name| (gid, name))
                        .ok_or(gid),
                )
            }
            Self::Synthesized(range) => {
                let gid = GlyphId::new(range.next()?);
                Some(Ok((gid, GlyphName::synthesize(gid))))
            }
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (GlyphId, GlyphName);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_name()? {
            Ok((gid, name)) if name.is_empty() => Some((gid, GlyphName::synthesize(gid))),
            Ok(gid_name) => Some(gid_name),
            Err(gid) => Some((gid, GlyphName::synthesize(gid))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raw::{FontData, FontRead};

    #[test]
    fn synthesized_glyph_names() {
        let count = 58;
        let names = GlyphNames {
            inner: Inner::Synthesized(58),
        };
        let names_buf = (0..count).map(|i| format!("gid{i}")).collect::<Vec<_>>();
        let expected_names = names_buf.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        for (_, name) in names.iter() {
            assert!(name.is_synthesized())
        }
        check_names(&names, &expected_names, GlyphNameSource::Synthesized);
    }

    #[test]
    fn synthesize_for_empty_names() {
        let mut post_data = font_test_data::post::SIMPLE.to_vec();
        // last name in this post data is "hola" so pop 5 bytes and then
        // push a 0 to simulate an empty name
        post_data.truncate(post_data.len() - 5);
        post_data.push(0);
        let post = Post::read(FontData::new(&post_data)).unwrap();
        let gid = GlyphId::new(9);
        assert!(post.glyph_name(gid.try_into().unwrap()).unwrap().is_empty());
        let names = GlyphNames {
            inner: Inner::Post(post, 10),
        };
        assert_eq!(names.get(gid).unwrap(), "gid9");
        assert_eq!(names.iter().last().unwrap().1, "gid9");
    }

    #[test]
    fn cff_glyph_names() {
        let font = FontRef::new(font_test_data::NOTO_SERIF_DISPLAY_TRIMMED).unwrap();
        let names = GlyphNames::new(&font);
        assert_eq!(names.source(), GlyphNameSource::Cff);
        let expected_names = [".notdef", "i", "j", "k", "l"];
        check_names(&names, &expected_names, GlyphNameSource::Cff);
    }

    #[test]
    fn post_glyph_names() {
        let font = FontRef::new(font_test_data::HVAR_WITH_TRUNCATED_ADVANCE_INDEX_MAP).unwrap();
        let names = GlyphNames::new(&font);
        let expected_names = [
            ".notdef",
            "space",
            "A",
            "I",
            "T",
            "Aacute",
            "Agrave",
            "Iacute",
            "Igrave",
            "Amacron",
            "Imacron",
            "acutecomb",
            "gravecomb",
            "macroncomb",
            "A.001",
            "A.002",
            "A.003",
            "A.004",
            "A.005",
            "A.006",
            "A.007",
            "A.008",
            "A.009",
            "A.010",
        ];
        check_names(&names, &expected_names, GlyphNameSource::Post);
    }

    #[test]
    fn post_glyph_names_partial() {
        let font = FontRef::new(font_test_data::HVAR_WITH_TRUNCATED_ADVANCE_INDEX_MAP).unwrap();
        let mut names = GlyphNames::new(&font);
        let Inner::Post(_, len) = &mut names.inner else {
            panic!("it's a post table!");
        };
        // Increase count by 4 so we synthesize the remaining names
        *len += 4;
        let expected_names = [
            ".notdef",
            "space",
            "A",
            "I",
            "T",
            "Aacute",
            "Agrave",
            "Iacute",
            "Igrave",
            "Amacron",
            "Imacron",
            "acutecomb",
            "gravecomb",
            "macroncomb",
            "A.001",
            "A.002",
            "A.003",
            "A.004",
            "A.005",
            "A.006",
            "A.007",
            "A.008",
            "A.009",
            "A.010",
            // synthesized names...
            "gid24",
            "gid25",
            "gid26",
            "gid27",
        ];
        check_names(&names, &expected_names, GlyphNameSource::Post);
    }

    fn check_names(names: &GlyphNames, expected_names: &[&str], expected_source: GlyphNameSource) {
        assert_eq!(names.source(), expected_source);
        let iter_names = names.iter().collect::<Vec<_>>();
        assert_eq!(iter_names.len(), expected_names.len());
        for (i, expected) in expected_names.iter().enumerate() {
            let gid = GlyphId::new(i as u32);
            let name = names.get(gid).unwrap();
            assert_eq!(name, expected);
            assert_eq!(iter_names[i].0, gid);
            assert_eq!(iter_names[i].1, expected);
        }
    }
}
