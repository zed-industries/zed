use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::path::Path;

pub(crate) mod xx3;

struct TokenTreeDeepIterator {
    stack: Vec<proc_macro::token_stream::IntoIter>,
}

impl Iterator for TokenTreeDeepIterator {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut iter = self.stack.pop()?;
            let Some(token) = iter.next() else {
                continue;
            };
            self.stack.push(iter);
            if let TokenTree::Group(group) = &token {
                self.stack.push(group.stream().into_iter());
            }
            return Some(token);
        }
    }
}

/// Content (position-independent) identity for an ignored token. Groups
/// contribute only their delimiter and the deep iterator visits children
/// separately.
fn token_content(token: &TokenTree) -> String {
    match token {
        TokenTree::Group(group) => match group.delimiter() {
            Delimiter::Parenthesis => "(".to_string(),
            Delimiter::Brace => "{".to_string(),
            Delimiter::Bracket => "[".to_string(),
            Delimiter::None => String::new(),
        },
        TokenTree::Ident(ident) => ident.to_string(),
        TokenTree::Punct(punct) => punct.as_char().to_string(),
        TokenTree::Literal(literal) => literal.to_string(),
    }
}

/// Hash location of every token in `tokens`. Tokens under `ignore_base` contribute
/// content instead, so def-site spans from that crate don't shift the hash.
#[allow(clippy::unnecessary_map_or)]
pub(crate) fn location_hash(tokens: TokenStream, ignore_base: Option<&Path>) -> u64 {
    let iterator = TokenTreeDeepIterator {
        stack: vec![tokens.into_iter()],
    };

    // TODO: Can we avoid doing multiple hashes?
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    let mut last_hash = 0_u64;
    for token in iterator {
        let span = token.span();
        buffer.clear();
        buffer.extend_from_slice(&last_hash.to_be_bytes());

        let ignored = ignore_base.map_or(false, |base| {
            crate::fallback::local_file(&span).map_or(false, |file| file.starts_with(base))
        });

        if ignored {
            buffer.extend_from_slice(token_content(&token).as_bytes());
        } else {
            let line = crate::fallback::line(&span);
            let column = crate::fallback::column(&span);
            let file = crate::fallback::file(&span);
            buffer.extend_from_slice(&line.to_be_bytes());
            buffer.extend_from_slice(&column.to_be_bytes());
            buffer.extend_from_slice(file.as_bytes());
        }

        last_hash = crate::hash::xx3::xx3hash_bytes(&buffer);
    }

    last_hash
}
