use crate::common_macro::schema_imports::*;

#[track_caller]
fn test_ok<T: BorshSchema>(want: usize) {
    let schema = BorshSchemaContainer::for_type::<T>();
    assert_eq!(Ok(want), schema.max_serialized_size());
}

#[track_caller]
fn test_err<T: BorshSchema>(err: SchemaMaxSerializedSizeError) {
    let schema = BorshSchemaContainer::for_type::<T>();
    assert_eq!(Err(err), schema.max_serialized_size());
}

const MAX_LEN: usize = u32::MAX as usize;

#[test]
fn max_serialized_size_primitives() {
    test_ok::<()>(0);
    test_ok::<bool>(1);

    test_ok::<f32>(4);
    test_ok::<f64>(8);

    test_ok::<i8>(1);
    test_ok::<i16>(2);
    test_ok::<i32>(4);
    test_ok::<i64>(8);
    test_ok::<i128>(16);

    test_ok::<u8>(1);
    test_ok::<u16>(2);
    test_ok::<u32>(4);
    test_ok::<u64>(8);
    test_ok::<u128>(16);

    test_ok::<core::num::NonZeroI8>(1);
    test_ok::<core::num::NonZeroI16>(2);
    test_ok::<core::num::NonZeroI32>(4);
    test_ok::<core::num::NonZeroI64>(8);
    test_ok::<core::num::NonZeroI128>(16);

    test_ok::<core::num::NonZeroU8>(1);
    test_ok::<core::num::NonZeroU16>(2);
    test_ok::<core::num::NonZeroU32>(4);
    test_ok::<core::num::NonZeroU64>(8);
    test_ok::<core::num::NonZeroU128>(16);

    test_ok::<isize>(8);
    test_ok::<usize>(8);
    test_ok::<core::num::NonZeroUsize>(8);
}

#[test]
fn max_serialized_size_built_in_types() {
    test_ok::<core::ops::RangeFull>(0);
    test_ok::<core::ops::RangeInclusive<u8>>(2);
    test_ok::<core::ops::RangeToInclusive<u64>>(8);

    test_ok::<Option<()>>(1);
    test_ok::<Option<u8>>(2);
    test_ok::<Result<u8, usize>>(9);
    test_ok::<Result<u8, Vec<u8>>>(1 + 4 + MAX_LEN);

    test_ok::<()>(0);
    test_ok::<(u8,)>(1);
    test_ok::<(u8, u32)>(5);

    test_ok::<[u8; 0]>(0);
    test_ok::<[u8; 16]>(16);
    test_ok::<[[u8; 4]; 4]>(16);

    test_ok::<[u16; 0]>(0);
    test_ok::<[u16; 16]>(32);
    test_ok::<[[u16; 4]; 4]>(32);

    test_ok::<Vec<u8>>(4 + MAX_LEN);
    test_ok::<String>(4 + MAX_LEN);

    test_err::<Vec<Vec<u8>>>(SchemaMaxSerializedSizeError::Overflow);
    test_ok::<Vec<Vec<()>>>(4 + MAX_LEN * 4);
    test_ok::<[[[(); MAX_LEN]; MAX_LEN]; MAX_LEN]>(0);
}

#[test]
fn max_serialized_size_derived_types() {
    #[derive(BorshSchema)]
    pub struct Empty;

    #[derive(BorshSchema)]
    pub struct Named {
        _foo: usize,
        _bar: [u8; 15],
    }

    #[derive(BorshSchema)]
    #[allow(unused)]
    pub struct Unnamed(usize, [u8; 15]);

    #[derive(BorshSchema)]
    struct Multiple {
        _usz0: usize,
        _usz1: usize,
        _usz2: usize,
        _vec0: Vec<usize>,
        _vec1: Vec<usize>,
    }

    #[derive(BorshSchema)]
    #[allow(unused)]
    struct Recursive(Option<Box<Recursive>>);

    test_ok::<Empty>(0);
    test_ok::<Named>(23);
    test_ok::<Unnamed>(23);
    test_ok::<Multiple>(3 * 8 + 2 * (4 + MAX_LEN * 8));
    test_err::<BorshSchemaContainer>(SchemaMaxSerializedSizeError::Overflow);
    test_err::<Recursive>(SchemaMaxSerializedSizeError::Recursive);
}

#[test]
fn max_serialized_size_custom_enum() {
    #[allow(dead_code)]
    enum Maybe<const N: u8, T> {
        Just(T),
        Nothing,
    }

    impl<const N: u8, T: BorshSchema> BorshSchema for Maybe<N, T> {
        fn declaration() -> Declaration {
            let res = format!(r#"Maybe<{}, {}>"#, N, T::declaration());
            res
        }
        fn add_definitions_recursively(definitions: &mut BTreeMap<Declaration, Definition>) {
            let definition = Definition::Enum {
                tag_width: N,
                variants: vec![
                    (0, "Just".into(), T::declaration()),
                    (1, "Nothing".into(), <()>::declaration()),
                ],
            };
            add_definition(Self::declaration(), definition, definitions);
            T::add_definitions_recursively(definitions);
            <()>::add_definitions_recursively(definitions);
        }
    }

    test_ok::<Maybe<0, ()>>(0);
    test_ok::<Maybe<0, u16>>(2);
    test_ok::<Maybe<0, u64>>(8);

    test_ok::<Maybe<1, ()>>(1);
    test_ok::<Maybe<1, u16>>(3);
    test_ok::<Maybe<1, u64>>(9);

    test_ok::<Maybe<4, ()>>(4);
    test_ok::<Maybe<4, u16>>(6);
    test_ok::<Maybe<4, u64>>(12);
}

#[test]
fn max_serialized_size_bound_vec() {
    #[allow(dead_code)]
    struct BoundVec<const W: u8, const N: u64>;

    impl<const W: u8, const N: u64> BorshSchema for BoundVec<W, N> {
        fn declaration() -> Declaration {
            format!("BoundVec<{}, {}>", W, N)
        }
        fn add_definitions_recursively(definitions: &mut BTreeMap<Declaration, Definition>) {
            let definition = Definition::Sequence {
                length_width: W,
                length_range: 0..=N,
                elements: "u8".to_string(),
            };
            add_definition(Self::declaration(), definition, definitions);
            u8::add_definitions_recursively(definitions);
        }
    }

    test_ok::<BoundVec<4, 0>>(4);
    test_ok::<BoundVec<4, { u16::MAX as u64 }>>(4 + u16::MAX as usize);
    test_ok::<BoundVec<4, 20>>(24);

    test_ok::<BoundVec<1, 0>>(1);
    test_ok::<BoundVec<1, { u16::MAX as u64 }>>(1 + u16::MAX as usize);
    test_ok::<BoundVec<1, 20>>(21);

    test_ok::<BoundVec<0, 0>>(0);
    test_ok::<BoundVec<0, { u16::MAX as u64 }>>(u16::MAX as usize);
    test_ok::<BoundVec<0, 20>>(20);
}

#[test]
fn max_serialized_size_small_vec() {
    #[allow(dead_code)]
    struct SmallVec<T>(core::marker::PhantomData<T>);

    impl<T: BorshSchema> BorshSchema for SmallVec<T> {
        fn declaration() -> Declaration {
            format!(r#"SmallVec<{}>"#, T::declaration())
        }
        fn add_definitions_recursively(definitions: &mut BTreeMap<Declaration, Definition>) {
            let definition = Definition::Sequence {
                length_width: 1,
                length_range: 0..=u8::MAX as u64,
                elements: T::declaration(),
            };
            add_definition(Self::declaration(), definition, definitions);
            T::add_definitions_recursively(definitions);
        }
    }

    test_ok::<SmallVec<u8>>(u8::MAX as usize + 1);
    test_ok::<SmallVec<u16>>(u8::MAX as usize * 2 + 1);
}
