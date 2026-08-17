mod __attribute_helpers;
mod __method_msg_send;
mod __msg_send_parse;
mod __rewrite_self_param;
mod declare_class;
mod extern_category;
mod extern_class;
mod extern_methods;
mod extern_protocol;

/// Gets a reference to an [`AnyClass`] from the given name.
///
/// If you have an object that implements [`ClassType`], consider using the
/// [`ClassType::class`] method instead.
///
/// [`AnyClass`]: crate::runtime::AnyClass
/// [`ClassType`]: crate::ClassType
/// [`ClassType::class`]: crate::ClassType::class
///
///
/// # Panics
///
/// Panics if no class with the given name can be found.
///
/// To dynamically check for a class that may not exist, use [`AnyClass::get`].
///
/// [`AnyClass::get`]: crate::runtime::AnyClass::get
///
///
/// # Features
///
/// If the experimental `"unstable-static-class"` feature is enabled, this
/// will emit special statics that will be replaced by dyld when the program
/// starts up.
///
/// Errors that were previously runtime panics may now turn into linker errors
/// if you try to use a class which is not available. Additionally, you may
/// have to call `msg_send![cls, class]` on the result if you want to use it
/// in a dynamic context (e.g. dynamically declaring classes).
///
/// See the [corresponding section][sel#features] in the [`sel!`] macro for
/// more details on the limitations of this. The
/// `"unstable-static-class-inlined"` corresponds to the
/// `"unstable-static-sel-inlined"` feature here.
///
/// [sel#features]: crate::sel#features
/// [`sel!`]: crate::sel
///
///
/// # Examples
///
/// Get and compare the class with one returned from [`ClassType::class`].
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{class, ClassType};
///
/// let cls1 = class!(NSObject);
/// let cls2 = NSObject::class();
/// assert_eq!(cls1, cls2);
/// ```
///
/// Try to get a non-existing class (this will panic, or fail to link).
///
#[cfg_attr(not(feature = "unstable-static-class"), doc = "```should_panic")]
#[cfg_attr(feature = "unstable-static-class", doc = "```ignore")]
/// use objc2::class;
///
/// let _ = class!(NonExistantClass);
/// ```
#[macro_export]
macro_rules! class {
    ($name:ident) => {{
        $crate::__class_inner!(
            $crate::__macro_helpers::stringify!($name),
            $crate::__hash_idents!($name)
        )
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-static-class"))]
macro_rules! __class_inner {
    ($name:expr, $_hash:expr) => {{
        static CACHED_CLASS: $crate::__macro_helpers::CachedClass =
            $crate::__macro_helpers::CachedClass::new();
        #[allow(unused_unsafe)]
        unsafe {
            CACHED_CLASS.get($crate::__macro_helpers::concat!($name, '\0'))
        }
    }};
}

/// Register a selector with the Objective-C runtime.
///
/// Returns the [`Sel`] corresponding to the specified selector.
///
/// [`Sel`]: crate::runtime::Sel
///
///
/// # Panics
///
/// Panics if the runtime failed allocating space for the selector.
///
///
/// # Specification
///
/// This has similar syntax and functionality as the `@selector` directive in
/// Objective-C.
///
/// This calls [`Sel::register`] internally. The result is cached for
/// efficiency. The cache for certain common selectors (`alloc`, `init` and
/// `new`) is deduplicated to reduce code-size.
///
/// Non-ascii identifiers are ill-tested, if supported at all.
///
/// [`Sel::register`]: crate::runtime::Sel::register
///
///
/// # Features
///
/// If the experimental `"unstable-static-sel"` feature is enabled, this will
/// emit special statics that will be replaced by the dynamic linker (dyld)
/// when the program starts up - in exactly the same manner as normal
/// Objective-C code does.
/// This should be significantly faster (and allow better native debugging),
/// however due to the Rust compilation model, and since we don't have
/// low-level control over it, it is currently unlikely that this will work
/// correctly in all cases.
/// See the source code and [rust-lang/rust#53929] for more info.
///
/// Concretely, this may fail at:
/// - link-time (likely)
/// - dynamic link-time/just before the program is run (fairly likely)
/// - runtime, causing UB (unlikely)
///
/// The `"unstable-static-sel-inlined"` feature is the even more extreme
/// version - it yields the best performance and is closest to real
/// Objective-C code, but probably won't work unless your code and its
/// inlining is written in a very certain way.
///
/// Enabling LTO greatly increases the chance that these features work.
///
/// [rust-lang/rust#53929]: https://github.com/rust-lang/rust/issues/53929
///
///
/// # Examples
///
/// Get a few different selectors:
///
/// ```rust
/// use objc2::sel;
/// let sel = sel!(alloc);
/// let sel = sel!(description);
/// let sel = sel!(_privateMethod);
/// let sel = sel!(storyboardWithName:bundle:);
/// let sel = sel!(
///     otherEventWithType:
///     location:
///     modifierFlags:
///     timestamp:
///     windowNumber:
///     context:
///     subtype:
///     data1:
///     data2:
/// );
/// ```
///
/// Whitespace is ignored:
///
/// ```
/// # use objc2::sel;
/// let sel1 = sel!(setObject:forKey:);
/// let sel2 = sel!(  setObject  :
///
///     forKey  : );
/// assert_eq!(sel1, sel2);
/// ```
///
/// Invalid selector:
///
/// ```compile_fail
/// # use objc2::sel;
/// let sel = sel!(aSelector:withoutTrailingColon);
/// ```
///
/// A selector with internal colons:
///
/// ```
/// # use objc2::sel;
/// let sel = sel!(sel::with:::multiple:internal::::colons:::);
///
/// // Yes, that is possible! The following Objective-C would work:
/// //
/// // @interface MyThing: NSObject
/// // + (void)test:(int)a :(int)b arg:(int)c :(int)d;
/// // @end
/// ```
///
/// Unsupported usage that you may run into when using macros - fails to
/// compile when the `"unstable-static-sel"` feature is enabled.
///
/// Instead, define a wrapper function that retrieves the selector.
///
#[cfg_attr(not(feature = "unstable-static-sel"), doc = "```no_run")]
#[cfg_attr(feature = "unstable-static-sel", doc = "```compile_fail")]
/// use objc2::sel;
/// macro_rules! x {
///     ($x:ident) => {
///         // One of these is fine
///         sel!($x);
///         // But using the identifier again in the same way is not!
///         sel!($x);
///     };
/// }
/// // Identifier `abc`
/// x!(abc);
/// ```
#[macro_export]
macro_rules! sel {
    (new) => ({
        $crate::__macro_helpers::new_sel()
    });
    (init) => ({
        $crate::__macro_helpers::init_sel()
    });
    (alloc) => ({
        $crate::__macro_helpers::alloc_sel()
    });
    (dealloc) => ({
        $crate::__macro_helpers::dealloc_sel()
    });
    ($sel:ident) => ({
        $crate::__sel_inner!(
            $crate::__sel_data!($sel),
            $crate::__hash_idents!($sel)
        )
    });
    ($($sel:ident :)*) => ({
        $crate::__sel_inner!(
            $crate::__sel_data!($($sel :)*),
            $crate::__hash_idents!($($sel :)*)
        )
    });
    ($($sel:tt)*) => {
        $crate::__sel_inner!(
            $crate::__sel_helper! {
                ()
                $($sel)*
            },
            $crate::__hash_idents!($($sel)*)
        )
    };
}

/// Handle selectors with internal colons.
///
/// Required since `::` is a different token than `:`.
#[doc(hidden)]
#[macro_export]
macro_rules! __sel_helper {
    // Base-case
    {
        ($($parsed_sel:tt)*)
    } => ({
        $crate::__sel_data!($($parsed_sel)*)
    });
    // Single identifier
    {
        ()
        $ident:ident
    } => {
        $crate::__sel_helper! {
            ($ident)
        }
    };
    // Parse identitifer + colon token
    {
        ($($parsed_sel:tt)*)
        $($ident:ident)? : $($rest:tt)*
    } => {
        $crate::__sel_helper! {
            ($($parsed_sel)* $($ident)? :)
            $($rest)*
        }
    };
    // Parse identitifer + path separator token
    {
        ($($parsed_sel:tt)*)
        $($ident:ident)? :: $($rest:tt)*
    } => {
        $crate::__sel_helper! {
            // Notice space between these
            ($($parsed_sel)* $($ident)? : :)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __sel_data {
    ($first:ident $(: $($($rest:ident)? :)*)?) => {
        $crate::__macro_helpers::concat!(
            $crate::__macro_helpers::stringify!($first),
            $(':', $($($crate::__macro_helpers::stringify!($rest),)? ':',)*)?
            '\0',
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-static-sel"))]
macro_rules! __sel_inner {
    ($data:expr, $_hash:expr) => {{
        static CACHED_SEL: $crate::__macro_helpers::CachedSel =
            $crate::__macro_helpers::CachedSel::new();
        #[allow(unused_unsafe)]
        unsafe {
            CACHED_SEL.get($data)
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __statics_string_to_known_length_bytes {
    ($inp:ident) => {{
        // Convert the `&[u8]` slice to an array with known length, so
        // that we can place that directly in a static.
        let mut res: [$crate::__macro_helpers::u8; $inp.len()] = [0; $inp.len()];
        let mut i = 0;
        while i < $inp.len() {
            res[i] = $inp[i];
            i += 1;
        }
        res
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_image_info {
    ($hash:expr) => {
        /// We always emit the image info tag, since we need it to:
        /// - End up in the same codegen unit as the other statics below.
        /// - End up in the final binary so it can be read by dyld.
        ///
        /// Unfortunately however, this leads to duplicated tags - the linker
        /// reports `__DATA/__objc_imageinfo has unexpectedly large size XXX`,
        /// but things still seems to work.
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__OBJC,__image_info,regular"
        )]
        #[export_name = $crate::__macro_helpers::concat!("\x01L_OBJC_IMAGE_INFO_", $hash)]
        #[used] // Make sure this reaches the linker
        static _IMAGE_INFO: $crate::ffi::__ImageInfo = $crate::ffi::__ImageInfo::system();
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_module_info {
    ($hash:expr) => {
        #[link_section = "__TEXT,__cstring,cstring_literals"]
        #[export_name = $crate::__macro_helpers::concat!("\x01L_OBJC_CLASS_NAME_", $hash, "_MODULE_INFO")]
        static MODULE_INFO_NAME: [$crate::__macro_helpers::u8; 1] = [0];

        /// Emit module info.
        ///
        /// This is similar to image info, and must be present in the final
        /// binary on macOS 32-bit.
        #[link_section = "__OBJC,__module_info,regular,no_dead_strip"]
        #[export_name = $crate::__macro_helpers::concat!("\x01L_OBJC_MODULES_", $hash)]
        #[used] // Make sure this reaches the linker
        static _MODULE_INFO: $crate::__macro_helpers::ModuleInfo =
            $crate::__macro_helpers::ModuleInfo::new(MODULE_INFO_NAME.as_ptr());
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_sel {
    {
        ($data:expr)
        ($hash:expr)
    } => {
        const X: &[$crate::__macro_helpers::u8] = $data.as_bytes();

        /// Clang marks this with LLVM's `unnamed_addr`.
        /// See rust-lang/rust#18297
        /// Should only be an optimization (?)
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            link_section = "__TEXT,__objc_methname,cstring_literals",
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__TEXT,__cstring,cstring_literals",
        )]
        #[export_name = $crate::__macro_helpers::concat!(
            "\x01L_OBJC_METH_VAR_NAME_",
            $hash,
        )]
        static NAME_DATA: [$crate::__macro_helpers::u8; X.len()] = $crate::__statics_string_to_known_length_bytes!(X);

        /// Place the constant value in the correct section.
        ///
        /// We use `UnsafeCell` because this somewhat resembles internal
        /// mutation - this pointer will be changed by dyld at startup, so we
        /// _must_ prevent Rust/LLVM from trying to "peek inside" it and just
        /// use a pointer to `NAME_DATA` directly.
        ///
        /// Clang does this by marking `REF` with LLVM's
        /// `externally_initialized`.
        ///
        /// `static mut` is used so that we don't need to wrap the
        /// `UnsafeCell` in something that implements `Sync`.
        ///
        ///
        /// # Safety
        ///
        /// I'm quite uncertain of how safe this is, since the Rust abstract
        /// machine has no concept of a static that is initialized outside of
        /// it - perhaps it would be better to use `read_volatile` instead of
        /// relying on `UnsafeCell`? Or perhaps `MaybeUninit` would help?
        ///
        /// See the [`ctor`](https://crates.io/crates/ctor) crate for more
        /// info on "life before main".
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            // Clang uses `no_dead_strip` in the link section for some reason,
            // which other tools (notably some LLVM tools) now assume is
            // present, so we have to add it as well.
            link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip",
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__OBJC,__message_refs,literal_pointers,no_dead_strip",
        )]
        #[export_name = $crate::__macro_helpers::concat!("\x01L_OBJC_SELECTOR_REFERENCES_", $hash)]
        static mut REF: $crate::__macro_helpers::UnsafeCell<$crate::runtime::Sel> = unsafe {
            $crate::__macro_helpers::UnsafeCell::new($crate::runtime::Sel::__internal_from_ptr(NAME_DATA.as_ptr().cast()))
        };

        $crate::__statics_image_info!($hash);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(target_vendor = "apple"))]
macro_rules! __statics_sel {
    ($($args:tt)*) => {
        // TODO
        $crate::__macro_helpers::compile_error!(
            "The `\"unstable-static-sel\"` feature is not yet supported on GNUStep!"
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    target_vendor = "apple",
    not(all(target_os = "macos", target_arch = "x86"))
))]
macro_rules! __statics_class {
    {
        ($name:expr)
        ($hash:expr)
    } => {
        extern "C" {
            /// Link to the Objective-C class static.
            ///
            /// This uses the special symbol that static and dynamic linkers
            /// knows about.
            ///
            /// Failure modes:
            /// - Unknown class: Static linker error.
            /// - OS version < Class introduced version: Dynamic linker error
            ///   on program startup.
            /// - Deployment target > Class introduced version: No error,
            ///   though _should_ be a static linker error.
            ///
            /// Ideally, we'd have some way of allowing this to be weakly
            /// linked, and return `Option<&AnyClass>` in that case, but Rust
            /// doesn't have the capability to do so yet!
            /// <https://github.com/rust-lang/rust/issues/29603>
            /// <https://stackoverflow.com/a/16936512>
            /// <http://sealiesoftware.com/blog/archive/2010/4/8/Do-it-yourself_Objective-C_weak_import.html>
            #[link_name = $crate::__macro_helpers::concat!("OBJC_CLASS_$_", $name)]
            static CLASS: $crate::runtime::AnyClass;
        }

        /// SAFETY: Same as `REF` above in `__statics_sel!`.
        #[link_section = "__DATA,__objc_classrefs,regular,no_dead_strip"]
        #[export_name = $crate::__macro_helpers::concat!(
            "\x01L_OBJC_CLASSLIST_REFERENCES_$_",
            $hash,
        )]
        static mut REF: $crate::__macro_helpers::UnsafeCell<&$crate::runtime::AnyClass> = unsafe {
            $crate::__macro_helpers::UnsafeCell::new(&CLASS)
        };

        $crate::__statics_image_info!($hash);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(target_vendor = "apple", all(target_os = "macos", target_arch = "x86")))]
macro_rules! __statics_class {
    {
        ($name:expr)
        ($hash:expr)
    } => {
        const X: &[$crate::__macro_helpers::u8] = $name.as_bytes();

        /// Similar to NAME_DATA above in `__statics_sel!`.
        #[link_section = "__TEXT,__cstring,cstring_literals"]
        #[export_name = $crate::__macro_helpers::concat!(
            "\x01L_OBJC_CLASS_NAME_",
            $hash,
        )]
        static NAME_DATA: [$crate::__macro_helpers::u8; X.len()] = $crate::__statics_string_to_known_length_bytes!(X);

        /// SAFETY: Same as `REF` above in `__statics_sel!`.
        #[link_section = "__OBJC,__cls_refs,literal_pointers,no_dead_strip"]
        #[export_name = $crate::__macro_helpers::concat!(
            "\x01L_OBJC_CLASS_REFERENCES_",
            $hash,
        )]
        static mut REF: $crate::__macro_helpers::UnsafeCell<&$crate::runtime::AnyClass> = unsafe {
            let ptr: *const $crate::runtime::AnyClass = NAME_DATA.as_ptr().cast();
            $crate::__macro_helpers::UnsafeCell::new(&*ptr)
        };

        $crate::__statics_image_info!($hash);
        $crate::__statics_module_info!($hash);
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-sel",
    not(feature = "unstable-static-sel-inlined")
))]
macro_rules! __sel_inner {
    ($data:expr, $hash:expr) => {{
        $crate::__statics_sel! {
            ($data)
            ($hash)
        }

        /// HACK: Wrap the access in a non-generic, `#[inline(never)]`
        /// function to make the compiler group it into the same codegen unit
        /// as the statics.
        ///
        /// See the following link for details on how the compiler decides
        /// to partition code into codegen units:
        /// <https://doc.rust-lang.org/1.61.0/nightly-rustc/rustc_monomorphize/partitioning/index.html>
        #[inline(never)]
        fn objc_static_workaround() -> $crate::runtime::Sel {
            // SAFETY: The actual selector is replaced by dyld when the
            // program is loaded.
            //
            // This is similar to a volatile read, except it can be stripped
            // if unused.
            unsafe { *REF.get() }
        }

        objc_static_workaround()
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "unstable-static-sel-inlined")]
macro_rules! __sel_inner {
    ($data:expr, $hash:expr) => {{
        $crate::__statics_sel! {
            ($data)
            ($hash)
        }

        #[allow(unused_unsafe)]
        // SAFETY: See above
        unsafe {
            *REF.get()
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-class",
    not(feature = "gnustep-1-7"),
    not(feature = "unstable-static-class-inlined")
))]
macro_rules! __class_inner {
    ($name:expr, $hash:expr) => {{
        $crate::__statics_class! {
            ($name)
            ($hash)
        }

        #[inline(never)]
        fn objc_static_workaround() -> &'static $crate::runtime::AnyClass {
            // SAFETY: Same as __sel_inner
            unsafe { *REF.get() }
        }

        objc_static_workaround()
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-class",
    not(feature = "gnustep-1-7"),
    feature = "unstable-static-class-inlined"
))]
macro_rules! __class_inner {
    ($name:expr, $hash:expr) => {{
        $crate::__statics_class! {
            ($name)
            ($hash)
        }

        #[allow(unused_unsafe)]
        // SAFETY: See above
        unsafe {
            *REF.get()
        }
    }};
}

#[doc(hidden)]
#[macro_export]
// The linking changed in libobjc2 v2.0
#[cfg(all(feature = "gnustep-1-7", not(feature = "gnustep-2-0"),))]
macro_rules! __class_static_name {
    ($name:expr) => {
        $crate::__macro_helpers::concat!("_OBJC_CLASS_", $name)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "gnustep-2-0")]
macro_rules! __class_static_name {
    ($name:expr) => {
        $crate::__macro_helpers::concat!("._OBJC_CLASS_", $name)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(feature = "unstable-static-class", feature = "gnustep-1-7"))]
macro_rules! __class_inner {
    ($name:expr, $_hash:expr) => {{
        // NOTE: This is not verified for correctness in any way whatsoever.
        extern "C" {
            #[link_name = $crate::__class_static_name!($name)]
            static CLASS: $crate::runtime::AnyClass;

            // Others:
            // __objc_class_name_
            // _OBJC_CLASS_REF_
        }

        #[allow(unused_unsafe)]
        unsafe {
            $crate::__macro_helpers::disallow_in_static(&CLASS)
        }
    }};
}

/// Send a message to an object or class.
///
/// This is wildly `unsafe`, even more so than sending messages in
/// Objective-C, because this macro can't inspect header files to see the
/// expected types, and because Rust has more safety invariants to uphold.
/// Make sure to review the safety section below!
///
/// The recommended way of using this macro is by defining a wrapper function:
///
/// ```
/// # use std::os::raw::{c_int, c_char};
/// # use objc2::msg_send;
/// # use objc2::runtime::NSObject;
/// unsafe fn do_something(obj: &NSObject, arg: c_int) -> *const c_char {
///     msg_send![obj, doSomething: arg]
/// }
/// ```
///
/// This way we are clearly communicating to Rust that: The method
/// `doSomething:` works with a shared reference to the object. It takes a
/// C-style signed integer, and returns a pointer to what is probably a
/// C-compatible string. Now it's much, _much_ easier to make a safe
/// abstraction around this!
///
/// There exists a variant of this macro, [`msg_send_id!`], which can help
/// with upholding certain requirements of methods that return Objective-C's
/// `id`, or other object pointers. Use that whenever you want to call such a
/// method!
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Specification
///
/// The syntax is somewhat similar to the message syntax in Objective-C,
/// except with a comma between arguments. Eliding the comma is possible,
/// but it is soft-deprecated, and will be fully deprecated in a future
/// release. The deprecation can be tried out with the
/// `"unstable-msg-send-always-comma"` feature flag.
///
/// The first expression, know as the "receiver", can be any type that
/// implements [`MessageReceiver`], like a reference or a pointer to an
/// object. Additionally, it can even be a reference to an [`rc::Retained`]
/// containing an object.
///
/// The expression can be wrapped in `super`, with an optional superclass
/// as the second argument. If no specific superclass is specified, the
/// direct superclass is retrieved from [`ClassType`].
///
/// All arguments, as well as the return type, must implement [`Encode`] (bar
/// the exceptions below).
///
/// If the last argument is the special marker `_`, the macro will return a
/// `Result<(), Retained<E>>`, see below.
///
/// This macro roughly translates into a call to [`sel!`], and afterwards a
/// fully qualified call to [`MessageReceiver::send_message`]. Note that this
/// means that auto-dereferencing of the receiver is not supported, and that
/// the receiver is consumed. You may encounter a little trouble with `&mut`
/// references, try refactoring into a separate method or reborrowing the
/// reference.
///
/// Variadic arguments are currently not supported.
///
/// [`MessageReceiver`]: crate::runtime::MessageReceiver
/// [`rc::Retained`]: crate::rc::Retained
/// [`ClassType`]: crate::ClassType
/// [`Encode`]: crate::Encode
/// [`sel!`]: crate::sel
/// [`MessageReceiver::send_message`]: crate::runtime::MessageReceiver::send_message
///
///
/// # `bool` handling
///
/// Objective-C's `BOOL` is slightly different from Rust's [`bool`], and hence
/// a conversion step must be performed before using it. This is _very_ easy
/// to forget (because it'll happen to work in _most_ cases), so this macro
/// does the conversion step automatically whenever an argument or the return
/// type is `bool`.
///
/// That means that any Objective-C method that take or return `BOOL` can be
/// translated to use `bool` on the Rust side.
///
/// If you want to handle the conversion explicitly, or the Objective-C method
/// expects e.g. a pointer to a `BOOL`, use [`runtime::Bool`] instead.
///
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// # Out-parameters
///
/// Parameters like `NSString**` in Objective-C are passed by "writeback",
/// which means that the callee autoreleases any value that they may write
/// into the parameter.
///
/// This macro has support for passing such parameters using the following
/// types:
/// - `&mut Retained<_>`
/// - `Option<&mut Retained<_>>`
/// - `&mut Option<Retained<_>>`,
/// - `Option<&mut Option<Retained<_>>>`
///
/// Beware with the first two, since they will cause undefined behaviour if
/// the method overwrites the value with `nil`.
///
/// See [clang's documentation][clang-out-params] for more details.
///
/// [clang-out-params]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#passing-to-an-out-parameter-by-writeback
///
///
/// # Errors
///
/// The most common place you'll see out-parameters is as `NSError**` the last
/// parameter, which is used to communicate errors to the caller, see [Error
/// Handling Programming Guide For Cocoa][cocoa-error].
///
/// Similar to Swift's [importing of error parameters][swift-error], this
/// macro supports an even more convenient version than the out-parameter
/// support, which transforms methods whose last parameter is `NSError**` and
/// returns `BOOL`, into the Rust equivalent, the [`Result`] type.
///
/// In particular, if you make the last argument the special marker `_`, then
/// the macro will return a `Result<(), Retained<E>>` (where you must specify
/// `E` yourself, usually you'd use `objc2_foundation::NSError`).
///
/// At runtime, we create the temporary error variable for you on the stack
/// and send it as the out-parameter to the method. If the method then returns
/// `NO`/`false` (or in the case of `msg_send_id!`, `NULL`), the error
/// variable is loaded and returned in [`Err`].
///
/// Do beware that this is only valid on methods that return `BOOL`, see
/// [`msg_send_id!`] for methods that return instance types.
///
/// [cocoa-error]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ErrorHandlingCocoa/ErrorHandling/ErrorHandling.html
/// [swift-error]: https://developer.apple.com/documentation/swift/about-imported-cocoa-error-parameters
///
///
/// # Panics
///
/// Panics if the `"catch-all"` feature is enabled and the Objective-C method
/// throws an exception. Exceptions may still cause UB until
/// `extern "C-unwind"` is stable, see [RFC-2945].
///
/// Panics if `debug_assertions` are enabled and the Objective-C method's
/// encoding does not match the encoding of the given arguments and return.
///
/// And panics if the `NSError**` handling functionality described above is
/// used, and the error object was unexpectedly `NULL`.
///
/// [RFC-2945]: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
///
///
/// # Safety
///
/// Similar to defining and calling an `extern` function in a foreign function
/// interface. In particular, you must uphold the following requirements:
///
/// 1. The selector corresponds to a valid method that is available on the
///    receiver.
///
/// 2. The argument types match what the receiver excepts for this selector.
///
/// 3. The return type match what the receiver returns for this selector.
///
/// 4. The call must not violate Rust's mutability rules, for example if
///    passing an `&T`, the Objective-C method must not mutate the variable
///    (except if the variable is inside [`std::cell::UnsafeCell`] or
///    derivatives).
///
/// 5. If the receiver is a raw pointer it must be valid (aligned,
///    dereferenceable, initialized and so on). Messages to `null` pointers
///    are allowed (though heavily discouraged), but _only_ if the return type
///    itself is a pointer.
///
/// 6. The method must not (yet) throw an exception.
///
/// 7. You must uphold any additional safety requirements (explicit and
///    implicit) that the method has. For example:
///    - Methods that take pointers usually require that the pointer is valid,
///      and sometimes non-null.
///    - Sometimes, a method may only be called on the main thread.
///    - The lifetime of returned pointers usually follows certain rules, and
///      may not be valid outside of an [`autoreleasepool`] ([`msg_send_id!`]
///      can greatly help with that).
///
/// 8. Each out-parameter must have the correct nullability, and the method
///    must not have any attributes that changes the how it handles memory
///    management for these.
///
/// 9. TODO: Maybe more?
///
/// [`autoreleasepool`]: crate::rc::autoreleasepool
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Examples
///
/// Sending messages to an object.
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::runtime::NSObject;
///
/// let obj: *mut NSObject;
/// # obj = 0 as *mut NSObject;
/// let description: *const NSObject = unsafe { msg_send![obj, description] };
/// // Usually you'd use msg_send_id here ^
/// let _: () = unsafe { msg_send![obj, setArg1: 1i32, arg2: true] };
/// let arg1: i32 = unsafe { msg_send![obj, getArg1] };
/// let arg2: bool = unsafe { msg_send![obj, getArg2] };
/// ```
///
/// Sending messages to the direct superclass of an object.
///
/// ```no_run
/// use objc2::msg_send;
/// #
/// # use objc2::runtime::NSObject;
/// # use objc2::{declare_class, mutability, ClassType, DeclaredClass};
/// #
/// # declare_class!(
/// #     struct MyObject;
/// #
/// #     unsafe impl ClassType for MyObject {
/// #         type Super = NSObject;
/// #         type Mutability = mutability::InteriorMutable;
/// #         const NAME: &'static str = "MyObject";
/// #     }
/// #
/// #     impl DeclaredClass for MyObject {}
/// # );
///
/// let obj: &MyObject; // Some object that implements ClassType
/// # obj = todo!();
/// let _: () = unsafe { msg_send![super(obj), someMethod] };
/// ```
///
/// Sending messages to a specific superclass of an object.
///
/// ```no_run
/// # use objc2::class;
/// use objc2::msg_send;
/// use objc2::runtime::{AnyClass, NSObject};
///
/// // Since we specify the superclass ourselves, this doesn't need to
/// // implement ClassType
/// let obj: *mut NSObject;
/// # obj = 0 as *mut NSObject;
/// let superclass: &AnyClass;
/// # superclass = class!(NSObject);
/// let arg3: u32 = unsafe { msg_send![super(obj, superclass), getArg3] };
/// ```
///
/// Sending a message with automatic error handling.
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::rc::Retained;
///
/// # type NSBundle = objc2::runtime::NSObject;
/// # type NSError = objc2::runtime::NSObject;
/// let obj: &NSBundle;
/// # obj = todo!();
/// // The `_` tells the macro that the return type should be `Result`.
/// let res: Result<(), Retained<NSError>> = unsafe {
///     msg_send![obj, preflightAndReturnError: _]
/// };
/// ```
///
/// Sending a message with an out parameter _and_ automatic error handling.
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::rc::Retained;
///
/// # type NSFileManager = objc2::runtime::NSObject;
/// # type NSURL = objc2::runtime::NSObject;
/// # type NSError = objc2::runtime::NSObject;
/// let obj: &NSFileManager;
/// # obj = todo!();
/// let url: &NSURL;
/// # url = todo!();
/// let mut result_url: Option<Retained<NSURL>> = None;
/// unsafe {
///     msg_send![
///         obj,
///         trashItemAtURL: url,
///         resultingItemURL: Some(&mut result_url),
///         error: _
///     ]?
/// //   ^ is possible on error-returning methods, if the return type is specified
/// };
///
/// // Use `result_url` here
///
/// # Ok::<(), Retained<NSError>>(())
/// ```
#[macro_export]
macro_rules! msg_send {
    [super($obj:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_super_message_static_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_super_message_static)

            ($crate::__msg_send_helper)
            ($obj)
        }
    };
    [super($obj:expr, $superclass:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_super_message_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_super_message)

            ($crate::__msg_send_helper)
            ($obj, $superclass)
        }
    };
    [$obj:expr, $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_message_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_message)

            ($crate::__msg_send_helper)
            ($obj)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_helper {
    {
        ($($fn_args:tt)+)
        ($fn:ident)
        ($($selector:tt)*)
        ($($argument:expr,)*)
    } => ({
        // Assign to intermediary variable for better UI, and to prevent
        // miscompilation on older Rust versions.
        //
        // Note: This can be accessed from any expression in `fn_args` and
        // `arguments` - we won't (yet) bother with preventing that though.
        let result;
        // Always add trailing comma after each argument, so that we get a
        // 1-tuple if there is only one.
        //
        // And use `::<_, _>` for better UI
        result = $crate::__macro_helpers::MsgSend::$fn::<_, _>($($fn_args)+, $crate::sel!($($selector)*), ($($argument,)*));
        result
    });
}

/// Deprecated. Use [`msg_send!`] instead.
#[macro_export]
#[deprecated = "use a normal msg_send! instead, it will perform the conversion for you"]
macro_rules! msg_send_bool {
    [$($msg_send_args:tt)+] => ({
        // Use old impl for backwards compat
        let result: $crate::runtime::Bool = $crate::msg_send![$($msg_send_args)+];
        result.as_bool()
    });
}

/// [`msg_send!`] for methods returning `id`, `NSObject*`, or similar object
/// pointers.
///
/// Object pointers in Objective-C have certain rules for when they should be
/// retained and released across function calls. This macro helps doing that,
/// and returns an [`rc::Retained`] with the object, optionally wrapped in an
/// [`Option`] if you want to handle failures yourself.
///
/// [`rc::Retained`]: crate::rc::Retained
///
///
/// # A little history
///
/// Objective-C's type system is... limited, so you can't tell without
/// consulting the documentation who is responsible for releasing an object.
/// To remedy this problem, Apple/Cocoa introduced (approximately) the
/// following rule:
///
/// The caller is responsible for releasing objects return from methods that
/// begin with `new`, `alloc`, `copy`, `mutableCopy` or `init`, and method
/// that begins with `init` takes ownership of the receiver. See [Cocoa's
/// Memory Management Policy][mmRules] for a user-friendly introduction to
/// this concept.
///
/// In the past, users had to do `retain` and `release` calls themselves to
/// properly follow these rules. To avoid the memory management problems
/// associated with manual stuff like that, they [introduced "ARC"][arc-rel],
/// which codifies the rules as part of the language, and inserts the required
/// `retain` and `release` calls automatically.
///
/// [`msg_send!`] is similar to pre-ARC; you have to know when to retain and
/// when to release an object. [`msg_send_id!`] is similar to ARC; the rules
/// are simple enough that we can do them automatically!
///
/// [mmRules]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW1
/// [arc-rel]: https://developer.apple.com/library/archive/releasenotes/ObjectiveC/RN-TransitioningToARC/Introduction/Introduction.html#//apple_ref/doc/uid/TP40011226
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Specification
///
/// The syntax is the same as in [`msg_send!`].
///
/// Attributes like `objc_method_family`, `ns_returns_retained`, `ns_consumed`
/// and so on must not present on the method - if they are, you should do
/// manual memory management using the [`msg_send!`] macro instead.
///
/// The accepted receiver and return types, and how we handle them, differ
/// depending on which, if any, of the [recognized selector
/// families][sel-families] the selector belongs to:
///
/// - The `new` family: The receiver may be anything that implements
///   [`MessageReceiver`] (though often you'll want to use `&AnyClass`). The
///   return type is a generic `Retained<T>` or `Option<Retained<T>>`.
///
/// - The `alloc` family: The receiver must be `&AnyClass`, and the return
///   type is a generic `Allocated<T>`.
///
/// - The `init` family: The receiver must be `Allocated<T>` as returned from
///   `alloc`, or if sending messages to the superclass, it must be
///   `PartialInit<T>`.
///
///   The receiver is consumed, and a the now-initialized `Retained<T>` or
///   `Option<Retained<T>>` (with the same `T`) is returned.
///
/// - The `copy` family: The receiver may be anything that implements
///   [`MessageReceiver`] and the return type is a generic `Retained<T>` or
///   `Option<Retained<T>>`.
///
/// - The `mutableCopy` family: Same as the `copy` family.
///
/// - No family: The receiver may be anything that implements
///   [`MessageReceiver`]. The result is retained using
///   [`Retained::retain_autoreleased`], and a generic `Retained<T>` or
///   `Option<Retained<T>>` is returned. This retain is in most cases faster
///   than using autorelease pools!
///
/// See [the clang documentation][arc-retainable] for the precise
/// specification of Objective-C's ownership rules.
///
/// As you may have noticed, the return type is usually either `Retained` or
/// `Option<Retained>`. Internally, the return type is always
/// `Option<Retained>` (for example: almost all `new` methods can fail if the
/// allocation failed), but for convenience, if the return type is
/// `Retained<T>`, this macro will automatically unwrap the object, or panic
/// with an error message if it couldn't be retrieved.
///
/// As a special case, if the last argument is the marker `_`, the macro will
/// return a `Result<Retained<T>, Retained<E>>`, see below.
///
/// The `retain`, `release` and `autorelease` selectors are not supported, use
/// [`Retained::retain`], [`Retained::drop`] and [`Retained::autorelease`] for
/// that.
///
/// [sel-families]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families
/// [`MessageReceiver`]: crate::runtime::MessageReceiver
/// [`Retained::retain_autoreleased`]: crate::rc::Retained::retain_autoreleased
/// [arc-retainable]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments
/// [`Retained::retain`]: crate::rc::Retained::retain
/// [`Retained::drop`]: crate::rc::Retained::drop
/// [`Retained::autorelease`]: crate::rc::Retained::autorelease
///
///
/// # Errors
///
/// Very similarly to [`msg_send!`], this macro supports transforming the
/// return type of methods whose last parameter is `NSError**` into the Rust
/// equivalent, the [`Result`] type.
///
/// In particular, you can make the last argument the special marker `_`, and
/// then the macro will return a `Result<Retained<T>, Retained<E>>` (where you
/// must specify `E` yourself, usually you'd use `objc2_foundation::NSError`).
///
///
/// # Panics
///
/// Panics if the return type is specified as `Retained<_, _>` and the method
/// returned NULL.
///
/// Additional panicking cases are documented in [`msg_send!`].
///
///
/// # Safety
///
/// Same as [`msg_send!`], with an expected return type of `id`,
/// `instancetype`, `NSObject*`, or other such object pointers. The method
/// must not have any attributes that changes the how it handles memory
/// management.
///
/// Note that if you're using this inside a context that expects unwinding to
/// have Objective-C semantics (like [`exception::catch`]), you should make
/// sure that the return type is `Option<Retained<_, _>>` so that you don't
/// get an unexpected unwind through incompatible ABIs!
///
#[cfg_attr(
    feature = "exception",
    doc = "[`exception::catch`]: crate::exception::catch"
)]
#[cfg_attr(
    not(feature = "exception"),
    doc = "[`exception::catch`]: crate::exception#feature-not-enabled"
)]
///
///
/// # Examples
///
/// ```no_run
/// use objc2::{class, msg_send_id};
/// use objc2::ffi::NSUInteger;
/// use objc2::rc::Retained;
/// use objc2::runtime::NSObject;
/// // Allocate new object
/// let obj = unsafe { msg_send_id![class!(NSObject), alloc] };
/// // Consume the allocated object, return initialized object
/// let obj: Retained<NSObject> = unsafe { msg_send_id![obj, init] };
/// // Copy the object
/// let copy: Retained<NSObject> = unsafe { msg_send_id![&obj, copy] };
/// // Call ordinary selector that returns an object
/// // This time, we handle failures ourselves
/// let s: Option<Retained<NSObject>> = unsafe { msg_send_id![&obj, description] };
/// let s = s.expect("description was NULL");
/// ```
#[macro_export]
macro_rules! msg_send_id {
    [super($obj:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_super_message_id_static_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_super_message_id_static)

            ($crate::__msg_send_id_helper)
            ($obj)
            () // No retain semantics
            (MsgSendSuperId)
        }
    };
    [super($obj:expr, $superclass:expr), $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_super_message_id_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_super_message_id)

            ($crate::__msg_send_id_helper)
            ($obj, $superclass)
            () // No retain semantics
            (MsgSendSuperId)
        }
    };
    [$obj:expr, new $(,)?] => ({
        let result;
        result = <$crate::__macro_helpers::New as $crate::__macro_helpers::MsgSendId<_, _>>::send_message_id(
            $obj,
            $crate::sel!(new),
            (),
        );
        result
    });
    [$obj:expr, alloc $(,)?] => ({
        let result;
        result = $crate::__macro_helpers::Alloc::send_message_id_alloc($obj);
        result
    });
    [$obj:expr, init $(,)?] => ({
        let result;
        result = <$crate::__macro_helpers::Init as $crate::__macro_helpers::MsgSendId<_, _>>::send_message_id(
            $obj,
            $crate::sel!(init),
            (),
        );
        result
    });
    [$obj:expr, $($selector_and_arguments:tt)+] => {
        $crate::__msg_send_parse! {
            (send_message_id_error)
            ()
            ()
            ($($selector_and_arguments)+)
            (send_message_id)

            ($crate::__msg_send_id_helper)
            ($obj)
            () // No retain semantics
            (MsgSendId)
        }
    };
}

/// Helper macro to avoid exposing these in the docs for [`msg_send_id!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_id_helper {
    {
        ($($fn_args:tt)+)
        ($($retain_semantics:ident)?)
        ($trait:ident)
        ($fn:ident)
        (retain)
        ()
    } => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, retain] is not supported. Use `Retained::retain` instead"
        )
    }};
    {
        ($($fn_args:tt)+)
        ($($retain_semantics:ident)?)
        ($trait:ident)
        ($fn:ident)
        (release)
        ()
    } => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, release] is not supported. Drop an `Retained` instead"
        )
    }};
    {
        ($($fn_args:tt)+)
        ($($retain_semantics:ident)?)
        ($trait:ident)
        ($fn:ident)
        (autorelease)
        ()
    } => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, autorelease] is not supported. Use `Retained::autorelease`"
        )
    }};
    {
        ($($fn_args:tt)+)
        ($($retain_semantics:ident)?)
        ($trait:ident)
        ($fn:ident)
        (dealloc)
        ()
    } => {{
        $crate::__macro_helpers::compile_error!(
            "msg_send_id![obj, dealloc] is not supported. Drop an `Retained` instead"
        )
    }};
    {
        ($($fn_args:tt)+)
        ($retain_semantics:ident)
        ($trait:ident)
        ($fn:ident)
        ($($selector:tt)*)
        ($($argument:expr,)*)
    } => ({
        <$crate::__macro_helpers::$retain_semantics as $crate::__macro_helpers::$trait<_, _>>::$fn(
            $($fn_args)+,
            $crate::sel!($($selector)*),
            ($($argument,)*),
        )
    });
    {
        ($($fn_args:tt)+)
        ()
        ($trait:ident)
        ($fn:ident)
        ($($selector:tt)*)
        ($($argument:expr,)*)
    } => ({
        // Don't use `sel!`, otherwise we'd end up with defining this data twice.
        const __SELECTOR_DATA: &$crate::__macro_helpers::str = $crate::__sel_data!(
            $($selector)*
        );
        let result;
        result = <$crate::__macro_helpers::RetainSemantics<{
            $crate::__macro_helpers::retain_semantics(__SELECTOR_DATA)
        }> as $crate::__macro_helpers::$trait<_, _>>::$fn(
            $($fn_args)+,
            $crate::__sel_inner!(
                __SELECTOR_DATA,
                $crate::__hash_idents!($($selector)*)
            ),
            ($($argument,)*),
        );
        result
    });
}
