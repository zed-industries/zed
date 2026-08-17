use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{
    parenthesized, punctuated::Punctuated, token::Comma, Attribute, DeriveInput, Field, LitStr,
    Meta, Token, Type, Variant,
};

macro_rules! assert_attribute {
    ($e:expr, $err:expr, $input:expr) => {
        if !$e {
            return Err(syn::Error::new_spanned($input, $err));
        }
    };
}

macro_rules! fail {
    ($t:expr, $m:expr) => {
        return Err(syn::Error::new_spanned($t, $m))
    };
}

macro_rules! try_set {
    ($i:ident, $v:expr, $t:expr) => {
        match $i {
            None => $i = Some($v),
            Some(_) => fail!($t, "duplicate attribute"),
        }
    };
}

pub struct TypeName {
    pub val: String,
    pub span: Span,
}

impl TypeName {
    pub fn get(&self) -> TokenStream {
        let val = &self.val;
        quote_spanned! { self.span => #val }
    }
}

#[derive(Copy, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum RenameAll {
    LowerCase,
    SnakeCase,
    UpperCase,
    ScreamingSnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
}

pub struct SqlxContainerAttributes {
    pub transparent: bool,
    pub type_name: Option<TypeName>,
    pub rename_all: Option<RenameAll>,
    pub repr: Option<Ident>,
    pub no_pg_array: bool,
    pub default: bool,
}

pub enum JsonAttribute {
    NonNullable,
    Nullable,
}

pub struct SqlxChildAttributes {
    pub rename: Option<String>,
    pub default: bool,
    pub flatten: bool,
    pub try_from: Option<Type>,
    pub skip: bool,
    pub json: Option<JsonAttribute>,
}

pub fn parse_container_attributes(input: &[Attribute]) -> syn::Result<SqlxContainerAttributes> {
    let mut transparent = None;
    let mut repr = None;
    let mut type_name = None;
    let mut rename_all = None;
    let mut no_pg_array = None;
    let mut default = None;

    for attr in input {
        if attr.path().is_ident("sqlx") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("transparent") {
                    try_set!(transparent, true, attr);
                } else if meta.path.is_ident("no_pg_array") {
                    try_set!(no_pg_array, true, attr);
                } else if meta.path.is_ident("default") {
                    try_set!(default, true, attr);
                } else if meta.path.is_ident("rename_all") {
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;

                    let val = match lit.value().as_str() {
                        "lowercase" => RenameAll::LowerCase,
                        "snake_case" => RenameAll::SnakeCase,
                        "UPPERCASE" => RenameAll::UpperCase,
                        "SCREAMING_SNAKE_CASE" => RenameAll::ScreamingSnakeCase,
                        "kebab-case" => RenameAll::KebabCase,
                        "camelCase" => RenameAll::CamelCase,
                        "PascalCase" => RenameAll::PascalCase,
                        _ => fail!(lit, "unexpected value for rename_all"),
                    };

                    try_set!(rename_all, val, lit)
                } else if meta.path.is_ident("type_name") {
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    let name = TypeName {
                        val: lit.value(),
                        span: lit.span(),
                    };

                    try_set!(type_name, name, lit)
                } else {
                    fail!(meta.path, "unexpected attribute")
                }

                Ok(())
            })?;
        } else if attr.path().is_ident("repr") {
            let list: Punctuated<Meta, Token![,]> =
                attr.parse_args_with(<Punctuated<Meta, Token![,]>>::parse_terminated)?;

            if let Some(path) = list.iter().find_map(|f| f.require_path_only().ok()) {
                try_set!(repr, path.get_ident().unwrap().clone(), list);
            }
        }
    }

    Ok(SqlxContainerAttributes {
        transparent: transparent.unwrap_or(false),
        repr,
        type_name,
        rename_all,
        no_pg_array: no_pg_array.unwrap_or(false),
        default: default.unwrap_or(false),
    })
}

pub fn parse_child_attributes(input: &[Attribute]) -> syn::Result<SqlxChildAttributes> {
    let mut rename = None;
    let mut default = false;
    let mut try_from = None;
    let mut flatten = false;
    let mut skip: bool = false;
    let mut json = None;

    for attr in input.iter().filter(|a| a.path().is_ident("sqlx")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                meta.input.parse::<Token![=]>()?;
                let val: LitStr = meta.input.parse()?;
                try_set!(rename, val.value(), val);
            } else if meta.path.is_ident("try_from") {
                meta.input.parse::<Token![=]>()?;
                let val: LitStr = meta.input.parse()?;
                try_set!(try_from, val.parse()?, val);
            } else if meta.path.is_ident("default") {
                default = true;
            } else if meta.path.is_ident("flatten") {
                flatten = true;
            } else if meta.path.is_ident("skip") {
                skip = true;
            } else if meta.path.is_ident("json") {
                if meta.input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in meta.input);
                    let literal: Ident = content.parse()?;
                    assert_eq!(literal.to_string(), "nullable", "Unrecognized `json` attribute. Valid values are `json` or `json(nullable)`");
                    json = Some(JsonAttribute::Nullable);
                } else {
                    json = Some(JsonAttribute::NonNullable);
                }
            }

            Ok(())
        })?;

        if json.is_some() && flatten {
            fail!(
                attr,
                "Cannot use `json` and `flatten` together on the same field"
            );
        }
    }

    Ok(SqlxChildAttributes {
        rename,
        default,
        flatten,
        try_from,
        skip,
        json,
    })
}

pub fn check_transparent_attributes(
    input: &DeriveInput,
    field: &Field,
) -> syn::Result<SqlxContainerAttributes> {
    let attributes = parse_container_attributes(&input.attrs)?;

    assert_attribute!(
        attributes.rename_all.is_none(),
        "unexpected #[sqlx(rename_all = ..)]",
        field
    );

    let ch_attributes = parse_child_attributes(&field.attrs)?;

    assert_attribute!(
        ch_attributes.rename.is_none(),
        "unexpected #[sqlx(rename = ..)]",
        field
    );

    Ok(attributes)
}

pub fn check_enum_attributes(input: &DeriveInput) -> syn::Result<SqlxContainerAttributes> {
    let attributes = parse_container_attributes(&input.attrs)?;

    assert_attribute!(
        !attributes.transparent,
        "unexpected #[sqlx(transparent)]",
        input
    );

    Ok(attributes)
}

pub fn check_weak_enum_attributes(
    input: &DeriveInput,
    variants: &Punctuated<Variant, Comma>,
) -> syn::Result<SqlxContainerAttributes> {
    let attributes = check_enum_attributes(input)?;

    assert_attribute!(attributes.repr.is_some(), "expected #[repr(..)]", input);

    assert_attribute!(
        attributes.rename_all.is_none(),
        "unexpected #[sqlx(c = ..)]",
        input
    );

    for variant in variants {
        let attributes = parse_child_attributes(&variant.attrs)?;

        assert_attribute!(
            attributes.rename.is_none(),
            "unexpected #[sqlx(rename = ..)]",
            variant
        );
    }

    Ok(attributes)
}

pub fn check_strong_enum_attributes(
    input: &DeriveInput,
    _variants: &Punctuated<Variant, Comma>,
) -> syn::Result<SqlxContainerAttributes> {
    let attributes = check_enum_attributes(input)?;

    assert_attribute!(attributes.repr.is_none(), "unexpected #[repr(..)]", input);

    Ok(attributes)
}

pub fn check_struct_attributes(
    input: &DeriveInput,
    fields: &Punctuated<Field, Comma>,
) -> syn::Result<SqlxContainerAttributes> {
    let attributes = parse_container_attributes(&input.attrs)?;

    assert_attribute!(
        !attributes.transparent,
        "unexpected #[sqlx(transparent)]",
        input
    );

    assert_attribute!(
        attributes.rename_all.is_none(),
        "unexpected #[sqlx(rename_all = ..)]",
        input
    );

    assert_attribute!(attributes.repr.is_none(), "unexpected #[repr(..)]", input);

    for field in fields {
        let attributes = parse_child_attributes(&field.attrs)?;

        assert_attribute!(
            attributes.rename.is_none(),
            "unexpected #[sqlx(rename = ..)]",
            field
        );
    }

    Ok(attributes)
}
