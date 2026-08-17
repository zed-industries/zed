use crate::common_macro::schema_imports::*;

#[allow(unused)]
#[derive(borsh::BorshSchema)]
struct CRecC {
    a: String,
    b: BTreeMap<String, CRecC>,
}

#[test]
pub fn recursive_struct_schema() {
    let mut defs = Default::default();
    CRecC::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
           "CRecC" => Definition::Struct {
                fields: Fields::NamedFields(
                    vec![
                        (
                            "a".to_string(),
                            "String".to_string(),
                        ),
                        (
                            "b".to_string(),
                            "BTreeMap<String, CRecC>".to_string(),
                        ),
                    ]

                )

            },
            "BTreeMap<String, CRecC>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "(String, CRecC)".to_string(),
            },
            "(String, CRecC)" => Definition::Tuple {elements: vec!["String".to_string(), "CRecC".to_string()]},
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
