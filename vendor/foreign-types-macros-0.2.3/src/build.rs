use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::parse::{ForeignType, Input};

fn ref_name(input: &ForeignType) -> Ident {
    Ident::new(&format!("{}Ref", input.name), input.name.span())
}

pub fn build(input: Input) -> TokenStream {
    let types = input
        .types
        .iter()
        .map(|t| build_foreign_type(&input.crate_, t));
    quote! {
        #(#types)*
    }
}

fn build_foreign_type(crate_: &Path, input: &ForeignType) -> TokenStream {
    let decls = build_decls(crate_, input);
    let oibits = build_oibits(crate_, input);
    let foreign_impls = build_foreign_impls(crate_, input);
    let drop_impl = build_drop_impl(crate_, input);
    let deref_impls = build_deref_impls(crate_, input);
    let borrow_impls = build_borrow_impls(crate_, input);
    let as_ref_impls = build_as_ref_impls(crate_, input);
    let clone_impl = build_clone_impl(crate_, input);
    let to_owned_impl = build_to_owned_impl(crate_, input);

    quote! {
        #decls
        #oibits
        #foreign_impls
        #drop_impl
        #deref_impls
        #borrow_impls
        #as_ref_impls
        #clone_impl
        #to_owned_impl
    }
}

fn build_decls(crate_: &Path, input: &ForeignType) -> TokenStream {
    let attrs = &input.attrs;
    let vis = &input.visibility;
    let name = &input.name;
    let generics = &input.generics;
    let ctype = &input.ctype;
    let phantom_data = input
        .phantom_data
        .as_ref()
        .map(|d| quote!(, #crate_::export::PhantomData<#d>));
    let ref_name = ref_name(input);
    let ref_docs = format!(
        "A borrowed reference to a [`{name}`](struct.{name}.html).",
        name = name
    );

    quote! {
        #(#attrs)*
        #[repr(transparent)]
        #vis struct #name #generics(#crate_::export::NonNull<#ctype> #phantom_data);

        #[doc = #ref_docs]
        #vis struct #ref_name #generics(#crate_::Opaque #phantom_data);
    }
}

fn build_oibits(crate_: &Path, input: &ForeignType) -> TokenStream {
    let oibits = input.oibits.iter().map(|t| build_oibit(crate_, input, t));

    quote! {
        #(#oibits)*
    }
}

fn build_oibit(crate_: &Path, input: &ForeignType, oibit: &Ident) -> TokenStream {
    let name = &input.name;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        unsafe impl #impl_generics #crate_::export::#oibit for #name #ty_generics {}
        unsafe impl #impl_generics #crate_::export::#oibit for #ref_name #ty_generics {}
    }
}

fn build_foreign_impls(crate_: &Path, input: &ForeignType) -> TokenStream {
    let name = &input.name;
    let ctype = &input.ctype;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let phantom_data = input
        .phantom_data
        .as_ref()
        .map(|_| quote!(, #crate_::export::PhantomData));

    quote! {
        unsafe impl #impl_generics #crate_::ForeignType for #name #ty_generics {
            type CType = #ctype;
            type Ref = #ref_name #ty_generics;

            #[inline]
            unsafe fn from_ptr(ptr: *mut #ctype) -> #name #ty_generics {
                debug_assert!(!ptr.is_null());
                #name(<#crate_::export::NonNull<_>>::new_unchecked(ptr) #phantom_data)
            }

            #[inline]
            fn as_ptr(&self) -> *mut #ctype {
                <#crate_::export::NonNull<_>>::as_ptr(self.0)
            }
        }

        unsafe impl #impl_generics #crate_::ForeignTypeRef for #ref_name #ty_generics {
            type CType = #ctype;
        }
    }
}

fn build_drop_impl(crate_: &Path, input: &ForeignType) -> TokenStream {
    let name = &input.name;
    let drop = &input.drop;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::Drop for #name #ty_generics {
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    (#drop)(#crate_::ForeignType::as_ptr(self));
                }
            }
        }
    }
}

fn build_deref_impls(crate_: &Path, input: &ForeignType) -> TokenStream {
    let name = &input.name;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::Deref for #name #ty_generics {
            type Target = #ref_name #ty_generics;

            #[inline]
            fn deref(&self) -> &#ref_name #ty_generics {
                unsafe {
                    #crate_::ForeignTypeRef::from_ptr(#crate_::ForeignType::as_ptr(self))
                }
            }
        }

        impl #impl_generics #crate_::export::DerefMut for #name #ty_generics {
            #[inline]
            fn deref_mut(&mut self) -> &mut #ref_name #ty_generics {
                unsafe {
                    #crate_::ForeignTypeRef::from_ptr_mut(#crate_::ForeignType::as_ptr(self))
                }
            }
        }
    }
}

fn build_borrow_impls(crate_: &Path, input: &ForeignType) -> TokenStream {
    let name = &input.name;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::Borrow<#ref_name #ty_generics> for #name #ty_generics {
            #[inline]
            fn borrow(&self) -> &#ref_name #ty_generics {
                &**self
            }
        }

        impl #impl_generics #crate_::export::BorrowMut<#ref_name #ty_generics> for #name #ty_generics {
            #[inline]
            fn borrow_mut(&mut self) -> &mut #ref_name #ty_generics {
                &mut **self
            }
        }
    }
}

fn build_as_ref_impls(crate_: &Path, input: &ForeignType) -> TokenStream {
    let name = &input.name;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::AsRef<#ref_name #ty_generics> for #name #ty_generics {
            #[inline]
            fn as_ref(&self) -> &#ref_name #ty_generics {
                &**self
            }
        }

        impl #impl_generics #crate_::export::AsMut<#ref_name #ty_generics> for #name #ty_generics {
            #[inline]
            fn as_mut(&mut self) -> &mut #ref_name #ty_generics {
                &mut **self
            }
        }
    }
}

fn build_clone_impl(crate_: &Path, input: &ForeignType) -> TokenStream {
    let clone = match &input.clone {
        Some(clone) => clone,
        None => return quote!(),
    };
    let name = &input.name;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::Clone for #name #ty_generics {
            #[inline]
            fn clone(&self) -> #name #ty_generics {
                unsafe {
                    let ptr = (#clone)(#crate_::ForeignType::as_ptr(self));
                    #crate_::ForeignType::from_ptr(ptr)
                }
            }
        }
    }
}

#[cfg(feature = "std")]
fn build_to_owned_impl(crate_: &Path, input: &ForeignType) -> TokenStream {
    let clone = match &input.clone {
        Some(clone) => clone,
        None => return quote!(),
    };
    let name = &input.name;
    let ref_name = ref_name(input);
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #crate_::export::ToOwned for #ref_name #ty_generics {
            type Owned = #name #ty_generics;

            #[inline]
            fn to_owned(&self) -> #name #ty_generics {
                unsafe {
                    let ptr = (#clone)(#crate_::ForeignTypeRef::as_ptr(self));
                    #crate_::ForeignType::from_ptr(ptr)
                }
            }
        }
    }
}

#[cfg(not(feature = "std"))]
fn build_to_owned_impl(_: &Path, _: &ForeignType) -> TokenStream {
    quote!()
}
