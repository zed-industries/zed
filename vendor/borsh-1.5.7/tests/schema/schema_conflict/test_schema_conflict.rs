use crate::common_macro::schema_imports::*;
use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::ToString,
    vec::Vec,
};

struct ConflictingSchema;

impl BorshSchema for ConflictingSchema {
    #[inline]
    fn add_definitions_recursively(definitions: &mut BTreeMap<Declaration, Definition>) {
        let fields = Fields::Empty;
        let def = Definition::Struct { fields };
        add_definition(Self::declaration(), def, definitions);
    }
    #[inline]
    fn declaration() -> Declaration {
        "i64".into()
    }
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_conflict() {
    let mut defs = Default::default();
    <Vec<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);

    <ConflictingSchema as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_vec() {
    let mut defs = Default::default();
    <Vec<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <Vec<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_range() {
    let mut defs = Default::default();
    <core::ops::Range<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <core::ops::Range<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(
        &mut defs,
    );
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_slice() {
    let mut defs = Default::default();
    <[i64] as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <[ConflictingSchema] as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_array() {
    let mut defs = Default::default();
    <[i64; 10] as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <[ConflictingSchema; 10] as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_option() {
    let mut defs = Default::default();
    <Option<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <Option<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[allow(unused)]
#[derive(borsh::BorshSchema)]
struct GenericStruct<T> {
    field: T,
}

#[test]
fn test_implicit_conflict_struct() {
    let mut defs = Default::default();
    <GenericStruct<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <GenericStruct<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(
        &mut defs,
    );
    // NOTE: the contents of `defs` depend on the order of 2 above lines
    // this loophole is needed to enable derives for recursive structs/enums
}

#[allow(unused)]
#[derive(borsh::BorshSchema)]
struct SelfConflictingStruct {
    field_1: i64,
    field_2: ConflictingSchema,
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_self_conflicting_struct() {
    let mut defs = Default::default();
    <SelfConflictingStruct as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[allow(unused)]
#[derive(borsh::BorshSchema)]
enum GenericEnum<T> {
    A { field: T },
    B(u64),
}

#[test]
fn test_implicit_conflict_enum() {
    let mut defs = Default::default();
    <GenericEnum<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <GenericEnum<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    // NOTE: the contents of `defs` depend on the order of 2 above lines
    // this loophole is needed to enable derives for recursive structs/enums
}

#[allow(unused)]
#[derive(borsh::BorshSchema)]
enum SelfConflictingEnum {
    A { field: i64 },
    B { field: ConflictingSchema },
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_self_conflicting_enum() {
    let mut defs = Default::default();
    <SelfConflictingEnum as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_result() {
    let mut defs = Default::default();
    <Result<u8, i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <Result<u8, ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_btreemap() {
    let mut defs = Default::default();
    <BTreeMap<i64, u8> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <BTreeMap<ConflictingSchema, u8> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_btreeset() {
    let mut defs = Default::default();
    <BTreeSet<i64> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <BTreeSet<ConflictingSchema> as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}

#[test]
#[should_panic(expected = "Redefining type schema for i64")]
fn test_implicit_conflict_tuple() {
    let mut defs = Default::default();
    <(i64, u8) as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
    <(ConflictingSchema, u8) as borsh::BorshSchema>::add_definitions_recursively(&mut defs);
}
