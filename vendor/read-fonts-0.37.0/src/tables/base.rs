//! The [BASE](https://learn.microsoft.com/en-us/typography/opentype/spec/base) table

use super::{layout::DeviceOrVariationIndex, variations::ItemVariationStore};

include!("../../generated/generated_base.rs");

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;
    use font_types::MajorMinor;

    use super::*;

    #[test]
    /// https://learn.microsoft.com/en-us/typography/opentype/spec/base#base-table-examples
    fn example_1() {
        let data = BeBuffer::new()
            .push(MajorMinor::VERSION_1_0)
            .push(8u16) // horizaxis offset
            .push(0x10c_u16) // verticalaxis
            // axis table
            .push(4u16) //basetaglist
            .push(0x12_u16) // basescript list
            // base tag list
            .push(3u16) // count
            .push(Tag::new(b"hang"))
            .push(Tag::new(b"ideo"))
            .push(Tag::new(b"romn"))
            // basescriptlist
            .push(4u16) // basescript count
            .push(Tag::new(b"cyrl"))
            .push(0x1a_u16)
            .push(Tag::new(b"devn"))
            .push(0x60_u16)
            .push(Tag::new(b"hani"))
            .push(0x8a_u16)
            .push(Tag::new(b"latn"))
            .push(0xb4_u16);

        let base = Base::read(data.data().into()).unwrap();
        assert_eq!(base.version(), MajorMinor::VERSION_1_0);
        let horiz = base.horiz_axis().unwrap().unwrap();
        let base_tag = horiz.base_tag_list().unwrap().unwrap();
        assert_eq!(
            base_tag.baseline_tags(),
            &[Tag::new(b"hang"), Tag::new(b"ideo"), Tag::new(b"romn")]
        );
        assert_eq!(base_tag.min_byte_range().end, 14);
        let base_script = horiz.base_script_list().unwrap();
        assert_eq!(
            base_script.base_script_records()[3].base_script_tag(),
            Tag::new(b"latn")
        );
    }
}
