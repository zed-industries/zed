pub mod inner {
    use snafu::Snafu;

    #[derive(Debug)]
    pub struct Dummy0;

    #[derive(Debug)]
    pub struct Dummy1;

    #[derive(Debug, Snafu)]
    #[snafu(module, visibility(pub))]
    pub struct PubError;

    #[derive(Debug, Snafu)]
    #[snafu(module(custom_pub), visibility(pub))]
    pub struct PubWithCustomModError;

    #[derive(Debug, Snafu)]
    #[snafu(module(custom_pub_crate), visibility(pub(crate)))]
    pub(crate) struct PubCrateWithCustomModError;

    #[derive(Debug, Snafu)]
    #[snafu(module, visibility(pub(in crate::module)))]
    pub struct RestrictedError;
}

use self::inner::Dummy1;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module)]
pub struct SomeError;

#[derive(Debug, Snafu)]
#[snafu(module)]
pub struct QualifiedError {
    unqualified: Dummy1,
    mod_struct: inner::Dummy0,
    self_struct: self::Dummy1,
    crate_struct: crate::module::Dummy1,
    boxed_trait: Box<dyn ::core::any::Any>,
}

#[test]
fn can_use_qualified_names_in_module() {
    let _ = qualified_error::QualifiedSnafu {
        unqualified: Dummy1,
        mod_struct: inner::Dummy0,
        self_struct: self::Dummy1,
        crate_struct: crate::module::Dummy1,
        boxed_trait: Box::new(()) as Box<_>,
    }
    .build();
}

#[test]
fn can_set_module() {
    let _ = some_error::SomeSnafu.build();
}

#[test]
fn can_set_module_visibility_pub() {
    let _ = inner::pub_error::PubSnafu.build();
}

#[test]
fn can_set_module_visibility_restricted() {
    let _ = inner::restricted_error::RestrictedSnafu.build();
}

#[test]
fn can_set_module_visibility_pub_with_custom_name() {
    let _ = inner::custom_pub::PubWithCustomModSnafu.build();
}

#[test]
fn can_set_module_visibility_pub_crate_with_custom_name() {
    let _ = inner::custom_pub_crate::PubCrateWithCustomModSnafu.build();
}
