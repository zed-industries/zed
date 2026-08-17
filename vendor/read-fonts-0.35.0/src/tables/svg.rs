//! The [SVG](https://learn.microsoft.com/en-us/typography/opentype/spec/svg) table

use core::cmp::Ordering;

include!("../../generated/generated_svg.rs");

impl<'a> Svg<'a> {
    /// Get the raw data of the SVG document. Is not guaranteed to be valid and might be compressed.
    pub fn glyph_data(&self, glyph_id: GlyphId) -> Result<Option<&'a [u8]>, ReadError> {
        let document_list = self.svg_document_list()?;
        let svg_document = document_list
            .document_records()
            .binary_search_by(|r| {
                if r.start_glyph_id.get() > glyph_id {
                    Ordering::Greater
                } else if r.end_glyph_id.get() < glyph_id {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .and_then(|index| document_list.document_records().get(index))
            .and_then(|r| {
                let all_data = document_list.data.as_bytes();
                all_data.get(
                    r.svg_doc_offset.get() as usize
                        ..(r.svg_doc_offset.get() + r.svg_doc_length.get()) as usize,
                )
            });

        Ok(svg_document)
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn read_dummy_svg_file() {
        let data: [u16; 32] = [
            0, // Version
            0, 10, // SVGDocumentListOffset
            0, 0, // Reserved
            // SVGDocumentList
            3, // numEntries
            // documentRecords
            // Record 1
            1, // startGlyphID
            3, // endGlyphID
            0, 38, // svgDocOffset
            0, 10, // svgDocLength
            // Record 2
            6, // startGlyphID
            7, // endGlyphID
            0, 48, // svgDocOffset
            0, 6, // svgDocLength
            // Record 3
            9, // startGlyphID
            9, // endGlyphID
            0, 38, // svgDocOffset
            0, 10,
            // svgDocLength
            // SVG Documents. Not actual valid SVGs, but just dummy data.
            1, 0, 0, 0, 1, // Document 1
            2, 0, 0, // Document 2
        ];

        let mut buf = BeBuffer::new();
        buf = buf.extend(data);

        let table = Svg::read(buf.data().into()).unwrap();

        let first_document = &[0, 1, 0, 0, 0, 0, 0, 0, 0, 1][..];
        let second_document = &[0, 2, 0, 0, 0, 0][..];

        assert_eq!(table.glyph_data(GlyphId::new(0)).unwrap(), None);
        assert_eq!(
            table.glyph_data(GlyphId::new(1)).unwrap(),
            Some(first_document)
        );
        assert_eq!(
            table.glyph_data(GlyphId::new(2)).unwrap(),
            Some(first_document)
        );
        assert_eq!(
            table.glyph_data(GlyphId::new(3)).unwrap(),
            Some(first_document)
        );
        assert_eq!(table.glyph_data(GlyphId::new(4)).unwrap(), None);
        assert_eq!(table.glyph_data(GlyphId::new(5)).unwrap(), None);
        assert_eq!(
            table.glyph_data(GlyphId::new(6)).unwrap(),
            Some(second_document)
        );
        assert_eq!(
            table.glyph_data(GlyphId::new(7)).unwrap(),
            Some(second_document)
        );
        assert_eq!(table.glyph_data(GlyphId::new(8)).unwrap(), None);
        assert_eq!(
            table.glyph_data(GlyphId::new(9)).unwrap(),
            Some(first_document)
        );
        assert_eq!(table.glyph_data(GlyphId::new(10)).unwrap(), None);
    }
}
