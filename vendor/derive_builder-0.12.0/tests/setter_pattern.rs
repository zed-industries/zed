#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(pattern = "immutable")]
struct Lorem {
    immutable: u32,
    #[builder(pattern = "mutable")]
    mutable_override: u32,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(pattern = "mutable")]
struct Ipsum {
    mutable: u32,
    #[builder(pattern = "owned")]
    owned_override: u32,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(pattern = "owned")]
struct Dolor {
    #[builder(pattern = "immutable")]
    immutable_override: u32,
    owned: u32,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Sit {
    default: u32,
}

type ImmutableSetter<T, U> = fn(&T, U) -> T;
type OwnedSetter<T, U> = fn(T, U) -> T;
type MutableSetter<T, U> = fn(&mut T, U) -> &mut T;

#[test]
fn mutable_by_default() {
    // the setter must have the correct signature
    let mutable_setter: MutableSetter<SitBuilder, u32> = SitBuilder::default;

    let mut old = <SitBuilder as Default>::default();
    mutable_setter(&mut old, 42);
    assert_eq!(old.default, Some(42));
}

#[test]
fn mutable() {
    // the setter must have the correct signature
    let mutable_setter: MutableSetter<IpsumBuilder, u32> = IpsumBuilder::mutable;

    let mut old = IpsumBuilder::default();
    mutable_setter(&mut old, 42);
    assert_eq!(old.mutable, Some(42));
}

#[test]
fn mutable_override() {
    // the setter must have the correct signature
    let mutable_setter: MutableSetter<LoremBuilder, u32> = LoremBuilder::mutable_override;

    let mut old = LoremBuilder::default();
    mutable_setter(&mut old, 42);
    assert_eq!(old.mutable_override, Some(42));
}

#[test]
fn immutable() {
    // the setter must have the correct signature
    let immutable_setter: ImmutableSetter<LoremBuilder, u32> = LoremBuilder::immutable;

    let old = LoremBuilder::default();
    let new = immutable_setter(&old, 42);
    assert_eq!(new.immutable, Some(42));
}

#[test]
fn immutable_override() {
    // the setter must have the correct signature
    let immutable_setter: ImmutableSetter<DolorBuilder, u32> = DolorBuilder::immutable_override;

    let old = DolorBuilder::default();
    let new = immutable_setter(&old, 42);
    assert_eq!(new.immutable_override, Some(42));
}

#[test]
fn owned() {
    // the setter must have the correct signature
    let owned_setter: OwnedSetter<DolorBuilder, u32> = DolorBuilder::owned;

    let old = DolorBuilder::default();
    let new = owned_setter(old, 42);
    assert_eq!(new.owned, Some(42));
}

#[test]
fn owned_override() {
    // the setter must have the correct signature
    let owned_setter: OwnedSetter<IpsumBuilder, u32> = IpsumBuilder::owned_override;

    let old = IpsumBuilder::default();
    let new = owned_setter(old, 42);
    assert_eq!(new.owned_override, Some(42));
}
