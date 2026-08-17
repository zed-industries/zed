use sea_query::{error::*, tests_cfg::*, *};

#[test]
fn insert_values_1() {
    let mut insert = Query::insert();
    let result = insert
        .into_table(Glyph::Table)
        .columns([Glyph::Image, Glyph::Aspect])
        .values([String::from("").into()]);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        Error::ColValNumMismatch {
            col_len: 2,
            val_len: 1,
        }
    );
}
