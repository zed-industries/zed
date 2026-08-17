use types::{GlyphId16, MajorMinor};

use super::*;
use crate::tables::layout::{ClassDefFormat2, DeltaFormat, DeviceOrVariationIndex};
use font_test_data::gdef as test_data;

#[test]
fn gdef_header() {
    let table = Gdef::read(test_data::GDEF_HEADER.into()).unwrap();
    assert_eq!(table.version(), MajorMinor::VERSION_1_0);
    assert_eq!(table.mark_attach_class_def_offset(), 0x5a);
}

#[test]
fn glyph_class_def_table() {
    let table = ClassDefFormat2::read(test_data::GLYPHCLASSDEF_TABLE.into()).unwrap();
    assert_eq!(table.class_range_count(), 4);
    let last_record = &table.class_range_records()[3];
    assert_eq!(last_record.start_glyph_id(), GlyphId16::new(0x18f));
    assert_eq!(last_record.end_glyph_id(), GlyphId16::new(0x18f));
}

#[test]
fn attach_list_table() {
    let table = AttachList::read(test_data::ATTACHLIST_TABLE.into()).unwrap();
    assert_eq!(table.glyph_count(), 2);
    assert_eq!(table.attach_point_offsets().len(), 2);
    let attach_point = table.attach_points().get(1).unwrap();
    assert_eq!(attach_point.point_indices()[0].get(), 14);
    assert_eq!(attach_point.point_indices()[1].get(), 23);
}

#[test]
fn lig_caret_list() {
    let table = LigCaretList::read(test_data::LIGCARETLIST_TABLE.into()).unwrap();
    let glyph1 = table.lig_glyphs().get(0).unwrap();
    let glyph2 = table.lig_glyphs().get(1).unwrap();
    assert_eq!(glyph1.caret_value_offsets().len(), 1);
    assert_eq!(glyph2.caret_value_offsets().len(), 2);
    let g1c0: CaretValueFormat1 = glyph1.caret_value_offsets()[0]
        .get()
        .resolve(glyph1.offset_data())
        .unwrap();
    assert_eq!(g1c0.coordinate(), 603);

    let g2c1: CaretValueFormat1 = glyph2.caret_value_offsets()[1]
        .get()
        .resolve(glyph2.offset_data())
        .unwrap();
    assert_eq!(g2c1.coordinate(), 1206);
}

#[test]
fn caretvalueformat3() {
    let table = CaretValueFormat3::read(test_data::CARETVALUEFORMAT3_TABLE.into()).unwrap();
    assert_eq!(table.coordinate(), 1206);
    let DeviceOrVariationIndex::Device(device) = table.device().unwrap() else {
        panic!("not a device table");
    };
    assert_eq!(device.start_size(), 12);
    assert_eq!(device.end_size(), 17);
    assert_eq!(device.delta_format(), DeltaFormat::Local4BitDeltas);
    assert_eq!(
        vec![0x1111, 0x2200],
        device
            .delta_value()
            .iter()
            .map(|x| x.get())
            .collect::<Vec<_>>()
    );
}
