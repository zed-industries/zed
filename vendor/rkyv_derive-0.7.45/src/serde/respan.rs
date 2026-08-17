// Code in this file is taken whole or in part from serde: https://github.com/serde-rs/serde
// The original license for this code is included in LICENSE

use proc_macro2::{Group, Span, TokenStream, TokenTree};

pub(crate) fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}
