use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, ItemTrait};

/// Generates an implementation of `Pointee` for structs with a DST as its last
/// field.
#[proc_macro_derive(Pointee)]
pub fn pointee_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(derive_pointee_impl(&input))
}

fn derive_pointee_impl(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    let last_field_ty = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if let Some(result) = fields.named.last() {
                    &result.ty
                } else {
                    return Error::new(ident.span(), "dynamically sized structs must contain at least one field").to_compile_error();
                }
            },
            Fields::Unnamed(ref fields) => {
                if let Some(result) = fields.unnamed.last() {
                    &result.ty
                } else {
                    return Error::new(ident.span(), "dynamically sized structs must contain at least one field").to_compile_error();
                }
            },
            Fields::Unit => return Error::new(ident.span(), "unit structs cannot be dynamically sized").to_compile_error(),
        },
        Data::Enum(_) => return Error::new(ident.span(), "enums cannot be dynamically sized").to_compile_error(),
        Data::Union(_) => return Error::new(ident.span(), "unions cannot be dynamically sized").to_compile_error(),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        const _: () = {
            use ptr_meta::Pointee;

            impl #impl_generics Pointee for #ident #ty_generics #where_clause
            where
                #last_field_ty: Pointee,
            {
                type Metadata = <#last_field_ty as Pointee>::Metadata;
            }
        };
    }
}

/// Generates an implementation of `Pointee` for trait objects.
#[proc_macro_attribute]
pub fn pointee(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let ident = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let result = quote! {
        #input

        const _: () = {
            use ptr_meta::{DynMetadata, Pointee};

            impl #impl_generics Pointee for (dyn #ident #ty_generics #where_clause + '_) {
                type Metadata = DynMetadata<Self>;
            }
        };
    };

    proc_macro::TokenStream::from(result)
}
