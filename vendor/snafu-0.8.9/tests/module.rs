pub mod inner {
    use snafu::Snafu;

    #[derive(Debug)]
    pub struct Dummy0;

    #[derive(Debug)]
    pub struct Dummy1;

    #[derive(Debug)]
    pub struct Dummy2;

    #[derive(Debug)]
    pub struct Dummy3;

    #[derive(Debug, Snafu)]
    #[snafu(module, visibility(pub))]
    pub enum PubError {
        Variant { v: Dummy0 },
    }

    #[derive(Debug, Snafu)]
    #[snafu(module(custom_pub), visibility(pub))]
    pub enum PubWithCustomModError {
        Variant { v: Dummy1 },
    }

    #[derive(Debug, Snafu)]
    #[snafu(module(custom_pub_crate), visibility(pub(crate)))]
    pub(crate) enum PubCrateWithCustomModError {
        Variant { v: Dummy2 },
    }

    mod child {
        use super::Dummy3;
        use snafu::Snafu;

        #[derive(Debug, Snafu)]
        #[snafu(module, visibility(pub(in crate::inner)))]
        pub enum RestrictedError {
            Variant { v: Dummy3 },
        }
    }

    #[test]
    fn can_set_module_visibility_restricted() {
        let _ = self::child::restricted_error::VariantSnafu { v: Dummy3 }.build();
    }
}

use self::inner::Dummy1;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum SomeError {
    Variant { v: i32 },
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum QualifiedError {
    Variant {
        unqualified: Dummy1,
        mod_struct: inner::Dummy0,
        self_struct: self::Dummy1,
        crate_struct: crate::Dummy1,
        boxed_trait: Box<dyn ::core::any::Any>,
    },
}

#[test]
fn can_use_qualified_names_in_module() {
    let _ = qualified_error::VariantSnafu {
        unqualified: Dummy1,
        mod_struct: inner::Dummy0,
        self_struct: self::Dummy1,
        crate_struct: crate::Dummy1,
        boxed_trait: Box::new(()) as Box<_>,
    }
    .build();
}

#[test]
fn can_set_module() {
    let _ = some_error::VariantSnafu { v: 0i32 }.build();
}

#[test]
fn can_set_module_visibility_pub() {
    let _ = inner::pub_error::VariantSnafu { v: inner::Dummy0 }.build();
}

#[test]
fn can_set_module_visibility_pub_with_custom_name() {
    let _ = inner::custom_pub::VariantSnafu { v: inner::Dummy1 }.build();
}

#[test]
fn can_set_module_visibility_pub_crate_with_custom_name() {
    let _ = inner::custom_pub_crate::VariantSnafu { v: inner::Dummy2 }.build();
}
