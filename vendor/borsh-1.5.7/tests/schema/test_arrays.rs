use crate::common_macro::schema_imports::*;
#[test]
fn simple_array() {
    let actual_name = <[u64; 32]>::declaration();
    let mut actual_defs = schema_map!();
    <[u64; 32]>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("[u64; 32]", actual_name);
    assert_eq!(
        schema_map! {
            "[u64; 32]" => Definition::Sequence {
                length_width: Definition::ARRAY_LENGTH_WIDTH,
                length_range: 32..=32,
                elements: "u64".to_string()
            },
            "u64" => Definition::Primitive(8)
        },
        actual_defs
    );
}

#[test]
fn nested_array() {
    let actual_name = <[[[u64; 9]; 10]; 32]>::declaration();
    let mut actual_defs = schema_map!();
    <[[[u64; 9]; 10]; 32]>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("[[[u64; 9]; 10]; 32]", actual_name);
    assert_eq!(
        schema_map! {
            "[u64; 9]" => Definition::Sequence {
                length_width: Definition::ARRAY_LENGTH_WIDTH,
                length_range: 9..=9,
                elements: "u64".to_string()
            },
            "[[u64; 9]; 10]" => Definition::Sequence {
                length_width: Definition::ARRAY_LENGTH_WIDTH,
                length_range: 10..=10,
                elements: "[u64; 9]".to_string()
            },
            "[[[u64; 9]; 10]; 32]" => Definition::Sequence {
                length_width: Definition::ARRAY_LENGTH_WIDTH,
                length_range: 32..=32,
                elements: "[[u64; 9]; 10]".to_string()
            },
            "u64" => Definition::Primitive(8)
        },
        actual_defs
    );
}
