/// Create a new trait to represent a protocol.
///
/// This is similar to a `@protocol` declaration in Objective-C.
///
/// See [Protocols - The Objective-C Programming Language][protocols] and
/// [Working with Protocols - Programming with Objective-C][working-with] for
/// general information about protocols in Objective-C.
///
/// This macro will create an `unsafe` trait with methods that provide access
/// to the functionality exposed by the protocol.
///
/// Conforming to the protocol can be done in two ways:
/// - For external classes, use the [`extern_conformance!`] macro.
/// - For custom classes created with the [`define_class!`] macro, implement
///   the trait inside the macro.
///
/// Objective-C has a smart feature where you can write `id<MyProtocol>`, and
/// then work with the protocol as-if it was an object; this is very similar
/// to `dyn` traits in Rust, and we implement it in a similar way, see
/// [`ProtocolObject`] for details.
///
/// [protocols]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html
/// [working-with]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html
/// [`extern_conformance!`]: crate::extern_conformance
/// [`ProtocolObject`]: crate::runtime::ProtocolObject
///
///
/// # Specification
///
/// The syntax is similar enough to Rust syntax that if you invoke the macro
/// with parentheses (as opposed to curly brackets), `rustfmt` will be able to
/// format the contents.
///
/// This macro creates an `unsafe` trait with the specified methods. A default
/// implementation of the method is generated based on the selector specified
/// with `#[unsafe(method(a:selector:))]`. Similar to [`extern_methods!`], you
/// can use the `#[unsafe(method_family = ...)]` attribute to override the
/// inferred method family.
///
/// Other protocols that this protocol conforms to / inherits can be specified
/// as supertraits.
///
/// The new trait `T` is automatically implemented for
/// [`ProtocolObject<dyn T>`], which also means that [`ProtocolType`] is
/// implemented for `dyn T`.
///
/// Finally, you can use the `#[optional]` attribute to mark optional methods.
/// This currently doesn't have any effect, but probably will have one in the
/// future when implementing protocols in [`define_class!`].
///
/// This macro otherwise shares similarities with [`extern_class!`] and
/// [`extern_methods!`].
///
/// [`ProtocolObject<dyn T>`]: crate::runtime::ProtocolObject
/// [`ProtocolType`]: crate::ProtocolType
/// [`define_class!`]: crate::define_class
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
///
///
/// # Safety
///
/// The following are required for using the macro itself:
/// - The specified name must be an existing Objective-C protocol.
/// - The protocol must actually inherit/conform to the protocols specified
///   as supertraits.
///
/// Each method is annotated with `#[unsafe(method(...))]`, where you are
/// responsible for ensuring that the declaration is correct.
///
/// While the following are required when implementing the `unsafe` trait for
/// a new type:
/// - The type must represent an object that implements the protocol.
///
///
/// # Examples
///
/// Create a trait to represent the `NSItemProviderWriting` protocol (in
/// practice, you would import this from `objc2-foundation`, this is just for
/// demonstration purposes).
///
/// ```
/// use std::ffi::c_void;
/// use objc2::ffi::NSInteger;
/// use objc2::rc::Retained;
/// use objc2::runtime::{NSObject, NSObjectProtocol};
/// use objc2::extern_protocol;
/// # type NSArray<T> = T;
/// # type NSString = NSObject;
/// # type NSProgress = NSObject;
/// # type NSItemProviderRepresentationVisibility = NSInteger;
/// # #[cfg(defined_in_foundation)]
/// use objc2_foundation::{NSArray, NSString, NSProgress, NSItemProviderRepresentationVisibility};
///
/// extern_protocol!(
///     /// This comment will appear on the trait as expected.
///     //
///     // We could have named the trait something else on the Rust-side, and
///     // then used this to keep it correct from Objective-C.
///     // #[name = "NSItemProviderWriting"]
///     //
///     // SAFETY:
///     // - The name is correct.
///     // - The protocol does inherit from `NSObjectProtocol`.
///     pub unsafe trait NSItemProviderWriting: NSObjectProtocol {
///         //                                  ^^^^^^^^^^^^^^^^
///         // This protocol inherits from the `NSObject` protocol
///
///         // This method we mark as `unsafe`, since we aren't using the
///         // correct type for the completion handler.
///         #[unsafe(method(loadDataWithTypeIdentifier:forItemProviderCompletionHandler:))]
///         unsafe fn loadData(
///             &self,
///             type_identifier: &NSString,
///             completion_handler: *mut c_void,
///         ) -> Option<Retained<NSProgress>>;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(writableTypeIdentifiersForItemProvider))]
///         fn writableTypeIdentifiersForItemProvider_class()
///             -> Retained<NSArray<NSString>>;
///
///         // The rest of these are optional, which means that a user of
///         // `define_class!` would not need to implement them.
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(writableTypeIdentifiersForItemProvider))]
///         #[optional]
///         fn writableTypeIdentifiersForItemProvider(&self)
///             -> Retained<NSArray<NSString>>;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(itemProviderVisibilityForRepresentationWithTypeIdentifier:))]
///         #[optional]
///         fn itemProviderVisibilityForRepresentation_class(
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///
///         // SAFETY: The method is correctly specified.
///         #[unsafe(method(itemProviderVisibilityForRepresentationWithTypeIdentifier:))]
///         #[optional]
///         fn itemProviderVisibilityForRepresentation(
///             &self,
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///     }
/// );
///
/// // Types can now implement `NSItemProviderWriting`, and use the methods
/// // from it as we specified.
/// ```
///
/// See the source code of `objc2-foundation` for many more examples.
#[doc(alias = "@protocol")]
#[macro_export]
macro_rules! extern_protocol {
    (
        // The special #[name = $name:literal] attribute is supported here.
        $(#[$($attrs:tt)*])*
        $v:vis unsafe trait $protocol:ident $(: $conforms_to:ident $(+ $conforms_to_rest:ident)*)? {
            $($methods:tt)*
        }
    ) => {
        $crate::__extract_struct_attributes! {
            ($(#[$($attrs)*])*)

            ($crate::__inner_extern_protocol)
            ($protocol)
            ($v unsafe trait $protocol $(: $conforms_to $(+ $conforms_to_rest)*)? {
                $crate::__extern_protocol_rewrite_methods! {
                    $($methods)*
                }
            })
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_protocol {
    (
        ($protocol:ident)
        ($protocol_definition:item)

        ($($superclasses:tt)*)
        ($($thread_kind:tt)*)
        ($($name:tt)*)
        ($($ivars:tt)*)
        ($($derives:tt)*)
        ($($attr_protocol:tt)*)
        ($($attr_impl:tt)*)
    ) => {
        $($attr_protocol)*
        $protocol_definition

        $($attr_impl)*
        unsafe impl<T> $protocol for $crate::runtime::ProtocolObject<T>
        where
            T: ?$crate::__macro_helpers::Sized + $protocol
        {}

        // SAFETY: The specified name is ensured by caller to be a protocol,
        // and is correctly defined.
        $($attr_impl)*
        unsafe impl $crate::ProtocolType for dyn $protocol {
            const NAME: &'static $crate::__macro_helpers::str = $crate::__fallback_if_not_set! {
                ($($name)*)
                ($crate::__macro_helpers::stringify!($protocol))
            };
            const __INNER: () = ();
        }

        // SAFETY: Anything that implements the protocol is valid to convert
        // to `ProtocolObject<dyn [PROTO]>`.
        $($attr_impl)*
        unsafe impl<T> $crate::runtime::ImplementedBy<T> for dyn $protocol
        where
            T: ?$crate::__macro_helpers::Sized + $crate::Message + $protocol
        {
            const __INNER: () = ();
        }

        // TODO: Should we also implement `ImplementedBy` for `Send + Sync`
        // types, as is done for `NSObjectProtocol`?

        $crate::__extern_protocol_check_no_super!($($superclasses)*);

        $crate::__extern_protocol_check_no_thread_kind!($($thread_kind)*);

        $crate::__extern_protocol_check_no_ivars!($($ivars)*);

        $crate::__extern_protocol_check_no_derives!($($derives)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_super {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macro_helpers::compile_error!("#[super] is not supported in extern_protocol!");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_thread_kind {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macro_helpers::compile_error!(
            "#[thread_kind = ...] is not supported in extern_protocol!. Add MainThreadOnly or AnyThread bound instead"
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_ivars {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macro_helpers::compile_error!("#[ivars] is not supported in extern_protocol!");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_check_no_derives {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macro_helpers::compile_error!(
            "#[derive(...)] is not supported in extern_protocol!"
        );
    };
}

/// tt-munch each protocol method.
#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_rewrite_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_method_attributes)
            ($(#[$($m)*])*)

            ($crate::__extern_protocol_method_out)
            ($v unsafe fn $name($($params)*) $(-> $ret)?)
            ($($($where : $bound ,)+)?)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($params:tt)*) $(-> $ret:ty)?
        // TODO: Handle where bounds better
        $(where $($where:ty : $bound:path),+ $(,)?)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_method_attributes)
            ($(#[$($m)*])*)

            ($crate::__extern_protocol_method_out)
            ($v fn $name($($params)*) $(-> $ret)?)
            ($($($where : $bound ,)+)?)
        }

        $crate::__extern_protocol_rewrite_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_out {
    // Instance method
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        ($method_or_method_id:ident($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)
    } => {
        $($attr_method)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::Message
            $(, $where : $bound)*
        {
            $crate::__extern_methods_method_id_deprecated!($method_or_method_id($($sel)*));

            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                    ($($method_family)*)
                }
            }
        }
    };

    // Class method
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_class_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        ($method_or_method_id:ident($($sel:tt)*))
        ($($method_family:tt)*)
        ($($optional:tt)*)
        ($($attr_method:tt)*)
        ($($attr_use:tt)*)
    } => {
        $($attr_method)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::ClassType
            $(, $where : $bound)*
        {
            $crate::__extern_methods_method_id_deprecated!($method_or_method_id($($sel)*));

            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                    ($($method_family)*)
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_protocol_method_id_deprecated {
    (method($($sel:tt)*)) => {};
    (method_id($($sel:tt)*)) => {{
        #[deprecated = $crate::__macro_helpers::concat!(
            "using #[unsafe(method_id(",
            $crate::__macro_helpers::stringify!($($sel)*),
            "))] inside extern_protocol! is deprecated.\nUse #[unsafe(method(",
            $crate::__macro_helpers::stringify!($($sel)*),
            "))] instead",
        )]
        #[inline]
        fn method_id() {}
        method_id();
    }};
}

#[cfg(test)]
mod tests {
    use crate::{extern_protocol, ProtocolType};

    #[test]
    fn explicit_name() {
        extern_protocol!(
            #[allow(clippy::missing_safety_doc)]
            #[name = "NSObject"]
            unsafe trait Foo {}
        );

        let proto = <dyn Foo>::protocol().unwrap();
        assert_eq!(proto.name().to_str().unwrap(), "NSObject");
        assert_eq!(<dyn Foo>::NAME, "NSObject");
    }
}
