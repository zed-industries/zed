//! The [language tag](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ltag.html) table.

include!("../../generated/generated_ltag.rs");

impl<'a> Ltag<'a> {
    /// Returns an iterator yielding the index and string value of each
    /// tag in the table.
    pub fn tag_indices(&self) -> impl Iterator<Item = (u32, &'a str)> {
        let table_data = self.offset_data().as_bytes();
        self.tag_ranges()
            .iter()
            .enumerate()
            .filter_map(move |(index, range)| {
                let start = range.offset() as usize;
                // These are u16 so can't overflow even in 32-bit
                let range = start..start + range.length() as usize;
                let string_bytes = table_data.get(range)?;
                let s = core::str::from_utf8(string_bytes).ok()?;
                Some((index as u32, s))
            })
    }

    /// Returns the index of the given language tag.
    pub fn index_for_tag(&self, tag: &str) -> Option<u32> {
        self.tag_indices().find(|x| x.1 == tag).map(|x| x.0)
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn tags() {
        // Second sample at <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ltag.html>
        let mut buf = BeBuffer::new();
        // header
        buf = buf.extend([1u32, 0, 3]);
        // tag records
        buf = buf.extend([24u16, 2, 26, 2, 28, 2]);
        // string data
        buf = buf.extend("enspsr".as_bytes().iter().copied());
        let expected_tags = [(0, "en"), (1, "sp"), (2, "sr")];
        let ltag = Ltag::read(buf.data().into()).unwrap();
        let tags = ltag.tag_indices().collect::<Vec<_>>();
        assert_eq!(tags, expected_tags);
        assert_eq!(ltag.index_for_tag("en"), Some(0));
        assert_eq!(ltag.index_for_tag("sp"), Some(1));
        assert_eq!(ltag.index_for_tag("sr"), Some(2));
        assert_eq!(ltag.index_for_tag("ar"), None);
        assert_eq!(ltag.index_for_tag("hi"), None);
    }
}
