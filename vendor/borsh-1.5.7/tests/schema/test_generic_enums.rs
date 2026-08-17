use crate::common_macro::schema_imports::*;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[test]
pub fn complex_enum_generics() {
    #[derive(borsh::BorshSchema)]
    struct Tomatoes;
    #[derive(borsh::BorshSchema)]
    struct Cucumber;
    #[derive(borsh::BorshSchema)]
    struct Oil;
    #[derive(borsh::BorshSchema)]
    struct Wrapper;
    #[derive(borsh::BorshSchema)]
    struct Filling;
    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    enum A<C, W> {
        Bacon,
        Eggs,
        Salad(Tomatoes, C, Oil),
        Sausage { wrapper: W, filling: Filling },
    }
    assert_eq!(
        "A<Cucumber, Wrapper>".to_string(),
        <A<Cucumber, Wrapper>>::declaration()
    );
    let mut defs = Default::default();
    <A<Cucumber, Wrapper>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "Cucumber" => Definition::Struct {fields: Fields::Empty},
        "ASalad<Cucumber>" => Definition::Struct{
            fields: Fields::UnnamedFields(vec!["Tomatoes".to_string(), "Cucumber".to_string(), "Oil".to_string()])
        },
        "ABacon" => Definition::Struct {fields: Fields::Empty},
        "Oil" => Definition::Struct {fields: Fields::Empty},
        "A<Cucumber, Wrapper>" => Definition::Enum {
            tag_width: 1,
            variants: vec![
                (0, "Bacon".to_string(), "ABacon".to_string()),
                (1, "Eggs".to_string(), "AEggs".to_string()),
                (2, "Salad".to_string(), "ASalad<Cucumber>".to_string()),
                (3, "Sausage".to_string(), "ASausage<Wrapper>".to_string())
            ]
        },
        "Wrapper" => Definition::Struct {fields: Fields::Empty},
        "Tomatoes" => Definition::Struct {fields: Fields::Empty},
        "ASausage<Wrapper>" => Definition::Struct {
            fields: Fields::NamedFields(vec![
            ("wrapper".to_string(), "Wrapper".to_string()),
            ("filling".to_string(), "Filling".to_string())
            ])
        },
        "AEggs" => Definition::Struct {fields: Fields::Empty},
        "Filling" => Definition::Struct {fields: Fields::Empty}
        },
        defs
    );
}

// Checks that recursive definitions work. Also checks that re-instantiations of templated types work.
#[cfg(hash_collections)]
#[test]
pub fn complex_enum_generics2() {
    #[derive(borsh::BorshSchema)]
    struct Tomatoes;
    #[derive(borsh::BorshSchema)]
    struct Cucumber;
    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    struct Oil<K, V> {
        seeds: HashMap<K, V>,
        liquid: Option<K>,
    }
    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    struct Wrapper<T> {
        foo: Option<T>,
        bar: Box<A<T, T>>,
    }
    #[derive(borsh::BorshSchema)]
    struct Filling;
    #[allow(unused)]
    #[derive(borsh::BorshSchema)]
    enum A<C, W> {
        Bacon,
        Eggs,
        Salad(Tomatoes, C, Oil<u64, String>),
        Sausage { wrapper: W, filling: Filling },
    }
    assert_eq!(
        "A<Cucumber, Wrapper<String>>".to_string(),
        <A<Cucumber, Wrapper<String>>>::declaration()
    );
    let mut defs = Default::default();
    <A<Cucumber, Wrapper<String>>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "A<Cucumber, Wrapper<String>>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "Bacon".to_string(), "ABacon".to_string()),
                    (1, "Eggs".to_string(), "AEggs".to_string()),
                    (2, "Salad".to_string(), "ASalad<Cucumber>".to_string()),
                    (3, "Sausage".to_string(), "ASausage<Wrapper<String>>".to_string())
                ]
            },
            "A<String, String>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "Bacon".to_string(), "ABacon".to_string()),
                    (1, "Eggs".to_string(), "AEggs".to_string()),
                    (2, "Salad".to_string(), "ASalad<String>".to_string()),
                    (3, "Sausage".to_string(), "ASausage<String>".to_string())
                ]
            },
        "ABacon" => Definition::Struct {fields: Fields::Empty},
        "AEggs" => Definition::Struct {fields: Fields::Empty},
        "ASalad<Cucumber>" => Definition::Struct {fields: Fields::UnnamedFields(vec!["Tomatoes".to_string(), "Cucumber".to_string(), "Oil<u64, String>".to_string()])},
        "ASalad<String>" => Definition::Struct { fields: Fields::UnnamedFields( vec!["Tomatoes".to_string(), "String".to_string(), "Oil<u64, String>".to_string() ])},
        "ASausage<Wrapper<String>>" => Definition::Struct {fields: Fields::NamedFields(vec![("wrapper".to_string(), "Wrapper<String>".to_string()), ("filling".to_string(), "Filling".to_string())])},
        "ASausage<String>" => Definition::Struct{ fields: Fields::NamedFields(vec![("wrapper".to_string(), "String".to_string()), ("filling".to_string(), "Filling".to_string())])},
        "Cucumber" => Definition::Struct {fields: Fields::Empty},
        "Filling" => Definition::Struct {fields: Fields::Empty},
            "HashMap<u64, String>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "(u64, String)".to_string(),
            },
        "Oil<u64, String>" => Definition::Struct { fields: Fields::NamedFields(vec![("seeds".to_string(), "HashMap<u64, String>".to_string()), ("liquid".to_string(), "Option<u64>".to_string())])},
            "Option<String>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "None".to_string(), "()".to_string()),
                    (1, "Some".to_string(), "String".to_string())
                ]
            },
            "Option<u64>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "None".to_string(), "()".to_string()),
                    (1, "Some".to_string(), "u64".to_string())
                ]
            },
        "Tomatoes" => Definition::Struct {fields: Fields::Empty},
        "(u64, String)" => Definition::Tuple {elements: vec!["u64".to_string(), "String".to_string()]},
        "Wrapper<String>" => Definition::Struct{ fields: Fields::NamedFields(vec![("foo".to_string(), "Option<String>".to_string()), ("bar".to_string(), "A<String, String>".to_string())])},
        "u64" => Definition::Primitive(8),
        "()" => Definition::Primitive(0),
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
        "EnumParametrized<String, u32, i8, u16>" => Definition::Enum {
            tag_width: 1,
            variants: vec![
                (0, "B".to_string(), "EnumParametrizedB<u32, i8, u16>".to_string()),
                (1, "C".to_string(), "EnumParametrizedC<String>".to_string())
            ]
        },
        "EnumParametrizedB<u32, i8, u16>" => Definition::Struct { fields: Fields::NamedFields(vec![
            ("x".to_string(), "BTreeMap<u32, u16>".to_string()),
            ("y".to_string(), "String".to_string()),
            ("z".to_string(), "i8".to_string())
        ])},
        "EnumParametrizedC<String>" => Definition::Struct{ fields: Fields::UnnamedFields(vec!["String".to_string(), "u16".to_string()])},
        "BTreeMap<u32, u16>" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "(u32, u16)".to_string(),
        },
        "(u32, u16)" => Definition::Tuple { elements: vec!["u32".to_string(), "u16".to_string()]},
        "u32" => Definition::Primitive(4),
        "i8" => Definition::Primitive(1),
        "u16" => Definition::Primitive(2),
        "String" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "u8".to_string()
        },
        "u8" => Definition::Primitive(1)
    }
}

#[test]
pub fn generic_associated_item1() {
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
    enum EnumParametrized<T, K, V>
    where
        K: TraitName,
        K: core::cmp::Ord,
        V: core::cmp::Ord,
    {
        B {
            x: BTreeMap<K, V>,
            y: String,
            z: K::Associated,
        },
        C(T, u16),
    }

    assert_eq!(
        "EnumParametrized<String, u32, i8, u16>".to_string(),
        <EnumParametrized<String, u32, u16>>::declaration()
    );

    let mut defs = Default::default();
    <EnumParametrized<String, u32, u16>>::add_definitions_recursively(&mut defs);
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
    enum EnumParametrized<T, K, V>
    where
        K: TraitName,
        K: core::cmp::Ord,
        V: core::cmp::Ord,
    {
        B {
            x: BTreeMap<K, V>,
            y: String,
            #[borsh(schema(params = "K => <K as TraitName>::Associated"))]
            z: <K as TraitName>::Associated,
        },
        C(T, u16),
    }

    assert_eq!(
        "EnumParametrized<String, u32, i8, u16>".to_string(),
        <EnumParametrized<String, u32, u16>>::declaration()
    );

    let mut defs = Default::default();
    <EnumParametrized<String, u32, u16>>::add_definitions_recursively(&mut defs);

    assert_eq!(common_map_associated(), defs);
}
