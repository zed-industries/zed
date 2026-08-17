//! Define COM interfaces to call or implement.
//!
//! Take a look at [macro@interface] for an example.
//!
//! Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>

use quote::quote;
use syn::spanned::Spanned;

/// Defines a COM interface to call or implement.
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
pub fn interface(
    attributes: proc_macro::TokenStream,
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let guid = syn::parse_macro_input!(attributes as Guid);
    let interface = syn::parse_macro_input!(original_type as Interface);
    let tokens = match interface.gen_tokens(&guid) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };
    tokens.into()
}

macro_rules! bail {
    ($item:expr, $($msg:tt),*) => {
        return Err(syn::Error::new($item.span(), std::fmt::format(format_args!($($msg),*))));
    };

}

macro_rules! unexpected_token {
    ($item:expr, $msg:expr) => {
        if let Some(i) = $item {
            bail!(i, "unexpected {}", $msg);
        }
    };
}
macro_rules! expected_token {
    ($sig:tt.$item:tt(), $msg:expr) => {
        if let None = $sig.$item() {
            bail!($sig, "expected {}", $msg);
        }
    };
}

/// Parsed interface
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
/// //^ parses this
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Interface {
    visibility: syn::Visibility,
    name: syn::Ident,
    parent: Option<syn::Path>,
    methods: Vec<InterfaceMethod>,
    docs: Vec<syn::Attribute>,
}

impl Interface {
    /// Generates all the code needed for a COM interface
    fn gen_tokens(&self, guid: &Guid) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.visibility;
        let name = &self.name;
        let docs = &self.docs;
        let parent = self.parent_type();
        let vtable_name = quote::format_ident!("{}_Vtbl", name);
        let guid = guid.to_tokens()?;
        let implementation = self.gen_implementation();
        let com_trait = self.get_com_trait();
        let vtable = self.gen_vtable(&vtable_name);
        let conversions = self.gen_conversions();

        Ok(quote! {
            #[repr(transparent)]
            #(#docs)*
            #vis struct #name(#parent);
            #implementation
            unsafe impl ::windows_core::Interface for #name {
                type Vtable = #vtable_name;
                const IID: ::windows_core::GUID = #guid;
            }
            impl ::windows_core::RuntimeName for #name {}
            impl ::core::ops::Deref for #name {
                type Target = #parent;
                fn deref(&self) -> &Self::Target {
                    unsafe { ::core::mem::transmute(self) }
                }
            }
            #com_trait
            #vtable
            #conversions
        })
    }

    /// Generates the methods users can call on the COM interface pointer
    fn gen_implementation(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let vis = &m.visibility;
                let name = &m.name;

                let generics = m.gen_consume_generics();
                let params = m.gen_consume_params();
                let args = m.gen_consume_args();
                let ret = &m.ret;

                if m.is_result() {
                    quote! {
                    #[inline(always)]
                    #vis unsafe fn #name<#(#generics),*>(&self, #(#params),*) #ret {
                            (::windows_core::Interface::vtable(self).#name)(::windows_core::Interface::as_raw(self), #(#args),*).ok()
                        }
                    }
                } else {
                    quote! {
                        #[inline(always)]
                        #vis unsafe fn #name<#(#generics),*>(&self, #(#params),*) #ret {
                            (::windows_core::Interface::vtable(self).#name)(::windows_core::Interface::as_raw(self), #(#args),*)
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl #name {
                #(#methods)*
            }
        }
    }

    fn get_com_trait(&self) -> proc_macro2::TokenStream {
        let name = quote::format_ident!("{}_Impl", self.name);
        let vis = &self.visibility;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let docs = &m.docs;
                let args = m.gen_args();
                let ret = &m.ret;
                quote! {
                    #(#docs)*
                    unsafe fn #name(&self, #(#args),*) #ret;
                }
            })
            .collect::<Vec<_>>();
        let parent = self.parent_trait_constraint();

        quote! {
            #[allow(non_camel_case_types)]
            #vis trait #name: Sized + #parent {
                #(#methods)*
            }
        }
    }

    /// Generates the vtable for a COM interface
    fn gen_vtable(&self, vtable_name: &syn::Ident) -> proc_macro2::TokenStream {
        let vis = &self.visibility;
        let name = &self.name;
        let trait_name = quote::format_ident!("{}_Impl", name);
        let implvtbl_name = quote::format_ident!("{}_ImplVtbl", name);

        let vtable_entries = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let ret = &m.ret;
                let args = m.gen_args();

                if m.is_result() {
                    quote! {
                        pub #name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) -> ::windows_core::HRESULT,
                    }
                } else {
                    quote! {
                        pub #name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) #ret,
                    }
                }
            })
            .collect::<Vec<_>>();

        let parent_vtable_generics = quote!(Identity, OFFSET);
        let parent_vtable = self.parent_vtable();

        // or_parent_matches will be `|| parent::matches(iid)` if this interface inherits from another
        // interface (except for IUnknown) or will be empty if this is not applicable. This is what allows
        // QueryInterface to work correctly for all interfaces in an inheritance chain, e.g.
        // IFoo3 derives from IFoo2 derives from IFoo.
        //
        // We avoid matching IUnknown because object identity depends on the uniqueness of the IUnknown pointer.
        let or_parent_matches = match parent_vtable.as_ref() {
            Some(parent) if !self.parent_is_iunknown() => quote! (|| <#parent>::matches(iid)),
            _ => quote!(),
        };

        let functions = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let args = m.gen_args();
                let params = &m
                    .args
                    .iter()
                    .map(|a| {
                        let pat = &a.pat;
                        quote! { #pat }
                    })
                    .collect::<Vec<_>>();
                let ret = &m.ret;

                let ret = if m.is_result() {
                    quote! { -> ::windows_core::HRESULT }
                } else {
                    quote! { #ret }
                };

                if parent_vtable.is_some() {
                    quote! {
                        unsafe extern "system" fn #name<
                            Identity: ::windows_core::IUnknownImpl,
                            const OFFSET: isize
                        >(
                            this: *mut ::core::ffi::c_void, // <-- This is the COM "this" pointer, which is not the same as &T or &T_Impl.
                            #(#args),*
                        ) #ret
                        where
                            Identity : #trait_name
                        {
                            // This step is essentially a virtual dispatch adjustor thunk. Its purpose is to adjust
                            // the "this" pointer from the address used by the COM interface to the root of the
                            // MyApp_Impl object.  Since a given MyApp_Impl may implement more than one COM interface
                            // (and more than one COM interface chain), we need to know how to get from COM's "this"
                            // back to &MyApp_Impl. The OFFSET constant gives us the value (in pointer-sized units).
                            let this_outer: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);

                            // Last, we invoke the implementation function.
                            // We use explicit <Impl as IFoo_Impl> so that we can select the correct method
                            // for situations where IFoo3 derives from IFoo2 and both declare a method with
                            // the same name.
                            <Identity as #trait_name>::#name(this_outer, #(#params),*).into()
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #name<Impl: #trait_name>(this: *mut ::core::ffi::c_void, #(#args),*) #ret {
                            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
                            let this = (*this).this as *const Impl;
                            (*this).#name(#(#params),*).into()
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        if let Some(parent_vtable) = parent_vtable {
            let entries = self
                .methods
                .iter()
                .map(|m| {
                    let name = &m.name;
                    quote!(#name: #name::<Identity, OFFSET>)
                })
                .collect::<Vec<_>>();

            quote! {
                #[repr(C)]
                #[doc(hidden)]
                #vis struct #vtable_name {
                    pub base__: #parent_vtable,
                    #(#vtable_entries)*
                }
                impl #vtable_name {
                    pub const fn new<
                        Identity: ::windows_core::IUnknownImpl,
                        const OFFSET: isize,
                    >() -> Self
                    where
                        Identity : #trait_name
                    {
                        #(#functions)*
                        Self { base__: #parent_vtable::new::<#parent_vtable_generics>(), #(#entries),* }
                    }

                    #[inline(always)]
                    pub fn matches(iid: &::windows_core::GUID) -> bool {
                        *iid == <#name as ::windows_core::Interface>::IID
                        #or_parent_matches
                    }
                }
            }
        } else {
            let entries = self
                .methods
                .iter()
                .map(|m| {
                    let name = &m.name;
                    quote!(#name: #name::<Impl>)
                })
                .collect::<Vec<_>>();

            quote! {
                #[repr(C)]
                #[doc(hidden)]
                #vis struct #vtable_name {
                    #(#vtable_entries)*
                }
                impl #vtable_name {
                    pub const fn new<Impl: #trait_name>() -> Self {
                        #(#functions)*
                        Self { #(#entries),* }
                    }
                }
                struct #implvtbl_name<T: #trait_name> (::core::marker::PhantomData<T>);
                impl<T: #trait_name> #implvtbl_name<T> {
                    const VTABLE: #vtable_name = #vtable_name::new::<T>();
                }
                impl #name {
                    fn new<'a, T: #trait_name>(this: &'a T) -> ::windows_core::ScopedInterface<'a, #name> {
                        let this = ::windows_core::ScopedHeap { vtable: &#implvtbl_name::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                        let this = ::core::mem::ManuallyDrop::new(::windows_core::imp::Box::new(this));
                        unsafe { ::windows_core::ScopedInterface::new(::core::mem::transmute(&this.vtable)) }
                    }
                }
            }
        }
    }

    /// Generates various conversions such as from and to `IUnknown`
    fn gen_conversions(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let name_string = format!("{name}");
        quote! {
            impl ::core::convert::From<#name> for ::windows_core::IUnknown {
                fn from(value: #name) -> Self {
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            impl ::core::convert::From<&#name> for ::windows_core::IUnknown {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::core::cmp::Eq for #name {}
            impl ::core::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    f.debug_tuple(#name_string).field(&::windows_core::Interface::as_raw(self)).finish()
                }
            }
        }
    }

    fn parent_type(&self) -> proc_macro2::TokenStream {
        if let Some(parent) = &self.parent {
            quote!(#parent)
        } else {
            quote!(::core::ptr::NonNull<::core::ffi::c_void>)
        }
    }

    fn parent_vtable(&self) -> Option<proc_macro2::TokenStream> {
        if let Some((ident, path)) = self.parent_path().split_last() {
            let ident = quote::format_ident!("{}_Vtbl", ident);
            Some(quote! { #(#path::)* #ident })
        } else {
            None
        }
    }

    fn parent_is_iunknown(&self) -> bool {
        if let Some(ident) = self.parent_path().last() {
            ident == "IUnknown"
        } else {
            false
        }
    }

    fn parent_path(&self) -> Vec<syn::Ident> {
        if let Some(parent) = &self.parent {
            parent
                .segments
                .iter()
                .map(|segment| segment.ident.clone())
                .collect()
        } else {
            vec![]
        }
    }

    /// Gets the parent trait constraint which is nothing if the parent is IUnknown
    fn parent_trait_constraint(&self) -> proc_macro2::TokenStream {
        if let Some((ident, path)) = self.parent_path().split_last() {
            if ident != "IUnknown" {
                let ident = quote::format_ident!("{}_Impl", ident);
                return quote! { #(#path::)* #ident };
            }
        }

        quote! {}
    }
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let mut docs = Vec::new();
        for attr in attributes.into_iter() {
            let path = attr.path();
            if path.is_ident("doc") {
                docs.push(attr);
            } else {
                return Err(syn::Error::new(path.span(), "Unrecognized attribute "));
            }
        }

        let visibility = input.parse::<syn::Visibility>()?;
        _ = input.parse::<syn::Token![unsafe]>()?;
        _ = input.parse::<syn::Token![trait]>()?;
        let name = input.parse::<syn::Ident>()?;
        _ = input.parse::<syn::Token![:]>();
        let parent = input.parse::<syn::Path>().ok();
        let content;
        syn::braced!(content in input);
        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse::<InterfaceMethod>()?);
        }
        Ok(Self {
            visibility,
            methods,
            name,
            parent,
            docs,
        })
    }
}

/// Parsed interface guid attribute
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
///                              //^ parses this
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Guid(Option<syn::LitStr>);

impl Guid {
    fn to_tokens(&self) -> syn::Result<proc_macro2::TokenStream> {
        fn hex_lit(num: &str) -> syn::LitInt {
            syn::LitInt::new(&format!("0x{num}"), proc_macro2::Span::call_site())
        }

        fn ensure_length(
            part: Option<&str>,
            index: usize,
            length: usize,
            span: proc_macro2::Span,
        ) -> syn::Result<String> {
            let part = match part {
                Some(p) => p,
                None => {
                    return Err(syn::Error::new(
                        span,
                        format!("The IID missing part at index {index}"),
                    ))
                }
            };

            if part.len() != length {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "The IID part at index {} must be {} characters long but was {} characters",
                        index,
                        length,
                        part.len()
                    ),
                ));
            }

            Ok(part.to_owned())
        }

        if let Some(value) = &self.0 {
            let guid_value = value.value();
            let mut delimited = guid_value.split('-').fuse();
            let chunks = [
                ensure_length(delimited.next(), 0, 8, value.span())?,
                ensure_length(delimited.next(), 1, 4, value.span())?,
                ensure_length(delimited.next(), 2, 4, value.span())?,
                ensure_length(delimited.next(), 3, 4, value.span())?,
                ensure_length(delimited.next(), 4, 12, value.span())?,
            ];

            let data1 = hex_lit(&chunks[0]);
            let data2 = hex_lit(&chunks[1]);
            let data3 = hex_lit(&chunks[2]);
            let (data4_1, data4_2) = chunks[3].split_at(2);
            let data4_1 = hex_lit(data4_1);
            let data4_2 = hex_lit(data4_2);
            let (data4_3, rest) = chunks[4].split_at(2);
            let data4_3 = hex_lit(data4_3);

            let (data4_4, rest) = rest.split_at(2);
            let data4_4 = hex_lit(data4_4);

            let (data4_5, rest) = rest.split_at(2);
            let data4_5 = hex_lit(data4_5);

            let (data4_6, rest) = rest.split_at(2);
            let data4_6 = hex_lit(data4_6);

            let (data4_7, data4_8) = rest.split_at(2);
            let data4_7 = hex_lit(data4_7);
            let data4_8 = hex_lit(data4_8);
            Ok(quote! {
                ::windows_core::GUID {
                    data1: #data1,
                    data2: #data2,
                    data3: #data3,
                    data4: [#data4_1, #data4_2, #data4_3, #data4_4, #data4_5, #data4_6, #data4_7, #data4_8]
                }
            })
        } else {
            Ok(quote! {
                ::windows_core::GUID::zeroed()
            })
        }
    }
}

impl syn::parse::Parse for Guid {
    fn parse(cursor: syn::parse::ParseStream) -> syn::Result<Self> {
        let string: Option<syn::LitStr> = cursor.parse().ok();

        Ok(Self(string))
    }
}

/// A parsed interface method
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
///   //^ parses this
/// }
/// ```
struct InterfaceMethod {
    pub name: syn::Ident,
    pub visibility: syn::Visibility,
    pub args: Vec<InterfaceMethodArg>,
    pub ret: syn::ReturnType,
    pub docs: Vec<syn::Attribute>,
}

impl InterfaceMethod {
    fn is_result(&self) -> bool {
        if let syn::ReturnType::Type(_, ty) = &self.ret {
            if let syn::Type::Path(path) = &**ty {
                if let Some(segment) = path.path.segments.last() {
                    let ident = segment.ident.to_string();
                    if ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if args.args.len() == 1 {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Generates arguments (of the form `$pat: $type`)
    fn gen_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .map(|a| {
                let pat = &a.pat;
                let ty = &a.ty;
                quote! { #pat: #ty }
            })
            .collect::<Vec<_>>()
    }

    fn gen_consume_generics(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .enumerate()
            .filter_map(|(generic_index, a)| {
                if let Some((ty, ident)) = a.borrow_type() {
                    let generic_ident = quote::format_ident!("P{generic_index}");
                    if ident == "Ref" {
                        Some(quote! { #generic_ident: ::windows_core::Param<#ty> })
                    } else {
                        Some(quote! { #generic_ident: ::windows_core::OutParam<#ty> })
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn gen_consume_params(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .enumerate()
            .map(|(generic_index, a)| {
                let pat = &a.pat;

                if a.borrow_type().is_some() {
                    let generic_ident = quote::format_ident!("P{generic_index}");
                    quote! { #pat: #generic_ident }
                } else {
                    let ty = &a.ty;
                    quote! { #pat: #ty }
                }
            })
            .collect::<Vec<_>>()
    }

    fn gen_consume_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .map(|a| {
                let pat = &a.pat;

                if let Some((_, ident)) = a.borrow_type() {
                    if ident == "Ref" {
                        quote! { #pat.param().borrow() }
                    } else {
                        quote! { #pat.borrow_mut() }
                    }
                } else {
                    quote! { #pat }
                }
            })
            .collect::<Vec<_>>()
    }
}

impl syn::parse::Parse for InterfaceMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let docs = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.parse::<syn::Visibility>()?;
        let method = input.parse::<syn::TraitItemFn>()?;
        unexpected_token!(docs.iter().find(|a| !a.path().is_ident("doc")), "attribute");
        unexpected_token!(method.default, "default method implementation");
        let sig = method.sig;
        unexpected_token!(sig.abi, "abi declaration");
        unexpected_token!(sig.asyncness, "async declaration");
        unexpected_token!(sig.generics.params.iter().next(), "generics declaration");
        unexpected_token!(sig.constness, "const declaration");
        expected_token!(
            sig.receiver(),
            "the method to have &self as its first argument"
        );
        unexpected_token!(sig.variadic, "variadic args");
        let args = sig
            .inputs
            .into_iter()
            .filter_map(|a| match a {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(p) => Some(p),
            })
            .map(|p| {
                Ok(InterfaceMethodArg {
                    ty: p.ty,
                    pat: p.pat,
                })
            })
            .collect::<Result<Vec<InterfaceMethodArg>, syn::Error>>()?;

        let ret = sig.output;
        Ok(Self {
            name: sig.ident,
            visibility,
            args,
            ret,
            docs,
        })
    }
}

/// An argument to an interface method
struct InterfaceMethodArg {
    /// The type of the argument
    pub ty: Box<syn::Type>,
    /// The name of the argument
    pub pat: Box<syn::Pat>,
}

impl InterfaceMethodArg {
    fn borrow_type(&self) -> Option<(syn::Type, String)> {
        if let syn::Type::Path(path) = &*self.ty {
            if let Some(segment) = path.path.segments.last() {
                let ident = segment.ident.to_string();
                if matches!(ident.as_str(), "Ref" | "OutRef") {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if args.args.len() == 1 {
                            if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                                return Some((ty.clone(), ident));
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
