use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use syn::Error;

#[cfg(not(target_os = "linux"))]
static SQLITE: std::sync::LazyLock<sqlez::thread_safe_connection::ThreadSafeConnection> =
    std::sync::LazyLock::new(|| {
        sqlez::thread_safe_connection::ThreadSafeConnection::new(
            ":memory:",
            false,
            None,
            Some(sqlez::thread_safe_connection::locking_queue()),
        )
    });

#[proc_macro]
pub fn sql(tokens: TokenStream) -> TokenStream {
    let (spans, sql) = make_sql(tokens);

    #[cfg(not(target_os = "linux"))]
    let error = SQLITE.sql_has_syntax_error(sql.trim());

    #[cfg(target_os = "linux")]
    let error: Option<(String, usize)> = None;

    let formatted_sql = sqlformat::format(&sql, &sqlformat::QueryParams::None, Default::default());

    if let Some((error, error_offset)) = error {
        create_error(spans, error_offset, error, &formatted_sql)
    } else {
        format!("r#\"{}\"#", &formatted_sql).parse().unwrap()
    }
}

fn create_error(
    spans: Vec<(usize, Span)>,
    error_offset: usize,
    error: String,
    formatted_sql: &String,
) -> TokenStream {
    let error_span = spans
        .into_iter()
        .skip_while(|(offset, _)| offset <= &error_offset)
        .map(|(_, span)| span)
        .next()
        .unwrap_or_else(Span::call_site);
    let error_text = format!("Sql Error: {}\nFor Query: {}", error, formatted_sql);
    TokenStream::from(Error::new(error_span.into(), error_text).into_compile_error())
}

fn make_sql(tokens: TokenStream) -> (Vec<(usize, Span)>, String) {
    let mut sql_tokens = vec![];
    flatten_stream(tokens, &mut sql_tokens);
    // Lookup of spans by offset at the end of the token
    let mut spans: Vec<(usize, Span)> = Vec::new();
    let mut sql = String::new();
    for (token_text, span) in sql_tokens {
        sql.push_str(&token_text);
        spans.push((sql.len(), span));
    }
    (spans, sql)
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
                result.push((format!("{} ", ident), ident.span()));
            }
            leaf_tree => result.push((leaf_tree.to_string(), leaf_tree.span())),
        }
    }
}

fn open_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => "( ".to_string(),
        Delimiter::Brace => "[ ".to_string(),
        Delimiter::Bracket => "{ ".to_string(),
        Delimiter::None => "".to_string(),
    }
}

fn close_delimiter(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Parenthesis => " ) ".to_string(),
        Delimiter::Brace => " ] ".to_string(),
        Delimiter::Bracket => " } ".to_string(),
        Delimiter::None => "".to_string(),
    }
}
