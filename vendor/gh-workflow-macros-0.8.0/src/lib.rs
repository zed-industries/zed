use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Context)]
pub fn derive_expr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let ctor_name = struct_name.to_string().to_snake_case();
    let ctor_id = syn::Ident::new(&ctor_name, struct_name.span());

    // Ensure it's a struct and get its fields
    let fields = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            fields
        } else {
            panic!("#[derive(Context)] only supports structs with named fields")
        }
    } else {
        panic!("#[derive(Context)] can only be used with structs");
    };

    // Generate methods for each field
    let methods = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            pub fn #field_name(&self) -> Context<#field_type> {
                self.select::<#field_type>(#field_name_str)
            }
        }
    });

    // Generate the output code
    let expanded = quote! {
        impl Context<#struct_name> {
            #(#methods)*

            pub fn #ctor_id() -> Self {
                Context::<Github>::new().select(stringify!(#ctor_name))
            }
        }
    };

    // eprintln!("Generated code:\n{}", expanded);

    TokenStream::from(expanded)
}
