use std::iter::FromIterator;

use proc_macro2::{Group, Span, TokenStream, TokenTree};

/// Deeply change the span of some tokens, ensuring that the output only references `span`.
///
/// Macros such as `quote_spanned` preserve the spans of interpolated tokens, which is useful.
/// However, in some very specific scenarios it is desirable to suppress the original span
/// information in favor of a different one.
///
/// For more information, see [dtolnay/syn#309](https://github.com/dtolnay/syn/issues/309).
pub(crate) fn change_span(tokens: TokenStream, span: Span) -> TokenStream {
    let mut result = vec![];
    for mut token in tokens {
        match token {
            TokenTree::Group(group) => {
                let mut new_group =
                    Group::new(group.delimiter(), change_span(group.stream(), span));
                new_group.set_span(span);
                result.push(TokenTree::Group(new_group));
            }
            _ => {
                token.set_span(span);
                result.push(token);
            }
        }
    }
    FromIterator::from_iter(result.into_iter())
}
