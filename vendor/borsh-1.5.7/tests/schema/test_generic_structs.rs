use crate::common_macro::schema_imports::*;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[test]
pub fn wrapper_struct() {
    #[derive(borsh::BorshSchema)]
    struct A<T>(T);
    assert_eq!("A<u64>".to_string(), <A<u64>>::declaration());
    let mut defs = Default::default();
    <A<u64>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "A<u64>" => Definition::Struct {fields: Fields::UnnamedFields(vec!["u64".to_string()])},
        "u64" => Definition::Primitive(8)
        },
        defs
    );
}

#[test]
pub fn tuple_struct_params() {
    #[derive(borsh::BorshSchema)]
    struct A<K, V>(K, V);
    assert_eq!(
        "A<u64, String>".to_string(),
        <A<u64, String>>::declaration()
    );
    let mut defs = Default::default();
    <A<u64, String>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "A<u64, String>" => Definition::Struct { fields: Fields::UnnamedFields(vec![
            "u64".to_string(), "String".to_string()
        ])},
        "u64" => Definition::Primitive(8),
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

#[cfg(hash_collections)]
#[test]
pub fn simple_generics() {
    #[derive(borsh::BorshSchema)]
    struct A<K, V> {
        _f1: HashMap<K, V>,
        _f2: String,
    }
    assert_eq!(
        "A<u64, String>".to_string(),
        <A<u64, String>>::declaration()
    );
    let mut defs = Default::default();
    <A<u64, String>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "A<u64, String>" => Definition::Struct {
        fields: Fields::NamedFields(vec![
        ("_f1".to_string(), "HashMap<u64, String>".to_string()),
        ("_f2".to_string(), "String".to_string())
        ])
        },
            "HashMap<u64, String>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "(u64, String)".to_string(),
            },
        "(u64, String)" => Definition::Tuple{elements: vec!["u64".to_string(), "String".to_string()]},
        "u64" => Definition::Primitive(8),
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

fn common_map_associated() -> BTreeMap<String, Definition> {
    schema_map! {

        "Parametrized<String, i8>" => Definition::Struct {
            fields: Fields::NamedFields(vec![
                ("field".to_string(), "i8".to_string()),
                ("another".to_string(), "String".to_string())
            ])
        },
        "i8" => Definition::Primitive(1),
        "String" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "u8".to_string()
        },
        "u8" => Definition::Primitive(1)
    }
}

#[test]
pub fn generic_associated_item() {
    trait TraitName {
        type Associated;
        #[allow(unused)]
        fn method(&self);
    }

    impl TraitName for u32 {
        type Associated = i8;
        fn method(&self) {}
    }

    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    struct Parametrized<V, T>
    where
        T: TraitName,
    {
        field: T::Associated,
        another: V,
    }

    assert_eq!(
        "Parametrized<String, i8>".to_string(),
        <Parametrized<String, u32>>::declaration()
    );

    let mut defs = Default::default();
    <Parametrized<String, u32>>::add_definitions_recursively(&mut defs);
    assert_eq!(common_map_associated(), defs);
}

#[test]
pub fn generic_associated_item2() {
    trait TraitName {
        type Associated;
        #[allow(unused)]
        fn method(&self);
    }

    impl TraitName for u32 {
        type Associated = i8;
        fn method(&self) {}
    }

    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    struct Parametrized<V, T>
    where
        T: TraitName,
    {
        #[borsh(schema(params = "T => <T as TraitName>::Associated"))]
        field: <T as TraitName>::Associated,
        another: V,
    }

    assert_eq!(
        "Parametrized<String, i8>".to_string(),
        <Parametrized<String, u32>>::declaration()
    );

    let mut defs = Default::default();
    <Parametrized<String, u32>>::add_definitions_recursively(&mut defs);
    assert_eq!(common_map_associated(), defs);
}

#[test]
pub fn generic_associated_item3() {
    trait TraitName {
        type Associated;
        #[allow(unused)]
        fn method(&self);
    }

    impl TraitName for u32 {
        type Associated = i8;
        fn method(&self) {}
    }

    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    struct Parametrized<V, T>
    where
        T: TraitName,
    {
        #[borsh(schema(params = "T => T, T => <T as TraitName>::Associated"))]
        field: (<T as TraitName>::Associated, T),
        another: V,
    }

    assert_eq!(
        "Parametrized<String, u32, i8>".to_string(),
        <Parametrized<String, u32>>::declaration()
    );

    let mut defs = Default::default();
    <Parametrized<String, u32>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "Parametrized<String, u32, i8>" => Definition::Struct {
                fields: Fields::NamedFields(vec![
                    ("field".to_string(), "(i8, u32)".to_string()),
                    ("another".to_string(), "String".to_string())
                ])
            },
            "(i8, u32)" => Definition::Tuple {
                elements: vec!["i8".to_string(), "u32".to_string()]
            },
            "i8" => Definition::Primitive(1),
            "u32" => Definition::Primitive(4),
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

