use std::borrow::Cow;

use crate::indentation::Indentation;
use crate::inline_block::InlineBlock;
use crate::params::Params;
use crate::tokenizer::{Token, TokenKind};
use crate::{FormatOptions, QueryParams};

pub(crate) fn format(tokens: &[Token<'_>], params: &QueryParams, options: FormatOptions) -> String {
    let mut formatter = Formatter::new(tokens, params, options);
    let mut formatted_query = String::new();
    for (index, token) in tokens.iter().enumerate() {
        formatter.index = index;

        if token.kind == TokenKind::Whitespace {
            // ignore (we do our own whitespace formatting)
        } else if token.kind == TokenKind::LineComment {
            formatter.format_line_comment(token, &mut formatted_query);
        } else if token.kind == TokenKind::BlockComment {
            formatter.format_block_comment(token, &mut formatted_query);
        } else if token.kind == TokenKind::ReservedTopLevel {
            formatter.format_top_level_reserved_word(token, &mut formatted_query);
            formatter.previous_reserved_word = Some(token);
        } else if token.kind == TokenKind::ReservedTopLevelNoIndent {
            formatter.format_top_level_reserved_word_no_indent(token, &mut formatted_query);
            formatter.previous_reserved_word = Some(token);
        } else if token.kind == TokenKind::ReservedNewline {
            formatter.format_newline_reserved_word(token, &mut formatted_query);
            formatter.previous_reserved_word = Some(token);
        } else if token.kind == TokenKind::Reserved {
            formatter.format_with_spaces(token, &mut formatted_query);
            formatter.previous_reserved_word = Some(token);
        } else if token.kind == TokenKind::OpenParen {
            formatter.format_opening_parentheses(token, &mut formatted_query);
        } else if token.kind == TokenKind::CloseParen {
            formatter.format_closing_parentheses(token, &mut formatted_query);
        } else if token.kind == TokenKind::Placeholder {
            formatter.format_placeholder(token, &mut formatted_query);
        } else if token.value == "," {
            formatter.format_comma(token, &mut formatted_query);
        } else if token.value == ":" {
            formatter.format_with_space_after(token, &mut formatted_query);
        } else if token.value == "." {
            formatter.format_without_spaces(token, &mut formatted_query);
        } else if token.value == ";" {
            formatter.format_query_separator(token, &mut formatted_query);
        } else {
            formatter.format_with_spaces(token, &mut formatted_query);
        }
    }
    formatted_query.trim().to_string()
}

struct Formatter<'a> {
    index: usize,
    previous_reserved_word: Option<&'a Token<'a>>,
    tokens: &'a [Token<'a>],
    params: Params<'a>,
    options: FormatOptions,
    indentation: Indentation,
    inline_block: InlineBlock,
}

impl<'a> Formatter<'a> {
    fn new(tokens: &'a [Token<'a>], params: &'a QueryParams, options: FormatOptions) -> Self {
        Formatter {
            index: 0,
            previous_reserved_word: None,
            tokens,
            params: Params::new(params),
            options,
            indentation: Indentation::new(options),
            inline_block: InlineBlock::new(),
        }
    }

    fn format_line_comment(&self, token: &Token<'_>, query: &mut String) {
        let is_whitespace_followed_by_special_token =
            self.next_token(1).map_or(false, |current_token| {
                current_token.kind == TokenKind::Whitespace
                    && self.next_token(2).map_or(false, |next_token| {
                        matches!(
                            next_token.kind,
                            TokenKind::Number
                                | TokenKind::String
                                | TokenKind::Word
                                | TokenKind::ReservedTopLevel
                                | TokenKind::ReservedTopLevelNoIndent
                                | TokenKind::ReservedNewline
                                | TokenKind::Reserved
                        )
                    })
            });

        let previous_token = self.previous_token(1);
        if previous_token.is_some()
            && previous_token.unwrap().value.contains('\n')
            && is_whitespace_followed_by_special_token
        {
            self.add_new_line(query);
        } else if let Some(Token { value, .. }) = self.previous_token(2) {
            if *value == "," {
                self.trim_all_spaces_end(query);
                query.push_str("  ");
            }
        }
        query.push_str(token.value);
        self.add_new_line(query);
    }

    fn format_block_comment(&self, token: &Token<'_>, query: &mut String) {
        self.add_new_line(query);
        query.push_str(&self.indent_comment(token.value));
        self.add_new_line(query);
    }

    fn format_top_level_reserved_word(&mut self, token: &Token<'_>, query: &mut String) {
        self.indentation.decrease_top_level();
        self.add_new_line(query);
        self.indentation.increase_top_level();
        query.push_str(&self.equalize_whitespace(&self.format_reserved_word(token.value)));
        self.add_new_line(query);
    }

    fn format_top_level_reserved_word_no_indent(&mut self, token: &Token<'_>, query: &mut String) {
        self.indentation.decrease_top_level();
        self.add_new_line(query);
        query.push_str(&self.equalize_whitespace(&self.format_reserved_word(token.value)));
        self.add_new_line(query);
    }

    fn format_newline_reserved_word(&self, token: &Token<'_>, query: &mut String) {
        self.add_new_line(query);
        query.push_str(&self.equalize_whitespace(&self.format_reserved_word(token.value)));
        query.push(' ');
    }

    fn format_with_spaces(&self, token: &Token<'_>, query: &mut String) {
        if token.kind == TokenKind::Reserved {
            let value = self.equalize_whitespace(&self.format_reserved_word(token.value));
            query.push_str(&value);
            query.push(' ');
        } else {
            query.push_str(token.value);
            query.push(' ');
        };
    }

    // Opening parentheses increase the block indent level and start a new line
    fn format_opening_parentheses(&mut self, token: &Token<'_>, query: &mut String) {
        const PRESERVE_WHITESPACE_FOR: &[TokenKind] = &[
            TokenKind::Whitespace,
            TokenKind::OpenParen,
            TokenKind::LineComment,
        ];

        // Take out the preceding space unless there was whitespace there in the original query
        // or another opening parens or line comment
        let previous_token = self.previous_token(1);
        if previous_token.is_none()
            || !PRESERVE_WHITESPACE_FOR.contains(&previous_token.unwrap().kind)
        {
            self.trim_spaces_end(query);
        }
        if self.options.uppercase {
            query.push_str(&token.value.to_uppercase());
        } else {
            query.push_str(token.value);
        };

        self.inline_block.begin_if_possible(self.tokens, self.index);

        if !self.inline_block.is_active() {
            self.indentation.increase_block_level();
            self.add_new_line(query);
        }
    }

    // Closing parentheses decrease the block indent level
    fn format_closing_parentheses(&mut self, token: &Token<'_>, query: &mut String) {
        let mut token = token.clone();
        let value = if self.options.uppercase {
            token.value.to_uppercase()
        } else {
            token.value.to_string()
        };
        token.value = &value;

        if self.inline_block.is_active() {
            self.inline_block.end();
            self.format_with_space_after(&token, query);
        } else {
            self.indentation.decrease_block_level();
            self.add_new_line(query);
            self.format_with_spaces(&token, query);
        }
    }

    fn format_placeholder(&mut self, token: &'a Token<'a>, query: &mut String) {
        query.push_str(self.params.get(token));
        query.push(' ');
    }

    // Commas start a new line (unless within inline parentheses or SQL "LIMIT" clause)
    fn format_comma(&self, token: &Token<'_>, query: &mut String) {
        self.trim_spaces_end(query);
        query.push_str(token.value);
        query.push(' ');

        if self.inline_block.is_active() {
            return;
        }
        if self
            .previous_reserved_word
            .map(|word| word.value.to_lowercase() == "limit")
            .unwrap_or(false)
        {
            return;
        }
        self.add_new_line(query);
    }

    fn format_with_space_after(&self, token: &Token<'_>, query: &mut String) {
        self.trim_spaces_end(query);
        query.push_str(token.value);
        query.push(' ');
    }

    fn format_without_spaces(&self, token: &Token<'_>, query: &mut String) {
        self.trim_spaces_end(query);
        query.push_str(token.value);
    }

    fn format_query_separator(&mut self, token: &Token<'_>, query: &mut String) {
        self.indentation.reset_indentation();
        self.trim_spaces_end(query);
        query.push_str(token.value);
        for _ in 0..self.options.lines_between_queries {
            query.push('\n');
        }
    }

    fn add_new_line(&self, query: &mut String) {
        self.trim_spaces_end(query);
        if !query.ends_with('\n') {
            query.push('\n');
        }
        query.push_str(&self.indentation.get_indent());
    }

    fn trim_spaces_end(&self, query: &mut String) {
        query.truncate(query.trim_end_matches([' ', '\t']).len());
    }

    fn trim_all_spaces_end(&self, query: &mut String) {
        query.truncate(query.trim_end_matches(|c: char| c.is_whitespace()).len());
    }

    fn indent_comment(&self, token: &str) -> String {
        let mut combined = String::with_capacity(token.len() + 4);
        for (i, line) in token.split('\n').enumerate() {
            if i == 0 {
                combined.push_str(line)
            } else if line.starts_with([' ', '\t']) {
                let indent = self.indentation.get_indent();
                let start_trimmed = line.trim_start_matches([' ', '\t']);
                combined.reserve(indent.len() + start_trimmed.len() + 2);
                combined.push('\n');
                combined.push_str(&indent);
                combined.push(' ');
                combined.push_str(start_trimmed);
            } else {
                combined.reserve(line.len() + 1);
                combined.push('\n');
                combined.push_str(line);
            }
        }
        combined
    }

    fn format_reserved_word<'t>(&self, token: &'t str) -> Cow<'t, str> {
        if self.options.uppercase {
            Cow::Owned(token.to_uppercase())
        } else {
            Cow::Borrowed(token)
        }
    }

    /// Replace any sequence of whitespace characters with single space
    fn equalize_whitespace(&self, token: &str) -> String {
        let mut combined = String::with_capacity(token.len());
        for s in token.split(char::is_whitespace).filter(|s| !s.is_empty()) {
            if !combined.is_empty() {
                combined.push(' ');
            }
            combined.push_str(s);
        }
        combined
    }

    fn previous_token(&self, idx: usize) -> Option<&Token<'_>> {
        let index = self.index.checked_sub(idx);
        if let Some(index) = index {
            self.tokens.get(index)
        } else {
            None
        }
    }

    fn next_token(&self, idx: usize) -> Option<&Token<'_>> {
        let index = self.index.checked_add(idx);
        if let Some(index) = index {
            self.tokens.get(index)
        } else {
            None
        }
    }
}
