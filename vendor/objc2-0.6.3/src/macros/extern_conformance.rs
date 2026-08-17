/// Implement a protocol for an external class.
///
/// This creates a given protocol implementation with regards to the
/// type-system. Useful on types created with [`extern_class!`], not use this
/// on custom classes created with [`define_class!`], instead, implement the
/// protocol within that macro.
///
/// [`define_class!`]: crate::define_class!
///
/// # Examples
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::NSObjectProtocol;
/// # use objc2::runtime::NSObjectProtocol;
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, extern_conformance};
///
/// extern_class!(
///     #[unsafe(super(NSObject))]
///     #[derive(PartialEq, Eq, Hash, Debug)]
///     pub struct MyClass;
/// );
///
/// // SAFETY: The class `MyClass` conforms to the `NSObject` protocol (since
/// // its superclass `NSObject` does).
/// extern_conformance!(unsafe impl NSObjectProtocol for MyClass {});
/// ```
///
/// See the [`extern_class!`] macro for more examples.
///
/// [`extern_class!`]: crate::extern_class!
#[doc(alias = "@interface")]
#[macro_export]
macro_rules! extern_conformance {
    (unsafe impl $(<$($t:ident : $($bound:ident)? $(?$sized:ident)? $(+ $b:path)*),* $(,)?>)? $ty:ident for $protocol:ty {}) => {
        unsafe impl $(<$($t : $($bound)? $(?$sized)? $(+ $b)*),*>)? $ty for $protocol {
            // TODO(breaking): Add private marker here.
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::runtime::NSObject;
    use crate::{extern_class, extern_protocol};

    extern_class!(
        #[unsafe(super(NSObject))]
        #[name = "NSObject"]
        struct OldSyntax;
    );

    extern_protocol!(
        #[name = "NSObjectProtocol"]
        #[allow(clippy::missing_safety_doc)]
        unsafe trait Protocol {}
    );

    // Old syntax, must continue to work until next breaking version.
    unsafe impl Protocol for OldSyntax {}
}
