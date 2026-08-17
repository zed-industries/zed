use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, File};

use crate::BindgenOptions;

mod merge_extern_blocks;
mod sort_semantically;

use merge_extern_blocks::merge_extern_blocks;
use sort_semantically::sort_semantically;

struct PostProcessingPass {
    should_run: fn(&BindgenOptions) -> bool,
    run: fn(&mut File),
}

// TODO: This can be a const fn when mutable references are allowed in const
// context.
macro_rules! pass {
    ($pass:ident) => {
        PostProcessingPass {
            should_run: |options| options.$pass,
            run: |file| $pass(file),
        }
    };
}

const PASSES: &[PostProcessingPass] =
    &[pass!(merge_extern_blocks), pass!(sort_semantically)];

pub(crate) fn postprocessing(
    items: Vec<TokenStream>,
    options: &BindgenOptions,
) -> TokenStream {
    let items = items.into_iter().collect();
    let require_syn = PASSES.iter().any(|pass| (pass.should_run)(options));

    if !require_syn {
        return items;
    }

    // This syn business is a hack, for now. This means that we are re-parsing already
    // generated code using `syn` (as opposed to `quote`) because `syn` provides us more
    // control over the elements.
    // The `unwrap` here is deliberate because bindgen should generate valid rust items at all
    // times.
    let mut file = parse2::<File>(items).unwrap();

    for pass in PASSES {
        if (pass.should_run)(options) {
            (pass.run)(&mut file);
        }
    }

    file.into_token_stream()
}
