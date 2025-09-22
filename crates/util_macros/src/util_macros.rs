use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Expr, ExprArray, ExprLit, Fields, Lit, LitStr, MetaNameValue, Token,
    parse_macro_input, punctuated::Punctuated,
};

/// A macro used in tests for cross-platform path string literals in tests. On Windows it replaces
/// `/` with `\\` and adds `C:` to the beginning of absolute paths. On other platforms, the path is
/// returned unmodified.
///
/// # Example
/// ```rust
/// use util_macros::path;
///
/// let path = path!("/Users/user/file.txt");
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, "C:\\Users\\user\\file.txt");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(path, "/Users/user/file.txt");
/// ```
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);

    #[cfg(target_os = "windows")]
    {
        let mut path = path.value();
        path = path.replace("/", "\\");
        if path.starts_with("\\") {
            path = format!("C:{}", path);
        }
        return TokenStream::from(quote! {
            #path
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        let path = path.value();
        return TokenStream::from(quote! {
            #path
        });
    }
}

/// This macro replaces the path prefix `file:///` with `file:///C:/` for Windows.
/// But if the target OS is not Windows, the URI is returned as is.
///
/// # Example
/// ```rust
/// use util_macros::uri;
///
/// let uri = uri!("file:///path/to/file");
/// #[cfg(target_os = "windows")]
/// assert_eq!(uri, "file:///C:/path/to/file");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(uri, "file:///path/to/file");
/// ```
#[proc_macro]
pub fn uri(input: TokenStream) -> TokenStream {
    let uri = parse_macro_input!(input as LitStr);
    let uri = uri.value();

    #[cfg(target_os = "windows")]
    let uri = uri.replace("file:///", "file:///C:/");

    TokenStream::from(quote! {
        #uri
    })
}

/// This macro replaces the line endings `\n` with `\r\n` for Windows.
/// But if the target OS is not Windows, the line endings are returned as is.
///
/// # Example
/// ```rust
/// use util_macros::line_endings;
///
/// let text = line_endings!("Hello\nWorld");
/// #[cfg(target_os = "windows")]
/// assert_eq!(text, "Hello\r\nWorld");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(text, "Hello\nWorld");
/// ```
#[proc_macro]
pub fn line_endings(input: TokenStream) -> TokenStream {
    let text = parse_macro_input!(input as LitStr);
    let text = text.value();

    #[cfg(target_os = "windows")]
    let text = text.replace("\n", "\r\n");

    TokenStream::from(quote! {
        #text
    })
}

#[proc_macro_derive(FieldAccessByEnum, attributes(field_access_by_enum))]
pub fn derive_field_access_by_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let mut enum_name = None;
    let mut enum_attrs: Vec<TokenStream2> = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("field_access_by_enum") {
            let name_values: Punctuated<MetaNameValue, Token![,]> =
                attr.parse_args_with(Punctuated::parse_terminated).unwrap();
            for name_value in name_values {
                if name_value.path.is_ident("enum_name") {
                    let value = name_value.value;
                    match value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(name),
                            ..
                        }) => enum_name = Some(name.value()),
                        _ => panic!("Expected string literal in enum_name attribute"),
                    }
                } else if name_value.path.is_ident("enum_attrs") {
                    let value = name_value.value;
                    match value {
                        Expr::Array(ExprArray { elems, .. }) => {
                            for elem in elems {
                                enum_attrs.push(quote!(#[#elem]));
                            }
                        }
                        _ => panic!("Expected array literal in enum_attr attribute"),
                    }
                } else {
                    if let Some(ident) = name_value.path.get_ident() {
                        panic!("Unrecognized argument name {}", ident);
                    } else {
                        panic!("Unrecognized argument {:?}", name_value.path);
                    }
                }
            }
        }
    }
    let Some(enum_name) = enum_name else {
        panic!("#[field_access_by_enum(enum_name = \"...\")] attribute is required");
    };
    let enum_ident = format_ident!("{}", enum_name);

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("FieldAccessByEnum can only be derived for structs with named fields"),
        },
        _ => panic!("FieldAccessByEnum can only be derived for structs"),
    };

    if fields.is_empty() {
        panic!("FieldAccessByEnum cannot be derived for structs with no fields");
    }

    let mut enum_variants = Vec::new();
    let mut get_match_arms = Vec::new();
    let mut set_match_arms = Vec::new();
    let mut field_types = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let variant_name = field_name.to_string().to_case(Case::Pascal);
        let variant_ident = format_ident!("{}", variant_name);
        let field_type = &field.ty;

        enum_variants.push(variant_ident.clone());
        field_types.push(field_type);

        get_match_arms.push(quote! {
            #enum_ident::#variant_ident => &self.#field_name,
        });

        set_match_arms.push(quote! {
            #enum_ident::#variant_ident => self.#field_name = value,
        });
    }

    let first_type = &field_types[0];
    let all_same_type = field_types
        .iter()
        .all(|ty| quote!(#ty).to_string() == quote!(#first_type).to_string());
    if !all_same_type {
        panic!("Fields have different types.");
    }
    let field_value_type = quote! { #first_type };

    let expanded = quote! {
        #(#enum_attrs)*
        pub enum #enum_ident {
            #(#enum_variants),*
        }

        impl util::FieldAccessByEnum<#field_value_type> for #struct_name {
            type Field = #enum_ident;

            fn get_field_by_enum(&self, field: Self::Field) -> &#field_value_type {
                match field {
                    #(#get_match_arms)*
                }
            }

            fn set_field_by_enum(&mut self, field: Self::Field, value: #field_value_type) {
                match field {
                    #(#set_match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
