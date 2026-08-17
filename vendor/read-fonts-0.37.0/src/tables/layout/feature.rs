//! Additional support for working with OpenType features.

use super::{Feature, FeatureList, ReadError, TaggedElement};

impl<'a> FeatureList<'a> {
    /// Returns the tag and feature at the given index.
    pub fn get(&self, index: u16) -> Result<TaggedElement<Feature<'a>>, ReadError> {
        self.feature_records()
            .get(index as usize)
            .ok_or(ReadError::OutOfBounds)
            .and_then(|rec| {
                Ok(TaggedElement::new(
                    rec.feature_tag(),
                    rec.feature(self.offset_data())?,
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{FontRef, TableProvider, Tag};

    #[test]
    fn feature_list_get() {
        let font = FontRef::new(font_test_data::NOTOSERIF_AUTOHINT_SHAPING).unwrap();
        let gsub = font.gsub().unwrap();
        let feature_list = gsub.feature_list().unwrap();
        assert_eq!(feature_list.get(0).unwrap().tag, Tag::new(b"c2sc"));
        assert_eq!(feature_list.get(1).unwrap().tag, Tag::new(b"liga"));
        assert!(feature_list.get(2).is_err());
    }
}
