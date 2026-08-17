use crate::common_macro::schema_imports::*;

// use alloc::collections::BTreeMap;

#[allow(unused)]
struct ThirdParty<K, V>(BTreeMap<K, V>);
#[allow(unused)]
mod third_party_impl {
    use crate::common_macro::schema_imports::*;

    pub(super) fn declaration<K: borsh::BorshSchema, V: borsh::BorshSchema>(
    ) -> borsh::schema::Declaration {
        let params = vec![<K>::declaration(), <V>::declaration()];
        format!(r#"{}<{}>"#, "ThirdParty", params.join(", "))
    }

    pub(super) fn add_definitions_recursively<K: borsh::BorshSchema, V: borsh::BorshSchema>(
        definitions: &mut BTreeMap<borsh::schema::Declaration, borsh::schema::Definition>,
    ) {
        let fields = borsh::schema::Fields::UnnamedFields(vec![
            <BTreeMap<K, V> as borsh::BorshSchema>::declaration(),
        ]);
        let definition = borsh::schema::Definition::Struct { fields };
        let no_recursion_flag = definitions.get(&declaration::<K, V>()).is_none();
        borsh::schema::add_definition(declaration::<K, V>(), definition, definitions);
        if no_recursion_flag {
            <BTreeMap<K, V> as borsh::BorshSchema>::add_definitions_recursively(definitions);
        }
    }
}

#[allow(unused)]
#[derive(BorshSchema)]
struct A<K, V> {
    #[borsh(schema(with_funcs(
        declaration = "third_party_impl::declaration::<K, V>",
        definitions = "third_party_impl::add_definitions_recursively::<K, V>"
    )))]
    x: ThirdParty<K, V>,
    y: u64,
}

#[allow(unused)]
#[derive(BorshSchema)]
enum C<K, V> {
    C3(u64, u64),
    C4(
        u64,
        #[borsh(schema(with_funcs(
            declaration = "third_party_impl::declaration::<K, V>",
            definitions = "third_party_impl::add_definitions_recursively::<K, V>"
        )))]
        ThirdParty<K, V>,
    ),
}

#[test]
pub fn struct_overriden() {
    assert_eq!(
        "A<u64, String>".to_string(),
        <A<u64, String>>::declaration()
    );
    let mut defs = Default::default();
    <A<u64, String>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "A<u64, String>" => Definition::Struct { fields: Fields::NamedFields(vec![
                ("x".to_string(), "ThirdParty<u64, String>".to_string()),
                ("y".to_string(), "u64".to_string())]
            )},
            "ThirdParty<u64, String>" => Definition::Struct { fields: Fields::UnnamedFields(vec![
                "BTreeMap<u64, String>".to_string(),
            ]) },
            "BTreeMap<u64, String>"=> Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "(u64, String)".to_string(),
            },
            "(u64, String)" => Definition::Tuple { elements: vec!["u64".to_string(), "String".to_string()]},
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
pub fn enum_overriden() {
    assert_eq!(
        "C<u64, String>".to_string(),
        <C<u64, String>>::declaration()
    );
    let mut defs = Default::default();
    <C<u64, String>>::add_definitions_recursively(&mut defs);
    assert_eq!(
        schema_map! {
            "C<u64, String>" => Definition::Enum {
                tag_width: 1,
                variants: vec![
                    (0, "C3".to_string(), "CC3".to_string()),
                    (1, "C4".to_string(), "CC4<u64, String>".to_string())
                ]
            },
            "CC3" => Definition::Struct { fields: Fields::UnnamedFields(vec!["u64".to_string(), "u64".to_string()]) },
            "CC4<u64, String>" => Definition::Struct { fields: Fields::UnnamedFields(vec![
                "u64".to_string(), "ThirdParty<u64, String>".to_string()
            ]) },
            "ThirdParty<u64, String>" => Definition::Struct { fields: Fields::UnnamedFields(vec![
                "BTreeMap<u64, String>".to_string(),
            ]) },
            "BTreeMap<u64, String>"=> Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "(u64, String)".to_string(),
            },
            "(u64, String)" => Definition::Tuple { elements: vec!["u64".to_string(), "String".to_string()]},
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
