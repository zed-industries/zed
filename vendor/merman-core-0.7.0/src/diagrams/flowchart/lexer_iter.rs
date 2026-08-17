use super::{LexError, Lexer, Tok};

impl<'input> Iterator for Lexer<'input> {
    type Item = std::result::Result<(usize, Tok, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.pending.pop_front() {
            return Some(Ok(tok));
        }

        if self.pos >= self.input.len() {
            return None;
        }

        if let Some(sep) = self.lex_sep() {
            self.allow_header_direction = false;
            return Some(Ok(sep));
        }
        self.skip_ws();
        if self.pos >= self.input.len() {
            return None;
        }
        if let Some(sep) = self.lex_sep() {
            self.allow_header_direction = false;
            return Some(Ok(sep));
        }
        if let Some(sep) = self.lex_comment() {
            self.allow_header_direction = false;
            return Some(Ok(sep));
        }
        self.skip_ws();
        if self.pos >= self.input.len() {
            return None;
        }

        let start = self.pos;
        if let Some(tok) = self.lex_direction_stmt() {
            return Some(Ok(tok));
        }
        if let Some(res) = self.lex_style_stmt() {
            return Some(res);
        }
        if let Some(res) = self.lex_classdef_stmt() {
            return Some(res);
        }
        if let Some(res) = self.lex_class_assign_stmt() {
            return Some(res);
        }
        if let Some(res) = self.lex_click_stmt() {
            return Some(res);
        }
        if let Some(res) = self.lex_link_style_stmt() {
            return Some(res);
        }
        if let Some(tok) = self.lex_shape_data() {
            return Some(Ok(tok));
        }
        if self.starts_with_kw("flowchart-elk") {
            self.pos += "flowchart-elk".len();
            self.allow_header_direction = true;
            return Some(Ok((start, Tok::KwFlowchartElk, self.pos)));
        }
        if self.starts_with_kw("flowchart") {
            self.pos += "flowchart".len();
            self.allow_header_direction = true;
            return Some(Ok((start, Tok::KwFlowchart, self.pos)));
        }
        if self.starts_with_kw("graph") {
            self.pos += "graph".len();
            self.allow_header_direction = true;
            return Some(Ok((start, Tok::KwGraph, self.pos)));
        }
        if self.starts_with_kw("subgraph") {
            self.pos += "subgraph".len();
            if let Some(header) = self.lex_subgraph_header_after_keyword() {
                self.pending.push_back(header);
            }
            return Some(Ok((start, Tok::KwSubgraph, self.pos)));
        }
        if self.starts_with_kw("end") {
            self.pos += "end".len();
            return Some(Ok((start, Tok::KwEnd, self.pos)));
        }

        if let Some(tok) = self.lex_style_sep() {
            return Some(Ok(tok));
        }

        if let Some(tok) = self.lex_direction() {
            return Some(Ok(tok));
        }
        self.allow_header_direction = false;

        if let Some(res) = self.lex_node_label() {
            return Some(res);
        }

        if let Some(res) = self.lex_arrow_and_label() {
            return Some(res);
        }

        if let Some(tok) = self.lex_edge_id() {
            return Some(Ok(tok));
        }

        if let Some(tok) = self.lex_id() {
            return Some(Ok(tok));
        }

        if let Some(tok) = self.lex_amp() {
            return Some(Ok(tok));
        }

        // Skip unknown single byte to avoid infinite loops.
        let _ = self.bump();
        Some(Err(LexError {
            message: format!("Unexpected character at {start}"),
        }))
    }
}
