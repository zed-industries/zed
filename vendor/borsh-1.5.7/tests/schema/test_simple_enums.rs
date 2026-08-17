use crate::common_macro::schema_imports::*;

use borsh::{try_from_slice_with_schema, try_to_vec_with_schema};

#[test]
pub fn simple_enum() {
    #[allow(dead_code)]
    #[derive(borsh::BorshSchema)]
    enum A {
        Bacon,
        Eggs,
    }
    // https://github.com/near/borsh-rs/issues/112
    #[allow(unused)]
    impl A {
        pub fn declaration() -> usize {
            42
        }
    }
    assert_eq!("A".to_string(), <A as borsh::BorshSchema>::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "ABacon" => Definition::Struct{ fields: Fields::Empty },
        "AEggs" => Definition::Struct{ fields: Fields::Empty },
            "A" => Definition::Enum {
                tag_width: 1,
                variants: vec![(0, "Bacon".to_string(), "ABacon".to_string()), (1, "Eggs".to_string(), "AEggs".to_string())]
            }
        },
        defs
    );
}

#[test]
pub fn single_field_enum() {
    #[allow(dead_code)]
    #[derive(borsh::BorshSchema)]
    enum A {
        Bacon,
    }
    assert_eq!("A".to_string(), A::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "ABacon" => Definition::Struct {fields: Fields::Empty},
            "A" => Definition::Enum {
                tag_width: 1,
                variants: vec![(0, "Bacon".to_string(), "ABacon".to_string())]
            }
        },
        defs
    );
}

#[test]
pub fn complex_enum_with_schema() {
    #[derive(
        borsh::BorshSchema,
        Default,
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        PartialEq,
        Debug,
    )]
    struct Tomatoes;
    #[derive(
        borsh::BorshSchema,
        Default,
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        PartialEq,
        Debug,
    )]
    struct Cucumber;
    #[derive(
        borsh::BorshSchema,
        Default,
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        PartialEq,
        Debug,
    )]
    struct Oil;
    #[derive(
        borsh::BorshSchema,
        Default,
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        PartialEq,
        Debug,
    )]
    struct Wrapper;
    #[derive(
        borsh::BorshSchema,
        Default,
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        PartialEq,
        Debug,
    )]
    struct Filling;
    #[derive(
        borsh::BorshSchema, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq, Debug,
    )]
    enum A {
        Bacon,
        Eggs,
        Salad(Tomatoes, Cucumber, Oil),
        Sausage { wrapper: Wrapper, filling: Filling },
    }

    impl Default for A {
        fn default() -> Self {
            A::Sausage {
                wrapper: Default::default(),
                filling: Default::default(),
            }
        }
    }
    // First check schema.
    assert_eq!("A".to_string(), A::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "Cucumber" => Definition::Struct {fields: Fields::Empty},
        "ASalad" => Definition::Struct{ fields: Fields::UnnamedFields(vec!["Tomatoes".to_string(), "Cucumber".to_string(), "Oil".to_string()])},
        "ABacon" => Definition::Struct {fields: Fields::Empty},
        "Oil" => Definition::Struct {fields: Fields::Empty},
            "A" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "Bacon".to_string(), "ABacon".to_string()),
                    (1, "Eggs".to_string(), "AEggs".to_string()),
                    (2, "Salad".to_string(), "ASalad".to_string()),
                    (3, "Sausage".to_string(), "ASausage".to_string())
                ]
            },
        "Wrapper" => Definition::Struct {fields: Fields::Empty},
        "Tomatoes" => Definition::Struct {fields: Fields::Empty},
        "ASausage" => Definition::Struct { fields: Fields::NamedFields(vec![
        ("wrapper".to_string(), "Wrapper".to_string()),
        ("filling".to_string(), "Filling".to_string())
        ])},
        "AEggs" => Definition::Struct {fields: Fields::Empty},
        "Filling" => Definition::Struct {fields: Fields::Empty}
        },
        defs
    );
    // Then check that we serialize and deserialize with schema.
    let obj = A::default();
    let data = try_to_vec_with_schema(&obj).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let obj2: A = try_from_slice_with_schema(&data).unwrap();
    assert_eq!(obj, obj2);
}
