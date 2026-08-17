//! Generates output for the `implement` proc macro.
//!
//! Each function in this module focuses on generating one thing, or one kind of thing.
//! Each takes `ImplementInputs` as its input.  `gen_all` calls all of the `gen_*` functions
//! and merges them into the final list of output items.
//!
//! We use `parse_quote` so that we can verify that a given function generates a well-formed AST
//! item, within the narrowest possible scope. This allows us to detect errors more quickly during
//! development. If the input to `parse_quote` cannot be parsed, then the macro will panic and
//! the panic will point to the specific `parse_quote` call, rather than the entire output of the
//! `implement` proc macro being unparsable. This greatly aids in development.

use super::*;
use quote::{quote, quote_spanned};
use syn::{parse_quote, parse_quote_spanned};

/// Generates code for the `#[implements]` macro.
pub(crate) fn gen_all(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items: Vec<syn::Item> = Vec::with_capacity(64);

    items.push(gen_original_impl(inputs));
    items.push(gen_impl_struct(inputs));
    items.push(gen_impl_deref(inputs));
    items.push(gen_impl_impl(inputs));
    items.push(gen_iunknown_impl(inputs));
    items.push(gen_impl_com_object_inner(inputs));
    items.extend(gen_impl_from(inputs));
    items.extend(gen_impl_com_object_interfaces(inputs));

    for (i, interface_chain) in inputs.interface_chains.iter().enumerate() {
        items.push(gen_impl_as_impl(inputs, interface_chain, i));
    }

    items
}

/// Generates an `impl` block for the original `Foo` type.
///
/// This `impl` block will contain `into_outer` and `into_static` (if applicable).
fn gen_original_impl(inputs: &ImplementInputs) -> syn::Item {
    let original_ident = &inputs.original_ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;

    let mut output: syn::ItemImpl = parse_quote! {
        impl #generics #original_ident::#generics where #constraints {}
    };

    output.items.push(gen_into_outer(inputs));

    // Static COM objects have a lot of constraints. They can't be generic (open parameters),
    // because that would be meaningless (an open generic type cannot have a known representation).
    //
    // Right now, we can't generate static COM objects that have base classes because we rely on
    // boxing and then unboxing during construction of aggregated types.
    if !inputs.is_generic {
        output.items.push(gen_into_static(inputs));
    }

    syn::Item::Impl(output)
}

/// Generates the structure definition for the `Foo_Impl` type.
fn gen_impl_struct(inputs: &ImplementInputs) -> syn::Item {
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let original_ident = &inputs.original_ident;
    let vis = &inputs.original_type.vis;

    let mut impl_fields = quote! {
        identity: &'static ::windows_core::IInspectable_Vtbl,
    };

    for interface_chain in inputs.interface_chains.iter() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let chain_field_ident = &interface_chain.field_ident;
        impl_fields.extend(quote! {
            #chain_field_ident: &'static #vtbl_ty,
        });
    }

    impl_fields.extend(quote! {
        this: #original_ident::#generics,
        count: ::windows_core::imp::WeakRefCount,
    });

    parse_quote! {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #vis struct #impl_ident #generics where #constraints {
            #impl_fields
        }
    }
}

/// Generates the implementation of `core::ops::Deref` for the generated `Foo_Impl` type.
fn gen_impl_deref(inputs: &ImplementInputs) -> syn::Item {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let original_ident = &inputs.original_type.ident;
    let impl_ident = &inputs.impl_ident;

    parse_quote! {
        impl #generics ::core::ops::Deref for #impl_ident::#generics where #constraints {
            type Target = #original_ident::#generics;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.this
            }
        }
    }
}

/// Generates an `impl` block for the generated `Foo_Impl` block.
///
/// This generates:
///
/// ```rust,ignore
/// const VTABLE_IDENTITY = IInspectable_Vtbl = ...;
/// const VTABLE_INTERFACE1_IFOO: IFoo_Vtbl = ...;
/// const VTABLE_INTERFACE2_IBAR: IBar_Vtbl = ...;
/// ```
///
/// These constants are used when constructing vtables. The benefit of using constants instead
/// of directly generating these expressions is that it allows us to overcome limitations in
/// using generics in constant contexts. Right now, Rust has a lot of limitations around using
/// constants in constant contexts. Fortunately, associated constants (constants defined within
/// `impl` blocks) work in stable Rust, even for generic types.
fn gen_impl_impl(inputs: &ImplementInputs) -> syn::Item {
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;

    let mut output: syn::ItemImpl = parse_quote! {
        impl #generics #impl_ident::#generics where #constraints {}
    };

    // This is here so that IInspectable::GetRuntimeClassName can work properly.
    // For a test case for this, see crates/tests/misc/component_client.
    let identity_type = if let Some(first) = inputs.interface_chains.first() {
        first.implement.to_ident()
    } else {
        quote! { ::windows_core::IInspectable }
    };

    output.items.push(parse_quote! {
        const VTABLE_IDENTITY: ::windows_core::IInspectable_Vtbl =
            ::windows_core::IInspectable_Vtbl::new::<
                #impl_ident::#generics,
                #identity_type,
                0,
            >();
    });

    for (interface_index, interface_chain) in inputs.interface_chains.iter().enumerate() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let vtable_const_ident = &interface_chain.vtable_const_ident;

        let chain_offset_in_pointers: isize = -1 - interface_index as isize;
        output.items.push(parse_quote! {
            const #vtable_const_ident: #vtbl_ty = #vtbl_ty::new::<
                #impl_ident::#generics,
                #chain_offset_in_pointers,
            >();
        });
    }

    syn::Item::Impl(output)
}

/// Generates the `IUnknownImpl` implementation for the `Foo_Impl` type.
fn gen_iunknown_impl(inputs: &ImplementInputs) -> syn::Item {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;
    let original_ident = &inputs.original_type.ident;

    let trust_level = proc_macro2::Literal::usize_unsuffixed(inputs.trust_level);

    let mut output: syn::ItemImpl = parse_quote! {
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
            fn into_inner(self) -> Self::Impl {
                self.this
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

            #[inline(always)]
            fn is_reference_count_one(&self) -> bool {
                self.count.is_one()
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
                if value.is_null() {
                    return ::windows_core::imp::E_POINTER;
                }
                *value = #trust_level;
                ::windows_core::HRESULT(0)
            }

            fn to_object(&self) -> ::windows_core::ComObject<Self::Impl> {
                self.count.add_ref();
                unsafe {
                    ::windows_core::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(self as *const Self as *mut Self)
                    )
                }
            }
        }
    };

    let query_interface_fn = gen_query_interface(inputs);
    output.items.push(syn::ImplItem::Fn(query_interface_fn));

    syn::Item::Impl(output)
}

/// Generates the implementation of `ComObjectInner`.
fn gen_impl_com_object_inner(inputs: &ImplementInputs) -> syn::Item {
    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    parse_quote! {
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
                let boxed = ::windows_core::imp::Box::<#impl_ident::#generics>::new(self.into_outer());
                unsafe {
                    let ptr = ::windows_core::imp::Box::into_raw(boxed);
                    ::windows_core::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(ptr)
                    )
                }
            }
        }
    }
}

/// Generates the `query_interface` method.
fn gen_query_interface(inputs: &ImplementInputs) -> syn::ImplItemFn {
    let queries = inputs.interface_chains.iter().map(|interface_chain| {
        let chain_ty = interface_chain.implement.to_vtbl_ident();
        let chain_field = &interface_chain.field_ident;
        quote_spanned! {
            interface_chain.implement.span =>
            if #chain_ty::matches(&iid) {
                break 'found &self.#chain_field as *const _ as *const ::core::ffi::c_void;
            }
        }
    });

    // Dynamic casting requires that the object not contain non-static lifetimes.
    let enable_dyn_casting = inputs.original_type.generics.lifetimes().count() == 0;
    let dynamic_cast_query = if enable_dyn_casting {
        quote! {
            if iid == ::windows_core::DYNAMIC_CAST_IID {
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

    let identity_query = if inputs.agile {
        quote! {
            if iid == <::windows_core::IUnknown as ::windows_core::Interface>::IID
            || iid == <::windows_core::IInspectable as ::windows_core::Interface>::IID
            || iid == <::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID {
                break 'found &self.identity as *const _ as *const ::core::ffi::c_void;
            }
        }
    } else {
        quote! {
            if iid == <::windows_core::IUnknown as ::windows_core::Interface>::IID
            || iid == <::windows_core::IInspectable as ::windows_core::Interface>::IID {
                break 'found &self.identity as *const _ as *const ::core::ffi::c_void;
            }
        }
    };

    let marshal_query = if inputs.agile {
        quote! {
            #[cfg(windows)]
            if iid == <::windows_core::imp::IMarshal as ::windows_core::Interface>::IID {
                return ::windows_core::imp::marshaler(self.to_interface(), interface);
            }
        }
    } else {
        quote! {}
    };

    let tear_off_query = quote! {
        let tear_off_ptr = self.count.query(&iid, &self.identity as *const _ as *mut _);
        if !tear_off_ptr.is_null() {
            *interface = tear_off_ptr;
            return ::windows_core::HRESULT(0);
        }
    };

    parse_quote! {
        unsafe fn QueryInterface(
            &self,
            iid: *const ::windows_core::GUID,
            interface: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            unsafe {
                if iid.is_null() || interface.is_null() {
                    return ::windows_core::imp::E_POINTER;
                }

                let iid = *iid;

                let interface_ptr: *const ::core::ffi::c_void = 'found: {
                    #identity_query
                    #(#queries)*
                    #marshal_query
                    #dynamic_cast_query
                    #tear_off_query

                    *interface = ::core::ptr::null_mut();
                    return ::windows_core::imp::E_NOINTERFACE;
                };

                debug_assert!(!interface_ptr.is_null());
                *interface = interface_ptr as *mut ::core::ffi::c_void;
                self.count.add_ref();
                return ::windows_core::HRESULT(0);
            }
        }
    }
}

/// Generates the `T::into_outer` function. This function is part of how we construct a
/// `ComObject<T>` from a `T`.
fn gen_into_outer(inputs: &ImplementInputs) -> syn::ImplItem {
    let generics = &inputs.generics;
    let impl_ident = &inputs.impl_ident;

    let mut initializers = quote! {
        identity: &#impl_ident::#generics::VTABLE_IDENTITY,
    };

    for interface_chain in inputs.interface_chains.iter() {
        let vtbl_field_ident = &interface_chain.field_ident;
        let vtable_const_ident = &interface_chain.vtable_const_ident;

        initializers.extend(quote_spanned! {
            interface_chain.implement.span =>
            #vtbl_field_ident: &#impl_ident::#generics::#vtable_const_ident,
        });
    }

    // If the type is generic then into_outer() cannot be a const fn.
    let maybe_const = if inputs.is_generic {
        quote!()
    } else {
        quote!(const)
    };

    parse_quote! {
        // This constructs an "outer" object. This should only be used by the implementation
        // of the outer object, never by application code.
        //
        // The callers of this function (`into_static` and `into_object`) are both responsible
        // for maintaining one of our invariants: Application code never has an owned instance
        // of the outer (implementation) type. into_static() maintains this invariant by
        // returning a wrapped StaticComObject value, which owns its contents but never gives
        // application code a way to mutably access its contents. This prevents the refcount
        // shearing problem.
        //
        // TODO: Make it impossible for app code to call this function, by placing it in a
        // module and marking this as private to the module.
        #[inline(always)]
        #maybe_const fn into_outer(self) -> #impl_ident::#generics {
            #impl_ident::#generics {
                #initializers
                count: ::windows_core::imp::WeakRefCount::new(),
                this: self,
            }
        }
    }
}

/// Generates the `T::into_static` function. This function is part of how we construct a
/// `StaticComObject<T>` from a `T`.
fn gen_into_static(inputs: &ImplementInputs) -> syn::ImplItem {
    assert!(!inputs.is_generic);
    parse_quote! {
        /// This converts a partially-constructed COM object (in the sense that it contains
        /// application state but does not yet have vtable and reference count constructed)
        /// into a `StaticComObject`. This allows the COM object to be stored in static
        /// (global) variables.
        pub const fn into_static(self) -> ::windows_core::StaticComObject<Self> {
            ::windows_core::StaticComObject::from_outer(self.into_outer())
        }
    }
}

/// Generates `From`-based conversions.
///
/// These conversions convert from the user's type `T` to `ComObject<T>` or to an interface
/// implemented by `T`. These conversions are shorthand for calling `ComObject::new(value)`.
///
/// We can only generate conversions from `T` to the roots of each interface chain. We can't
/// generate `From` conversions from `T` to an interface that is inherited by an interface chain,
/// because this proc macro does not have access to any information about the inheritance chain
/// of interfaces that are referenced.
///
/// For example:
///
/// ```rust,ignore
/// #[implement(IFoo3)]
/// struct MyType;
/// ```
///
/// If `IFoo3` inherits from `IFoo2`, then this code will _not_ generate a conversion for `IFoo2`.
/// However, user code can still do this:
///
/// ```rust,ignore
/// let ifoo2 = IFoo3::from(MyType).into();
/// ```
///
/// This works because the `IFoo3` type has an `Into` impl for `IFoo2`.
fn gen_impl_from(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items = Vec::new();

    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;

    items.push(parse_quote! {
        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IUnknown where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }
    });

    items.push(parse_quote! {
        impl #generics ::core::convert::From<#original_ident::#generics> for ::windows_core::IInspectable where #constraints {
            #[inline(always)]
            fn from(this: #original_ident::#generics) -> Self {
                let com_object = ::windows_core::ComObject::new(this);
                com_object.into_interface()
            }
        }
    });

    for interface_chain in inputs.interface_chains.iter() {
        let interface_ident = interface_chain.implement.to_ident();

        items.push(parse_quote_spanned! {
            interface_chain.implement.span =>
            impl #generics ::core::convert::From<#original_ident::#generics> for #interface_ident where #constraints {
                #[inline(always)]
                fn from(this: #original_ident::#generics) -> Self {
                    let com_object = ::windows_core::ComObject::new(this);
                    com_object.into_interface()
                }
            }
        });
    }

    items
}

/// Generates the `ComObjectInterface` implementation for each interface chain.
///
/// Each of these `impl` blocks says "this COM object implements this COM interface".
/// It allows the `ComObject` type to do conversions from the `ComObject` to `IFoo` instances,
/// _without_ doing a `QueryInterface` call.
fn gen_impl_com_object_interfaces(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items = Vec::new();

    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    items.push(parse_quote! {
        impl #generics ::windows_core::ComObjectInterface<::windows_core::IUnknown> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
                unsafe {
                    let interface_ptr = &self.identity;
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }
    });

    items.push(parse_quote! {
        impl #generics ::windows_core::ComObjectInterface<::windows_core::IInspectable> for #impl_ident::#generics where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IInspectable> {
                unsafe {
                    let interface_ptr = &self.identity;
                    ::core::mem::transmute(interface_ptr)
                }
            }
        }
    });

    for interface_chain in inputs.interface_chains.iter() {
        let chain_field = &interface_chain.field_ident;
        let interface_ident = interface_chain.implement.to_ident();

        items.push(parse_quote_spanned! {
            interface_chain.implement.span =>
            #[allow(clippy::needless_lifetimes)]
            impl #generics ::windows_core::ComObjectInterface<#interface_ident> for #impl_ident::#generics where #constraints {
                #[inline(always)]
                fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, #interface_ident> {
                    unsafe {
                        ::core::mem::transmute(&self.#chain_field)
                    }
                }
            }
        });
    }

    items
}

/// Generates the implementation of the `AsImpl` trait for a given interface chain.
fn gen_impl_as_impl(
    inputs: &ImplementInputs,
    interface_chain: &InterfaceChain,
    interface_chain_index: usize,
) -> syn::Item {
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let interface_ident = interface_chain.implement.to_ident();
    let original_ident = &inputs.original_type.ident;
    let impl_ident = &inputs.impl_ident;

    parse_quote_spanned! {
        interface_chain.implement.span =>
        impl #generics ::windows_core::AsImpl<#original_ident::#generics> for #interface_ident where #constraints {
            // SAFETY: the offset is guaranteed to be in bounds, and the implementation struct
            // is guaranteed to live at least as long as `self`.
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics> {
                unsafe {
                    let this = ::windows_core::Interface::as_raw(self);
                    // Subtract away the vtable offset plus 1, for the `identity` field, to get
                    // to the impl struct which contains that original implementation type.
                    let this = (this as *mut *mut ::core::ffi::c_void).sub(1 + #interface_chain_index) as *mut #impl_ident::#generics;
                    ::core::ptr::NonNull::new_unchecked(::core::ptr::addr_of!((*this).this) as *const #original_ident::#generics as *mut #original_ident::#generics)
                }
            }
        }
    }
}
