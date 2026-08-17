use std::borrow::Cow;

use crate::{encoding_size, Encode, Section, SectionId};

/// A custom section holding arbitrary data.
#[derive(Clone, Debug)]
pub struct CustomSection<'a> {
    /// The name of this custom section.
    pub name: Cow<'a, str>,
    /// This custom section's data.
    pub data: Cow<'a, [u8]>,
}

impl Encode for CustomSection<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        let encoded_name_len = encoding_size(u32::try_from(self.name.len()).unwrap());
        (encoded_name_len + self.name.len() + self.data.len()).encode(sink);
        self.name.encode(sink);
        sink.extend(&*self.data);
    }
}

impl Section for CustomSection<'_> {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

/// A raw custom section where the bytes specified contain the leb-encoded
/// length of the custom section, the custom section's name, and the custom
/// section's data.
#[derive(Clone, Debug)]
pub struct RawCustomSection<'a>(pub &'a [u8]);

impl Encode for RawCustomSection<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.extend(self.0);
    }
}

impl Section for RawCustomSection<'_> {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_section() {
        let custom = CustomSection {
            name: "test".into(),
            data: Cow::Borrowed(&[11, 22, 33, 44]),
        };

        let mut encoded = vec![];
        custom.encode(&mut encoded);

        #[rustfmt::skip]
        assert_eq!(encoded, vec![
            // LEB128 length of section.
            9,
            // LEB128 length of name.
            4,
            // Name.
            b't', b'e', b's', b't',
            // Data.
            11, 22, 33, 44,
        ]);
    }
}
