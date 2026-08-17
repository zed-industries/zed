use crate::common_macro::schema_imports::*;

#[test]
fn range() {
    assert_eq!("RangeFull", <core::ops::RangeFull>::declaration());
    let mut actual_defs = schema_map!();
    <core::ops::RangeFull>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(
        schema_map! {
            "RangeFull" => Definition::Struct {
                fields: Fields::Empty
            }
        },
        actual_defs
    );

    let actual_name = <core::ops::Range<u64>>::declaration();
    let mut actual_defs = schema_map!();
    <core::ops::Range<u64>>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("Range<u64>", actual_name);
    assert_eq!(
        schema_map! {
            "Range<u64>" => Definition::Struct {
                fields: Fields::NamedFields(vec![
                    ("start".into(), "u64".into()),
                    ("end".into(), "u64".into()),
                ])
            },
            "u64" => Definition::Primitive(8)
        },
        actual_defs
    );

    let actual_name = <core::ops::RangeTo<u64>>::declaration();
    let mut actual_defs = schema_map!();
    <core::ops::RangeTo<u64>>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("RangeTo<u64>", actual_name);
    assert_eq!(
        schema_map! {
            "RangeTo<u64>" => Definition::Struct {
                fields: Fields::NamedFields(vec![
                    ("end".into(), "u64".into()),
                ])
            },
            "u64" => Definition::Primitive(8)
        },
        actual_defs
    );
}
