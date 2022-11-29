use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use sqlez::thread_safe_connection::ThreadSafeConnection;
use syn::Error;

lazy_static::lazy_static! {
    static ref SQLITE: ThreadSafeConnection = ThreadSafeConnection::new(":memory:", false);
}

#[proc_macro]
pub fn sql(tokens: TokenStream) -> TokenStream {
    let mut sql_tokens = vec![];
    flatten_stream(tokens.clone(), &mut sql_tokens);

    // Lookup of spans by offset at the end of the token
    let mut spans: Vec<(usize, Span)> = Vec::new();
    let mut sql = String::new();
    for (token_text, span) in sql_tokens {
        sql.push_str(&token_text);
        spans.push((sql.len(), span));
    }

    let error = SQLITE.sql_has_syntax_error(sql.trim());

    if let Some((error, error_offset)) = error {
        let error_span = spans
            .into_iter()
            .skip_while(|(offset, _)| offset <= &error_offset)
            .map(|(_, span)| span)
            .next()
            .unwrap_or(Span::call_site());

        let error_text = format!("Sql Error: {}\nFor Query: {}", error, sql);
        TokenStream::from(Error::new(error_span.into(), error_text).into_compile_error())
    } else {
        format!("r#\"{}\"#", &sql).parse().unwrap()
    }
}

/// This method exists to normalize the representation of groups
/// to always include spaces between tokens. This is why we don't use the usual .to_string().
/// This allows our token search in token_at_offset to resolve
/// ambiguity of '(tokens)' vs. '( token )', due to sqlite requiring byte offsets
fn flatten_stream(tokens: TokenStream, result: &mut Vec<(String, Span)>) {
    for token_tree in tokens.into_iter() {
        match token_tree {
            TokenTree::Group(group) => {
                // push open delimiter
                result.push((open_delimiter(group.delimiter()), group.span()));
                // recurse
                flatten_stream(group.stream(), result);
                // push close delimiter
                result.push((close_delimiter(group.delimiter()), group.span()));
            }
            TokenTree::Ident(ident) => {
                result.push((format!("{} ", ident.to_string()), ident.span()));
            }
            leaf_tree => result.push((leaf_tree.to_string(), leaf_tree.span())),
        }
    }
}

fn open_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => "(".to_string(),
        Delimiter::Brace => "[".to_string(),
        Delimiter::Bracket => "{".to_string(),
        Delimiter::None => "".to_string(),
    }
}

fn close_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => ")".to_string(),
        Delimiter::Brace => "]".to_string(),
        Delimiter::Bracket => "}".to_string(),
        Delimiter::None => "".to_string(),
    }
}
