use super::*;
use font_test_data::layout as test_data;

#[test]
fn example_1_scripts() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#example-1-scriptlist-table-and-scriptrecords

    let table = ScriptList::read(test_data::SCRIPTS.into()).unwrap();
    assert_eq!(table.script_count(), 3);
    assert_eq!(table.script_records()[0].script_tag(), Tag::new(b"hani"));
    assert_eq!(table.script_records()[1].script_tag(), Tag::new(b"kana"));
    assert_eq!(table.script_records()[2].script_tag(), Tag::new(b"latn"));
}

#[test]
fn example_2_scripts_and_langs() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#example-2-script-table-langsysrecord-and-langsys-table

    let table = Script::read(test_data::SCRIPTS_AND_LANGUAGES.into()).unwrap();
    let def_sys = table.default_lang_sys().unwrap().unwrap();
    assert_eq!(def_sys.required_feature_index(), 0xffff);
    assert_eq!(def_sys.feature_index_count(), 3);
    assert_eq!(table.lang_sys_count(), 1);

    let urdu_record = &table.lang_sys_records()[0];
    assert_eq!(urdu_record.lang_sys_tag(), Tag::new(b"URD "));
    let urdu_sys = urdu_record.lang_sys(table.offset_data()).unwrap();
    assert_eq!(urdu_sys.required_feature_index(), 3);
    assert_eq!(urdu_sys.feature_index_count(), 3);
}

#[test]
fn example_3_featurelist_and_feature() {
    // https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#example-3-featurelist-table-and-feature-table
    let table = FeatureList::read(test_data::FEATURELIST_AND_FEATURE.into()).unwrap();
    assert_eq!(table.feature_count(), 3);
    let turkish_liga_record = &table.feature_records()[0];
    let feature = turkish_liga_record.feature(table.offset_data()).unwrap();
    assert!(feature.feature_params_offset().is_null());
    assert_eq!(feature.lookup_list_indices().len(), 1);
}
