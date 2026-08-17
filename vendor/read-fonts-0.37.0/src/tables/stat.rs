//! The [STAT](https://learn.microsoft.com/en-us/typography/opentype/spec/stat) table

include!("../../generated/generated_stat.rs");

impl AxisValue<'_> {
    /// Returns axis value for format 1, 2 and 3 axis value tables
    ///
    /// For axis value format 2, returns the nominal value.
    pub fn value(&self) -> Option<Fixed> {
        match self {
            Self::Format1(item) => Some(item.value()),
            Self::Format2(item) => Some(item.nominal_value()),
            Self::Format3(item) => Some(item.value()),
            Self::Format4(_) => None,
        }
    }

    /// Returns linked value for format 3 axis value tables
    pub fn linked_value(&self) -> Option<Fixed> {
        match self {
            Self::Format3(item) => Some(item.linked_value()),
            _ => None,
        }
    }

    /// Returns axis index for format 1, 2 and 3 axis value tables
    pub fn axis_index(&self) -> Option<u16> {
        match self {
            Self::Format1(item) => Some(item.axis_index()),
            Self::Format2(item) => Some(item.axis_index()),
            Self::Format3(item) => Some(item.axis_index()),
            Self::Format4(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use types::{Fixed, NameId};

    use crate::{table_provider::TableProvider, FontRef};

    use super::*;

    #[test]
    fn smoke_test() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let table = font.stat().unwrap();
        assert_eq!(table.design_axis_count(), 1);
        let axis_record = &table.design_axes().unwrap()[0];
        assert_eq!(axis_record.axis_tag(), Tag::new(b"wght"));
        assert_eq!(axis_record.axis_name_id(), NameId::new(257));
        assert_eq!(axis_record.axis_ordering(), 0);
        let axis_values = table.offset_to_axis_values().unwrap().unwrap();
        let axis_values = axis_values
            .axis_values()
            .iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        assert_eq!(axis_values.len(), 3);
        let last = &axis_values[2];
        if let AxisValue::Format1(table) = last {
            assert_eq!(table.axis_index(), 0);
            assert_eq!(table.value_name_id(), NameId::new(264));
            assert_eq!(table.value(), Fixed::from_f64(700.0));
        }
    }
}
