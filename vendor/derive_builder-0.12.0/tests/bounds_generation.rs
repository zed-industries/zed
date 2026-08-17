#[macro_use]
extern crate derive_builder;

/// A struct that deliberately doesn't implement `Clone`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Dolor(String);

/// Notice that this type derives Builder without disallowing
/// `Lorem<Dolor>`.
#[derive(Debug, Clone, Builder, PartialEq, Eq, Default)]
#[builder(field(private), setter(into))]
pub struct Lorem<T> {
    ipsum: T,
}

#[derive(Debug, Clone, Builder, PartialEq, Eq, Default)]
#[builder(field(private))]
pub struct VecLorem<T> {
    ipsum: Vec<T>,
}

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
#[builder(pattern = "owned", field(private))]
pub struct OwnedLorem<T> {
    ipsum: T,
}

#[test]
fn generic_field_with_clone_has_builder_impl() {
    assert_eq!(
        LoremBuilder::default().ipsum(10).build().unwrap(),
        Lorem { ipsum: 10 }
    );
}

/// The `LoremBuilder` type does not require that `T: Clone`, so we should
/// be able to name a `LoremBuilder<Dolor>`. But all the methods are on an impl
/// bound with `T: Clone`, so we can't do anything with it.
#[test]
fn builder_with_non_clone_generic_compiles() {
    let _: LoremBuilder<Dolor>;
}

/// In this case, we're trying to emit something that converts into a thing that
/// implements clone.
#[test]
fn builder_with_into_generic_compiles() {
    assert_eq!(
        LoremBuilder::<String>::default().ipsum("").build().unwrap(),
        Lorem::default()
    );
}

#[test]
fn builder_with_vec_t_compiles() {
    assert_eq!(
        VecLoremBuilder::<String>::default()
            .ipsum(vec!["Hello".to_string()])
            .build()
            .unwrap(),
        VecLorem {
            ipsum: vec!["Hello".to_string()],
        }
    );
}

#[test]
fn generic_field_without_clone_has_owned_builder() {
    assert_eq!(
        OwnedLoremBuilder::default()
            .ipsum(Dolor::default())
            .build()
            .unwrap(),
        OwnedLorem {
            ipsum: Dolor::default()
        }
    );
}
