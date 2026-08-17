//! The [head](https://docs.microsoft.com/en-us/typography/opentype/spec/head) table

include!("../../generated/generated_head.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, TableProvider};
    use font_test_data::bebuffer::BeBuffer;

    #[test]
    fn smoke_text() {
        let buf = BeBuffer::new()
            .extend([1u16, 0u16])
            .push(Fixed::from_f64(2.8))
            .extend([42u32, 0x5f0f3cf5])
            .extend([16u16, 4096]) // flags, upm
            .extend([LongDateTime::new(-500), LongDateTime::new(101)])
            .extend([-100i16, -50, 400, 711])
            .extend([0u16, 12]) // mac_style / ppem
            .extend([2i16, 1, 0]);

        let head = super::Head::read(buf.data().into()).unwrap();
        assert_eq!(head.version(), MajorMinor::VERSION_1_0);
        assert_eq!(head.font_revision(), Fixed::from_f64(2.8));
        assert_eq!(head.units_per_em(), 4096);
        assert_eq!(head.created().as_secs(), -500);
        assert_eq!(head.y_min(), -50);
        assert_eq!(head.flags(), Flags::INSTRUCTIONS_MAY_ALTER_ADVANCE_WIDTH);
    }

    #[test]
    fn flags() {
        let head = FontRef::new(font_test_data::TINOS_SUBSET)
            .unwrap()
            .head()
            .unwrap();
        assert_eq!(
            head.flags(),
            Flags::BASELINE_AT_Y_0
                | Flags::FORCE_INTEGER_PPEM
                | Flags::INSTRUCTIONS_MAY_ALTER_ADVANCE_WIDTH
        );
    }
}
