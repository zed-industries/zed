//! Implement COM interfaces for Rust types.
//!
//! Take a look at [macro@implement] for an example.
//!
//! Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>

use quote::{quote, ToTokens};

mod r#gen;
use r#gen::gen_all;

#[cfg(test)]
mod tests;

/// Implements one or more COM interfaces.
///
/// # Example
/// ```rust,no_run
/// use windows_core::*;
///
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value_Impl {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// let object: IValue = Value(123).into();
/// // Call interface methods...
/// ```
#[proc_macro_attribute]
pub fn implement(
    attributes: proc_macro::TokenStream,
    type_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    implement_core(attributes.into(), type_tokens.into()).into()
}

fn implement_core(
    attributes: proc_macro2::TokenStream,
    item_tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let attributes = syn::parse2::<ImplementAttributes>(attributes).unwrap();
    let original_type = syn::parse2::<syn::ItemStruct>(item_tokens).unwrap();

    // Do a little thinking and assemble ImplementInputs.  We pass ImplementInputs to
    // all of our gen_* function.
    let inputs = ImplementInputs {
        original_ident: original_type.ident.clone(),
        interface_chains: convert_implements_to_interface_chains(attributes.implement),
        trust_level: attributes.trust_level,
        agile: attributes.agile,
        impl_ident: quote::format_ident!("{}_Impl", &original_type.ident),
        constraints: {
            if let Some(where_clause) = &original_type.generics.where_clause {
                where_clause.predicates.to_token_stream()
            } else {
                quote!()
            }
        },
        generics: if !original_type.generics.params.is_empty() {
            let mut params = quote! {};
            original_type.generics.params.to_tokens(&mut params);
            quote! { <#params> }
        } else {
            quote! { <> }
        },
        is_generic: !original_type.generics.params.is_empty(),
        original_type,
    };

    let items = gen_all(&inputs);
    let mut tokens = inputs.original_type.into_token_stream();
    for item in items {
        tokens.extend(item.into_token_stream());
    }

    tokens
}

/// This provides the inputs to the `gen_*` functions, which generate the proc macro output.
struct ImplementInputs {
    /// The user's type that was marked with `#[implement]`.
    original_type: syn::ItemStruct,

    /// The identifier for the user's original type definition.
    original_ident: syn::Ident,

    /// The list of interface chains that this type implements.
    interface_chains: Vec<InterfaceChain>,

    /// The "trust level", which is returned by `IInspectable::GetTrustLevel`.
    trust_level: usize,

    /// Determines whether `IAgileObject` and `IMarshal` are implemented automatically.
    agile: bool,

    /// The identifier of the `Foo_Impl` type.
    impl_ident: syn::Ident,

    /// The list of constraints needed for this `Foo_Impl` type.
    constraints: proc_macro2::TokenStream,

    /// The list of generic parameters for this `Foo_Impl` type, including `<` and `>`.
    /// If there are no generics, this contains `<>`.
    generics: proc_macro2::TokenStream,

    /// True if the user type has any generic parameters.
    is_generic: bool,
}

/// Describes one COM interface chain.
struct InterfaceChain {
    /// The name of the field for the vtable chain, e.g. `interface4_ifoo`.
    field_ident: syn::Ident,

    /// The name of the associated constant item for the vtable chain's initializer,
    /// e.g. `INTERFACE4_IFOO_VTABLE`.
    vtable_const_ident: syn::Ident,

    implement: ImplementType,
}

struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,

    /// The best span for diagnostics.
    span: proc_macro2::Span,
}

impl ImplementType {
    fn to_ident(&self) -> proc_macro2::TokenStream {
        let type_name = syn::parse_str::<proc_macro2::TokenStream>(&self.type_name)
            .expect("Invalid token stream");
        let generics = self.generics.iter().map(|g| g.to_ident());
        quote! { #type_name<#(#generics,)*> }
    }
    fn to_vtbl_ident(&self) -> proc_macro2::TokenStream {
        let ident = self.to_ident();
        quote! {
            <#ident as ::windows_core::Interface>::Vtable
        }
    }
}

#[derive(Default)]
struct ImplementAttributes {
    pub implement: Vec<ImplementType>,
    pub trust_level: usize,
    pub agile: bool,
}

impl syn::parse::Parse for ImplementAttributes {
    fn parse(cursor: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut input = Self {
            agile: true,
            ..Default::default()
        };

        while !cursor.is_empty() {
            input.parse_implement(cursor)?;
        }

        Ok(input)
    }
}

impl ImplementAttributes {
    fn parse_implement(&mut self, cursor: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(&tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<syn::Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(
        &mut self,
        tree: &UseTree2,
        namespace: &mut String,
    ) -> syn::parse::Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(&input.tree, namespace)?;
            }
            UseTree2::Name(_) => {
                self.implement.push(tree.to_element_type(namespace)?);
            }
            UseTree2::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(tree, namespace)?;
                }
            }
            UseTree2::TrustLevel(input) => self.trust_level = *input,
            UseTree2::Agile(agile) => self.agile = *agile,
        }

        Ok(())
    }
}

enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
    TrustLevel(usize),
    Agile(bool),
}

impl UseTree2 {
    fn to_element_type(&self, namespace: &mut String) -> syn::parse::Result<ImplementType> {
        match self {
            Self::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(namespace)
            }
            Self::Name(input) => {
                let mut type_name = input.ident.to_string();
                let span = input.ident.span();

                if !namespace.is_empty() {
                    type_name = format!("{namespace}::{type_name}");
                }

                let mut generics = vec![];

                for g in &input.generics {
                    generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(ImplementType {
                    type_name,
                    generics,
                    span,
                })
            }
            Self::Group(input) => Err(syn::parse::Error::new(
                input.brace_token.span.join(),
                "Syntax not supported",
            )),
            _ => unimplemented!(),
        }
    }
}

struct UsePath2 {
    pub ident: syn::Ident,
    pub tree: Box<UseTree2>,
}

struct UseName2 {
    pub ident: syn::Ident,
    pub generics: Vec<UseTree2>,
}

struct UseGroup2 {
    pub brace_token: syn::token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, syn::Token![,]>,
}

impl syn::parse::Parse for UseTree2 {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(syn::Ident::parse_any)?;
            if input.peek(syn::Token![::]) {
                input.parse::<syn::Token![::]>()?;
                Ok(Self::Path(UsePath2 {
                    ident,
                    tree: Box::new(input.parse()?),
                }))
            } else if input.peek(syn::Token![=]) {
                if ident == "TrustLevel" {
                    input.parse::<syn::Token![=]>()?;
                    let span = input.span();
                    let value = input.call(syn::Ident::parse_any)?;
                    match value.to_string().as_str() {
                        "Partial" => Ok(Self::TrustLevel(1)),
                        "Full" => Ok(Self::TrustLevel(2)),
                        _ => Err(syn::parse::Error::new(
                            span,
                            "`TrustLevel` must be `Partial` or `Full`",
                        )),
                    }
                } else if ident == "Agile" {
                    input.parse::<syn::Token![=]>()?;
                    let span = input.span();
                    let value = input.call(syn::Ident::parse_any)?;
                    match value.to_string().as_str() {
                        "true" => Ok(Self::Agile(true)),
                        "false" => Ok(Self::Agile(false)),
                        _ => Err(syn::parse::Error::new(
                            span,
                            "`Agile` must be `true` or `false`",
                        )),
                    }
                } else {
                    Err(syn::parse::Error::new(
                        ident.span(),
                        "Unrecognized key-value pair",
                    ))
                }
            } else {
                let generics = if input.peek(syn::Token![<]) {
                    input.parse::<syn::Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<Self>()?);

                        if input.parse::<syn::Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<syn::Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(Self::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            let brace_token = syn::braced!(content in input);
            let items = content.parse_terminated(Self::parse, syn::Token![,])?;

            Ok(Self::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}

fn convert_implements_to_interface_chains(implements: Vec<ImplementType>) -> Vec<InterfaceChain> {
    let mut chains = Vec::with_capacity(implements.len());

    for (i, implement) in implements.into_iter().enumerate() {
        // Create an identifier for this interface chain.
        // We only use this for naming fields; it is never visible to the developer.
        // This helps with debugging.
        //
        // We use i + 1 so that it matches the numbering of our interface offsets. Interface 0
        // is the "identity" interface.

        let mut ident_string = format!("interface{}", i + 1);

        let suffix = get_interface_ident_suffix(&implement.type_name);
        if !suffix.is_empty() {
            ident_string.push('_');
            ident_string.push_str(&suffix);
        }
        let field_ident = syn::Ident::new(&ident_string, implement.span);

        let mut vtable_const_string = ident_string.clone();
        vtable_const_string.make_ascii_uppercase();
        vtable_const_string.insert_str(0, "VTABLE_");
        let vtable_const_ident = syn::Ident::new(&vtable_const_string, implement.span);

        chains.push(InterfaceChain {
            implement,
            field_ident,
            vtable_const_ident,
        });
    }

    chains
}

fn get_interface_ident_suffix(type_name: &str) -> String {
    let mut suffix = String::new();
    for c in type_name.chars() {
        let c = c.to_ascii_lowercase();

        if suffix.len() >= 20 {
            break;
        }

        if c.is_ascii_alphanumeric() {
            suffix.push(c);
        }
    }

    suffix
}
