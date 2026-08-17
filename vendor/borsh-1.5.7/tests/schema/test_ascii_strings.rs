use crate::common_macro::schema_imports::*;

#[test]
fn test_ascii_strings() {
    assert_eq!("AsciiString", ascii::AsciiStr::declaration());
    assert_eq!("AsciiString", ascii::AsciiString::declaration());
    assert_eq!("AsciiChar", ascii::AsciiChar::declaration());

    let want_char = schema_map! {
        "AsciiChar" => Definition::Primitive(1)
    };
    let mut actual_defs = schema_map!();
    ascii::AsciiChar::add_definitions_recursively(&mut actual_defs);
    assert_eq!(want_char, actual_defs);

    let want = schema_map! {
        "AsciiString" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "AsciiChar".to_string()
        },
        "AsciiChar" => Definition::Primitive(1)
    };

    let mut actual_defs = schema_map!();
    ascii::AsciiStr::add_definitions_recursively(&mut actual_defs);
    assert_eq!(want, actual_defs);

    let mut actual_defs = schema_map!();
    ascii::AsciiString::add_definitions_recursively(&mut actual_defs);
    assert_eq!(want, actual_defs);
}
