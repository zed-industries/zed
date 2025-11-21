// TODO kb move impl LspStore here all semantic token-related methods

use std::{iter::Peekable, slice::ChunksExact};

use collections::IndexMap;

use crate::lsp_command::SemanticTokensEdit;

/// All the semantic token tokens for a buffer.
///
/// This aggregates semantic tokens from multiple language servers in a specific order.
/// Semantic tokens later in the list will override earlier ones in case of overlap.
#[derive(Default, Debug, Clone)]
pub struct BufferSemanticTokens {
    pub servers: IndexMap<lsp::LanguageServerId, ServerSemanticTokens>,
}

struct BufferSemanticTokensIter<'a> {
    iters: Vec<(lsp::LanguageServerId, Peekable<SemanticTokensIter<'a>>)>,
}

/// All the semantic tokens for a buffer, from a single language server.
#[derive(Debug, Clone)]
pub struct ServerSemanticTokens {
    /// Each value is:
    /// data[5*i] - deltaLine: token line number, relative to the start of the previous token
    /// data[5*i+1] - deltaStart: token start character, relative to the start of the previous token (relative to 0 or the previous token’s start if they are on the same line)
    /// data[5*i+2] - length: the length of the token.
    /// data[5*i+3] - tokenType: will be looked up in SemanticTokensLegend.tokenTypes. We currently ask that tokenType < 65536.
    /// data[5*i+4] - tokenModifiers: each set bit will be looked up in SemanticTokensLegend.tokenModifiers
    ///
    /// See https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/ for more.
    data: Vec<u32>,

    pub(crate) result_id: Option<String>,
}

pub struct SemanticTokensIter<'a> {
    prev: Option<(u32, u32)>,
    data: ChunksExact<'a, u32>,
}

// A single item from `data`.
struct SemanticTokenValue {
    delta_line: u32,
    delta_start: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
}

/// A semantic token, independent of its position.
#[derive(Debug, PartialEq, Eq)]
pub struct SemanticToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers: u32,
}

impl BufferSemanticTokens {
    pub fn all_tokens(&self) -> impl Iterator<Item = (lsp::LanguageServerId, SemanticToken)> {
        let iters = self
            .servers
            .iter()
            .map(|(server_id, tokens)| (*server_id, tokens.tokens().peekable()))
            .collect();

        BufferSemanticTokensIter { iters }
    }
}

impl ServerSemanticTokens {
    pub fn from_full(data: Vec<u32>, result_id: Option<String>) -> Self {
        ServerSemanticTokens { data, result_id }
    }

    pub(crate) fn apply(&mut self, edits: &[SemanticTokensEdit]) {
        for edit in edits {
            let start = edit.start as usize;
            let end = start + edit.delete_count as usize;
            self.data.splice(start..end, edit.data.iter().copied());
        }
    }

    pub fn tokens(&self) -> SemanticTokensIter<'_> {
        SemanticTokensIter {
            prev: None,
            data: self.data.chunks_exact(5),
        }
    }
}

impl Iterator for BufferSemanticTokensIter<'_> {
    type Item = (lsp::LanguageServerId, SemanticToken);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, _) = self
            .iters
            // TODO kb can we avoid re-iterating each time?
            .iter_mut()
            .enumerate()
            .filter_map(|(i, (_, iter))| iter.peek().map(|peeked| (i, peeked)))
            .min_by_key(|(_, tok)| (tok.line, tok.start))?;

        let (id, iter) = &mut self.iters[i];
        Some((*id, iter.next()?))
    }
}

impl Iterator for SemanticTokensIter<'_> {
    type Item = SemanticToken;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.data.next()?;
        let token = SemanticTokenValue {
            delta_line: chunk[0],
            delta_start: chunk[1],
            length: chunk[2],
            token_type: chunk[3],
            token_modifiers: chunk[4],
        };

        let (line, start) = if let Some((last_line, last_start)) = self.prev {
            let line = last_line + token.delta_line;
            let start = if token.delta_line == 0 {
                last_start + token.delta_start
            } else {
                token.delta_start
            };
            (line, start)
        } else {
            (token.delta_line, token.delta_start)
        };

        self.prev = Some((line, start));

        Some(SemanticToken {
            line,
            start,
            length: token.length,
            token_type: token.token_type,
            token_modifiers: token.token_modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_tokens() {
        // Example from the spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens
        let tokens = ServerSemanticTokens::from_full(
            vec![2, 5, 3, 0, 3, 0, 5, 4, 1, 0, 3, 2, 7, 2, 0],
            None,
        )
        .tokens()
        .collect::<Vec<SemanticToken>>();

        // The spec uses 1-based line numbers, and 0-based character numbers. This test uses 0-based for both.
        assert_eq!(
            tokens,
            &[
                SemanticToken {
                    line: 2,
                    start: 5,
                    length: 3,
                    token_type: 0,
                    token_modifiers: 3
                },
                SemanticToken {
                    line: 2,
                    start: 10,
                    length: 4,
                    token_type: 1,
                    token_modifiers: 0
                },
                SemanticToken {
                    line: 5,
                    start: 2,
                    length: 7,
                    token_type: 2,
                    token_modifiers: 0
                }
            ]
        );
    }

    #[test]
    fn iterate_all_tokens() {
        // A token at 0,0 and at 1,0
        let tokens_1 = ServerSemanticTokens::from_full(vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], None);
        // A token at 0,5 and at 2,10
        let tokens_2 = ServerSemanticTokens::from_full(vec![0, 5, 0, 0, 0, 2, 10, 0, 0, 0], None);

        let buffer_tokens = BufferSemanticTokens {
            servers: IndexMap::from_iter([
                (lsp::LanguageServerId(1), tokens_1),
                (lsp::LanguageServerId(2), tokens_2),
            ]),
        };

        let all_tokens = buffer_tokens
            .all_tokens()
            .map(|(server, tok)| (server, tok.line, tok.start))
            .collect::<Vec<_>>();
        assert_eq!(
            all_tokens,
            [
                (lsp::LanguageServerId(1), 0, 0),
                (lsp::LanguageServerId(2), 0, 5),
                (lsp::LanguageServerId(1), 1, 0),
                (lsp::LanguageServerId(2), 2, 10),
            ]
        )
    }
}
