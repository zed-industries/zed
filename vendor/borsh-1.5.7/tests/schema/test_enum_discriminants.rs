use crate::common_macro::schema_imports::*;

#[allow(unused)]
#[derive(BorshSchema)]
#[borsh(use_discriminant = true)]
#[repr(i16)]
enum XY {
    A,
    B = 20,
    C,
    D(u32, u32),
    E = 10,
    F(u64),
}

#[test]
fn test_schema_discriminant_no_unit_type() {
    assert_eq!("XY".to_string(), XY::declaration());
    let mut defs = Default::default();
    XY::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "XY" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                     (0, "A".to_string(), "XYA".to_string()),
                     (20, "B".to_string(), "XYB".to_string()),
                     (21, "C".to_string(), "XYC".to_string()),
                     (22, "D".to_string(), "XYD".to_string()),
                     (10, "E".to_string(), "XYE".to_string()),
                     (11, "F".to_string(), "XYF".to_string())
                ]
            },
            "XYA" => Definition::Struct{ fields: Fields::Empty },
            "XYB" => Definition::Struct{ fields: Fields::Empty },
            "XYC" => Definition::Struct{ fields: Fields::Empty },
            "XYD" => Definition::Struct{ fields: Fields::UnnamedFields(
                vec!["u32".to_string(), "u32".to_string()]
            )},
            "XYE" => Definition::Struct{ fields: Fields::Empty },
            "XYF" => Definition::Struct{ fields: Fields::UnnamedFields(
                vec!["u64".to_string()]

            )},
            "u32" => Definition::Primitive(4),
            "u64" => Definition::Primitive(8)
        },
        defs
    );
}

#[allow(unused)]
#[derive(BorshSchema)]
#[borsh(use_discriminant = false)]
#[repr(i16)]
enum XYNoDiscriminant {
    A,
    B = 20,
    C,
    D(u32, u32),
    E = 10,
    F(u64),
}

#[test]
fn test_schema_discriminant_no_unit_type_no_use_discriminant() {
    assert_eq!(
        "XYNoDiscriminant".to_string(),
        XYNoDiscriminant::declaration()
    );
    let mut defs = Default::default();
    XYNoDiscriminant::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "XYNoDiscriminant" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                     (0, "A".to_string(), "XYNoDiscriminantA".to_string()),
                     (1, "B".to_string(), "XYNoDiscriminantB".to_string()),
                     (2, "C".to_string(), "XYNoDiscriminantC".to_string()),
                     (3, "D".to_string(), "XYNoDiscriminantD".to_string()),
                     (4, "E".to_string(), "XYNoDiscriminantE".to_string()),
                     (5, "F".to_string(), "XYNoDiscriminantF".to_string())
                ]
            },
            "XYNoDiscriminantA" => Definition::Struct{ fields: Fields::Empty },
            "XYNoDiscriminantB" => Definition::Struct{ fields: Fields::Empty },
            "XYNoDiscriminantC" => Definition::Struct{ fields: Fields::Empty },
            "XYNoDiscriminantD" => Definition::Struct{ fields: Fields::UnnamedFields(
                vec!["u32".to_string(), "u32".to_string()]
            )},
            "XYNoDiscriminantE" => Definition::Struct{ fields: Fields::Empty },
            "XYNoDiscriminantF" => Definition::Struct{ fields: Fields::UnnamedFields(
                vec!["u64".to_string()]

            )},
            "u32" => Definition::Primitive(4),
            "u64" => Definition::Primitive(8)
        },
        defs
    );
}
