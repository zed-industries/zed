use crate::common_macro::schema_imports::*;

use alloc::borrow::Cow;

#[test]
fn test_cow_str() {
    assert_eq!("String", <Cow<'_, str> as BorshSchema>::declaration());

    let mut actual_defs = schema_map!();
    <Cow<'_, str> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(
        schema_map! {
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
fn test_cow_byte_slice() {
    assert_eq!("Vec<u8>", <Cow<'_, [u8]> as BorshSchema>::declaration());

    let mut actual_defs = schema_map!();
    <Cow<'_, [u8]> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(
        schema_map! {
            "Vec<u8>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u8".to_string(),
            },
            "u8" => Definition::Primitive(1)
        },
        actual_defs
    );
}

#[test]
fn test_cow_slice_of_cow_str() {
    assert_eq!(
        "Vec<String>",
        <Cow<'_, [Cow<'_, str>]> as BorshSchema>::declaration()
    );

    let mut actual_defs = schema_map!();
    <Cow<'_, [Cow<'_, str>]> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(
        schema_map! {
            "Vec<String>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "String".to_string(),
            },
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
