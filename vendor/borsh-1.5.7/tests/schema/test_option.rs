use crate::common_macro::schema_imports::*;

#[test]
fn simple_option() {
    let actual_name = Option::<u64>::declaration();
    let mut actual_defs = schema_map!();
    Option::<u64>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("Option<u64>", actual_name);
    assert_eq!(
        schema_map! {
            "Option<u64>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "None".to_string(), "()".to_string()),
                    (1, "Some".to_string(), "u64".to_string()),
                ]
            },
            "u64" => Definition::Primitive(8),
            "()" => Definition::Primitive(0)
        },
        actual_defs
    );
}

#[test]
fn nested_option() {
    let actual_name = Option::<Option<u64>>::declaration();
    let mut actual_defs = schema_map!();
    Option::<Option<u64>>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("Option<Option<u64>>", actual_name);
    assert_eq!(
        schema_map! {
            "Option<u64>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "None".to_string(), "()".to_string()),
                    (1, "Some".to_string(), "u64".to_string()),
                ]
            },
            "Option<Option<u64>>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "None".to_string(), "()".to_string()),
                    (1, "Some".to_string(), "Option<u64>".to_string()),
                ]
            },
            "u64" => Definition::Primitive(8),
            "()" => Definition::Primitive(0)
        },
        actual_defs
    );
}
