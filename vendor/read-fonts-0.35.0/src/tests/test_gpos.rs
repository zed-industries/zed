use crate::tables::layout::DeltaFormat;

use super::*;
use font_test_data::gpos as test_data;

#[test]
fn singleposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-2-singleposformat1-subtable

    let table = SinglePosFormat1::read(test_data::SINGLEPOSFORMAT1.into()).unwrap();
    assert_eq!(table.value_format(), ValueFormat::Y_PLACEMENT);
    assert_eq!(table.value_record().y_placement.unwrap().get(), -80);
    let coverage = table.coverage().unwrap();
    assert_eq!(coverage.iter().count(), 10);
}

#[test]
fn singleposformat2() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-3-singleposformat2-subtable
    let table = SinglePosFormat2::read(test_data::SINGLEPOSFORMAT2.into()).unwrap();
    assert_eq!(
        table.value_format(),
        ValueFormat::X_PLACEMENT | ValueFormat::X_ADVANCE
    );
    assert_eq!(table.value_count(), 3);
    assert_eq!(
        table.value_records().get(0).unwrap().x_placement(),
        Some(50)
    );
    assert_eq!(table.value_records().get(1).unwrap().x_advance(), Some(25));
    assert_eq!(
        table.value_records().get(2).unwrap().x_placement(),
        Some(10)
    );

    assert!(table.value_records().get(3).is_err());
}

#[test]
fn pairposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-4-pairposformat1-subtable

    let table = PairPosFormat1::read(test_data::PAIRPOSFORMAT1.into()).unwrap();
    assert_eq!(table.value_format1(), ValueFormat::X_ADVANCE);
    assert_eq!(table.value_format2(), ValueFormat::X_PLACEMENT);
    assert_eq!(table.pair_set_count(), 2);

    let set1 = table.pair_sets().get(0).unwrap();
    let set2 = table.pair_sets().get(1).unwrap();
    assert_eq!(set1.pair_value_records().iter().count(), 1);
    assert_eq!(set2.pair_value_records().iter().count(), 1);

    let rec1 = set1.pair_value_records().get(0).unwrap();
    let rec2 = set2.pair_value_records().get(0).unwrap();

    assert_eq!(rec1.second_glyph(), GlyphId16::new(0x59));
    assert_eq!(rec1.value_record1().x_advance(), Some(-30));
    assert!(rec1.value_record1().x_placement().is_none());
    assert_eq!(rec1.value_record2().x_placement(), Some(-20));

    assert_eq!(rec2.second_glyph(), GlyphId16::new(0x59));
    assert_eq!(rec2.value_record1().x_advance(), Some(-40));
    assert_eq!(rec2.value_record2().x_placement(), Some(-25));
}

#[test]
fn pairposformat2() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-5-pairposformat2-subtable

    let table = PairPosFormat2::read(test_data::PAIRPOSFORMAT2.into()).unwrap();
    assert_eq!(table.value_format1().record_byte_len(), 2);
    assert_eq!(table.value_format2().record_byte_len(), 0);
    assert_eq!(table.class1_count(), 2);
    assert_eq!(table.class1_records().iter().count(), 2);

    let class2 = table.class_def2().unwrap();
    match class2 {
        ClassDef::Format1(_) => panic!("expected format2"),
        ClassDef::Format2(cls) => {
            assert_eq!(
                cls.class_range_records()[0].start_glyph_id.get(),
                GlyphId16::new(0x6A)
            );
        }
    }
}

#[test]
fn cursiveposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-6-cursiveposformat1-subtable

    let table = CursivePosFormat1::read(test_data::CURSIVEPOSFORMAT1.into()).unwrap();
    assert_eq!(table.entry_exit_count(), 2);
    assert_eq!(table.entry_exit_record().len(), 2);

    let record2 = &table.entry_exit_record()[1];
    let entry2: AnchorFormat1 = record2
        .entry_anchor_offset()
        .resolve(table.offset_data())
        .unwrap()
        .unwrap();
    assert_eq!(entry2.x_coordinate(), 1500);
    assert_eq!(entry2.y_coordinate(), 44);
}

#[test]
fn markbaseposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-7-markbaseposformat1-subtable
    let table = MarkBasePosFormat1::read(test_data::MARKBASEPOSFORMAT1.into()).unwrap();
    let base_array = table.base_array().unwrap();
    assert_eq!(base_array.base_records().iter().count(), 1);
    let record = base_array.base_records().get(0).unwrap();
    assert_eq!(record.base_anchor_offsets.len(), 2);
    let anchor1: AnchorFormat1 = record.base_anchor_offsets[1]
        .get()
        .resolve(base_array.offset_data())
        .unwrap()
        .unwrap();
    assert_eq!(anchor1.x_coordinate(), 830);
}

#[test]
fn markligposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-8-markligposformat1-subtable

    let table = MarkLigPosFormat1::read(test_data::MARKLIGPOSFORMAT1.into()).unwrap();
    let lig_array = table.ligature_array().unwrap();
    assert_eq!(lig_array.ligature_count(), 1);
    let lig_attach = lig_array.ligature_attaches().get(0).unwrap();
    assert_eq!(lig_attach.component_count(), 3);
    let comp_record = lig_attach
        .component_records()
        .iter()
        .nth(2)
        .unwrap()
        .unwrap();
    assert!(comp_record.ligature_anchor_offsets[0].get().is_null());
    assert!(comp_record.ligature_anchor_offsets[1].get().is_null());
}

#[test]
fn markmarkposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-9-markmarkposformat1-subtable

    let table = MarkMarkPosFormat1::read(test_data::MARKMARKPOSFORMAT1.into()).unwrap();
    assert_eq!(table.mark_class_count(), 1);
    let mark2array = table.mark2_array().unwrap();
    dbg!(mark2array.offset_data());
    assert_eq!(mark2array.mark2_count(), 1);
    assert_eq!(mark2array.mark2_records().iter().count(), 1);
    let record = mark2array.mark2_records().get(0).unwrap();
    assert_eq!(record.mark2_anchor_offsets.len(), 1);
    let anchor_off = record.mark2_anchor_offsets[0].get();
    let anchor: AnchorFormat1 = anchor_off
        .resolve(mark2array.offset_data())
        .unwrap()
        .unwrap();
    assert_eq!(anchor.x_coordinate(), 221);
    assert_eq!(anchor.y_coordinate(), 301);
}

#[test]
fn contextualposformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-10-contextual-positioning-format-1

    let _table =
        crate::tables::layout::SequenceContextFormat1::read(test_data::CONTEXTUALPOSFORMAT1.into())
            .unwrap();
}

#[test]
fn contextualposformat2() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-11-contextual-positioning-format-1
    let _table =
        crate::tables::layout::SequenceContextFormat2::read(test_data::CONTEXTUALPOSFORMAT2.into())
            .unwrap();
}

#[test]
fn contextualposformat3() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-12-contextual-positioning-format-3

    let _table =
        crate::tables::layout::SequenceContextFormat3::read(test_data::CONTEXTUALPOSFORMAT3.into())
            .unwrap();
}

//FIXME: we don't have a way to instantiate individual records right now?
#[test]
fn sequencelookuprecord() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-13-sequencelookuprecord
    let record = FontData::new(test_data::SEQUENCELOOKUPRECORD)
        .read_ref_at::<crate::tables::layout::SequenceLookupRecord>(0)
        .unwrap();
    assert_eq!(record.sequence_index(), 1);
    assert_eq!(record.lookup_list_index(), 1);
}

#[test]
fn valueformattable() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-14-valueformat-table-and-valuerecord

    let table = SinglePosFormat1::read(test_data::VALUEFORMATTABLE.into()).unwrap();
    assert_eq!(
        table.value_format(),
        ValueFormat::X_PLACEMENT
            | ValueFormat::Y_ADVANCE
            | ValueFormat::X_PLACEMENT_DEVICE
            | ValueFormat::Y_ADVANCE_DEVICE
    );
    let record = table.value_record();
    assert_eq!(record.y_advance(), Some(210));
    let DeviceOrVariationIndex::Device(device) = record
        .y_advance_device(table.offset_data())
        .unwrap()
        .unwrap()
    else {
        panic!("not a device");
    };

    assert_eq!((device.start_size(), device.end_size()), (11, 15));
    assert_eq!(device.delta_format(), DeltaFormat::Local2BitDeltas);
    assert_eq!(device.delta_value(), [0x5540]);
}

#[test]
fn anchorformat1() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-15-anchorformat1-table

    let table = AnchorFormat1::read(test_data::ANCHORFORMAT1.into()).unwrap();
    assert_eq!(table.x_coordinate(), 189);
    assert_eq!(table.y_coordinate(), -103);
}

#[test]
fn anchorformat2() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-16-anchorformat2-table

    let table = AnchorFormat2::read(test_data::ANCHORFORMAT2.into()).unwrap();
    assert_eq!(table.x_coordinate(), 322);
    assert_eq!(table.anchor_point(), 13);
}

#[test]
fn anchorformat3() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-17-anchorformat3-table

    let table = AnchorFormat3::read(test_data::ANCHORFORMAT3.into()).unwrap();
    assert_eq!(table.x_coordinate(), 279);
    assert_eq!(table.y_coordinate(), 1301);

    let x_dev = table.x_device().unwrap().unwrap();
    let y_dev = table.y_device().unwrap().unwrap();

    let (DeviceOrVariationIndex::Device(x_dev), DeviceOrVariationIndex::Device(y_dev)) =
        (x_dev, y_dev)
    else {
        panic!("missing device tables");
    };

    assert_eq!(x_dev.delta_format(), DeltaFormat::Local4BitDeltas);
    assert_eq!(x_dev.delta_value(), [0x1111, 0x2200]);

    assert_eq!(y_dev.delta_format(), DeltaFormat::Local4BitDeltas);
    assert_eq!(y_dev.delta_value(), [0x1111, 0x2200]);
}

//NOTE: I think the sample bytes are missing the actual anchor tables??
// and so we can't really round-trip this...
//#[test]
//fn markarraytable() {
//// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#example-18-markarray-table-and-markrecord

//let bytes = [0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x01, 0x00, 0x10];
//let table = MarkArray::read(&bytes).unwrap();
//let owned = table.to_owned_obj(&[]).unwrap();
//let dumped = crate::write::dump_table(&owned);

//assert_hex_eq!(&bytes, &dumped);
//}[1, 1, 1, 1, 1]
