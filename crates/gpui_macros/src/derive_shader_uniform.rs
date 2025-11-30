use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{TokenStreamExt, format_ident, quote, quote_spanned};
use syn::{Data, DeriveInput, Error, parse_macro_input};

fn has_repr_c(attrs: &[syn::Attribute]) -> Result<bool, Error> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let mut is_repr_c = false;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("C") {
                    is_repr_c = true;
                } else {
                    // Consume tokens
                    while !meta.input.is_empty() {
                        let _ = meta.input.parse::<TokenTree>()?;
                    }
                }

                Ok(())
            })?;

            if is_repr_c {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn derive_shader_uniform(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;

    match has_repr_c(&ast.attrs) {
        Ok(true) => {}
        Ok(false) => {
            return Error::new_spanned(type_name, "ShaderUniform: Struct must be #[repr(C)]")
                .into_compile_error()
                .into();
        }
        Err(err) => {
            return err.into_compile_error().into();
        }
    }

    let Data::Struct(data) = ast.data else {
        return Error::new_spanned(type_name, "ShaderUniform: Item must be a struct")
            .into_compile_error()
            .into();
    };

    if data.fields.is_empty() {
        return Error::new_spanned(type_name, "ShaderUniform: Struct cannot be empty")
            .into_compile_error()
            .into();
    }

    let mut def_fields = Vec::new();
    let mut align_tokens = TokenStream2::new();

    for (idx, field) in data.fields.iter().enumerate() {
        let Some(ident) = &field.ident else {
            return Error::new_spanned(field, "ShaderUniform: Struct fields must be named")
                .into_compile_error()
                .into();
        };

        let ty = &field.ty;
        def_fields.push(quote! { "    " });
        def_fields.push(quote! { stringify!(#ident) });
        def_fields.push(quote! { ": " });
        def_fields.push(quote! { <#ty as gpui::ShaderUniform>::NAME });
        def_fields.push(quote! { ",\n" });

        let offset_ident = format_ident!("_OFFSET{}", idx); // Use constants so that the `const _: ()` block can use it.
        let next_offset_ident = format_ident!("_OFFSET{}", idx + 1);
        align_tokens.append_all(quote_spanned! { ident.span() =>
            const _: () = { // Forces error to appear at the structure instead of where we use ALIGN
                if #offset_ident % <#ty as gpui::ShaderUniform>::ALIGN != 0 {
                    panic!(concat!(
                        "ShaderUniform: field `",
                        stringify!(#ident),
                        "` is not properly aligned. Reorder fields or insert explicit padding to ensure WGSL layout rules are followed."
                    ));
                }

                if size_of::<#ty>() == 0 {
                    panic!(concat!(
                       "ShaderUniform: field `",
                      stringify!(#ident),
                     "` has a size of zero."
                    ));
                }
            };

            const #next_offset_ident: usize = #offset_ident + size_of::<#ty>();
            align = if <#ty as gpui::ShaderUniform>::ALIGN > align {
                <#ty as gpui::ShaderUniform>::ALIGN
            } else {
                align
            };
        });
    }

    let generated = quote! {
        unsafe impl gpui::ShaderUniform for #type_name {
            const NAME: &str = stringify!(#type_name);
            const DEFINITION: Option<&str> = Some(gpui::private::const_format::concatcp!("struct ", stringify!(#type_name), " {\n", #(#def_fields),*, "}"));
            const ALIGN: usize = {
                const _OFFSET0: usize = 0;
                let mut align = 0;
                #align_tokens
                align
            };
        }
    };

    generated.into()
}
