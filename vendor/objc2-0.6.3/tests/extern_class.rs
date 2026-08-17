use objc2::{extern_class, ClassType};
use objc2_foundation::NSObject;
use static_assertions::assert_impl_all;

#[test]
fn allow_deprecated() {
    #![deny(deprecated)]

    // Test allow propagates to impls
    extern_class!(
        #[unsafe(super(NSObject))]
        #[deprecated]
        #[allow(deprecated)]
        struct AllowDeprecated;
    );
}

#[test]
fn cfg() {
    // Test `cfg`. We use `debug_assertions` here because it's something that we
    // know our CI already tests.

    extern_class!(
        #[unsafe(super(NSObject))]
        #[cfg(debug_assertions)]
        #[name = "NSObject"]
        struct OnlyOnDebugAssertions;
    );

    #[cfg(debug_assertions)]
    let _ = OnlyOnDebugAssertions::class();

    extern_class!(
        #[unsafe(super(NSObject))]
        #[cfg(not(debug_assertions))]
        #[name = "NSObject"]
        struct NeverOnDebugAssertions;
    );

    #[cfg(not(debug_assertions))]
    let _ = NeverOnDebugAssertions::class();
}

#[test]
fn derive() {
    extern_class!(
        #[rustfmt::skip]
        #[unsafe(super(NSObject))]
        #[derive(PartialEq)]
        #[derive()]
        #[derive(Eq,)]
        #[derive()]
        #[derive(Debug, Hash,)]
        struct Derive;
    );

    assert_impl_all!(Derive: PartialEq, Eq, core::hash::Hash, core::fmt::Debug);
}
