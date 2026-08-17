use super::builder_gen::input_struct::StructInputCtx;
use super::builder_gen::MacroOutput;
use crate::util::prelude::*;

pub(crate) fn generate(orig_struct: syn::ItemStruct) -> Result<TokenStream> {
    let struct_ident = orig_struct.ident.clone();
    let ctx = StructInputCtx::new(orig_struct)?;

    let MacroOutput {
        mut start_fn,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output()?;

    let impl_generics = std::mem::take(&mut start_fn.sig.generics);

    let (generics_decl, generic_args, where_clause) = impl_generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #generics_decl #struct_ident #generic_args
            #where_clause
        {
            #start_fn
        }

        #other_items
    })
}
