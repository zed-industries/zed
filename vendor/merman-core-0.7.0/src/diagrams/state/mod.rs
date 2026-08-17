use std::collections::VecDeque;

mod ast;
mod db;
mod parse;
mod render_model;

pub use render_model::{
    StateDiagramRenderEdge, StateDiagramRenderLink, StateDiagramRenderLinks,
    StateDiagramRenderModel, StateDiagramRenderNode, StateDiagramRenderNote,
    StateDiagramRenderState, StateDiagramRenderStyleClass,
};

pub use parse::{parse_state, parse_state_model_for_render};

pub(crate) use ast::{ClickStmt, Note, RelationStmt, StateStmt, Stmt};

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr, clippy::filter_map_identity)]
    state_grammar,
    "/diagrams/state_grammar.rs"
);

#[derive(Debug, Clone)]
pub(crate) enum Tok {
    Newline,
    Sd,
    Id(String),
    StyledId((String, String)),
    EdgeState,
    Descr(String),
    Arrow,
    StructStart,
    StructStop,
    As,
    Note,
    LeftOf,
    RightOf,
    NoteText(String),
    StateDescr(String),
    CompositState(String),
    Fork(String),
    Join(String),
    Choice(String),
    Concurrent,
    HideEmptyDescription,
    ScaleWidth(usize),
    ClassDef,
    ClassDefId(String),
    ClassDefStyleOpts(String),
    Class,
    ClassEntityIds(String),
    StyleClass(String),
    Style,
    StyleIds(String),
    StyleDefStyleOpts(String),
    Direction(String),
    AccTitle(String),
    AccDescr(String),
    AccDescrMultiline(String),
    Click,
    Href,
    StringLit(String),
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub(crate) struct LexError {
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Default,
    Struct,
    State,
    StateId,
}

struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    pending: VecDeque<(usize, Tok, usize)>,
    modes: Vec<Mode>,
    emitted_eof_newline: bool,
}

impl<'input> Lexer<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            pending: VecDeque::new(),
            modes: vec![Mode::Default],
            emitted_eof_newline: false,
        }
    }

    fn normalize_note_block_text(raw: &'input str) -> String {
        // Mermaid's state diagram note blocks do not preserve leading indentation for each line.
        // The upstream SVG baselines reflect line-wise trimming.
        let lines: Vec<&str> = raw
            .lines()
            .map(|l| l.trim_end_matches('\r').trim())
            .collect();

        let mut start = 0usize;
        let mut end = lines.len();
        while start < end && lines[start].is_empty() {
            start += 1;
        }
        while end > start && lines[end - 1].is_empty() {
            end -= 1;
        }

        lines[start..end].join("\n")
    }

    fn mode(&self) -> Mode {
        *self.modes.last().unwrap_or(&Mode::Default)
    }

    fn push_mode(&mut self, m: Mode) {
        self.modes.push(m);
    }

    fn pop_mode(&mut self) {
        if self.modes.len() > 1 {
            self.modes.pop();
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn starts_with(&self, s: &str) -> bool {
        let hay = self.input.as_bytes();
        let pat = s.as_bytes();
        hay.get(self.pos..)
            .is_some_and(|tail| tail.starts_with(pat))
    }

    fn starts_with_ci(&self, s: &str) -> bool {
        let hay = self.input.as_bytes();
        let pat = s.as_bytes();
        hay.len() >= self.pos + pat.len()
            && hay[self.pos..self.pos + pat.len()].eq_ignore_ascii_case(pat)
    }

    fn starts_with_word_ci(&self, s: &str) -> bool {
        if !self.starts_with_ci(s) {
            return false;
        }
        let after = self.pos + s.len();
        if after >= self.input.len() {
            return true;
        }
        let b = self.input.as_bytes()[after];
        b.is_ascii_whitespace() || matches!(b, b'{' | b'}' | b'[' | b']' | b'"' | b':' | b';')
    }

    fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.pos += 1;
                continue;
            }
            break;
        }
    }

    fn read_to_newline(&mut self) -> String {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'\n' {
                break;
            }
            self.pos += 1;
        }
        self.input[start..self.pos].to_string()
    }

    fn lex_newline(&mut self) -> Option<(usize, Tok, usize)> {
        if self.peek()? != b'\n' {
            return None;
        }
        let start = self.pos;
        while let Some(b'\n') = self.peek() {
            self.pos += 1;
        }
        if matches!(self.mode(), Mode::State | Mode::StateId) {
            self.pop_mode();
        }
        Some((start, Tok::Newline, self.pos))
    }

    fn skip_comment(&mut self) -> bool {
        if self.starts_with("%%") {
            let _ = self.read_to_newline();
            return true;
        }
        if self.peek() == Some(b'#') {
            let _ = self.read_to_newline();
            return true;
        }
        false
    }

    fn lex_string_lit(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        if self.peek()? != b'"' {
            return None;
        }
        let start = self.pos;
        self.pos += 1;
        let body_start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'"' {
                break;
            }
            self.pos += 1;
        }
        if self.peek() != Some(b'"') {
            return Some(Err(LexError {
                message: "Unterminated string literal; missing '\"'".to_string(),
            }));
        }
        let body = self.input[body_start..self.pos].to_string();
        self.pos += 1;
        Some(Ok((start, Tok::StringLit(body), self.pos)))
    }

    fn lex_sd_header(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.starts_with_ci("stateDiagram-v2") {
            self.pos += "stateDiagram-v2".len();
            return Some((start, Tok::Sd, self.pos));
        }
        if self.starts_with_ci("stateDiagram") {
            self.pos += "stateDiagram".len();
            return Some((start, Tok::Sd, self.pos));
        }
        None
    }

    fn lex_direction(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if !self.starts_with_word_ci("direction") {
            return None;
        }
        self.pos += "direction".len();
        self.skip_ws();
        let dir_start = self.pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_alphabetic() {
                self.pos += 1;
                continue;
            }
            break;
        }
        let dir = self.input[dir_start..self.pos].trim().to_string();
        if matches!(dir.as_str(), "TB" | "BT" | "RL" | "LR") {
            let _ = self.read_to_newline();
            return Some((start, Tok::Direction(dir), self.pos));
        }
        None
    }

    fn lex_accessibility(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if self.starts_with_word_ci("accTitle") {
            self.pos += "accTitle".len();
            self.skip_ws();
            if self.peek() != Some(b':') {
                return None;
            }
            self.pos += 1;
            self.skip_ws();
            let value = self.read_to_newline();
            return Some(Ok((
                start,
                Tok::AccTitle(value.trim().to_string()),
                self.pos,
            )));
        }

        if !self.starts_with_word_ci("accDescr") {
            return None;
        }
        self.pos += "accDescr".len();
        self.skip_ws();
        if self.peek() == Some(b':') {
            self.pos += 1;
            self.skip_ws();
            let value = self.read_to_newline();
            return Some(Ok((
                start,
                Tok::AccDescr(value.trim().to_string()),
                self.pos,
            )));
        }
        if self.peek() == Some(b'{') {
            self.pos += 1;
            let body_start = self.pos;
            let tail = &self.input.as_bytes()[self.pos..];
            let Some(end_rel) = tail.iter().position(|&b| b == b'}') else {
                return Some(Err(LexError {
                    message: "Unterminated accDescr block; missing '}'".to_string(),
                }));
            };
            let body = self.input[body_start..body_start + end_rel].to_string();
            self.pos = body_start + end_rel + 1;
            return Some(Ok((start, Tok::AccDescrMultiline(body), self.pos)));
        }
        None
    }

    fn lex_stmt_line(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;

        if self.starts_with_ci("hide empty description") {
            self.pos += "hide empty description".len();
            let _ = self.read_to_newline();
            return Some(Ok((start, Tok::HideEmptyDescription, self.pos)));
        }

        if self.starts_with_word_ci("scale") {
            self.pos += "scale".len();
            self.skip_ws();

            let width_start = self.pos;
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    self.pos += 1;
                    continue;
                }
                break;
            }
            if self.pos == width_start {
                return Some(Err(LexError {
                    message: "Expected a width number after 'scale'".to_string(),
                }));
            }
            let width: usize = match self.input[width_start..self.pos].parse() {
                Ok(v) => v,
                Err(_) => {
                    return Some(Err(LexError {
                        message: "Invalid width number after 'scale'".to_string(),
                    }));
                }
            };

            self.skip_ws();
            if !self.starts_with_word_ci("width") {
                return Some(Err(LexError {
                    message: "Expected 'width' after `scale <n>`".to_string(),
                }));
            }
            self.pos += "width".len();
            let _ = self.read_to_newline();
            return Some(Ok((start, Tok::ScaleWidth(width), self.pos)));
        }

        if self.starts_with_word_ci("click") {
            self.pos += "click".len();
            return Some(Ok((start, Tok::Click, self.pos)));
        }
        if self.starts_with_word_ci("href") {
            self.pos += "href".len();
            return Some(Ok((start, Tok::Href, self.pos)));
        }

        if self.starts_with_word_ci("note") {
            let kw_end = self.pos + "note".len();
            self.pos = kw_end;
            self.skip_ws();

            // Floating note: note "text" as id
            if self.peek() == Some(b'"') {
                let Some(Ok((_s, Tok::StringLit(text), _e))) = self.lex_string_lit() else {
                    return Some(Err(LexError {
                        message: "Unterminated note string; missing '\"'".to_string(),
                    }));
                };
                self.skip_ws();
                if !self.starts_with_word_ci("as") {
                    return Some(Err(LexError {
                        message: "Expected 'as' in floating note statement".to_string(),
                    }));
                }
                let as_start = self.pos;
                self.pos += "as".len();
                self.skip_ws();
                let id_start = self.pos;
                let id = self.read_to_newline().trim().to_string();

                self.pending.push_back((start, Tok::Note, kw_end));
                self.pending
                    .push_back((kw_end, Tok::NoteText(text), as_start));
                self.pending.push_back((as_start, Tok::As, id_start));
                self.pending.push_back((id_start, Tok::Id(id), self.pos));
                return self.pending.pop_front().map(Ok);
            }

            // Positioned note: note left of|right of ID : text
            let pos_start = self.pos;
            let pos_tok = if self.starts_with_ci("left of") {
                self.pos += "left of".len();
                Tok::LeftOf
            } else if self.starts_with_ci("right of") {
                self.pos += "right of".len();
                Tok::RightOf
            } else {
                return Some(Err(LexError {
                    message: "Expected 'left of' or 'right of' after 'note'".to_string(),
                }));
            };

            self.skip_ws();
            let id_start = self.pos;
            while let Some(b) = self.peek() {
                if b == b':' || b == b'\n' || b.is_ascii_whitespace() || b == b'-' {
                    break;
                }
                self.pos += 1;
            }
            let id = self.input[id_start..self.pos].trim().to_string();

            self.skip_ws();
            let text_start = self.pos;
            let text = if self.peek() == Some(b':') {
                self.pos += 1;

                self.read_to_newline().trim().to_string()
            } else {
                let Some(rest) = self.input.get(self.pos..) else {
                    return Some(Err(LexError {
                        message: "Internal lexer error: invalid UTF-8 boundary".to_string(),
                    }));
                };
                let rest_lower = rest.to_ascii_lowercase();
                let Some(idx) = rest_lower.find("end note") else {
                    return Some(Err(LexError {
                        message: "Unterminated note block; missing 'end note'".to_string(),
                    }));
                };
                let t = Self::normalize_note_block_text(&rest[..idx]);
                self.pos += idx + "end note".len();
                t
            };

            self.pending.push_back((start, Tok::Note, kw_end));
            self.pending.push_back((pos_start, pos_tok, id_start));
            self.pending.push_back((id_start, Tok::Id(id), text_start));
            self.pending
                .push_back((text_start, Tok::NoteText(text), self.pos));
            return self.pending.pop_front().map(Ok);
        }

        if self.starts_with_word_ci("classDef") {
            let kw_end = self.pos + "classDef".len();
            self.pos = kw_end;
            self.skip_ws();
            let id_start = self.pos;
            while let Some(b) = self.peek() {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    self.pos += 1;
                    continue;
                }
                break;
            }
            let id_end = self.pos;
            let id = self.input[id_start..id_end].trim().to_string();
            self.skip_ws();
            let raw = self.read_to_newline().trim().to_string();

            self.pending.push_back((start, Tok::ClassDef, kw_end));
            self.pending
                .push_back((id_start, Tok::ClassDefId(id), id_end));
            self.pending
                .push_back((id_end, Tok::ClassDefStyleOpts(raw), self.pos));
            return self.pending.pop_front().map(Ok);
        }

        if self.starts_with_word_ci("class") {
            let kw_end = self.pos + "class".len();
            self.pos = kw_end;
            self.skip_ws();
            let ids_start = self.pos;
            let mut ids_end = self.pos;
            loop {
                let word_start = self.pos;
                while let Some(b) = self.peek() {
                    if b.is_ascii_alphanumeric() || b == b'_' {
                        self.pos += 1;
                        continue;
                    }
                    break;
                }
                if self.pos == word_start {
                    break;
                }
                ids_end = self.pos;
                let after_word = self.pos;
                self.skip_ws();
                if self.peek() == Some(b',') {
                    self.pos += 1;
                    self.skip_ws();
                    continue;
                }
                self.pos = after_word;
                break;
            }
            let ids = self.input[ids_start..ids_end].trim().to_string();
            self.skip_ws();
            let style = self.read_to_newline().trim().to_string();

            self.pending.push_back((start, Tok::Class, kw_end));
            self.pending
                .push_back((ids_start, Tok::ClassEntityIds(ids), ids_end));
            self.pending
                .push_back((ids_end, Tok::StyleClass(style), self.pos));
            return self.pending.pop_front().map(Ok);
        }

        if self.starts_with_word_ci("style") {
            let kw_end = self.pos + "style".len();
            self.pos = kw_end;
            self.skip_ws();
            let ids_start = self.pos;
            while let Some(b) = self.peek() {
                if b.is_ascii_alphanumeric() || b == b'_' || b == b',' {
                    self.pos += 1;
                    continue;
                }
                break;
            }
            let ids_end = self.pos;
            let ids = self.input[ids_start..ids_end].trim().to_string();
            self.skip_ws();
            let raw = self.read_to_newline().trim().to_string();

            self.pending.push_back((start, Tok::Style, kw_end));
            self.pending
                .push_back((ids_start, Tok::StyleIds(ids), ids_end));
            self.pending
                .push_back((ids_end, Tok::StyleDefStyleOpts(raw), self.pos));
            return self.pending.pop_front().map(Ok);
        }

        if self.starts_with_word_ci("state") {
            self.pos += "state".len();
            self.push_mode(Mode::State);
            return None;
        }

        None
    }

    fn lex_state_mode_token(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        self.skip_ws();

        self.peek()?;

        if self.mode() == Mode::StateId {
            let body_start = self.pos;
            while let Some(b) = self.peek() {
                if b == b'\n' || b == b'{' {
                    break;
                }
                self.pos += 1;
            }
            let id = self.input[body_start..self.pos].trim().to_string();
            self.pop_mode(); // StateId
            self.pop_mode(); // State
            return Some(Ok((start, Tok::Id(id), self.pos)));
        }

        if self.peek() == Some(b'"') {
            self.pos += 1;
            let body_start = self.pos;
            while let Some(b) = self.peek() {
                if b == b'"' {
                    break;
                }
                self.pos += 1;
            }
            if self.peek() != Some(b'"') {
                return Some(Err(LexError {
                    message: "Unterminated state description string; missing '\"'".to_string(),
                }));
            }
            let body = self.input[body_start..self.pos].to_string();
            self.pos += 1;
            return Some(Ok((start, Tok::StateDescr(body), self.pos)));
        }

        if self.starts_with_word_ci("as") {
            self.pos += "as".len();
            self.push_mode(Mode::StateId);
            return Some(Ok((start, Tok::As, self.pos)));
        }

        // Fork/join/choice markers are recognized using the rest of the line.
        let Some(rel) = self.input.get(self.pos..) else {
            return Some(Err(LexError {
                message: "Internal lexer error: invalid UTF-8 boundary".to_string(),
            }));
        };
        let eol = rel.find('\n').unwrap_or(rel.len());
        let line = &rel[..eol];
        let trimmed = line.trim().to_string();
        let lower = trimmed.to_ascii_lowercase();
        for marker in ["<<fork>>", "[[fork]]"] {
            if lower.ends_with(marker) {
                let base = trimmed[..trimmed.len() - marker.len()].trim().to_string();
                self.pos += eol;
                self.pop_mode();
                return Some(Ok((start, Tok::Fork(base), self.pos)));
            }
        }
        for marker in ["<<join>>", "[[join]]"] {
            if lower.ends_with(marker) {
                let base = trimmed[..trimmed.len() - marker.len()].trim().to_string();
                self.pos += eol;
                self.pop_mode();
                return Some(Ok((start, Tok::Join(base), self.pos)));
            }
        }
        for marker in ["<<choice>>", "[[choice]]"] {
            if lower.ends_with(marker) {
                let base = trimmed[..trimmed.len() - marker.len()].trim().to_string();
                self.pos += eol;
                self.pop_mode();
                return Some(Ok((start, Tok::Choice(base), self.pos)));
            }
        }

        // Otherwise treat it as a composite state ID: read only the identifier token.
        let Some(id) = self.read_plain_id() else {
            return Some(Err(LexError {
                message: "Expected a state id".to_string(),
            }));
        };
        self.pop_mode();

        // Mermaid accepts `state <id>` with a `{ ... }` block that starts on the next line:
        //
        //   state Foo
        //   {
        //     ...
        //   }
        //
        // Treat the intervening whitespace/newlines as insignificant and advance to the `{` so the
        // parser sees `CompositState` followed immediately by a `Block`.
        let end = self.pos;
        let mut look = self.pos;
        while let Some(b) = self.input.as_bytes().get(look).copied() {
            if matches!(b, b' ' | b'\t' | b'\r') {
                look += 1;
                continue;
            }
            break;
        }
        if self.input.as_bytes().get(look) == Some(&b'\n') {
            let mut scan = look;
            while let Some(b) = self.input.as_bytes().get(scan).copied() {
                if matches!(b, b' ' | b'\t' | b'\r' | b'\n') {
                    scan += 1;
                    continue;
                }
                break;
            }
            if self.input.as_bytes().get(scan) == Some(&b'{') {
                self.pos = scan;
            }
        }

        Some(Ok((start, Tok::CompositState(id), end)))
    }

    fn lex_id(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        while let Some(b) = self.input.as_bytes().get(end).copied() {
            if b == b':' || b == b'\n' || b.is_ascii_whitespace() || b == b'-' || b == b'{' {
                break;
            }
            end += 1;
        }
        if end == start {
            return None;
        }
        self.pos = end;
        Some((start, Tok::Id(self.input[start..end].to_string()), self.pos))
    }

    fn lex_descr(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.peek()? != b':' {
            return None;
        }
        self.pos += 1;
        let body_start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'\n' || b == b';' || b == b':' {
                break;
            }
            self.pos += 1;
        }
        let body = self.input[body_start..self.pos].trim().to_string();
        Some((start, Tok::Descr(body), self.pos))
    }

    fn read_plain_id(&mut self) -> Option<String> {
        let start = self.pos;
        let mut end = self.pos;
        while let Some(b) = self.input.as_bytes().get(end).copied() {
            if b == b':' || b == b'\n' || b.is_ascii_whitespace() || b == b'-' || b == b'{' {
                break;
            }
            end += 1;
        }
        if end == start {
            return None;
        }
        self.pos = end;
        Some(self.input[start..end].to_string())
    }

    fn lex_styled_id(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;

        if self.starts_with("[*]:::") {
            self.pos += "[*]:::".len();
            let class_id = self.read_plain_id()?;
            return Some((
                start,
                Tok::StyledId(("[*]".to_string(), class_id)),
                self.pos,
            ));
        }

        // Look ahead: <id>:::<classId>
        let save = self.pos;
        let Some(id) = self.read_plain_id() else {
            self.pos = save;
            return None;
        };
        if !self.starts_with(":::") {
            self.pos = save;
            return None;
        }
        self.pos += 3;
        let Some(class_id) = self.read_plain_id() else {
            self.pos = save;
            return None;
        };
        Some((start, Tok::StyledId((id, class_id)), self.pos))
    }
}

impl Iterator for Lexer<'_> {
    type Item = std::result::Result<(usize, Tok, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.pending.pop_front() {
            return Some(Ok(item));
        }

        if self.pos >= self.input.len() {
            if self.emitted_eof_newline {
                return None;
            }
            self.emitted_eof_newline = true;
            return Some(Ok((self.pos, Tok::Newline, self.pos)));
        }
        self.skip_ws();
        if let Some(nl) = self.lex_newline() {
            return Some(Ok(nl));
        }
        if self.skip_comment() {
            return self.next();
        }

        if self.mode() == Mode::StateId {
            if let Some(tok) = self.lex_state_mode_token() {
                return Some(tok);
            }
        }

        if let Some(sd) = self.lex_sd_header() {
            return Some(Ok(sd));
        }

        if let Some(dir) = self.lex_direction() {
            return Some(Ok(dir));
        }

        if let Some(acc) = self.lex_accessibility() {
            return Some(acc);
        }

        if !matches!(self.mode(), Mode::State | Mode::StateId) {
            if let Some(tok) = self.lex_stmt_line() {
                return Some(tok);
            }
        }

        if self.mode() == Mode::State {
            if let Some(tok) = self.lex_state_mode_token() {
                return Some(tok);
            }
        }

        let start = self.pos;
        if self.starts_with("-->") {
            self.pos += 3;
            return Some(Ok((start, Tok::Arrow, self.pos)));
        }

        if self.mode() == Mode::Struct && self.starts_with("--") {
            self.pos += 2;
            return Some(Ok((start, Tok::Concurrent, self.pos)));
        }

        if self.starts_with("[*]") && !self.starts_with("[*]:::") {
            self.pos += 3;
            return Some(Ok((start, Tok::EdgeState, self.pos)));
        }

        if self.peek() == Some(b'{') {
            self.pos += 1;
            if self.mode() == Mode::State {
                self.pop_mode();
            }
            self.push_mode(Mode::Struct);
            return Some(Ok((start, Tok::StructStart, self.pos)));
        }

        if self.peek() == Some(b'}') {
            self.pos += 1;
            if self.mode() == Mode::Struct {
                self.pop_mode();
            }
            return Some(Ok((start, Tok::StructStop, self.pos)));
        }

        if let Some(tok) = self.lex_string_lit() {
            return Some(tok);
        }

        if let Some(tok) = self.lex_styled_id() {
            return Some(Ok(tok));
        }

        if let Some(tok) = self.lex_descr() {
            return Some(Ok(tok));
        }

        if let Some(tok) = self.lex_id() {
            return Some(Ok(tok));
        }

        let bad = self
            .input
            .get(self.pos..)
            .and_then(|s| s.chars().next())
            .unwrap_or('?');
        self.pos += bad.len_utf8().max(1);
        Some(Err(LexError {
            message: format!("Unexpected character '{bad}'"),
        }))
    }
}
