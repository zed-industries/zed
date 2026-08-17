use crate::common_macro::schema_imports::*;

#[test]
fn test_string() {
    let actual_name = str::declaration();
    assert_eq!("String", actual_name);
    let actual_name = String::declaration();
    assert_eq!("String", actual_name);
    let mut actual_defs = schema_map!();
    String::add_definitions_recursively(&mut actual_defs);
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

    let mut actual_defs = schema_map!();
    str::add_definitions_recursively(&mut actual_defs);
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
