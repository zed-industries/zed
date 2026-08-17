/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

use quote::{quote, ToTokens};

/// Implements one or more COM interfaces.
///
/// # Example
///
/// Here is a [more complete tutorial](https://kennykerr.ca/rust-getting-started/how-to-implement-com-interface.html).
///
/// ```rust,ignore
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// fn main() {
///     let rust_instance = Value(123);
///     let com_object: IValue = rust_instance.into();
///     // You can now call interface methods on com_object.
/// }
/// ```
#[proc_macro_attribute]
pub fn implement(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as ImplementAttributes);
    let interfaces_len = proc_macro2::Literal::usize_unsuffixed(attributes.implement.len());

    let identity_type = if let Some(first) = attributes.implement.first() {
        first.to_ident()
    } else {
        quote! { ::windows_core::IInspectable }
    };

    let original_type2 = original_type.clone();
    let original_type2 = syn::parse_macro_input!(original_type2 as syn::ItemStruct);
    let original_ident = original_type2.ident;
    let mut constraints = quote! {};

    if let Some(where_clause) = original_type2.generics.where_clause {
        where_clause.predicates.to_tokens(&mut constraints);
    }

    let generics = if original_type2.generics.lt_token.is_some() {
        let mut params = quote! {};
        original_type2.generics.params.to_tokens(&mut params);
        quote! { <#params> }
    } else {
        quote! { <> }
    };

    let impl_ident = quote::format_ident!("{}_Impl", original_ident);
    let vtbl_idents = attributes.implement.iter().map(|implement| implement.to_vtbl_ident());
    let vtbl_idents2 = vtbl_idents.clone();

    let vtable_news = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let offset = proc_macro2::Literal::isize_unsuffixed(-1 - enumerate as isize);
        quote! { #vtbl_ident::new::<Self, #original_ident::#generics, #offset>() }
    });

    let offset = attributes.implement.iter().enumerate().map(|(offset, _)| proc_macro2::Literal::usize_unsuffixed(offset));

    let queries = attributes.implement.iter().enumerate().map(|(count, implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let offset = proc_macro2::Literal::usize_unsuffixed(count);
        quote! {
            else if #vtbl_ident::matches(iid) {
                &self.vtables.#offset as *const _ as *mut _
            }
        }
    });

    let trust_level = proc_macro2::Literal::usize_unsuffixed(attributes.trust_level);

    let conversions = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let interface_ident = implement.to_ident();
        let offset = proc_macro2::Literal::usize_unsuffixed(enumerate);
        quote! {
            impl #generics ::core::convert::From<#original_ident::#generics> for #interface_ident where #constraints {
                fn from(this: #original_ident::#generics) -> Self {
                    let this = #impl_ident::#generics::new(this);
                    let mut this = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                    let vtable_ptr = &this.vtables.#offset;
                    // SAFETY: interfaces are in-memory equivalent to pointers to their vtables.
                    unsafe { ::core::mem::transmute(vtable_ptr) }
                }
            }
            impl #generics ::windows_core::AsImpl<#original_ident::#generics> for #interface_ident where #constraints {
                // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
                // is guaranteed to live at least as long as `self`.
                unsafe fn as_impl(&self) -> &#original_ident::#generics {
                    let this = ::windows_core::Interface::as_raw(self);
                    // Subtract away the vtable offset plus 1, for the `identity` field, to get
                    // to the impl struct which contains that original implementation type.
                    let this = (this as *mut *mut ::core::ffi::c_void).sub(1 + #offset) as *mut #impl_ident::#generics;
                    &(*this).this
                }
            }
        }
    });

    let tokens = quote! {
        #[repr(C)]
        struct #impl_ident #generics where #constraints {
            identity: *const ::windows_core::IInspectable_Vtbl,
            vtables: (#(*const #vtbl_idents,)*),
             this: #original_ident::#generics,
            count: ::windows_core::imp::WeakRefCount,
        }
        impl #generics #impl_ident::#generics where #constraints {
            const VTABLES: (#(#vtbl_idents2,)*) = (#(#vtable_news,)*);
            const IDENTITY: ::windows_core::IInspectable_Vtbl = ::windows_core::IInspectable_Vtbl::new::<Self, #identity_type, 0>();
            fn new(this: #original_ident::#generics) -> Self {
                Self {
                    identity: &Self::IDENTITY,
                    vtables:(#(&Self::VTABLES.#offset,)*),
                    this,
                    count: ::windows_core::imp::WeakRefCount::new(),
                }
            }
        }
         impl #generics ::windows_core::IUnknownImpl for #impl_ident::#generics where #constraints {
            type Impl = #original_ident::#generics;
            fn get_impl(&self) -> &Self::Impl {
                &self.this
            }
            unsafe fn QueryInterface(&self, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
                if iid.is_null() || interface.is_null() {
                    return ::windows_core::HRESULT(-2147467261); // E_POINTER
                }

                let iid = &*iid;

                *interface = if iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID
                    || iid == &<::windows_core::IInspectable as ::windows_core::Interface>::IID
                    || iid == &<::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID {
                        &self.identity as *const _ as *mut _
                } #(#queries)* else {
                    ::core::ptr::null_mut()
                };

                if !(*interface).is_null() {
                    self.count.add_ref();
                    return ::windows_core::HRESULT(0);
                }

                *interface = self.count.query(iid, &self.identity as *const _ as *mut _);

                if (*interface).is_null() {
                    ::windows_core::HRESULT(-2147467262) // E_NOINTERFACE
                } else {
                    ::windows_core::HRESULT(0)
                }
            }
            fn AddRef(&self) -> u32 {
                self.count.add_ref()
            }
            unsafe fn Release(&self) -> u32 {
                let remaining = self.count.release();
                if remaining == 0 {
                    _ = ::std::boxed::Box::from_raw(self as *const Self as *mut Self);
                }
                remaining
            }
            unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
                if value.is_null() {
                    return ::windows_core::HRESULT(-2147467261); // E_POINTER
                }
                *value = #trust_level;
                ::windows_core::HRESULT(0)
            }
         }
        impl #generics #original_ident::#generics where #constraints {
            /// Try casting as the provided interface
            ///
            /// # Safety
            ///
            /// This function can only be safely called if `self` has been heap allocated and pinned using
            /// the mechanisms provided by `implement` macro.
            unsafe fn cast<I: ::windows_core::Interface>(&self) -> ::windows_core::Result<I> {
                let boxed = (self as *const _ as *const *mut ::core::ffi::c_void).sub(1 + #interfaces_len) as *mut #impl_ident::#generics;
                let mut result = ::std::ptr::null_mut();
                _ = <#impl_ident::#generics as ::windows_core::IUnknownImpl>::QueryInterface(&*boxed, &I::IID, &mut result);
                ::windows_core::Type::from_abi(result)
            }
        }
        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            fn from(this: #original_ident::#generics) -> Self {
                let this = #impl_ident::#generics::new(this);
                let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                unsafe {
                    ::core::mem::transmute(&boxed.identity)
                }
            }
        }
        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IInspectable where #constraints {
            fn from(this: #original_ident::#generics) -> Self {
                let this = #impl_ident::#generics::new(this);
                let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                unsafe {
                    ::core::mem::transmute(&boxed.identity)
                }
            }
        }
        #(#conversions)*
    };

    let mut tokens: proc_macro::TokenStream = tokens.into();
    tokens.extend(core::iter::once(original_type));
    tokens
}

#[derive(Default)]
struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,
}

impl ImplementType {
    fn to_ident(&self) -> proc_macro2::TokenStream {
        let type_name = syn::parse_str::<proc_macro2::TokenStream>(&self.type_name).expect("Invalid token stream");
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
}

impl syn::parse::Parse for ImplementAttributes {
    fn parse(cursor: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let mut input = Self::default();

        while !cursor.is_empty() {
            input.parse_implement(cursor)?;
        }

        Ok(input)
    }
}

impl ImplementAttributes {
    fn parse_implement(&mut self, cursor: syn::parse::ParseStream<'_>) -> syn::parse::Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(&tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<syn::Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(&mut self, tree: &UseTree2, namespace: &mut String) -> syn::parse::Result<()> {
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
        }

        Ok(())
    }
}

enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
    TrustLevel(usize),
}

impl UseTree2 {
    fn to_element_type(&self, namespace: &mut String) -> syn::parse::Result<ImplementType> {
        match self {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(namespace)
            }
            UseTree2::Name(input) => {
                let mut type_name = input.ident.to_string();

                if !namespace.is_empty() {
                    type_name = format!("{namespace}::{type_name}");
                }

                let mut generics = vec![];

                for g in &input.generics {
                    generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(ImplementType { type_name, generics })
            }
            UseTree2::Group(input) => Err(syn::parse::Error::new(input.brace_token.span.join(), "Syntax not supported")),
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
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(syn::Ident::parse_any)?;
            if input.peek(syn::Token![::]) {
                input.parse::<syn::Token![::]>()?;
                Ok(UseTree2::Path(UsePath2 { ident, tree: Box::new(input.parse()?) }))
            } else if input.peek(syn::Token![=]) {
                if ident != "TrustLevel" {
                    return Err(syn::parse::Error::new(ident.span(), "Unrecognized key-value pair"));
                }
                input.parse::<syn::Token![=]>()?;
                let span = input.span();
                let value = input.call(syn::Ident::parse_any)?;
                match value.to_string().as_str() {
                    "Partial" => Ok(UseTree2::TrustLevel(1)),
                    "Full" => Ok(UseTree2::TrustLevel(2)),
                    _ => Err(syn::parse::Error::new(span, "`TrustLevel` must be `Partial` or `Full`")),
                }
            } else {
                let generics = if input.peek(syn::Token![<]) {
                    input.parse::<syn::Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<syn::Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<syn::Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            let brace_token = syn::braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse, syn::Token![,])?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}
