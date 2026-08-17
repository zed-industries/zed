use syn::{
    Attribute, Expr, Lit, LitBool, LitStr, Meta, MetaList, Result, Token, Type, TypePath,
    punctuated::Punctuated, spanned::Spanned,
};

// find the #[@attr_name] attribute in @attrs
fn find_attribute_meta(attrs: &[Attribute], attr_names: &[&str]) -> Result<Option<MetaList>> {
    // Find attribute with path matching one of the allowed attribute names,
    let search_result = attrs.iter().find_map(|a| {
        attr_names
            .iter()
            .find_map(|attr_name| a.path().is_ident(attr_name).then_some((attr_name, a)))
    });

    let (attr_name, meta) = match search_result {
        Some((attr_name, a)) => (attr_name, &a.meta),
        _ => return Ok(None),
    };
    match meta.require_list() {
        Ok(n) => Ok(Some(n.clone())),
        _ => Err(syn::Error::new(
            meta.span(),
            format!("{attr_name} meta must specify a meta list"),
        )),
    }
}

fn get_meta_value<'a>(meta: &'a Meta, attr: &str) -> Result<&'a Lit> {
    let meta = meta.require_name_value()?;
    get_expr_lit(&meta.value, attr)
}

fn get_expr_lit<'a>(expr: &'a Expr, attr: &str) -> Result<&'a Lit> {
    match expr {
        Expr::Lit(l) => Ok(&l.lit),
        // Macro variables are put in a group.
        Expr::Group(group) => get_expr_lit(&group.expr, attr),
        expr => Err(syn::Error::new(
            expr.span(),
            format!("attribute `{attr}`'s value must be a literal"),
        )),
    }
}

/// Compares `ident` and `attr` and in case they match ensures `value` is `Some` and contains a
/// [`struct@LitStr`]. Returns `true` in case `ident` and `attr` match, otherwise false.
///
/// # Errors
///
/// Returns an error in case `ident` and `attr` match but the value is not `Some` or is not a
/// [`struct@LitStr`].
pub fn match_attribute_with_str_value<'a>(
    meta: &'a Meta,
    attr: &str,
) -> Result<Option<&'a LitStr>> {
    if !meta.path().is_ident(attr) {
        return Ok(None);
    }

    match get_meta_value(meta, attr)? {
        Lit::Str(value) => Ok(Some(value)),
        _ => Err(syn::Error::new(
            meta.span(),
            format!("value of the `{attr}` attribute must be a string literal"),
        )),
    }
}

/// Compares `ident` and `attr` and in case they match ensures `value` is `Some` and contains a
/// [`struct@LitBool`]. Returns `true` in case `ident` and `attr` match, otherwise false.
///
/// # Errors
///
/// Returns an error in case `ident` and `attr` match but the value is not `Some` or is not a
/// [`struct@LitBool`].
pub fn match_attribute_with_bool_value<'a>(
    meta: &'a Meta,
    attr: &str,
) -> Result<Option<&'a LitBool>> {
    if meta.path().is_ident(attr) {
        match get_meta_value(meta, attr)? {
            Lit::Bool(value) => Ok(Some(value)),
            other => Err(syn::Error::new(
                other.span(),
                format!("value of the `{attr}` attribute must be a boolean literal"),
            )),
        }
    } else {
        Ok(None)
    }
}

pub fn match_attribute_with_str_list_value(meta: &Meta, attr: &str) -> Result<Option<Vec<String>>> {
    if meta.path().is_ident(attr) {
        let list = meta.require_list()?;
        let values = list
            .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)?
            .into_iter()
            .map(|s| s.value())
            .collect();

        Ok(Some(values))
    } else {
        Ok(None)
    }
}

/// Compares `ident` and `attr` and in case they match ensures `value` is `None`. Returns `true` in
/// case `ident` and `attr` match, otherwise false.
///
/// # Errors
///
/// Returns an error in case `ident` and `attr` match but the value is not `None`.
pub fn match_attribute_without_value(meta: &Meta, attr: &str) -> Result<bool> {
    if meta.path().is_ident(attr) {
        meta.require_path_only()?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Returns an iterator over the contents of all [`MetaList`]s with the specified identifier in an
/// array of [`Attribute`]s.
pub fn iter_meta_lists(
    attrs: &[Attribute],
    list_names: &[&str],
) -> Result<impl Iterator<Item = Meta>> {
    let meta = find_attribute_meta(attrs, list_names)?;

    Ok(meta
        .map(|meta| meta.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated))
        .transpose()?
        .into_iter()
        .flatten())
}

/// Generates one or more structures used for parsing attributes in proc macros.
///
/// Generated structures have one static method called parse that accepts a slice of [`Attribute`]s.
/// The method finds attributes that contain meta lists (look like `#[your_custom_ident(...)]`) and
/// fills a newly allocated structure with values of the attributes if any.
///
/// The expected input looks as follows:
///
/// ```
/// # use zvariant_utils::def_attrs;
/// def_attrs! {
///     crate zvariant;
///
///     /// A comment.
///     pub StructAttributes("struct") { foo str, bar str, baz none };
///     #[derive(Hash)]
///     FieldAttributes("field") { field_attr bool };
/// }
/// ```
///
/// Here we see multiple entries: an entry for an attributes group called `StructAttributes` and
/// another one for `FieldAttributes`. The former has three defined attributes: `foo`, `bar` and
/// `baz`. The generated structures will look like this in that case:
///
/// ```
/// /// A comment.
/// #[derive(Default, Clone, Debug)]
/// pub struct StructAttributes {
///     foo: Option<String>,
///     bar: Option<String>,
///     baz: bool,
/// }
///
/// #[derive(Hash)]
/// #[derive(Default, Clone, Debug)]
/// struct FieldAttributes {
///     field_attr: Option<bool>,
/// }
/// ```
///
/// `foo` and `bar` attributes got translated to fields with `Option<String>` type which contain the
/// value of the attribute when one is specified. They are marked with `str` keyword which stands
/// for string literals. The `baz` attribute, on the other hand, has `bool` type because it's an
/// attribute without value marked by the `none` keyword.
///
/// Currently the following literals are supported:
///
/// * `str` - string literals;
/// * `bool` - boolean literals;
/// * `[str]` - lists of string literals (`#[macro_name(foo("bar", "baz"))]`);
/// * `none` - no literal at all, the attribute is specified alone.
///
/// The strings between braces are embedded into error messages produced when an attribute defined
/// for one attribute group is used on another group where it is not defined. For example, if the
/// `field_attr` attribute was encountered by the generated `StructAttributes::parse` method, the
/// error message would say that it "is not allowed on structs".
///
/// # Nested attribute lists
///
/// It is possible to create nested lists for specific attributes. This is done as follows:
///
/// ```
/// # use zvariant_utils::def_attrs;
/// def_attrs! {
///     crate zvariant;
///
///     pub OuterAttributes("outer") {
///         simple_attr bool,
///         nested_attr {
///             /// An example of nested attributes.
///             pub InnerAttributes("inner") {
///                 inner_attr str
///             }
///         }
///     };
/// }
/// ```
///
/// The syntax for inner attributes is the same as for the outer attributes, but you can specify
/// only one inner attribute per outer attribute.
///
/// # Using attribute names for attribute lists
///
/// It is possible to use multiple different "crate" names as follows:
///
/// ```
/// # use zvariant_utils::def_attrs;
/// def_attrs! {
///     crate zvariant, zbus;
///
///     pub FooAttributes("foo") {
///         simple_attr bool
///     };
/// }
/// ```
///
/// It will be possible to use both `#[zvariant(...)]` and `#[zbus(...)]` attributes with
/// `FooAttributes`.
///
/// Don't forget to add all the supported attributes to your proc macro definition.
///
/// # Supporting the `crate` attribute
///
/// The macro supports a special `crate_path` field that maps to the `crate` attribute name.
/// This allows users to specify custom crate paths (e.g., when they've renamed zbus/zvariant
/// in their Cargo.toml). Example:
///
/// ```
/// # use zvariant_utils::def_attrs;
/// def_attrs! {
///     crate zvariant;
///
///     pub MyAttributes("struct") {
///         crate_path str
///     };
/// }
/// ```
///
/// Users can then write `#[zvariant(crate = "my_renamed_crate")]` in their code.
/// The field is named `crate_path` but matches the attribute `crate`.
/// Access the value via `attrs.crate_path`.
///
/// # Calling the macro multiple times
///
/// The macro generates static variables with hardcoded names. Calling the macro twice in the same
/// scope will cause a name alias and thus will fail to compile. You need to place each macro
/// invocation into a module in that case.
///
/// # Errors
///
/// The generated parse method checks for some error conditions:
///
/// 1. Unknown attributes. When multiple attribute groups are defined in the same macro invocation,
///    one gets a different error message when providing an attribute from a different attribute
///    group.
/// 2. Duplicate attributes.
/// 3. Missing attribute value or present attribute value when none is expected.
/// 4. Invalid literal type for attributes with values.
#[macro_export]
macro_rules! def_attrs {
    // Helper to get the attribute name string (for ALLOWED_ATTRS and matching)
    // Special case: crate_path field -> matches "crate" attribute
    (@attr_name crate_path $kind:tt) => { "crate" };
    (@attr_name $attr_name:ident $kind:tt) => { ::std::stringify!($attr_name) };

    (@attr_ty str) => {::std::option::Option<::std::string::String>};
    (@attr_ty bool) => {::std::option::Option<bool>};
    (@attr_ty [str]) => {::std::option::Option<::std::vec::Vec<::std::string::String>>};
    (@attr_ty none) => {bool};
    (@attr_ty {
        $(#[$m:meta])*
        $vis:vis $name:ident($what:literal) {
            $($attr_name:ident $kind:tt),+
        }
    }) => {::std::option::Option<$name>};

    (@match_attr_with $attr_name:ident, $meta:ident, $self:ident, $matched:expr, $display_name:expr) => {
        if let ::std::option::Option::Some(value) = $matched? {
            if $self.$attr_name.is_some() {
                return ::std::result::Result::Err(::syn::Error::new(
                    $meta.span(),
                    ::std::format!("duplicate `{}` attribute", $display_name)
                ));
            }

            $self.$attr_name = ::std::option::Option::Some(value.value());
            return Ok(());
        }
    };

    // Special case: crate_path field matches "crate" attribute
    (@match_attr str crate_path, $meta:ident, $self:ident) => {
        $crate::def_attrs!(
            @match_attr_with
            crate_path,
            $meta,
            $self,
            $crate::macros::match_attribute_with_str_value($meta, "crate"),
            "crate"
        )
    };
    (@match_attr str $attr_name:ident, $meta:ident, $self:ident) => {
        $crate::def_attrs!(
            @match_attr_with
            $attr_name,
            $meta,
            $self,
            $crate::macros::match_attribute_with_str_value(
                $meta,
                ::std::stringify!($attr_name),
            ),
            ::std::stringify!($attr_name)
        )
    };
    (@match_attr bool $attr_name:ident, $meta:ident, $self:ident) => {
        $crate::def_attrs!(
            @match_attr_with
            $attr_name,
            $meta,
            $self,
            $crate::macros::match_attribute_with_bool_value(
                $meta,
                ::std::stringify!($attr_name),
            ),
            ::std::stringify!($attr_name)
        )
    };
    (@match_attr [str] $attr_name:ident, $meta:ident, $self:ident) => {
        if let Some(list) = $crate::macros::match_attribute_with_str_list_value(
            $meta,
            ::std::stringify!($attr_name),
        )? {
            if $self.$attr_name.is_some() {
                return ::std::result::Result::Err(::syn::Error::new(
                    $meta.span(),
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }

            $self.$attr_name = Some(list);
            return Ok(());
        }
    };
    (@match_attr none $attr_name:ident, $meta:ident, $self:ident) => {
        if $crate::macros::match_attribute_without_value(
            $meta,
            ::std::stringify!($attr_name),
        )? {
            if $self.$attr_name {
                return ::std::result::Result::Err(::syn::Error::new(
                    $meta.span(),
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }

            $self.$attr_name = true;
            return Ok(());
        }
    };
    (@match_attr {
        $(#[$m:meta])*
        $vis:vis $name:ident($what:literal) $body:tt
    } $attr_name:ident, $meta:expr, $self:ident) => {
        if $meta.path().is_ident(::std::stringify!($attr_name)) {
            if $self.$attr_name.is_some() {
                return ::std::result::Result::Err(::syn::Error::new(
                    $meta.span(),
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }

            return match $meta {
                ::syn::Meta::List(meta) => {
                        $self.$attr_name = ::std::option::Option::Some($name::parse_nested_metas(
                            meta.parse_args_with(::syn::punctuated::Punctuated::<::syn::Meta, ::syn::Token![,]>::parse_terminated)?
                        )?);
                        ::std::result::Result::Ok(())
                    }
                    ::syn::Meta::Path(_) => {
                        $self.$attr_name = ::std::option::Option::Some($name::default());
                        ::std::result::Result::Ok(())
                    }
                    ::syn::Meta::NameValue(_) => Err(::syn::Error::new(
                        $meta.span(),
                        ::std::format!(::std::concat!(
                            "attribute `", ::std::stringify!($attr_name),
                            "` must be either a list or a path"
                        )),
                    ))
                };
        }
    };
    (@def_ty str) => {};
    (@def_ty bool) => {};
    (@def_ty [str]) => {};
    (@def_ty none) => {};
    (
        @def_ty {
            $(#[$m:meta])*
            $vis:vis $name:ident($what:literal) {
                $($attr_name:ident $kind:tt),+
            }
        }
    ) => {
        // Recurse further to potentially define nested lists.
        $($crate::def_attrs!(@def_ty $kind);)+

        $crate::def_attrs!(
            @def_struct
            $(#[$m])*
            $vis $name($what) {
                $($attr_name $kind),+
            }
        );
    };
    (
        @def_struct
        $(#[$m:meta])*
        $vis:vis $name:ident($what:literal) {
            $($attr_name:ident $kind:tt),+
        }
    ) => {
        $(#[$m])*
        #[derive(Default, Clone, Debug)]
        $vis struct $name {
            $(pub $attr_name: $crate::def_attrs!(@attr_ty $kind)),+
        }

        impl $name {
            pub fn parse_meta(
                &mut self,
                meta: &::syn::Meta
            ) -> ::syn::Result<()> {
                use ::syn::spanned::Spanned;

                // This creates subsequent if blocks for simplicity. Any block that is taken
                // either returns an error or sets the attribute field and returns success.
                $(
                    $crate::def_attrs!(@match_attr $kind $attr_name, meta, self);
                )+

                // None of the if blocks have been taken, return the appropriate error.
                let err = if ALLOWED_ATTRS.iter().any(|attr| meta.path().is_ident(attr)) {
                    ::std::format!(
                        ::std::concat!("attribute `{}` is not allowed on ", $what),
                        meta.path().get_ident().unwrap()
                    )
                } else {
                    ::std::format!("unknown attribute `{}`", meta.path().get_ident().unwrap())
                };
                return ::std::result::Result::Err(::syn::Error::new(meta.span(), err));
            }

            pub fn parse_nested_metas<I>(iter: I) -> syn::Result<Self>
            where
                I: ::std::iter::IntoIterator<Item=::syn::Meta>
            {
                let mut parsed = $name::default();
                for nested_meta in iter {
                    parsed.parse_meta(&nested_meta)?;
                }

                Ok(parsed)
            }

            pub fn parse(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
                let mut parsed = $name::default();

                for nested_meta in $crate::macros::iter_meta_lists(
                    attrs,
                    ALLOWED_LISTS,
                )? {
                    parsed.parse_meta(&nested_meta)?;
                }

                Ok(parsed)
            }
        }
    };
    (
        crate $($list_name:ident),+;
        $(
            $(#[$m:meta])*
            $vis:vis $name:ident($what:literal) {
                $($attr_name:ident $kind:tt),+
            }
        );+;
    ) => {
        static ALLOWED_ATTRS: &[&'static str] = &[
            $($($crate::def_attrs!(@attr_name $attr_name $kind),)+)+
        ];

        static ALLOWED_LISTS: &[&'static str] = &[
            $(::std::stringify!($list_name),)+
        ];

        $(
            $crate::def_attrs!(
                @def_ty {
                    $(#[$m])*
                    $vis $name($what) {
                        $($attr_name $kind),+
                    }
                }
            );
        )+
    }
}

/// Checks if a [`Type`]'s identifier is "Option".
pub fn ty_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => segments.last().unwrap().ident == "Option",
        _ => false,
    }
}
