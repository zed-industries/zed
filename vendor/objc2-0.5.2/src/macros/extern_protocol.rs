/// Create a new trait to represent a protocol.
///
/// This is similar to a `@protocol` declaration in Objective-C.
///
/// See [Protocols - The Objective-C Programming Language][protocols] and
/// [Working with Protocols - Programming with Objective-C][working-with] for
/// general information about protocols in Objective-C.
///
/// This macro will create an `unsafe` trait with methods which all have
/// default implementations, such that an external class that conforms to the
/// protocol can write `unsafe impl MyProtocol for MyClass {}`, and get access
/// to the functionality exposed by the protocol.
///
/// Note that that conforming to a protocol in a custom object requires
/// putting the implementation inside the [`declare_class!`] invocation.
///
/// Objective-C has a smart feature where you can write `id<MyProtocol>`, and
/// then work with the protocol as-if it was an object; this is very similar
/// to `dyn` traits in Rust, and we implement it in a similar way, see
/// [`ProtocolObject`] for details.
///
/// [protocols]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html
/// [working-with]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html
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
/// with `#[method(a:selector:)]` or `#[method_id(a:selector:)]`.
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
/// future when implementing protocols in [`declare_class!`].
///
/// This macro otherwise shares similarities with [`extern_class!`] and
/// [`extern_methods!`].
///
/// [`ProtocolObject<dyn T>`]: crate::runtime::ProtocolObject
/// [`ProtocolType`]: crate::ProtocolType
/// [`declare_class!`]: crate::declare_class
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
///
///
/// # Safety
///
/// The following are required for using the macro itself:
/// - The specified name must be an exisiting Objective-C protocol.
/// - The protocol must actually inherit/conform to the protocols specified
///   as supertraits.
/// - The protocol's methods must be correctly specified.
///
/// While the following are required when implementing the `unsafe` trait for
/// a new type:
/// - The type must represent an object that implements the protocol.
///
///
/// # Examples
///
/// Create a trait to represent the `NSItemProviderWriting` protocol.
///
/// ```
/// use std::ffi::c_void;
/// use objc2::ffi::NSInteger;
/// use objc2::rc::Retained;
/// use objc2::runtime::{NSObject, NSObjectProtocol};
/// use objc2::{extern_protocol, ProtocolType};
///
/// // Assume these were correctly defined, as if they came from
/// // `objc2-foundation`
/// type NSArray<T> = T;
/// type NSString = NSObject;
/// type NSProgress = NSObject;
/// type NSItemProviderRepresentationVisibility = NSInteger;
///
/// extern_protocol!(
///     /// This comment will appear on the trait as expected.
///     pub unsafe trait NSItemProviderWriting: NSObjectProtocol {
///         //                                  ^^^^^^^^^^^^^^^^
///         // This protocol inherits from the `NSObject` protocol
///
///         // This method we mark as `unsafe`, since we aren't using the correct
///         // type for the completion handler
///         #[method_id(loadDataWithTypeIdentifier:forItemProviderCompletionHandler:)]
///         unsafe fn loadData(
///             &self,
///             type_identifier: &NSString,
///             completion_handler: *mut c_void,
///         ) -> Option<Retained<NSProgress>>;
///
///         #[method_id(writableTypeIdentifiersForItemProvider)]
///         fn writableTypeIdentifiersForItemProvider_class()
///             -> Retained<NSArray<NSString>>;
///
///         // The rest of these are optional, which means that a user of
///         // `declare_class!` would not need to implement them.
///
///         #[optional]
///         #[method_id(writableTypeIdentifiersForItemProvider)]
///         fn writableTypeIdentifiersForItemProvider(&self)
///             -> Retained<NSArray<NSString>>;
///
///         #[optional]
///         #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
///         fn itemProviderVisibilityForRepresentation_class(
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///
///         #[optional]
///         #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
///         fn itemProviderVisibilityForRepresentation(
///             &self,
///             type_identifier: &NSString,
///         ) -> NSItemProviderRepresentationVisibility;
///     }
///
///     // SAFETY:
///     // - The name is correct
///     // - The protocol does inherit `NSObject`
///     // - The methods are correctly specified
///     unsafe impl ProtocolType for dyn NSItemProviderWriting {
///         //                       ^^^ - Remember to add this `dyn`.
///
///         // We could have named the trait something else on the Rust-side,
///         // and then used this to keep it correct from Objective-C.
///         //
///         // const NAME: &'static str = "NSItemProviderWriting";
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
        $(#[$m:meta])*
        $v:vis unsafe trait $name:ident $(: $conforms_to:ident $(+ $conforms_to_rest:ident)*)? {
            $($methods:tt)*
        }

        $(#[$impl_m:meta])*
        unsafe impl ProtocolType for dyn $for:ident {
            $(const NAME: &'static str = $name_const:expr;)?
        }
    ) => {
        $(#[$m])*
        $v unsafe trait $name $(: $conforms_to $(+ $conforms_to_rest)*)? {
            $crate::__extern_protocol_rewrite_methods! {
                $($methods)*
            }
        }

        $crate::__inner_extern_protocol!(
            ($(#[$impl_m])*)
            ($name)
            (dyn $for)
            ($crate::__select_name!($name; $($name_const)?))
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_protocol {
    (
        ($(#[$impl_m:meta])*)
        ($name:ident)
        (dyn $for:ident)
        ($name_str:expr)
    ) => {
        $(#[$impl_m])*
        unsafe impl<T> $name for $crate::runtime::ProtocolObject<T>
        where
            T: ?$crate::__macro_helpers::Sized + $name
        {}

        // SAFETY: The specified name is ensured by caller to be a protocol,
        // and is correctly defined.
        $(#[$impl_m])*
        unsafe impl ProtocolType for dyn $for {
            const NAME: &'static $crate::__macro_helpers::str = $name_str;
            const __INNER: () = ();
        }

        // SAFETY: Anything that implements the protocol is valid to convert
        // to `ProtocolObject<dyn [PROTO]>`.
        $(#[$impl_m])*
        unsafe impl<T> $crate::runtime::ImplementedBy<T> for dyn $for
        where
            T: ?$crate::__macro_helpers::Sized + $crate::Message + $name
        {
            const __INNER: () = ();
        }

        // TODO: Should we also implement `ImplementedBy` for `Send + Sync`
        // types, as is done for `NSObjectProtocol`?
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

            ($crate::__extract_custom_attributes)
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

            ($crate::__extract_custom_attributes)
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
    // Instance #[method(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[method($($sel:tt)*)])
        ()
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::Message
            $(, $where : $bound)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                }
            }
        }
    };

    // Instance #[method_id(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($retain_semantics:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::Message
            $(, $where : $bound)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send_id! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                    ($($retain_semantics)*)
                }
            }
        }
    };

    // Class #[method(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_class_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[method($($sel:tt)*)])
        ()
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::ClassType
            $(, $where : $bound)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                }
            }
        }
    };

    // Class #[method_id(...)]
    {
        ($($function_start:tt)*)
        ($($where:ty : $bound:path ,)*)

        (add_class_method)
        ($receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($retain_semantics:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $($m_checked)*
        $($function_start)*
        where
            Self: $crate::__macro_helpers::Sized + $crate::ClassType
            $(, $where : $bound)*
        {
            #[allow(unused_unsafe)]
            unsafe {
                $crate::__method_msg_send_id! {
                    ($receiver)
                    ($($sel)*)
                    ($($params_rest)*)

                    ()
                    ()
                    ($($retain_semantics)*)
                }
            }
        }
    };
}
