/// Create a new type to represent a class.
///
/// This is similar to an `@interface` declaration in Objective-C.
///
/// It is useful for things like `objc2-foundation`, which needs to create
/// interfaces to existing, externally defined classes like `NSString`,
/// `NSURL` and so on, but can also be useful for users that have custom
/// classes written in Objective-C that they want to access from Rust.
///
///
/// # Specification
///
/// The syntax is similar enough to Rust syntax that if you invoke the macro
/// with parentheses (as opposed to curly brackets), [`rustfmt` will be able to
/// format the contents][rustfmt-macros] (so e.g. as `extern_class!( ... );`).
///
/// The macro creates an opaque struct containing the superclass (which means
/// that auto traits are inherited from the superclass), and implements the
/// following traits for it to allow easier usage as an Objective-C object:
///
/// - [`RefEncode`][crate::RefEncode]
/// - [`Message`][crate::Message]
/// - [`Deref<Target = $superclass>`][core::ops::Deref]
/// - [`ClassType`][crate::ClassType]
/// - [`DowncastTarget`][$crate::DowncastTarget]
/// - [`AsRef<$inheritance_chain>`][AsRef]
/// - [`Borrow<$inheritance_chain>`][core::borrow::Borrow]
///
/// If generics are specified, these will be placed in a [`PhantomData`].
///
/// [rustfmt-macros]: https://github.com/rust-lang/rustfmt/discussions/5437
/// [`PhantomData`]: core::marker::PhantomData
///
///
/// ## Attributes
///
/// You can add most normal attributes to the class, including `#[cfg(...)]`,
/// `#[allow(...)]` and doc comments.
///
/// Exceptions and special attributes are noted below.
///
///
/// ### `#[unsafe(super(...))]` (required)
///
/// Controls the [superclass][crate::ClassType::Super] and the rest of the
/// inheritance chain. This attribute is required.
///
/// Due to Rust trait limitations, specifying e.g. the superclass `NSData`
/// would not give you the ability to convert via `AsRef` to `NSObject`.
/// Therefore, you can optionally specify additional parts of the inheritance
/// in this attribute.
///
///
/// ### `#[thread_kind = ...]` (optional)
///
/// Controls the [thread kind][crate::ClassType::ThreadKind], i.e. it can be
/// set to [`MainThreadOnly`] if the object is only usable on the main thread.
///
/// [`MainThreadOnly`]: crate::MainThreadOnly
///
///
/// ### `#[name = "..."]` (optional)
///
/// Controls the [name][crate::ClassType::NAME] of the class.
///
/// If not specified, this will default to the struct name.
///
///
/// ### `#[derive(...)]`
///
/// This is overridden, and only works with [`PartialEq`], [`Eq`], [`Hash`]
/// and [`Debug`].
///
/// [`Hash`]: std::hash::Hash
/// [`Debug`]: std::fmt::Debug
///
///
/// ### `#[cfg_attr(..., ...)]`
///
/// This is only supported for attributes that apply to the struct itself
/// (i.e. not supported for attributes that apply to implementations, or any
/// of the custom attributes).
///
///
/// ### `#[repr(...)]`
///
/// Not allowed (the macro uses this attribute internally).
///
///
/// # Safety
///
/// When writing `#[unsafe(super(...))]`, you must ensure that:
/// 1. The first superclass is correct.
/// 2. The thread kind is set to `MainThreadOnly` if the class can only be
///    used from the main thread.
///
///
/// # Examples
///
/// Create a new type to represent the `NSFormatter` class (for demonstration,
/// `objc2_foundation::NSFormatter` exist for exactly this purpose).
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::{NSCoding, NSCopying, NSObjectProtocol};
/// # use objc2::runtime::NSObjectProtocol;
/// use objc2::rc::Retained;
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, extern_conformance, msg_send, ClassType};
///
/// extern_class!(
///     /// An example description, to show that doc comments work.
///     // Specify the superclass, in this case `NSObject`
///     #[unsafe(super(NSObject))]
///     // We could specify that the class is only usable on the main thread.
///     // #[thread_kind = MainThreadOnly];
///     // And specify the name of the class, if it differed from the struct.
///     // #[name = "NSFormatter"];
///     // These derives use the superclass' implementation.
///     #[derive(PartialEq, Eq, Hash, Debug)]
///     pub struct NSFormatter;
/// );
///
/// // Note: We have to specify the protocols for the superclasses as well,
/// // since Rust doesn't do inheritance.
/// extern_conformance!(unsafe impl NSObjectProtocol for NSFormatter {});
/// # #[cfg(not_available)]
/// extern_conformance!(unsafe impl NSCopying for NSFormatter {});
/// # #[cfg(not_available)]
/// extern_conformance!(unsafe impl NSCoding for NSFormatter {});
///
/// fn main() {
///     // Provided by the implementation of `ClassType`
///     let cls = NSFormatter::class();
///
///     // `NSFormatter` implements `Message`:
///     let obj: Retained<NSFormatter> = unsafe { msg_send![cls, new] };
/// }
/// ```
///
/// Represent the `NSDateFormatter` class, using the `NSFormatter` type we
/// declared previously to specify as its superclass.
///
/// ```
/// # #[cfg(not_available)]
/// use objc2_foundation::{NSCoding, NSCopying, NSObjectProtocol};
/// # use objc2::runtime::NSObjectProtocol;
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, extern_conformance, ClassType};
/// #
/// # extern_class!(
/// #     #[unsafe(super(NSObject))]
/// #     #[derive(PartialEq, Eq, Hash, Debug)]
/// #     pub struct NSFormatter;
/// # );
///
/// extern_class!(
///     // Specify the correct inheritance chain
///     #[unsafe(super(NSFormatter, NSObject))]
///     #[derive(PartialEq, Eq, Hash, Debug)]
///     pub struct NSDateFormatter;
/// );
///
/// // Similarly, we can specify the protocols that this implements here:
/// extern_conformance!(unsafe impl NSObjectProtocol for NSDateFormatter {});
/// # #[cfg(not_available)]
/// extern_conformance!(unsafe impl NSCopying for NSDateFormatter {});
/// # #[cfg(not_available)]
/// extern_conformance!(unsafe impl NSCoding for NSDateFormatter {});
/// ```
///
/// See the source code of `objc2-foundation` for many more examples.
#[doc(alias = "@interface")]
#[macro_export]
macro_rules! extern_class {
    (
        // The following special attributes are supported:
        // - #[unsafe(super($($superclasses:path),*))]
        // - #[unsafe(super = $superclass:path)]
        // - #[thread_kind = $thread_kind:path]
        // - #[name = $name:literal]
        //
        // As well as the following standard attributes:
        // - #[derive(Eq, PartialEq, Hash, Debug)] (only those four are supported)
        // - #[cfg(...)]
        // - #[cfg_attr(..., ...)] (only for standard attributes)
        // - #[doc(...)]
        // - #[deprecated(...)]
        // - #[allow/expect/warn/deny/forbid]
        //
        // Note that `#[repr(...)]` and `#[non_exhaustive]` are intentionally not supported.
        $(#[$($attrs:tt)*])*
        $v:vis struct $class:ident;
    ) => {
        $crate::__extract_struct_attributes! {
            ($(#[$($attrs)*])*)

            ($crate::__extern_class_inner)
            ($v)
            ($class)
            () // No generics
        }
    };
    (
        // Generic version. Currently pretty ill supported.
        $(#[$($attrs:tt)*])*
        $v:vis struct $class:ident<
            $($generic:ident $(: $(?$bound_sized:ident)? $($bound:ident)?)? $(= $default:ty)?),*
            $(,)?
        >;
    ) => {
        $crate::__extract_struct_attributes! {
            ($(#[$($attrs)*])*)

            ($crate::__extern_class_inner)
            ($v)
            ($class)
            ($(
                ($generic)
                ($($(?$bound_sized)? $($bound)?)?)
                ($($default)?)
            )*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_inner {
    (
        ($v:vis)
        ($class:ident)
        ($($(
            ($generic:ident)
            ($($($bounds:tt)+)?)
            ($($default:ty)?)
        )+)?)

        ($($safety:tt $superclass:path $(, $superclasses:path)* $(,)?)?)
        ($($($thread_kind:tt)+)?)
        ($($name:tt)*)
        ($($ivars:tt)*)
        ($($derives:tt)*)
        ($($attr_struct:tt)*)
        ($($attr_impl:tt)*)
    ) => {
        // Ensure that the type has the same layout as the superclass.
        // #[repr(transparent)] doesn't work because the superclass is a ZST.
        #[repr(C)]
        $($attr_struct)*
        $v struct $class $(<$($generic $(: $($bounds)+)? $(= $default)?),*>)? {
            __superclass: $crate::__fallback_if_not_set! {
                ($($superclass)?)
                // For diagnostics / rust-analyzer's sake, we choose a default
                // superclass so that we can continue compiling, even though
                // we're going to error if the super class is not set in
                // `__extern_class_check_super_unsafe` below.
                ($crate::runtime::NSObject)
            },
            // Bind generics (and make them invariant).
            $(__generics: $crate::__macro_helpers::PhantomData<($(*mut $generic),+)>,)?
        }

        $crate::__extern_class_impl_traits! {
            ($($attr_impl)*)
            (unsafe impl $(<$($generic: $($($bounds)+ +)? $crate::Message),+>)?)
            ($class $(<$($generic),*>)?)
            ($($superclass, $($superclasses,)*)? $crate::runtime::AnyObject)
        }

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $(<$($generic: $($($bounds)+)?),+>)?)
            ($class $(<$($generic),*>)?)
            ($($derives)*)
        }

        // SAFETY: This maps `SomeClass<T, ...>` to a single `SomeClass<AnyObject, ...>` type and
        // implements `DowncastTarget` on that type. This is safe because the "base container" class
        // is the same and each generic argument is replaced with `AnyObject`, which can represent
        // any Objective-C class instance.
        $($attr_impl)*
        unsafe impl $crate::DowncastTarget for $class $(<$($crate::__extern_class_map_anyobject!($generic)),+>)? {}

        $($attr_impl)*
        unsafe impl $(<$($generic $(: $($bounds)+ + $crate::Message)?),*>)? $crate::ClassType for $class $(<$($generic),*>)? {
            type Super = $crate::__fallback_if_not_set! {
                ($($superclass)?)
                // See __superclass, this is still just for better diagnostics.
                ($crate::runtime::NSObject)
            };

            type ThreadKind = $crate::__fallback_if_not_set! {
                ($(dyn ($($thread_kind)+))?)
                // Default to the super class' thread kind
                (<<Self as $crate::ClassType>::Super as $crate::ClassType>::ThreadKind)
            };

            const NAME: &'static $crate::__macro_helpers::str = $crate::__fallback_if_not_set! {
                ($($name)*)
                ($crate::__macro_helpers::stringify!($class))
            };

            #[inline]
            fn class() -> &'static $crate::runtime::AnyClass {
                let _ = <Self as $crate::__macro_helpers::ValidThreadKind<<Self as $crate::ClassType>::ThreadKind>>::check;
                let _ = <Self as $crate::__macro_helpers::MainThreadOnlyDoesNotImplSendSync<_>>::check;
                let _ = <Self as $crate::__macro_helpers::DoesNotImplDrop<_>>::check;

                $crate::__class_inner!($crate::__fallback_if_not_set! {
                    ($($name)*)
                    ($crate::__macro_helpers::stringify!($class))
                }, $crate::__hash_idents!($class))
            }

            #[inline]
            fn as_super(&self) -> &Self::Super {
                &self.__superclass
            }

            const __INNER: () = ();

            type __SubclassingType = Self;
        }

        $($attr_impl)*
        $crate::__extern_class_check_super_unsafe!($($safety $superclass)?);

        $($attr_impl)*
        $crate::__extern_class_check_no_ivars!($($ivars)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_check_super_unsafe {
    (unsafe $($superclass:tt)+) => {};
    (safe $($superclass:tt)+) => {
        $crate::__macro_helpers::compile_error!(
            "#[super(...)] must be wrapped in `unsafe`, as in #[unsafe(super(...))]"
        );
    };
    () => {
        $crate::__macro_helpers::compile_error!(
            "must specify the superclass with #[unsafe(super(...))]"
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_map_anyobject {
    ($t:ident) => {
        $crate::runtime::AnyObject
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_check_no_ivars {
    () => {};
    ($($ivars:tt)*) => {
        $crate::__macro_helpers::compile_error!("#[ivars] is not supported in extern_class!");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_impl_traits {
    (
        ($($attr_impl:tt)*)
        (unsafe impl $($after_impl:tt)*)
        ($($for:tt)*)
        ($superclass:path $(, $remaining_superclasses:path)*)
    ) => {
        // SAFETY:
        // - The item is FFI-safe with `#[repr(C)]`.
        // - The encoding is taken from the inner item, and caller verifies
        //   that it actually inherits said object.
        // - The rest of the struct's fields are ZSTs, so they don't influence
        //   the layout.
        //
        // Be aware that very rarely, this implementation is wrong because the
        // class' instances do not have the encoding `Encoding::Object`.
        //
        // A known case is that `NSAutoreleasePool` has a different encoding.
        // This should be fairly problem-free though, since that is still
        // valid in Objective-C to represent that class' instances as
        // `NSObject*`.
        $($attr_impl)*
        unsafe impl $($after_impl)* $crate::RefEncode for $($for)* {
            const ENCODING_REF: $crate::Encoding
                = <$superclass as $crate::RefEncode>::ENCODING_REF;
        }

        // SAFETY: This is a newtype wrapper over `AnyObject` (we even ensure
        // that `AnyObject` is always last in our inheritance tree), so it is
        // always safe to reinterpret as that.
        //
        // That the object must work with standard memory management is
        // properly upheld by the fact that the superclass is required by
        // `ValidThreadKind` to implement `ClassType`, and hence must also be
        // a subclass of one of `NSObject`, `NSProxy` or some other class that
        // ensures this (e.g. the object itself is not a root class).
        $($attr_impl)*
        unsafe impl $($after_impl)* $crate::Message for $($for)* {}

        // SAFETY: An instance can always be _used_ in exactly the same way as
        // its superclasses (though not necessarily _constructed_ in the same
        // way, but `Deref` doesn't allow this).
        //
        // Remember; while we (the Rust side) may intentionally be forgetting
        // which instance we're holding, the Objective-C side will remember,
        // and will always dispatch to the correct method implementations.
        //
        // TODO: If the object has a lifetime, we must keep that lifetime
        // information, since all objects can be retained using
        // `Message::retain`, and that could possibly make it unsound to allow
        // non-`'static` here.
        //
        // `&NSMutableArray<T>` -> `&NSArray<T>` -> `Retained<NSArray<T>>` is
        // fine, but `&UserClass<'a>` -> `&NSObject` -> `Retained<NSObject>`
        // is not, and hence `&NSArray<UserClass<'a>>` -> `&NSObject` ->
        // `Retained<NSObject>` isn't either.
        $($attr_impl)*
        impl $($after_impl)* $crate::__macro_helpers::Deref for $($for)* {
            type Target = $superclass;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.__superclass
            }
        }

        $($attr_impl)*
        impl $($after_impl)* $crate::__macro_helpers::AsRef<Self> for $($for)* {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        $crate::__extern_class_impl_as_ref_borrow! {
            ($superclass $(, $remaining_superclasses)*)

            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            fn as_ref(&self) {
                // Triggers Deref coercion depending on return type
                &*self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_impl_as_ref_borrow {
    // Base case
    {
        ()

        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        fn as_ref($($self:tt)*) $as_ref:block
    } => {};

    // For each superclass
    {
        ($superclass:path $(, $remaining_superclasses:path)*)

        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        fn as_ref($($self:tt)*) $as_ref:block
    } => {
        $($attr_impl)*
        impl $($after_impl)* $crate::__macro_helpers::AsRef<$superclass> for $($for)* {
            #[inline]
            fn as_ref($($self)*) -> &$superclass $as_ref
        }

        // Borrow is correct, since subclasses behaves identical to the class
        // they inherit (message sending doesn't care).
        //
        // In particular, `Eq`, `Ord` and `Hash` all give the same results
        // after borrow.

        $($attr_impl)*
        impl $($after_impl)* $crate::__macro_helpers::Borrow<$superclass> for $($for)* {
            #[inline]
            fn borrow($($self)*) -> &$superclass $as_ref
        }

        $crate::__extern_class_impl_as_ref_borrow! {
            ($($remaining_superclasses),*)

            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            fn as_ref($($self)*) $as_ref
        }
    };
}

/// Note: We intentionally don't add e.g. `T: PartialEq`, as generic objects
/// are always comparable, hashable and debuggable, regardless of their
/// generic parameters.
#[doc(hidden)]
#[macro_export]
macro_rules! __extern_class_derives {
    // Base case
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        ($(,)*)
    ) => {};

    // Debug
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        (
            $(,)*
            Debug
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $($after_impl)* $crate::__macro_helpers::fmt::Debug for $($for)* {
            fn fmt(&self, f: &mut $crate::__macro_helpers::fmt::Formatter<'_>) -> $crate::__macro_helpers::fmt::Result {
                // Delegate to the superclass
                $crate::__macro_helpers::fmt::Debug::fmt(&self.__superclass, f)
            }
        }

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            ($($rest)*)
        }
    };

    // PartialEq
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        (
            $(,)*
            PartialEq
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $($after_impl)* $crate::__macro_helpers::PartialEq for $($for)* {
            #[inline]
            fn eq(&self, other: &Self) -> $crate::__macro_helpers::bool {
                // Delegate to the superclass
                $crate::__macro_helpers::PartialEq::eq(&self.__superclass, &other.__superclass)
            }
        }

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            ($($rest)*)
        }
    };

    // Eq
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        (
            $(,)*
            Eq
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $($after_impl)* $crate::__macro_helpers::Eq for $($for)* {}

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            ($($rest)*)
        }
    };

    // Hash
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        (
            $(,)*
            Hash
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $($after_impl)* $crate::__macro_helpers::Hash for $($for)* {
            #[inline]
            fn hash<H: $crate::__macro_helpers::Hasher>(&self, state: &mut H) {
                // Delegate to the superclass
                $crate::__macro_helpers::Hash::hash(&self.__superclass, state)
            }
        }

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            ($($rest)*)
        }
    };

    // Unhandled derive
    (
        ($($attr_impl:tt)*)
        (impl $($after_impl:tt)*)
        ($($for:tt)*)
        (
            $(,)*
            $derive:path
            $(, $($rest:tt)*)?
        )
    ) => {
        const _: () = {
            // For better diagnostics.
            #[derive($derive)]
            struct Derive;
        };
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::stringify!(
            #[derive($derive)] is not supported in extern_class!
        ));

        $crate::__extern_class_derives! {
            ($($attr_impl)*)
            (impl $($after_impl)*)
            ($($for)*)
            ($($($rest)*)?)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __select_name {
    ($_name:ident; $name_const:expr) => {
        $name_const
    };
    ($name:ident;) => {
        $crate::__macro_helpers::stringify!($name)
    };
}
