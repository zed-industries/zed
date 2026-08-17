use crate::common_macro::schema_imports::*;

#[test]
pub fn unit_struct() {
    #[derive(borsh::BorshSchema)]
    struct A;

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
        "A" => Definition::Struct {fields: Fields::Empty}
        },
        defs
    );
}

#[test]
pub fn simple_struct() {
    #[derive(borsh::BorshSchema)]
    struct A {
        _f1: u64,
        _f2: String,
    }
    assert_eq!("A".to_string(), A::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "A" => Definition::Struct{ fields: Fields::NamedFields(vec![
        ("_f1".to_string(), "u64".to_string()),
        ("_f2".to_string(), "String".to_string())
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

#[test]
pub fn tuple_struct() {
    #[derive(borsh::BorshSchema)]
    #[allow(unused)]
    struct A(u64, String);
    assert_eq!("A".to_string(), A::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
        "A" => Definition::Struct {fields: Fields::UnnamedFields(vec![
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

#[test]
pub fn boxed() {
    #[derive(borsh::BorshSchema)]
    struct A {
        _f1: Box<u64>,
        _f2: Box<str>,
        _f3: Box<[u8]>,
    }
    assert_eq!("A".to_string(), A::declaration());
    let mut defs = Default::default();
    A::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "Vec<u8>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u8".to_string(),
            },
        "A" => Definition::Struct{ fields: Fields::NamedFields(vec![
        ("_f1".to_string(), "u64".to_string()),
        ("_f2".to_string(), "String".to_string()),
        ("_f3".to_string(), "Vec<u8>".to_string())
        ])},
        "u64" => Definition::Primitive(8),
        "u8" => Definition::Primitive(1),
        "String" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "u8".to_string()
        }
        },
        defs
    );
}
