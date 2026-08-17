use crate::common_macro::schema_imports::*;

#[test]
fn test_unary_tuple_schema() {
    assert_eq!("(bool,)", <(bool,)>::declaration());
    let mut defs = Default::default();
    <(bool,)>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "(bool,)" => Definition::Tuple { elements: vec!["bool".to_string()] },
        "bool" => Definition::Primitive(1)
        },
        defs
    );
}

#[test]
fn simple_tuple() {
    let actual_name = <(u64, core::num::NonZeroU16, String)>::declaration();
    let mut actual_defs = schema_map!();
    <(u64, core::num::NonZeroU16, String)>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("(u64, NonZeroU16, String)", actual_name);
    assert_eq!(
        schema_map! {
            "(u64, NonZeroU16, String)" => Definition::Tuple {
                elements: vec![
                    "u64".to_string(),
                    "NonZeroU16".to_string(),
                    "String".to_string()
                ]
            },
            "u64" => Definition::Primitive(8),
            "NonZeroU16" => Definition::Primitive(2),
            "String" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u8".to_string()
            },
            "u8" => Definition::Primitive(1)
        },
        actual_defs
    );
}

#[test]
fn nested_tuple() {
    let actual_name = <(u64, (u8, bool), String)>::declaration();
    let mut actual_defs = schema_map!();
    <(u64, (u8, bool), String)>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("(u64, (u8, bool), String)", actual_name);
    assert_eq!(
        schema_map! {
            "(u64, (u8, bool), String)" => Definition::Tuple { elements: vec![
                "u64".to_string(),
                "(u8, bool)".to_string(),
                "String".to_string(),
            ]},
            "(u8, bool)" => Definition::Tuple { elements: vec![ "u8".to_string(), "bool".to_string()]},
            "u64" => Definition::Primitive(8),
            "u8" => Definition::Primitive(1),
            "bool" => Definition::Primitive(1),
            "String" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u8".to_string()
            }
        },
        actual_defs
    );
}
