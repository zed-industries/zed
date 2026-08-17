use super::*;
use font_test_data::gsub as test_data;

#[test]
fn singlesubstformat1() {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-2-singlesubstformat1-subtable
    let table = SingleSubstFormat1::read(test_data::SINGLESUBSTFORMAT1_TABLE.into()).unwrap();
    assert_eq!(table.delta_glyph_id(), 192);
}

#[test]
fn singlesubstformat2() {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-3-singlesubstformat2-subtable
    let table = SingleSubstFormat2::read(test_data::SINGLESUBSTFORMAT2_TABLE.into()).unwrap();
    assert_eq!(
        table.substitute_glyph_ids(),
        &[
            GlyphId16::new(305),
            GlyphId16::new(309),
            GlyphId16::new(318),
            GlyphId16::new(323)
        ],
    );
}

#[test]
fn multiplesubstformat1() {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-4-multiplesubstformat1-subtable
    let table = MultipleSubstFormat1::read(test_data::MULTIPLESUBSTFORMAT1_TABLE.into()).unwrap();
    assert_eq!(table.sequences().len(), 1);
    let seq0 = table.sequences().get(0).unwrap();
    assert_eq!(
        seq0.substitute_glyph_ids(),
        &[GlyphId16::new(26), GlyphId16::new(26), GlyphId16::new(29)]
    );
}

#[test]
fn alternatesubstformat1() {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-5-alternatesubstformat-1-subtable
    let table = AlternateSubstFormat1::read(test_data::ALTERNATESUBSTFORMAT1_TABLE.into()).unwrap();
    assert_eq!(table.alternate_sets().len(), 1);
    let altset0 = table.alternate_sets().get(0).unwrap();
    assert_eq!(
        altset0.alternate_glyph_ids(),
        &[GlyphId16::new(0xc9), GlyphId16::new(0xca)]
    );
}

#[test]
fn ligaturesubstformat1() {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-6-ligaturesubstformat1-subtable
    let table = LigatureSubstFormat1::read(test_data::LIGATURESUBSTFORMAT1_TABLE.into()).unwrap();
    assert_eq!(table.ligature_sets().len(), 2);
    let ligset0 = table.ligature_sets().get(0).unwrap();

    assert_eq!(ligset0.ligatures().len(), 1);
    let lig0 = ligset0.ligatures().get(0).unwrap();
    assert_eq!(lig0.ligature_glyph(), GlyphId16::new(347));
    assert_eq!(
        lig0.component_glyph_ids(),
        &[GlyphId16::new(0x28), GlyphId16::new(0x17)]
    );

    let ligset1 = table.ligature_sets().get(1).unwrap();
    let lig0 = ligset1.ligatures().get(0).unwrap();
    assert_eq!(lig0.ligature_glyph(), GlyphId16::new(0xf1));
    assert_eq!(
        lig0.component_glyph_ids(),
        &[GlyphId16::new(0x1a), GlyphId16::new(0x1d)]
    );
}

//TODO:
// - https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-7-contextual-substitution-format-1
// - https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-8-contextual-substitution-format-2
// - https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-9-contextual-substitution-format-3
// - https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#example-10-reversechainsinglesubstformat1-subtable
