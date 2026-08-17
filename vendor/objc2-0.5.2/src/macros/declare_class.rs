/// Declare a new class.
///
/// This is useful in many cases since Objective-C frameworks tend to favour a
/// design pattern using "delegates", where to hook into a piece of
/// functionality in a class, you implement that class' delegate protocol in
/// a custom class.
///
/// This macro is the declarative way of creating classes, in contrast with
/// the [`declare`] module which mostly contain ways of declaring classes in
/// an imperative fashion. It is highly recommended that you use this macro
/// though, since it contains a lot of extra debug assertions and niceties
/// that help ensure the soundness of your code.
///
/// [`declare`]: crate::declare
///
///
/// # Specification
///
/// This macro consists of the following parts (the first three are required):
/// - The type declaration.
/// - The [`ClassType`] implementation.
/// - The [`DeclaredClass`] implementation.
/// - Any number of inherent implementations.
/// - Any number of protocol implementations.
///
/// With the syntax generally resembling a combination of that in
/// [`extern_class!`] and [`extern_methods!`].
///
/// [`ClassType`]: crate::ClassType
/// [`DeclaredClass`]: crate::DeclaredClass
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
///
///
/// ## Type declaration
///
/// The type declaration works a lot like in [`extern_class!`], an opaque
/// struct is created and a lot of traits is implemented for that struct.
///
/// You are allowed to add most common attributes to the declaration,
/// including `#[cfg(...)]` and doc comments. ABI-modifying attributes like
/// `#[repr(...)]` are not allowed.
///
/// `#[derive(...)]` attributes are allowed, but heavily discouraged, as they
/// are likely to not work as you'd expect them to. This is being worked on in
/// [#267].
///
/// If the type implements [`Drop`], the macro will generate a `dealloc`
/// method for you, which will call `drop` automatically.
///
/// [#267]: https://github.com/madsmtm/objc2/issues/267
///
///
/// ## `ClassType` implementation
///
/// This also resembles the syntax in [`extern_class!`], except that
/// [`ClassType::NAME`] must be specified, and it must be unique across the
/// entire application.
///
/// If you're developing a library, good practice here would be to include
/// your crate name in the prefix (something like `"MyLibrary_MyClass"`).
///
/// The class is guaranteed to have been created and registered with the
/// Objective-C runtime after the [`ClassType::class`] function has been
/// called.
///
/// [`ClassType::NAME`]: crate::ClassType::NAME
/// [`ClassType::class`]: crate::ClassType::class
///
///
/// ## `DeclaredClass` implementation
///
/// The syntax here is as if you were implementing the trait yourself.
///
/// You may optionally specify the associated type [`Ivars`]; this is the
/// intended way to specify the data your class stores. If you don't specify
/// any ivars, the macro will default to [`()`][unit].
///
/// Beware that if you want to use the class' inherited initializers (such as
/// `init`), you must override the subclass' designated initializers, and
/// initialize your ivars properly in there.
///
/// [`Ivars`]: crate::DeclaredClass::Ivars
///
///
/// ## Inherent method definitions
///
/// Within the `impl` block you can define two types of functions;
/// ["associated functions"] and ["methods"]. These are then mapped to the
/// Objective-C equivalents "class methods" and "instance methods". In
/// particular, if you use `self` or the special name `this` (or `_this`),
/// your method will be registered as an instance method, and if you don't it
/// will be registered as a class method.
///
/// On instance methods, you can freely choose between different types of
/// receivers, e.g. `&self`, `this: *const Self`, `&mut self`, and so on. Note
/// though that using `&mut self` requires the class' mutability to be
/// [`IsAllowedMutable`].
/// If you need mutation of your class' instance variables, consider using
/// [`Cell`] or similar instead.
///
/// The desired selector can be specified using the `#[method(my:selector:)]`
/// or `#[method_id(my:selector:)]` attribute, similar to the
/// [`extern_methods!`] macro.
///
/// If the `#[method_id(...)]` attribute is used, the return type must be
/// `Option<Retained<T>>` or `Retained<T>`. Additionally, if the selector is
/// in the "init"-family, the `self`/`this` parameter must be
/// `Allocated<Self>`.
///
/// Putting other attributes on the method such as `cfg`, `allow`, `doc`,
/// `deprecated` and so on is supported. However, note that `cfg_attr` may not
/// work correctly, due to implementation difficulty - if you have a concrete
/// use-case, please [open an issue], then we can discuss it.
///
/// A transformation step is performed on the functions (to make them have the
/// correct ABI) and hence they shouldn't really be called manually. (You
/// can't mark them as `pub` for the same reason). Instead, use the
/// [`extern_methods!`] macro to create a Rust interface to the methods.
///
/// If the parameter or return type is [`bool`], a conversion is performed to
/// make it behave similarly to the Objective-C `BOOL`. Use [`runtime::Bool`]
/// if you want to control this manually.
///
/// Note that `&mut Retained<_>` and other such out parameters are not yet
/// supported, and may generate a panic at runtime.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// [`IsAllowedMutable`]: crate::mutability::IsAllowedMutable
/// [`Cell`]: core::cell::Cell
/// [open an issue]: https://github.com/madsmtm/objc2/issues/new
/// [`msg_send!`]: crate::msg_send
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// ## Protocol implementations
///
/// You can specify protocols that the class should implement, along with any
/// required/optional methods for said protocols.
///
/// The protocol must have been previously defined with [`extern_protocol!`].
///
/// The methods work exactly as normal, they're only put "under" the protocol
/// definition to make things easier to read.
///
/// Putting attributes on the `impl` item such as `cfg`, `allow`, `doc`,
/// `deprecated` and so on is supported.
///
/// [`extern_protocol!`]: crate::extern_protocol
///
///
/// # Panics
///
/// The implemented `ClassType::class` method may panic in a few cases, such
/// as if:
/// - A class with the specified name already exists.
/// - Debug assertions are enabled, and an overriden method's signature is not
///   equal to the one on the superclass.
/// - Debug assertions are enabled, and the protocol's required methods are not
///   implemented.
///
/// And possibly more similar cases in the future.
///
///
/// # Safety
///
/// Using this macro requires writing a few `unsafe` markers:
///
/// `unsafe impl ClassType for T` has the following safety requirements:
/// - Any invariants that the superclass [`ClassType::Super`] may have must be
///   upheld.
/// - [`ClassType::Mutability`] must be correct.
/// - If your type implements `Drop`, the implementation must abide by the
///   following rules:
///   - It must not call any overridden methods.
///   - It must not `retain` the object past the lifetime of the drop.
///   - It must not `retain` in the same scope that `&mut self` is active.
///   - TODO: And probably a few more. [Open an issue] if you would like
///     guidance on whether your implementation is correct.
///
/// `unsafe impl T { ... }` asserts that the types match those that are
/// expected when the method is invoked from Objective-C. Note that unlike
/// with [`extern_methods!`], there are no safe-guards here; you can write
/// `i8`, but if Objective-C thinks it's an `u32`, it will cause UB when
/// called!
///
/// `unsafe impl P for T { ... }` requires that all required methods of the
/// specified protocol is implemented, and that any extra requirements
/// (implicit or explicit) that the protocol has are upheld. The methods in
/// this definition has the same safety requirements as above.
///
/// [`ClassType::Super`]: crate::ClassType::Super
/// [`ClassType::Mutability`]: crate::ClassType::Mutability
/// [Open an issue]: https://github.com/madsmtm/objc2/issues/new
///
///
/// # Examples
///
/// Declare a class `MyCustomObject` that inherits `NSObject`, has a few
/// instance variables and methods, and implements the `NSCopying` protocol.
///
/// ```
/// use std::os::raw::c_int;
///
/// # use objc2::runtime::{NSObject, NSObjectProtocol, NSZone};
/// # #[cfg(available_in_foundation)]
/// use objc2_foundation::{NSCopying, NSObject, NSObjectProtocol, NSZone};
/// use objc2::rc::{Allocated, Retained};
/// use objc2::{
///     declare_class, extern_protocol, msg_send, msg_send_id, mutability, ClassType,
///     DeclaredClass, ProtocolType,
/// };
///
/// #[derive(Clone)]
/// struct Ivars {
///     foo: u8,
///     bar: c_int,
///     object: Retained<NSObject>,
/// }
///
/// declare_class!(
///     struct MyCustomObject;
///
///     // SAFETY:
///     // - The superclass NSObject does not have any subclassing requirements.
///     // - Interior mutability is a safe default.
///     // - `MyCustomObject` does not implement `Drop`.
///     unsafe impl ClassType for MyCustomObject {
///         type Super = NSObject;
///         type Mutability = mutability::InteriorMutable;
///         const NAME: &'static str = "MyCustomObject";
///     }
///
///     impl DeclaredClass for MyCustomObject {
///         type Ivars = Ivars;
///     }
///
///     unsafe impl MyCustomObject {
///         #[method_id(initWithFoo:)]
///         fn init_with(this: Allocated<Self>, foo: u8) -> Option<Retained<Self>> {
///             let this = this.set_ivars(Ivars {
///                 foo,
///                 bar: 42,
///                 object: NSObject::new(),
///             });
///             unsafe { msg_send_id![super(this), init] }
///         }
///
///         #[method(foo)]
///         fn __get_foo(&self) -> u8 {
///             self.ivars().foo
///         }
///
///         #[method_id(object)]
///         fn __get_object(&self) -> Retained<NSObject> {
///             self.ivars().object.clone()
///         }
///
///         #[method(myClassMethod)]
///         fn __my_class_method() -> bool {
///             true
///         }
/// #
/// #       #[method_id(copyWithZone:)]
/// #       fn copyWithZone(&self, _zone: *const NSZone) -> Retained<Self> {
/// #           let new = Self::alloc().set_ivars(self.ivars().clone());
/// #           unsafe { msg_send_id![super(new), init] }
/// #       }
///     }
///
///     unsafe impl NSObjectProtocol for MyCustomObject {}
///
///     # #[cfg(available_in_foundation)]
///     unsafe impl NSCopying for MyCustomObject {
///         #[method_id(copyWithZone:)]
///         fn copyWithZone(&self, _zone: *const NSZone) -> Retained<Self> {
///             let new = Self::alloc().set_ivars(self.ivars().clone());
///             unsafe { msg_send_id![super(new), init] }
///         }
///
///         // If we have tried to add other methods here, or had forgotten
///         // to implement the method, we would have gotten an error.
///     }
/// );
///
/// impl MyCustomObject {
///     pub fn new(foo: u8) -> Retained<Self> {
///         unsafe { msg_send_id![Self::alloc(), initWithFoo: foo] }
///     }
///
///     pub fn get_foo(&self) -> u8 {
///         unsafe { msg_send![self, foo] }
///     }
///
///     pub fn get_object(&self) -> Retained<NSObject> {
///         unsafe { msg_send_id![self, object] }
///     }
///
///     pub fn my_class_method() -> bool {
///         unsafe { msg_send![Self::class(), myClassMethod] }
///     }
/// }
///
/// fn main() {
///     let obj = MyCustomObject::new(3);
///     assert_eq!(obj.ivars().foo, 3);
///     assert_eq!(obj.ivars().bar, 42);
///     assert!(obj.ivars().object.is_kind_of::<NSObject>());
///
/// #   let obj: Retained<MyCustomObject> = unsafe { msg_send_id![&obj, copy] };
/// #   #[cfg(available_in_foundation)]
///     let obj = obj.copy();
///
///     assert_eq!(obj.get_foo(), 3);
///     assert!(obj.get_object().is_kind_of::<NSObject>());
///
///     assert!(MyCustomObject::my_class_method());
/// }
/// ```
///
/// Approximately equivalent to the following ARC-enabled Objective-C code.
///
/// ```text
/// #import <Foundation/Foundation.h>
///
/// @interface MyCustomObject: NSObject <NSCopying>
/// - (instancetype)initWithFoo:(uint8_t)foo;
/// - (uint8_t)foo;
/// - (NSObject*)object;
/// + (BOOL)myClassMethod;
/// @end
///
///
/// @implementation MyCustomObject {
///     // Instance variables
///     uint8_t foo;
///     int bar;
///     NSObject* _Nonnull object;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo_arg {
///     self = [super init];
///     if (self) {
///         self->foo = foo_arg;
///         self->bar = 42;
///         self->object = [NSObject new];
///     }
///     return self;
/// }
///
/// - (uint8_t)foo {
///     return self->foo;
/// }
///
/// - (NSObject*)object {
///     return self->object;
/// }
///
/// + (BOOL)myClassMethod {
///     return YES;
/// }
///
/// // NSCopying
///
/// - (id)copyWithZone:(NSZone *)_zone {
///     MyCustomObject* new = [[MyCustomObject alloc] initWithFoo: self->foo];
///     new->bar = self->bar;
///     new->obj = self->obj;
///     return new;
/// }
///
/// @end
/// ```
#[doc(alias = "@interface")]
#[doc(alias = "@implementation")]
#[macro_export]
macro_rules! declare_class {
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        unsafe impl ClassType for $for_class:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            type Mutability = $mutability:ty;

            const NAME: &'static str = $name_const:expr;
        }

        impl DeclaredClass for $for_declared:ty {
            $(type Ivars = $ivars:ty;)?
        }

        $($impls:tt)*
    } => {
        $(#[$m])*
        #[repr(C)]
        $v struct $name {
            // Superclasses are deallocated by calling `[super dealloc]`.
            __superclass: $crate::__macro_helpers::ManuallyDrop<$superclass>,
            // Include ivars for proper auto traits.
            __ivars: $crate::__macro_helpers::PhantomData<<Self as $crate::DeclaredClass>::Ivars>,
        }

        $crate::__extern_class_impl_traits! {
            // SAFETY: Upheld by caller
            unsafe impl () for $for_class {
                INHERITS = [$superclass, $($($inheritance_rest,)+)? $crate::runtime::AnyObject];

                fn as_super(&self) {
                    &*self.__superclass
                }

                fn as_super_mut(&mut self) {
                    &mut *self.__superclass
                }
            }
        }

        // Anonymous block to hide the shared statics
        const _: () = {
            static mut __OBJC2_CLASS: $crate::__macro_helpers::MaybeUninit<&'static $crate::runtime::AnyClass> = $crate::__macro_helpers::MaybeUninit::uninit();
            static mut __OBJC2_IVAR_OFFSET: $crate::__macro_helpers::MaybeUninit<$crate::__macro_helpers::isize> = $crate::__macro_helpers::MaybeUninit::uninit();
            static mut __OBJC2_DROP_FLAG_OFFSET: $crate::__macro_helpers::MaybeUninit<$crate::__macro_helpers::isize> = $crate::__macro_helpers::MaybeUninit::uninit();

            // Creation
            unsafe impl ClassType for $for_class {
                type Super = $superclass;
                type Mutability = $mutability;
                const NAME: &'static $crate::__macro_helpers::str = $name_const;

                fn class() -> &'static $crate::runtime::AnyClass {
                    $crate::__macro_helpers::assert_mutability_matches_superclass_mutability::<Self>();

                    // TODO: Use `std::sync::OnceLock`
                    static REGISTER_CLASS: $crate::__macro_helpers::Once = $crate::__macro_helpers::Once::new();

                    REGISTER_CLASS.call_once(|| {
                        let mut __objc2_builder = $crate::__macro_helpers::ClassBuilderHelper::<Self>::new();

                        // Implement protocols and methods
                        $crate::__declare_class_register_impls! {
                            (__objc2_builder)
                            $($impls)*
                        }

                        let (__objc2_cls, __objc2_ivar_offset, __objc2_drop_flag_offset) = __objc2_builder.register();

                        // SAFETY: Modification is ensured by `Once` to happen
                        // before any access to the variables.
                        unsafe {
                            __OBJC2_CLASS.write(__objc2_cls);
                            if <Self as $crate::__macro_helpers::DeclaredIvarsHelper>::HAS_IVARS {
                                __OBJC2_IVAR_OFFSET.write(__objc2_ivar_offset);
                            }
                            if <Self as $crate::__macro_helpers::DeclaredIvarsHelper>::HAS_DROP_FLAG {
                                __OBJC2_DROP_FLAG_OFFSET.write(__objc2_drop_flag_offset);
                            }
                        }
                    });

                    // SAFETY: We just registered the class, so is now available
                    unsafe { __OBJC2_CLASS.assume_init() }
                }

                #[inline]
                fn as_super(&self) -> &Self::Super {
                    &*self.__superclass
                }

                #[inline]
                fn as_super_mut(&mut self) -> &mut Self::Super {
                    &mut *self.__superclass
                }
            }

            impl DeclaredClass for $for_declared {
                type Ivars = $crate::__select_ivars!($($ivars)?);

                #[inline]
                fn __ivars_offset() -> $crate::__macro_helpers::isize {
                    // Only access ivar offset if we have an ivar.
                    //
                    // This makes the offset not be included in the final
                    // executable if it's not needed.
                    if <Self as $crate::__macro_helpers::DeclaredIvarsHelper>::HAS_IVARS {
                        // SAFETY: Accessing the offset is guaranteed to only be
                        // done after the class has been initialized.
                        unsafe { __OBJC2_IVAR_OFFSET.assume_init() }
                    } else {
                        // Fall back to an offset of zero.
                        //
                        // This is fine, since any reads here will only be via. zero-sized
                        // ivars, where the actual pointer doesn't matter.
                        0
                    }
                }

                #[inline]
                fn __drop_flag_offset() -> $crate::__macro_helpers::isize {
                    if <Self as $crate::__macro_helpers::DeclaredIvarsHelper>::HAS_DROP_FLAG {
                        // SAFETY: Same as above.
                        unsafe { __OBJC2_DROP_FLAG_OFFSET.assume_init() }
                    } else {
                        // Fall back to an offset of zero.
                        //
                        // This is fine, since the drop flag is never actually used in the
                        // cases where it was not added.
                        0
                    }
                }

                // SAFETY: The offsets are implemented correctly
                const __UNSAFE_OFFSETS_CORRECT: () = ();
            }
        };

        // Methods
        $crate::__declare_class_output_impls! {
            $($impls)*
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

#[doc(hidden)]
#[macro_export]
macro_rules! __select_ivars {
    ($ivars:ty) => {
        $ivars
    };
    () => {
        // Default ivars to unit
        ()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_output_impls {
    // Base-case
    () => {};

    // With protocol
    (
        $(#[$m:meta])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        // SAFETY: Upheld by caller
        $(#[$m])*
        unsafe impl $protocol for $for {}

        $(#[$m])*
        impl $for {
            $crate::__declare_class_output_methods! {
                $($methods)*
            }
        }

        $crate::__declare_class_output_impls!{
            $($rest)*
        }
    };

    // Without protocol
    (
        $(#[$m:meta])*
        unsafe impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__declare_class_output_methods! {
                $($methods)*
            }
        }

        $crate::__declare_class_output_impls! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_output_methods {
    // Base case
    {} => {};

    // Unsafe variant
    {
        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)

            ($crate::__declare_class_method_out)
            (unsafe)
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_output_methods! {
            $($rest)*
        }
    };

    // Safe variant
    {
        $(#[$($m:tt)*])*
        fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)

            ($crate::__declare_class_method_out)
            ()
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_output_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_impls {
    // Base-case
    (
        ($builder:ident)
    ) => {};

    // With protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            // Implement protocol
            #[allow(unused_mut)]
            let mut __objc2_protocol_builder = $builder.add_protocol_methods::<dyn $protocol>();

            // In case the user's function is marked `deprecated`
            #[allow(deprecated)]
            // In case the user did not specify any methods
            #[allow(unused_unsafe)]
            // SAFETY: Upheld by caller
            unsafe {
                $crate::__declare_class_register_methods! {
                    (__objc2_protocol_builder)

                    $($methods)*
                }
            }

            // Finished declaring protocol; get error message if any
            __objc2_protocol_builder.finish();
        }

        $crate::__declare_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };

    // Without protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($(#[$($m)*])*)

            // In case the user's function is marked `deprecated`
            #[allow(deprecated)]
            // In case the user did not specify any methods
            #[allow(unused_unsafe)]
            // SAFETY: Upheld by caller
            unsafe {
                $crate::__declare_class_register_methods! {
                    ($builder)

                    $($methods)*
                }
            }
        }

        $crate::__declare_class_register_impls! {
            ($builder)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_methods {
    // Base case
    {
        ($builder:ident)
    } => {};

    // Unsafe variant
    {
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)

            ($crate::__declare_class_register_out)
            ($builder)
            (unsafe)
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_register_methods! {
            ($builder)

            $($rest)*
        }
    };

    // Safe variant
    {
        ($builder:ident)

        $(#[$($m:tt)*])*
        fn $name:ident($($params:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_param! {
            ($($params)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)

            ($crate::__declare_class_register_out)
            ($builder)
            ()
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_register_methods! {
            ($builder)

            $($rest)*
        }
    };

    // Consume associated items for better UI.
    //
    // This will still fail inside __declare_class_output_methods!
    {
        ($builder:ident)

        $_associated_item:item

        $($rest:tt)*
    } => {
        $crate::__declare_class_output_methods! {
            ($builder)

            $($rest)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_method_out {
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($builder_method:ident)
        ($receiver:expr)
        ($receiver_ty:ty)
        ($($params_prefix:tt)*)
        ($($params_rest:tt)*)

        ($($m_method:tt)*)
        ($($retain_semantics:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $crate::__declare_class_rewrite_params! {
            ($($params_rest)*)
            ()
            ()

            ($crate::__declare_class_method_out_inner)

            ($($qualifiers)*)
            ($name)
            ($($ret)?)
            ($body)

            ($builder_method)
            ($receiver)
            ($receiver_ty)
            ($($params_prefix)*)

            ($($m_method)*)
            ($($retain_semantics)*)
            ($($m_optional)*)
            ($($m_checked)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_rewrite_params {
    // Convert _
    {
        (_ : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* _ : <$param_ty as $crate::__macro_helpers::ConvertArgument>::__Inner,)
            ($($body_prefix)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert mut
    {
        (mut $param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* $param : <$param_ty as $crate::__macro_helpers::ConvertArgument>::__Inner,)
            (
                $($body_prefix)*
                let mut $param = <$param_ty as $crate::__macro_helpers::ConvertArgument>::__from_declared_param($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert
    {
        ($param:ident : $param_ty:ty $(, $($params_rest:tt)*)?)
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_params! {
            ($($($params_rest)*)?)
            ($($params_converted)* $param : <$param_ty as $crate::__macro_helpers::ConvertArgument>::__Inner,)
            (
                $($body_prefix)*
                let $param = <$param_ty as $crate::__macro_helpers::ConvertArgument>::__from_declared_param($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Output result
    {
        ()
        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($($params_converted)*)
            ($($body_prefix)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_method_out_inner {
    // #[method(...)]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($params_prefix:tt)*)

        (#[method($($__sel:tt)*)])
        ()
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        #[allow(clippy::diverging_sub_expression)]
        $($qualifiers)* extern "C" fn $name(
            $($params_prefix)*
            $($params_converted)*
        ) $(-> <$ret as $crate::__macro_helpers::ConvertReturn>::__Inner)? {
            $($body_prefix)*
            $crate::__convert_result! {
                $body $(; $ret)?
            }
        }
    };

    // #[method_id(...)]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($ret:ty)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($receiver_ty:ty)
        ($($params_prefix:tt)*)

        (#[method_id($($sel:tt)*)])
        () // Specifying retain semantics is unsupported in declare_class! for now
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        #[allow(clippy::diverging_sub_expression)]
        $($qualifiers)* extern "C" fn $name(
            $($params_prefix)*
            $($params_converted)*
        ) -> $crate::__macro_helpers::IdReturnValue {
            // TODO: Somehow tell the compiler that `this: Allocated<Self>` is non-null.

            $($body_prefix)*

            let __objc2_result = $body;

            #[allow(unreachable_code)]
            <$crate::__macro_helpers::RetainSemantics<{
                $crate::__macro_helpers::retain_semantics(
                    $crate::__sel_helper! {
                        ()
                        $($sel)*
                    }
                )
            }> as $crate::__macro_helpers::MessageRecieveId<
                $receiver_ty,
                $ret,
            >>::into_return(__objc2_result)
        }
    };

    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ()
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($params_prefix:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($retain_semantics:tt)*)
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($params_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        $($qualifiers)* extern "C" fn $name() {
            $crate::__macro_helpers::compile_error!("`#[method_id(...)]` must have a return type")
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __convert_result {
    ($body:block) => {
        $body
    };
    ($body:block; $ret:ty) => {
        let __objc2_result = $body;
        #[allow(unreachable_code)]
        <$ret as $crate::__macro_helpers::ConvertReturn>::__into_declared_return(__objc2_result)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_out {
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($__ret:ty)?)
        ($__body:block)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__params_prefix:tt)*)
        ($($params_rest:tt)*)

        (#[$method_or_method_id:ident($($sel:tt)*)])
        ($($retain_semantics:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            ($($m_checked)*)

            $crate::__declare_class_invalid_selectors!(#[$method_or_method_id($($sel)*)]);
            $crate::__extern_methods_no_optional!($($m_optional)*);

            $builder.$builder_method(
                $crate::sel!($($sel)*),
                Self::$name as $crate::__fn_ptr! {
                    ($($qualifiers)*)
                    (_, _,)
                    $($params_rest)*
                },
            );
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_invalid_selectors {
    (#[method(dealloc)]) => {
        $crate::__macro_helpers::compile_error!(
            "`#[method(dealloc)]` is not supported. Implement `Drop` for the type instead"
        )
    };
    (#[method_id(dealloc)]) => {
        $crate::__macro_helpers::compile_error!(
            "`#[method_id(dealloc)]` is not supported. Implement `Drop` for the type instead"
        )
    };
    (#[method_id(alloc)]) => {
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::concat!(
            "`#[method_id(alloc)]` is not supported. ",
            "Use `#[method(alloc)]` and do the memory management yourself",
        ))
    };
    (#[method_id(retain)]) => {
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::concat!(
            "`#[method_id(retain)]` is not supported. ",
            "Use `#[method(retain)]` and do the memory management yourself",
        ))
    };
    (#[method_id(release)]) => {
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::concat!(
            "`#[method_id(release)]` is not supported. ",
            "Use `#[method(release)]` and do the memory management yourself",
        ))
    };
    (#[method_id(autorelease)]) => {
        $crate::__macro_helpers::compile_error!($crate::__macro_helpers::concat!(
            "`#[method_id(autorelease)]` is not supported. ",
            "Use `#[method(autorelease)]` and do the memory management yourself",
        ))
    };
    (#[$method_or_method_id:ident($($sel:tt)*)]) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_no_optional {
    () => {};
    (#[optional]) => {
        $crate::__macro_helpers::compile_error!(
            "`#[optional]` is only supported in `extern_protocol!`"
        )
    };
}

/// Create function pointer type with inferred parameters.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_ptr {
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $(,)?
    ) => {
        $($qualifiers)* extern "C" fn($($output)*) -> _
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        _ : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        mut $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
}
