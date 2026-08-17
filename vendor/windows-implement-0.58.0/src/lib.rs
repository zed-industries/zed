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
pub fn implement(
    attributes: proc_macro::TokenStream,
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as ImplementAttributes);
    let interfaces_len = proc_macro2::Literal::usize_unsuffixed(attributes.implement.len());

    let identity_type = if let Some(first) = attributes.implement.first() {
        first.to_ident()
    } else {
        quote! { ::windows_core::IInspectable }
    };

    let original_type2 = original_type.clone();
    let original_type2 = syn::parse_macro_input!(original_type2 as syn::ItemStruct);
    let vis = &original_type2.vis;
    let original_ident = &original_type2.ident;
    let mut constraints = quote! {};

    if let Some(where_clause) = &original_type2.generics.where_clause {
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
    let vtbl_idents = attributes
        .implement
        .iter()
        .map(|implement| implement.to_vtbl_ident());
    let vtbl_idents2 = vtbl_idents.clone();

    let vtable_news = attributes
        .implement
        .iter()
        .enumerate()
        .map(|(enumerate, implement)| {
            let vtbl_ident = implement.to_vtbl_ident();
            let offset = proc_macro2::Literal::isize_unsuffixed(-1 - enumerate as isize);
            quote! { #vtbl_ident::new::<Self, #offset>() }
        });

    let offset = attributes
        .implement
        .iter()
        .enumerate()
        .map(|(offset, _)| proc_macro2::Literal::usize_unsuffixed(offset));

    let queries = attributes
        .implement
        .iter()
        .enumerate()
        .map(|(count, implement)| {
            let vtbl_ident = implement.to_vtbl_ident();
            let offset = proc_macro2::Literal::usize_unsuffixed(count);
            quote! {
                else if #vtbl_ident::matches(iid) {
                    &self.vtables.#offset as *const _ as *mut _
                }
            }
        });

    // Dynamic casting requires that the object not contain non-static lifetimes.
    let enable_dyn_casting = original_type2.generics.lifetimes().count() == 0;
    let dynamic_cast_query = if enable_dyn_casting {
        quote! {
            else if *iid == ::windows_core::DYNAMIC_CAST_IID {
                // DYNAMIC_CAST_IID is special. We _do not_ increase the reference count for this pseudo-interface.
                // Also, instead of returning an interface pointer, we simply write the `&dyn Any` directly to the
                // 'interface' pointer. Since the size of `&dyn Any` is 2 pointers, not one, the caller must be
                // prepared for this. This is not a normal QueryInterface call.
                //
                // See the `Interface::cast_to_any` method, which is the only caller that should use DYNAMIC_CAST_ID.
                (interface as *mut *const dyn core::any::Any).write(self as &dyn ::core::any::Any as *const dyn ::core::any::Any);
                return ::windows_core::HRESULT(0);
            }
        }
    } else {
        quote!()
    };

    // The distance from the beginning of the generated type to the 'this' field, in units of pointers (not bytes).
    let offset_of_this_in_pointers = 1 + attributes.implement.len();
    let offset_of_this_in_pointers_token =
        proc_macro2::Literal::usize_unsuffixed(offset_of_this_in_pointers);

    let trust_level = proc_macro2::Literal::usize_unsuffixed(attributes.trust_level);

    let conversions = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let interface_ident = implement.to_ident();
        let offset = proc_macro2::Literal::usize_unsuffixed(enumerate);
        quote! {
            impl #generics ::core::convert::From<#original_ident::#generics> for #interface_ident where #constraints {
                #[inline(always)]
                fn from(this: #original_ident::#generics) -> Self {
                    let com_object = ::windows_core::ComObject::new(this);
                    com_object.into_interface()
                }
            }

            impl #generics ::windows_core::ComObjectInterface<#interface_ident> for #impl_ident::#generics where #constraints {
                #[inline(always)]
                fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, #interface_ident> {
                    unsafe {
                        let interface_ptr = &self.vtables.#offset;
                        ::core::mem::transmute(interface_ptr)
                    }
                }
            }

            impl #generics ::windows_core::AsImpl<#original_ident::#generics> for #interface_ident where #constraints {
                // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
                // is guaranteed to live at least as long as `self`.
                #[inline(always)]
                unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics> {
                    let this = ::windows_core::Interface::as_raw(self);
                    // Subtract away the vtable offset plus 1, for the `identity` field, to get
                    // to the impl struct which contains that original implementation type.
                    let this = (this as *mut *mut ::core::ffi::c_void).sub(1 + #offset) as *mut #impl_ident::#generics;
                    ::core::ptr::NonNull::new_unchecked(::core::ptr::addr_of!((*this).this) as *const #original_ident::#generics as *mut #original_ident::#generics)
                }
            }
        }
    });

    let tokens = quote! {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #vis struct #impl_ident #generics where #constraints {
            identity: &'static ::windows_core::IInspectable_Vtbl,
            vtables: (#(&'static #vtbl_idents,)*),
            this: #original_ident::#generics,
            count: ::windows_core::imp::WeakRefCount,
        }

        impl #generics #impl_ident::#generics where #constraints {
            const VTABLES: (#(#vtbl_idents2,)*) = (#(#vtable_news,)*);
            const IDENTITY: ::windows_core::IInspectable_Vtbl = ::windows_core::IInspectable_Vtbl::new::<Self, #identity_type, 0>();
        }

        impl #generics ::windows_core::ComObjectInner for #original_ident::#generics where #constraints {
            type Outer = #impl_ident::#generics;

            // IMPORTANT! This function handles assembling the "boxed" type of a COM object.
            // It immediately moves the box into a heap allocation (box) and returns only a ComObject
            // reference that points to it. We intentionally _do not_ expose any owned instances of
            // Foo_Impl to safe Rust code, because doing so would allow unsound behavior in safe Rust
            // code, due to the adjustments of the reference count that Foo_Impl permits.
            //
            // This is why this function returns ComObject<Self> instead of returning #impl_ident.

            fn into_object(self) -> ::windows_core::ComObject<Self> {
                let boxed = ::windows_core::imp::Box::new(#impl_ident::#generics {
                    identity: &#impl_ident::#generics::IDENTITY,
                    vtables: (#(&#impl_ident::#generics::VTABLES.#offset,)*),
                    this: self,
                    count: ::windows_core::imp::WeakRefCount::new(),
                });
                unsafe {
                    let ptr = ::windows_core::imp::Box::into_raw(boxed);
                    ::windows_core::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(ptr)
                    )
                }
            }
        }

        impl #generics ::windows_core::IUnknownImpl for #impl_ident::#generics where #constraints {
            type Impl = #original_ident::#generics;

            #[inline(always)]
            fn get_impl(&self) -> &Self::Impl {
                &self.this
            }

            #[inline(always)]
            fn get_impl_mut(&mut self) -> &mut Self::Impl {
                &mut self.this
            }

            #[inline(always)]
            fn is_reference_count_one(&self) -> bool {
                self.count.is_one()
            }

            #[inline(always)]
            fn into_inner(self) -> Self::Impl {
                self.this
            }

            unsafe fn QueryInterface(&self, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
                if iid.is_null() || interface.is_null() {
                    return ::windows_core::imp::E_POINTER;
                }

                let iid = &*iid;

                let interface_ptr: *mut ::core::ffi::c_void = if iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID
                    || iid == &<::windows_core::IInspectable as ::windows_core::Interface>::IID
                    || iid == &<::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID {
                        &self.identity as *const _ as *mut _
                }
                #(#queries)*
                #dynamic_cast_query
                else {
                    ::core::ptr::null_mut()
                };

                if !interface_ptr.is_null() {
                    *interface = interface_ptr;
                    self.count.add_ref();
                    return ::windows_core::HRESULT(0);
                }

                let interface_ptr = self.count.query(iid, &self.identity as *const _ as *mut _);
                *interface = interface_ptr;

                if interface_ptr.is_null() {
                    ::windows_core::imp::E_NOINTERFACE
                } else {
                    ::windows_core::HRESULT(0)
                }
            }

            #[inline(always)]
            fn AddRef(&self) -> u32 {
                self.count.add_ref()
            }

            #[inline(always)]
            unsafe fn Release(self_: *mut Self) -> u32 {
                let remaining = (*self_).count.release();
                if remaining == 0 {
                    _ = ::windows_core::imp::Box::from_raw(self_);
                }
                remaining
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
                if value.is_null() {
                    return ::windows_core::imp::E_POINTER;
                }
                *value = #trust_level;
                ::windows_core::HRESULT(0)
            }

            unsafe fn from_inner_ref(inner: &Self::Impl) -> &Self {
                &*((inner as *const Self::Impl as *const *const ::core::ffi::c_void)
                    .sub(#offset_of_this_in_pointers_token) as *const Self)
            }

            fn to_object(&self) -> ::windows_core::ComObject<Self::Impl> {
                self.count.add_ref();
                unsafe {
                    ::windows_core::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(self as *const Self as *mut Self)
                    )
                }
            }

            const INNER_OFFSET_IN_POINTERS: usize = #offset_of_this_in_pointers_token;
        }

        impl #generics #original_ident::#generics where #constraints {
            /// Try casting as the provided interface
            ///
            /// # Safety
            ///
            /// This function can only be safely called if `self` has been heap allocated and pinned using
            /// the mechanisms provided by `implement` macro.
            #[inline(always)]
            unsafe fn cast<I: ::windows_core::Interface>(&self) -> ::windows_core::Result<I> {
                let boxed = (self as *const _ as *const *mut ::core::ffi::c_void).sub(1 + #interfaces_len) as *mut #impl_ident::#generics;
                let mut result = ::core::ptr::null_mut();
                _ = <#impl_ident::#generics as ::windows_core::IUnknownImpl>::QueryInterface(&*boxed, &I::IID, &mut result);
                ::windows_core::Type::from_abi(result)
            }
        }

        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IInspectable where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl #generics ::windows_core::ComObjectInterface<::windows_core::IUnknown> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
                unsafe {
                    let interface_ptr = &self.identity;
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }

        impl #generics ::windows_core::ComObjectInterface<::windows_core::IInspectable> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IInspectable> {
                unsafe {
                    let interface_ptr = &self.identity;
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }

        impl #generics ::windows_core::AsImpl<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
            // is guaranteed to live at least as long as `self`.
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics> {
                let this = ::windows_core::Interface::as_raw(self);
                // Subtract away the vtable offset plus 1, for the `identity` field, to get
                // to the impl struct which contains that original implementation type.
                let this = (this as *mut *mut ::core::ffi::c_void).sub(1) as *mut #impl_ident::#generics;
                ::core::ptr::NonNull::new_unchecked(::core::ptr::addr_of!((*this).this) as *const #original_ident::#generics as *mut #original_ident::#generics)
            }
        }

        impl #generics ::core::ops::Deref for #impl_ident::#generics where #constraints {
            type Target = #original_ident::#generics;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.this
            }
        }

        // We intentionally do not provide a DerefMut impl, due to paranoia around soundness.

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

                Ok(ImplementType {
                    type_name,
                    generics,
                })
            }
            UseTree2::Group(input) => Err(syn::parse::Error::new(
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
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(syn::Ident::parse_any)?;
            if input.peek(syn::Token![::]) {
                input.parse::<syn::Token![::]>()?;
                Ok(UseTree2::Path(UsePath2 {
                    ident,
                    tree: Box::new(input.parse()?),
                }))
            } else if input.peek(syn::Token![=]) {
                if ident != "TrustLevel" {
                    return Err(syn::parse::Error::new(
                        ident.span(),
                        "Unrecognized key-value pair",
                    ));
                }
                input.parse::<syn::Token![=]>()?;
                let span = input.span();
                let value = input.call(syn::Ident::parse_any)?;
                match value.to_string().as_str() {
                    "Partial" => Ok(UseTree2::TrustLevel(1)),
                    "Full" => Ok(UseTree2::TrustLevel(2)),
                    _ => Err(syn::parse::Error::new(
                        span,
                        "`TrustLevel` must be `Partial` or `Full`",
                    )),
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
