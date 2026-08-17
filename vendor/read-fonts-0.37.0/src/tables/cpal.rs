//! The [CPAL](https://docs.microsoft.com/en-us/typography/opentype/spec/cpal) table

include!("../../generated/generated_cpal.rs");

#[cfg(test)]
mod tests {

    use crate::{FontRef, TableProvider};

    #[test]
    fn read_sample() {
        let font = FontRef::new(font_test_data::COLR_GRADIENT_RECT).unwrap();
        let table = font.cpal().unwrap();
        assert_eq!(table.version(), 0);
        assert_eq!(table.num_palette_entries(), 2);
        assert_eq!(table.num_palettes(), 2);
        assert_eq!(table.num_color_records(), 4);

        let color_records = table.color_records_array().unwrap().unwrap();

        assert_eq!(color_records.len(), 4);
        let color_tuples: Vec<[u8; 4]> = color_records
            .iter()
            .map(|cr| [cr.red(), cr.green(), cr.blue(), cr.alpha()])
            .collect();
        assert_eq!(
            color_tuples,
            vec![
                [0x00, 0x00, 0xFF, 0xFF],
                [0x00, 0xFF, 0xFF, 0xFF],
                [0xAA, 0x00, 0xFF, 0xFF],
                [0xAA, 0xFF, 0xFF, 0xFF],
            ]
        );
    }
}
