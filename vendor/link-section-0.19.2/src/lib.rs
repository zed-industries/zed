#![doc = include_str!("../docs/BUILD.md")]
//! # link-section
#![doc = include_str!("../docs/PREAMBLE.md")]
#![allow(unsafe_code)]
#![no_std]
#![recursion_limit = "256"]

#[doc = include_str!("../docs/LIFE_BEFORE_MAIN.md")]
pub mod life_before_main {}

mod item;
mod macros;
mod meta;
mod platform;
mod section_parse;
mod sections;

pub use item::SectionItemLocation;
pub use sections::{
    MovableBackref, MovableRef, Ref, Section, TypedMovableSection, TypedMutableSection,
    TypedReferenceSection, TypedSection,
};

/// Types for [`TypedReferenceSection`].
#[deprecated(since = "0.17.1", note = "Use [`Ref`] from the crate root instead.")]
pub mod reference {
    pub use crate::sections::Ref;
}

__declare_features!(
    section: __section_features;

    @default: type;

    /// Auxiliary sections are stored in a section near the main section. The
    /// aux path must be a valid reference to the main section.
    aux {
        attr: [(aux(main = $($aux_name:tt)*)) => (($($aux_name)*))];
        example: "aux(main = path::to::MAIN_SECTION)";
        validate: [(($aux_name:path))];
    };
    /// Specify a custom crate path for the `link-section` crate. Used when
    /// re-exporting the section macro.
    crate_path {
        attr: [(crate_path = $path:pat) => (($path))];
        example: "crate_path = ::path::to::link_section";
    };
    /// Specify a custom section name to allow the section to be used without a
    /// direct reference. If not specified, the section name will be generated
    /// using the item name and a path to the section.
    ///
    /// It is valid to specify multiple sections with the same name, and the linker
    /// will ensure that both sections contain the same items. The multiple sections
    /// must contain the same type, otherwise the section will `panic!` at runtime.
    ///
    /// While `name` accepts a path, this path does not refer to a specific Rust
    /// item path.
    name {
        attr: [(name = $($name_path:tt)*) => (($($name_path)*))];
        example: "name = my_crate::SECTION_NAME";
        validate: [(($name_path:path))];
    };
    /// Crate feature `proc_macro` (enables the `#[section]` attribute shim).
    proc_macro {
        feature: "proc_macro";
    };
    /// The type of the section.
    type {
        attr: [
            (type = $section_type:ident) => ($section_type)
        ];
        example: "untyped | typed | mutable | movable | reference";
        validate: [(untyped), (typed), (mutable), (movable), (reference)];
    };
    /// Allow the section to be used without a direct reference.
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
);

#[cfg(doc)]
__generate_docs!(__section_features);

__declare_features!(
    in_section: __in_section_features;

    @default: section;

    /// Specify an auxiliary section name to allow submission without a direct
    /// reference. Requires `unsafe`.
    aux {
        attr: [(aux(main = $($aux_name:tt)*)) => (($($aux_name)*))];
        example: "aux(main = my_crate::SECTION_NAME)";
        validate: [(($aux_name:path))];
    };
    /// Specify a custom section name to allow submission without a direct
    /// reference. Requires `unsafe`.
    ///
    /// While `name` accepts a path, this path does not refer to a specific Rust
    /// item path.
    name {
        attr: [(name = $($name_path:tt)*) => (($($name_path)*))];
        example: "name = my_crate::SECTION_NAME";
        validate: [(($name_is_path:path))];
    };
    /// Specify an ordinary section reference. The path must be a valid
    /// reference to the section.
    section {
        attr: [(section = $($section_path:tt)*) => (($($section_path)*))];
        example: "[section = ] ::path::to::SECTION";
        validate: [(($section_path:path))];
    };
    /// Specify the type of the section. Used for unsafe submission.
    section_type {
        attr: [(type = $section_type_name:ident) => ($section_type_name)];
        example: "type = untyped | typed | mutable | movable | reference";
        validate: [(untyped), (typed), (mutable), (movable), (reference)];
    };
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
);

#[cfg(target_family = "wasm")]
extern crate alloc;

/// Declarative forms of the `#[section]` and `#[in_section(...)]` macros.
///
/// The declarative forms wrap and parse a proc_macro-like syntax like so, and
/// are identical in expansion to the undecorated procedural macros. The
/// declarative forms support the same attribute parameters as the procedural
/// macros.
pub mod declarative {
    pub use crate::__in_section_parse as in_section;
    pub use crate::__section_parse as section;
}

#[doc(hidden)]
pub mod __support {
    pub use crate::__add_section_link_attribute as add_section_link_attribute;
    pub use crate::__in_section_crate as in_section_crate;
    pub use crate::__in_section_parse as in_section_parse;
    pub use crate::__section_parse as section_parse;

    pub use crate::sections::IsUntypedSection;
    pub use crate::{item::*, platform::*};

    #[cfg(feature = "proc_macro")]
    pub use linktime_proc_macro::combine;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __hash_no_proc_macro {
        (unsafe (($($__prefix:literal)*)) (($($name:ident)::*) $($literal2:literal ($($name2:ident)::*))?) (($($__suffix:literal)*)) $__hash_length:literal $__max_length:literal $__valid_section_chars:literal) => {
            concat!($($__prefix,)* $(stringify!($name)),* $( ,$literal2 $(, stringify!($name2))* )? $(,$__suffix)*)
        };
        ($($rest:tt)*) => {
            compile_error!(concat!("link-section: No proc_macro feature enabled: `unsafe` is required", stringify!($($rest)*)));
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __hash_proc_macro {
        // Unsafe sections are hashed if and only if the name is not valid for
        // the platform.
        (unsafe $prefix:tt $name:tt $suffix:tt $hash_length:literal $max_length:literal $valid_section_chars:literal) => {
            $crate::__support::combine!(output=string input=(
                __IF__(
                    test=(
                        __LE__(
                            a=(__LENGTH__(string=(
                                __TOIDENT__(input=(__RAW__(input=($name))))
                            )))
                            b=$max_length
                        )
                    )
                    then=(
                        $prefix
                        __TOIDENT__(input=(__RAW__(input=($name))))
                        $suffix
                    )
                    else=(
                        $prefix
                        __SUBSTRING__(input=(
                            __TOIDENT__(input=(__RAW__(input=($name))))
                        ) end=(__SUB__(a=$max_length b=$hash_length)))
                        __SUBSTRING__(input=(
                            __HASH__(string=(__RAW__(input=($name))))
                        ) length=$hash_length)
                        $suffix
                    )
                )
            ))
        };
        // Safe sections are always hashed. Note: `ignore_base` below avoids
        // churn on the expansion tests.
        ($definition:tt $prefix:tt $name:tt $suffix:tt $hash_length:literal $max_length:literal $valid_section_chars:literal) => {
            $crate::__support::combine!(output=string input=(
                $prefix
                __SUBSTRING__(input=(
                    __SUBSTRING__(input=(
                        __TOIDENT__(input=(__RAW__(input=($name))))
                    ) end=(__SUB__(a=$max_length b=$hash_length)))
                    __LOCATIONHASH__(of=($definition $name) alphabet=[_0-9a-zA-Z] ignore_base=("src/lib.rs"))
                ) length=$max_length)
                $suffix
            ))
        };
    }

    #[cfg(feature = "proc_macro")]
    pub use __hash_proc_macro as hash;

    #[cfg(not(feature = "proc_macro"))]
    pub use __hash_no_proc_macro as hash;

    #[cfg(miri)]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __address_of_symbol {
        ($ref_or_item:ident $section:ident $type:ident $name:tt) => {
            // Miri does not support any of these linker-defined extern statics
            // see: https://github.com/rust-lang/miri/blob/master/src/shims/extern_static.rs#L15
            ::core::ptr::null() as *const ()
        };
    }

    #[cfg(not(miri))]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __address_of_symbol {
        ($ref_or_item:ident $section:ident $type:ident $name:tt) => {
            {
                // These are not valid items, but they are valid pointers.
                // We cannot safely use them - only take pointers to them.
                $crate::__add_linktime_attributes_to_static!(
                    extern "C" {
                        #[link_name = $crate::__support::section_name!(string $ref_or_item $section $type $name)]
                        static __SYMBOL: u8;
                    }
                );
                // TODO: black_box when hint is stable
                // TODO: MSRV: we can use &raw const once we bump MSRV
                // unsafe { &raw const __SYMBOL as *const () }
                unsafe { ::core::ptr::addr_of!(__SYMBOL) as *const () }
            }
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __add_section_link_attribute(
        ($ref_or_item:ident $section:ident $type:ident $name:tt #[$attr:ident = __]
            $(#[$meta:meta])*
            $vis:vis static $($static:tt)*
        ) => {
            $crate::__add_linktime_attributes_to_static!(
                #[$attr = $crate::__support::section_name!(string $ref_or_item $section $type $name)]
                $(#[$meta])*
                $vis static $($static)*
            );
        };
        ($ref_or_item:ident $section:ident $type:ident $name:tt #[$attr:ident = __]
            extern "C" {
                $(#[$meta:meta])*
                $vis:vis static $($static:tt)*
            }
        ) => {
            $crate::__add_linktime_attributes_to_static!(
                extern "C" {
                    #[link_name = $crate::__support::section_name!(string $ref_or_item $section $type $name)]
                    $(#[$meta])*
                    $vis static $($static)*
                }
            );
        };
        ($ref_or_item:ident $section:ident $type:ident $name:tt #[$attr:ident = __]
            $($item:tt)*) => {
            $crate::__add_linktime_attributes_to_static!(
                #[$attr = $crate::__support::section_name!(string $ref_or_item $section $type $name)]
                $($item)*
            );
        };
    );

    #[cfg(target_family = "wasm")]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __if_wasm {
        (($($true:tt)*) ($($false:tt)*)) => {
            $($true)*
        };
    }

    #[cfg(not(target_family = "wasm"))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __if_wasm {
        (($($true:tt)*) ($($false:tt)*)) => {
            $($false)*
        };
    }

    #[macro_export]
    #[doc(hidden)]
    #[allow(unknown_lints, edition_2024_expr_fragment_specifier)]
    macro_rules! __in_section_crate {
        ((@v=0 ; (source=$source:ident) ; (type = untyped) ; (path = $path:path) ; (name = $name:ident) ; (meta = $meta:tt) ; (item = $item:tt))) => {
            $crate::__in_section_crate!(@untyped (($name)()), , $path, $meta $item);
        };
        ((@v=0 ; (source=$source:ident) ; (type = $section_type:ident) ; (path = $path:path) ; (name = $name:ident) ; (meta = $meta:tt) ; (item = $item:tt))) => {
            $crate::__in_section_crate!(@typed[$section_type] (($name)()), , $path, $meta $item);
        };
        ((@v=0 ; (source=$source:ident) ; (type = untyped) ; (section = $section:tt) $(; (path = $path:path) ; (name = $name:ident))? ; (meta = $meta:tt) ; (item = $item:tt))) => {
            $crate::__in_section_crate!(@untyped $section, , $($path)?, $meta $item);
        };
        ((@v=0 ; (source=$source:ident) ; (type = $section_type:ident) ; (section = $section:tt) $(; (path = $path:path) ; (name = $name:ident))? ; (meta = $meta:tt) ; (item = $item:tt))) => {
            $crate::__in_section_crate!(@typed[$section_type] $section, , $($path)?, $meta $item);
        };

        // Untyped items are placed in the data or code section as-is.
        (@untyped $section:tt, , $($path:path)?, ($($meta:tt)*) ($vis:vis fn $($rest:tt)*)) => {
            $crate::__add_section_link_attribute!(
                item code section $section
                #[link_section = __]
                $($meta)*
                $vis fn $($rest)*
            );
            $(
                const _: () = {
                    $crate::Section::__validate(&$path);
                };
            )?
        };
        (@untyped $section:tt, , $($path:path)?, ($($meta:tt)*) ($($rest:tt)*)) => {
            $crate::__add_section_link_attribute!(
                item data section $section
                #[link_section = __]
                $($meta)*
                $($rest)*
            );
            $(
                const _: () = {
                    $crate::Section::__validate(&$path);
                };
            )?
        };

        // Convert fn() with a body to a const item and a function pointer item.
        (@typed[$section_type:ident] $section:tt, , $($path:path)?, ($($meta:tt)*) ($vis:vis fn $ident_fn:ident($($args:tt)*) $(-> $ret:ty)? { $($body:tt)* })) => {
            $($meta)*
            $vis fn $ident_fn($($args)*) $(-> $ret)? {
                $crate::__in_section_crate!(@typed[$section_type] $section, , $($path)?, () (
                    const _: fn($($args)*) $(-> $ret)? = $ident_fn;
                ));

                $($body)*
            }
        };

        // If no path is provided, use the item type.
        (@typed[$section_type:ident] $section:tt, , , $meta:tt ($vis:vis $const_or_static:ident $name:tt : $ty:ty = $($rest:tt)*)) => {
            $crate::__in_section_crate!(@typed[$section_type] $section, , $crate::TypedSection::<$ty>, $meta (
                $vis $const_or_static $name: $ty = $($rest)*
            ));
        };

        (@type_select $path:path) => {
            <$path as $crate::__support::SectionItemType>::Item
        };

        // static items
        (@typed[typed] $section:tt, , $path:path, ($($meta:tt)*) ($vis:vis static $ident:ident : $ty:ty = $value:expr;)) => {
            $crate::__if_wasm!(
                (
                    compile_error!("static items are not supported on WASM: use const items instead");
                )
                (
                    $crate::__add_section_link_attribute!(
                        item data section $section
                        #[link_section = __]
                        $($meta)*
                        $vis static $ident: $crate::__in_section_crate!(@type_select $path) = const {
                            const _: () = {
                                let _: *const <$path as $crate::__support::SectionItemTyped<$ty>>::Item = ::core::ptr::null();
                            };

                            $value
                        };
                    );
                )
            );
        };

        // mutable const items live in SyncUnsafeCell
        (@typed[mutable] $section:tt, , $path:path, ($($meta:tt)*) ($vis:vis const $ident:tt: $ty:ty = $value:expr;)) => {
            $($meta)*
            $vis const $ident: $ty = const {
                type __InSecStoredTy = $crate::__in_section_crate!(@type_select $path);
                const __LINK_SECTION_CONST_ITEM_VALUE: __InSecStoredTy = $value;

                $crate::__register_wasm_item!(mutable, type=__InSecStoredTy, value=__LINK_SECTION_CONST_ITEM_VALUE, section=$section);

                $crate::__if_wasm!(() (
                    $crate::__add_section_link_attribute!(
                        item data section $section
                        #[link_section = __]
                        static __LINK_SECTION_CONST_ITEM: $crate::__support::SyncUnsafeCell<__InSecStoredTy> = $crate::__support::SyncUnsafeCell::new(__LINK_SECTION_CONST_ITEM_VALUE);
                    );
                ));

                __LINK_SECTION_CONST_ITEM_VALUE
            };
        };

        (@typed[mutable] $($rest:tt)*) => {
            compile_error!("Only const items are supported in mutable sections");
        };

        // movable static items expose a MovableRef and submit hidden value/backref records.
        (@typed[movable] $section:tt, , $path:path, ($($meta:tt)*) ($vis:vis static $ident:ident: $ty:ty = $value:expr;)) => {
            $($meta)*
            $vis static $ident: $crate::MovableRef<$crate::__in_section_crate!(@type_select $path)> = const {
                const __LINK_SECTION_CONST_ITEM_VALUE: __InSecStoredTy = $value;
                type __InSecStoredTy = $crate::__in_section_crate!(@type_select $path);

                $crate::__if_wasm!((
                    {
                        $crate::__register_wasm_item!(
                            movable,
                            type=__InSecStoredTy,
                            value=__LINK_SECTION_CONST_ITEM_VALUE,
                            slot=$crate::MovableRef::slot_ptr(&raw const $ident),
                            section=$section
                        );

                        // Import the section info so the handle can lazily flatten.
                        $crate::__import_section_info!(
                            $crate::__support::wasm::LinkSectionMovableInfo,
                            __LINK_SECTION_MOVABLE_INFO,
                            $section
                        );

                        $crate::MovableRef::new($crate::__support::MovableRefStorage::new(::core::ptr::null(), &raw const __LINK_SECTION_MOVABLE_INFO))
                    }
                )(
                    {
                        $crate::__add_section_link_attribute!(
                            item data section $section
                            #[link_section = __]
                            static __LINK_SECTION_CONST_ITEM: $crate::__support::SyncUnsafeCell<__InSecStoredTy> =
                                $crate::__support::SyncUnsafeCell::new(__LINK_SECTION_CONST_ITEM_VALUE);
                        );

                        $crate::__add_section_link_attribute!(
                            backref data section $section
                            #[link_section = __]
                            static __LINK_SECTION_MOVABLE_BACKREF: $crate::__support::SyncUnsafeCell<
                                $crate::MovableBackref<__InSecStoredTy>
                            > = $crate::__support::SyncUnsafeCell::new(
                                $crate::MovableBackref::new(
                                    $crate::MovableRef::slot_ptr(&raw const $ident),
                                )
                            );
                        );

                        $crate::MovableRef::new($crate::__support::MovableRefStorage::new(
                            (&raw const __LINK_SECTION_CONST_ITEM)
                                .cast::<__InSecStoredTy>(),
                        ))
                    }
                ))
            };
        };

        (@typed[movable] $($rest:tt)*) => {
            compile_error!("Only static items are supported in movable sections");
        };

        // const items are the same across all other types
        (@typed[$section_type:ident] $section:tt, , $path:path, ($($meta:tt)*) ($vis:vis const $ident:tt: $ty:ty = $value:expr;)) => {
            $($meta)*
            $vis const $ident: $ty = const {
                type __InSecStoredTy = $crate::__in_section_crate!(@type_select $path);
                const __LINK_SECTION_CONST_ITEM_VALUE: __InSecStoredTy = $value;

                $crate::__if_wasm!((
                    $crate::__register_wasm_item!($section_type, type=__InSecStoredTy, value=__LINK_SECTION_CONST_ITEM_VALUE, section=$section);
                ) (
                    $crate::__add_section_link_attribute!(
                        item data section $section
                        #[link_section = __]
                        static __LINK_SECTION_CONST_ITEM: __InSecStoredTy = __LINK_SECTION_CONST_ITEM_VALUE;
                    );
                ));

                __LINK_SECTION_CONST_ITEM_VALUE
            };
        };

        (@typed[reference] $section:tt, , $path:path, ($($meta:tt)*) ($vis:vis static $ident:ident: $ty:ty = $value:expr;)) => {
            $crate::__if_wasm!(
                (
                    $($meta)*
                    $vis static $ident: $crate::reference::Ref<$crate::__in_section_crate!(@type_select $path)> = {
                        type __InSecStoredTy = $crate::__in_section_crate!(@type_select $path);
                        const __LINK_SECTION_CONST_ITEM_VALUE: __InSecStoredTy = $value;
                        $crate::__register_wasm_item!(reference, type=__InSecStoredTy, value=__LINK_SECTION_CONST_ITEM_VALUE, ref=$ident, section=$section);
                        // Import the section info so the handle can lazily flatten.
                        $crate::__import_section_info!(
                            $crate::__support::wasm::LinkSectionInfo,
                            __LINK_SECTION_REF_INFO,
                            $section
                        );
                        $crate::reference::Ref::new($crate::__support::RefStorage::new(&raw const __LINK_SECTION_REF_INFO))
                    };
                )
                (
                    // Layout-compatible with the value, so stored directly in the section.
                    #[cfg(not(target_family="wasm"))]
                    $crate::__add_section_link_attribute!(
                        item data section $section
                        #[link_section = __]
                        $($meta)*
                        $vis static $ident: $crate::reference::Ref<$crate::__in_section_crate!(@type_select $path)> = $crate::reference::Ref::new($crate::__support::RefStorage::new($value));
                    );
                )
            );
        };

        ($($input:tt)*) => {
            compile_error!(concat!("Unexpected input to __in_section_crate: ", stringify!($($input)*)));
        };
    }
}

/// Define a link section.
///
/// The definition site generates two items: a static section struct that is
/// used to access the section, and a macro that is used to place items into the
/// section. The macro is used by the [`in_section`] procedural macro.
///
/// # Attributes
///
/// - `no_macro`: Does not generate the submission macro at the definition site.
///   This will require any associated [`in_section`] invocations to use the raw
///   name of the section.
/// - `aux(main = <name>)`: Specifies that this section is an auxiliary section, and
///   that the section is named `<name>+<aux>`.
///
/// # Example
/// ```rust
/// use link_section::{in_section, section};
///
/// #[section(untyped)]
/// pub static DATA_SECTION: link_section::Section;
///
/// #[in_section(DATA_SECTION)]
/// pub fn data_function() {
///     println!("data_function");
/// }
/// ```
#[cfg(feature = "proc_macro")]
pub use ::linktime_proc_macro::section;

/// Place an item into a link section.
///
/// # Functions and typed sections
///
/// As a special case, since function declarations by themselves are not sized,
/// functions in typed sections are split and stored as function pointers.
///
/// ## Raw items
///
/// This macro can place items into a section that is not normally visible to it
/// by using `#[in_section(unsafe, type = typed|movable|..., name =
/// SECTION_NAME, ...)`. Raw items are not validated at compile time, and must
/// be validated by the author.
#[cfg(feature = "proc_macro")]
pub use ::linktime_proc_macro::in_section;
