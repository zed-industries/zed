//! The [gasp](https://learn.microsoft.com/en-us/typography/opentype/spec/gasp) table

include!("../../generated/generated_gasp.rs");

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn smoke_test() {
        let buf = BeBuffer::new()
            .push(1u16) // version
            .push(2u16) // number of records
            .push(404u16) // record 1 ppem
            .push(GaspRangeBehavior::GASP_GRIDFIT | GaspRangeBehavior::GASP_DOGRAY)
            .push(u16::MAX)
            .push(
                GaspRangeBehavior::GASP_SYMMETRIC_GRIDFIT
                    | GaspRangeBehavior::GASP_SYMMETRIC_SMOOTHING,
            );

        let gasp = Gasp::read(buf.data().into()).unwrap();
        assert_eq!(gasp.version(), 1);
        assert_eq!(
            gasp.gasp_ranges()[0],
            GaspRange {
                range_max_ppem: 404.into(),
                range_gasp_behavior: (GaspRangeBehavior::GASP_GRIDFIT
                    | GaspRangeBehavior::GASP_DOGRAY)
                    .into(),
            }
        );
        assert_eq!(
            gasp.gasp_ranges()[1],
            GaspRange {
                range_max_ppem: u16::MAX.into(),
                range_gasp_behavior: (GaspRangeBehavior::GASP_SYMMETRIC_GRIDFIT
                    | GaspRangeBehavior::GASP_SYMMETRIC_SMOOTHING)
                    .into(),
            }
        );
    }
}
