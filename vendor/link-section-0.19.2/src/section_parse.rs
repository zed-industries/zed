//! Declarative `#[section(...)]` parsing pipeline.
//!
//! The `#[section]` attribute generates a unique, contentless, ZST struct which
//! behaves like a value when we add a deref to it.
//!
//! Note from author: I don't recall the original reason for using this over a
//! true static type - this might be an artifact from an earlier design.
//!
//! The submission macro is used to determine the insertion for items into the
//! section. The macro shares the same name and module path as the section
//! struct, which allows us to use a single path to access both items.
//!
//! We need a proc macro to submit anything more than a "this section is this
//! type" argument because of the arcane macro definition rules (ie: any
//! exportable macro will end up in the crate root, so we need unique names).

#[macro_export]
#[doc(hidden)]
macro_rules! __section_parse {
    ($($input:tt)*) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parallel[
                    $crate::__parse_item[$crate::__section_features],
                    $crate::__chain[
                        $crate::__extract_type,
                        $crate::__parse_type,
                    ],
                ],
                $crate::__section_parse_impl,
            ]
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __in_section_parse {
    ($($input:tt)*) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parallel[
                    $crate::__parse_item[$crate::__in_section_features],
                    $crate::__extract_type_assign,
                ],
                $crate::__in_section_parse_impl,
            ]
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __extract_type {
    (@entry next=$next:path[$next_args:tt], input=($(#[$meta:meta])* $vis:vis static $ident:ident : $($type_rest:tt)*) ) => {
        $next!($next_args, ($($type_rest)*));
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __extract_type_assign {
    (@entry next=$next:path[$next_args:tt], input=($(#[$meta:meta])* $vis:vis static $ident:tt : $type:ty = $($value:tt)*) ) => {
        $next!($next_args, ($type));
    };
    (@entry next=$next:path[$next_args:tt], input=($(#[$meta:meta])* $vis:vis const $ident:tt : $type:ty = $($value:tt)*) ) => {
        $next!($next_args, ($type));
    };
    (@entry next=$next:path[$next_args:tt], input=($(#[$meta:meta])* $vis:vis fn $ident:ident $args:tt $( -> $ret:ty )? $body:block )) => {
        $next!($next_args, (fn $args $( -> $ret )?));
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __section_parse_impl {
    (
        @entry next=$next:path[$next_args:tt],
        input=(
            features = (
                aux = $aux:tt: $aux_spec:ident,
                crate_path = $crate_path:tt: $crate_path_spec:ident,
                name = $name:tt: $name_spec:ident,
                proc_macro = $proc_macro:tt: $proc_macro_spec:ident,
                type = $section_type:tt: $section_type_spec:ident,
                unsafe = $unsafe:tt: $unsafe_spec:ident,
            ),
            self = ( $($inner:tt)* ),
            meta = ($(#[$meta:meta])*),
            item = ($vis:vis static $ident:ident : $($type_rest:tt)*)
            type = ($type:ty)
            prefix = $prefix:tt
            final = $final:ident
            generics = $generics:tt
        )
    ) => {
        // Validate the section type matches the attribute type
        $crate::__section_parse_impl!(@validate type=$section_type final=$final);
        // Validate the generic types match the section type
        $crate::__section_parse_impl!(@validate type=$section_type generics=$generics);

        // Parse the aux path so we can take its final segment
        $crate::__parse_type!(@entry next=$crate::__section_parse_impl[[
            @types features = (
                type=$section_type,
                macro=(
                    proc_macro=$proc_macro,
                ),
                definition=(self=$($inner)* meta=$($meta)* item=$vis static $ident : $($type_rest)*),
                ident=$ident,
                name=$name,
                generic=$generics,
                unsafe=$unsafe,
            ),
            item = ($(#[$meta])* $vis static $ident: $type),
            ]], input=$aux
        );
    };

    (@validate type=untyped final=Section) => {};
    (@validate type=$type:tt final=Section) => { compile_error!("Use #[section(untyped)] to create an untyped section"); };
    (@validate type=typed final=TypedSection) => {};
    (@validate type=$type:tt final=TypedSection) => { compile_error!("Use #[section(typed)] to create a typed section"); };
    (@validate type=mutable final=TypedMutableSection) => {};
    (@validate type=$type:tt final=TypedMutableSection) => { compile_error!("Use #[section(mutable)] to create a mutable typed section"); };
    (@validate type=movable final=TypedMovableSection) => {};
    (@validate type=$type:tt final=TypedMovableSection) => { compile_error!("Use #[section(movable)] to create a movable typed section"); };
    (@validate type=reference final=TypedReferenceSection) => {};
    (@validate type=$type:tt final=TypedReferenceSection) => { compile_error!("Use #[section(reference)] to create a reference section"); };
    (@validate type=$type:tt final=$final:ident) => { compile_error!(concat!("Unexpected section type: ", stringify!($type), " for section type: ", stringify!($final))); };

    (@validate type=() generics=$generics:tt) => {};
    (@validate type=untyped generics=()) => {};
    (@validate type=$type:tt generics=($generic:ty)) => {};
    (@validate type=$type:tt generics=($($generics:tt)*)) => {
        compile_error!(concat!("Unexpected generic types for section type: ", stringify!($type), ": ", stringify!($($generics)*)));
    };

    // Missing type (will have already errored)
    ([@types features = (
        type=(),
        $($rest:tt)*
    ),
    item = $item:tt,], $type:tt) => {};

    // No aux
    ([@types features = (
        type=$section_type:ident,
        macro=$macro:tt,
        definition=$definition:tt,
        ident=$ident:ident,
        name=$name:tt,
        generic=$generics:tt,
        unsafe=$unsafe:tt,
    ),
    item = $item:tt,], (
        type = ()
    )) => {
        $crate::__parse_type!(@entry next=$crate::__section_parse_impl[[
            @types features = (
                type=$section_type,
                macro=$macro,
                definition=$definition,
                ident=$ident,
                aux=(),
                generic=$generics,
                unsafe=$unsafe,
            ),
            item = $item,
        ]], input=$name);
    };

    // Yes aux
    ([@types features = (
        type=$section_type:ident,
        macro=$macro:tt,
        definition=$definition:tt,
        ident=$ident:ident,
        name=$name:tt,
        generic=$generics:tt,
        unsafe=$unsafe:tt,
    ),
    item = $item:tt,], (
        type = $type:tt
        prefix = ($(:: $name_prefix:ident)? $($name_rest:ident ::)*)
        final = $aux:ident
        generics = ()
    )) => {
        $crate::__parse_type!(@entry next=$crate::__section_parse_impl[[
            @types features = (
                type=$section_type,
                macro=$macro,
                definition=$definition,
                ident=$ident,
                aux=($(:: $name_prefix)? $($name_rest ::)* $aux),
                generic=$generics,
                unsafe=$unsafe,
            ),
            item = $item,
        ]], input=$name);
    };

    // No name
    ([@types features = (
        type=$section_type:ident,
        macro=$macro:tt,
        definition=$definition:tt,
        ident=$ident:ident,
        aux=$aux:tt,
        generic=$generics:tt,
        unsafe=$unsafe:tt,
    ),
    item = $item:tt,], (
        type = ()
    )) => {
        $crate::__section_parse_impl!(
            @generate features = (
                type=$section_type,
                macro=$macro,
                definition=$definition,
                aux=$aux,
                name=($ident),
                generic=$generics,
                unsafe=$unsafe,
            ),
            item = $item,
        );
    };

    // Yes name
    ([@types features = (
        type=$section_type:ident,
        macro=$macro:tt,
        definition=$definition:tt,
        ident=$ident:ident,
        aux=$aux:tt,
        generic=$generics:tt,
        unsafe=$unsafe:tt,
    ),
    item = $item:tt,], (
        type = $type:tt
        prefix = ($(:: $name_prefix:ident)? $($name_rest:ident ::)*)
        final = $name:ident
        generics = ()
    )) => {
        $crate::__section_parse_impl!(
            @generate features = (
                type=$section_type,
                macro=$macro,
                definition=$definition,
                aux=$aux,
                name=($(:: $name_prefix)? $($name_rest ::)* $name),
                generic=$generics,
                unsafe=$unsafe,
            ),
            item = $item,
        );
    };

    // Finalize name/aux w/unsafe
    (@generate
        features = (
            type=$section_type:ident,
            macro=(
                proc_macro=$proc_macro:tt,
            ),
            definition=$definition:tt,
            aux=$aux:tt,
            name=$name:tt,
            generic=$generic_ty:ty,
            unsafe=unsafe,
        ),
        item = $item:tt,
    ) => {
        $crate::__section_parse_impl!(
            @generate features = (
                type=$section_type,
                macro=(
                    proc_macro=$proc_macro,
                    unsafe=unsafe,
                    args=(
                        section=($name $aux unsafe),
                    ),
                ),
                name=($name $aux unsafe),
                generic=$generic_ty,
            ),
            item = $item,
        );
    };

    // Finalize name/aux w/unsafe
    (@generate
        features = (
            type=$section_type:ident,
            macro=(
                proc_macro=$proc_macro:tt,
            ),
            definition=$definition:tt,
            aux=$aux:tt,
            name=$name:tt,
            generic=$generic_ty:ty,
            unsafe=$unsafe:tt,
        ),
        item = $item:tt,
    ) => {
        $crate::__section_parse_impl!(
            @generate features = (
                type=$section_type,
                macro=(
                    proc_macro=$proc_macro,
                    unsafe=$unsafe,
                    args=(
                        section=($name $aux $definition),
                    ),
                ),
                name=($name $aux $definition),
                generic=$generic_ty,
            ),
            item = $item,
        );
    };

    // ... and go
    (@generate
        features = (
            type=$section_type:ident,
            macro=$macro:tt,
            name=$name:tt,
            generic=$generic_ty:ty,
        ),
        item = ($(#[$meta:meta])* $vis:vis static $ident:ident: $collection:ty),
    ) => {
        $(#[$meta])*
        #[allow(non_camel_case_types)]
        $vis struct $ident;

        $crate::__section_declare_submission_macro!(
            [$]
            macro=$macro
            type=$section_type
            vis=$vis
            name=$ident
        );

        impl $ident {
            /// Get a `const` reference to the underlying section. In
            /// non-const contexts, `deref` is sufficient.
            pub const fn const_deref(&self) -> &'static $collection {
                static SECTION: $collection = {
                    let section = $crate::__support::get_section!($section_type, name=$name, type=$generic_ty);
                    let name = $crate::__support::section_name!(
                        string item data bare $name
                    );
                    $crate::__support::validate_section_name(name);
                    unsafe { <$collection>::new(name, section) }
                };
                &SECTION
            }
        }

        impl ::core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::ops::Deref::deref(self).fmt(f)
            }
        }

        impl ::core::ops::Deref for $ident {
            type Target = $collection;
            fn deref(&self) -> &Self::Target {
                self.const_deref()
            }
        }

        $crate::__section_parse_impl!(@slice $section_type $ident $name : ($collection) ($generic_ty));
    };

    (@slice untyped $ident:ident $name:tt $($rest:tt)*) => {
        impl $crate::__support::IsUntypedSection for $ident {}

        const _: () = {
            // Ensure that untyped data sections are never empty.
            $crate::__add_section_link_attribute!(
                item data section $name
                #[link_section = __]
                static __LINK_SECTION_CONST_ITEM: u8 = 0;
            );
        };
    };

    (@slice $section_type:ident $ident:ident $name:tt : ($collection:ty) ($generic_ty:ty)) => {
        impl $crate::__support::SectionItemType for $ident {
            type Item = $generic_ty;
        }
        impl $crate::__support::SectionItemTyped<$generic_ty> for $ident {
            type Item = $generic_ty;
        }
        impl $ident {
            /// Get the section as a slice.
            pub fn as_slice(&self) -> &[$generic_ty] {
                self.const_deref().as_slice()
            }
        }
        impl ::core::iter::IntoIterator for $ident {
            type Item = &'static $generic_ty;
            type IntoIter = ::core::slice::Iter<'static, $generic_ty>;
            fn into_iter(self) -> Self::IntoIter {
                self.const_deref().as_slice().iter()
            }
        }
    };

    // ( $($bad:tt)* ) => {
    //     compile_error!(concat!("link-section: unexpected arguments `", stringify!($($bad)*), "` for `#[section]`"));
    // };
}

/// `in_section!` expects a macro alias named like the section static; declare it unless
/// `no_macro` was set on the attribute.
#[macro_export]
#[doc(hidden)]
#[allow(clippy::crate_in_macro_def)]
macro_rules! __section_declare_submission_macro {
    // No proc macro or unsafe, so we do nothing.
    ([$dollar:tt] macro=(proc_macro=(), $($mrest:tt)*) $($rest:tt)*) => {};
    ([$dollar:tt] macro=(proc_macro=proc_macro, unsafe=unsafe, $($mrest:tt)*) $($rest:tt)*) => {};

    // If aux is specified, we need to use a custom macro. Either we use the
    // proc macro, or we use the unique name provided by the user.

    // Proc macro available.
    ([$dollar:tt] macro=(proc_macro=proc_macro, unsafe=(), args=($($arg_name:ident = $arg_value:tt,)*),) type=$section_type:ident vis=$vis:vis name=$name:ident) => {
        $crate::__support::combine!(output=ident span=$name
            prefix=(#[allow(non_snake_case)] mod) input=(__ $name __ __FILE__(of=$name) _L __LINE__(of=$name) _C __COLUMN__(of=$name) _H __HASH__(string=($($arg_name = $arg_value,)*)) __private_macro) suffix=({
                $crate::__support::combine!(output=ident span=$name
                    prefix=(#[macro_export]
                    #[allow(non_snake_case)]
                    #[doc(hidden)]
                    macro_rules!) input=(__ $name __ __FILE__(of=$name) _L __LINE__(of=$name) _C __COLUMN__(of=$name) _H __HASH__(string=($($arg_name = $arg_value,)*)) __private_macro) suffix=({
                        (($dollar ($args:tt)*)) => {
                            $crate::__in_section_crate!((@v=0 ; (source=section) ; (type=$section_type) $(; ($arg_name = $arg_value) )* ; $dollar ($args)*));
                        };
                    })
                );

                $crate::__support::combine!(output=ident span=$name
                    prefix=(#[doc(hidden)]
                    pub use ) input=(__ $name __ __FILE__(of=$name) _L __LINE__(of=$name) _C __COLUMN__(of=$name) _H __HASH__(string=($($arg_name = $arg_value,)*)) __private_macro) suffix=(as __MACRO;)
                );
            })
        );

        $crate::__support::combine!(output=ident span=$name
            prefix=(#[allow(unused_imports)] #[doc(hidden)]
            $vis use ) input=(__ $name __ __FILE__(of=$name) _L __LINE__(of=$name) _C __COLUMN__(of=$name) _H __HASH__(string=($($arg_name = $arg_value,)*)) __private_macro) suffix=(::__MACRO as $name;)
        );
    };

    ($($rest:tt)*) => {
        compile_error!(concat!(
            "link-section: unexpected arguments `",
            stringify!($($rest)*),
            "` for `#[section]` submission macro declaration"
        ));
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __section_parse_internal {
    ( $features:path, $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_item[$features],
                $crate::__section_parse_impl,
            ]
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __in_section_parse_impl {
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            aux = $aux:tt: $aux_spec:ident,
            name = $name:tt: $name_spec:ident,
            section = $section:tt: $section_spec:ident,
            section_type = $section_type:tt: $type_spec:ident,
            unsafe = $unsafe:tt: $unsafe_spec:ident,
        ),
        self = ( $($inner:tt)* ),
        meta = $meta:tt,
        item = $item:tt
        $type:ty
    )) => {
        $crate::__in_section_parse_impl!(@dispatch features=(
            section = $section,
            raw = ($aux $name $section_type $unsafe)
        ) meta=$meta item=$item);
    };

    (@dispatch features=(
        section = (),
        raw = ($aux:tt $name:tt $section_type:tt $unsafe:tt)
    ) meta=$meta:tt item=$item:tt) => {
        // Raw, feed directly to __in_section_crate!
        $crate::__in_section_crate!((@v=0 ; (source=in_section) ; (type=$section_type) ; (section=($name $aux unsafe)) ; (meta=$meta) ; (item=$item)));
    };

    (@dispatch features=(
        section = $section:tt,
        raw = $raw:tt
    ) meta=$meta:tt item=$item:tt) => {
        $crate::__parse_type!(@entry next=$crate::__in_section_parse_impl[[@dispatch meta=$meta item=$item]], input=$section);
    };

    ([@dispatch meta=$meta:tt item=$item:tt], (type=($section:path) prefix=$prefix:tt final=$final:ident generics=$generics:tt)) => {
        $section!(((path=$section) ; (name=$final) ; (meta=$meta) ; (item=$item)));
    };

    ($($input:tt)*) => {
        compile_error!(concat!("link-section: unexpected arguments `", stringify!($($input)*), "` for `#[in_section]`"));
    };
}
