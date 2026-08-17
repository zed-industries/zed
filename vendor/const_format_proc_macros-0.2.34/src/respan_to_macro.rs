use crate::parse_utils::TokenTreeExt;

use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree as TokenTree2};

const MSG: &str = "Expected the macro to be called as `respan_to!((tokens) more tokens)`";

fn parse_paren(tt: TokenTree2) -> TokenStream2 {
    match tt {
        TokenTree2::Group(group) if group.delimiter() == Delimiter::Parenthesis => group.stream(),
        _ => panic!("{}", MSG),
    }
}

fn get_span(ts: TokenStream2) -> Span {
    let mut iter = ts.into_iter();

    match iter.next() {
        Some(TokenTree2::Group(group)) if group.delimiter() == Delimiter::None => {
            get_span(group.stream())
        }
        Some(first_tt) => {
            let mut span = first_tt.span();

            for tt in iter {
                span = span.join(tt.span()).unwrap_or(span);
            }
            span
        }
        None => Span::mixed_site(),
    }
}

pub(crate) fn implementation(ts: TokenStream2) -> TokenStream2 {
    let mut iter = ts.into_iter();

    let span_to = get_span(parse_paren(iter.next().expect(MSG)));

    iter.map(|tt| tt.set_span_recursive(span_to)).collect()
}
