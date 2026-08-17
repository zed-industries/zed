//! the [hhea (Horizontal Header)](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea) table

include!("../../generated/generated_hhea.rs");

impl Hhea<'_> {
    #[deprecated(since = "0.26.0", note = "use number_of_h_metrics instead")]
    pub fn number_of_long_metrics(&self) -> u16 {
        self.number_of_h_metrics()
    }
}
