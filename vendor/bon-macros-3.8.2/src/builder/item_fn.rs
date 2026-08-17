use super::builder_gen::input_fn::{FnInputCtx, FnInputCtxParams};
use super::builder_gen::{MacroOutput, TopLevelConfig};
use crate::normalization::SyntaxVariant;
use crate::util::prelude::*;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(
    config: TopLevelConfig,
    orig_fn: syn::ItemFn,
    namespace: &crate::normalization::GenericsNamespace,
) -> Result<TokenStream> {
    let mut norm_fn = orig_fn.clone();

    crate::normalization::NormalizeLifetimes::new(namespace).visit_item_fn_mut(&mut norm_fn);
    crate::normalization::NormalizeImplTraits::new(namespace).visit_item_fn_mut(&mut norm_fn);

    let fn_item = SyntaxVariant {
        orig: orig_fn,
        norm: norm_fn,
    };

    let ctx = FnInputCtx::new(FnInputCtxParams {
        namespace,
        fn_item,
        impl_ctx: None,
        config,
    })?;

    let adapted_fn = ctx.adapted_fn()?;
    let warnings = ctx.warnings();

    let MacroOutput {
        start_fn,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output()?;

    Ok(quote! {
        // Keep original function at the top. It seems like rust-analyzer
        // does better job of highlighting syntax when it is here. Assuming
        // this is because rust-analyzer prefers the first occurrence of the
        // span when highlighting.
        //
        // See this issue for details: https://github.com/rust-lang/rust-analyzer/issues/18438
        #adapted_fn
        #warnings

        #start_fn
        #other_items
    })
}
