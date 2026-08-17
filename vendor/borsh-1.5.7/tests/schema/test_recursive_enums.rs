use crate::common_macro::schema_imports::*;

#[allow(unused)]
#[derive(borsh::BorshSchema)]
enum ERecD {
    B { x: String, y: i32 },
    C(u8, Vec<ERecD>),
}

#[test]
pub fn recursive_enum_schema() {
    let mut defs = Default::default();
    ERecD::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
           "ERecD" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "B".to_string(), "ERecDB".to_string()),
                    (1, "C".to_string(), "ERecDC".to_string()),
                ]
            },
            "ERecDB" => Definition::Struct {
                fields: Fields::NamedFields (
                    vec![
                        ("x".to_string(), "String".to_string()),
                        ("y".to_string(), "i32".to_string()),
                    ]
                )
            },
            "ERecDC" => Definition::Struct {
                fields: Fields::UnnamedFields( vec![
                    "u8".to_string(),
                    "Vec<ERecD>".to_string(),
                ])
            },
            "Vec<ERecD>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "ERecD".to_string(),
            },
            "i32" => Definition::Primitive(4),
            "String" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u8".to_string()
            },
            "u8" => Definition::Primitive(1)
        },
        defs
    );
}
