use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) enum Tok {
    Newline,

    SequenceDiagram,
    Participant,
    ActorKw,
    Create,
    Destroy,
    As,

    Box,
    End,

    Loop,
    Rect,
    Opt,
    Alt,
    Else,
    Par,
    ParOver,
    And,
    Critical,
    Option,
    Break,

    Note,
    LeftOf,
    RightOf,
    Over,

    Links,
    Link,
    Properties,
    Details,

    Autonumber,
    Off,

    Activate,
    Deactivate,

    Comma,
    Plus,
    Minus,
    Central,

    Num(f64),
    Actor(String),
    Text(String),
    RestOfLine(String),
    SignalType(i32),
    Config(String),

    Title(String),
    CompatTitle(String),
    AccTitle(String),
    AccDescr(String),
    AccDescrMultiline(String),
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub(crate) struct LexError {
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Default,
    Line,
    AccDescrMultiline,
}

pub(super) struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    pending: VecDeque<(usize, Tok, usize)>,
    mode: Mode,
    // Keywords are also legal participant ids in Mermaid. These flags let parser context win over
    // keyword lexing for positions that must be actor ids.
    force_actor_id: bool,
    after_signal_type: bool,
}

impl<'input> Lexer<'input> {
    pub(super) fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            pending: VecDeque::new(),
            mode: Mode::Default,
            force_actor_id: false,
            after_signal_type: false,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn peek2(&self) -> Option<[u8; 2]> {
        if self.pos + 1 >= self.input.len() {
            return None;
        }
        Some([
            self.input.as_bytes()[self.pos],
            self.input.as_bytes()[self.pos + 1],
        ])
    }

    fn bump(&mut self) -> Option<u8> {
        if self.pos >= self.input.len() {
            return None;
        }
        let bytes = self.input.as_bytes();
        let b = bytes[self.pos];

        // Keep `self.pos` on a UTF-8 char boundary. Mermaid input can contain arbitrary Unicode
        // (including `encodeEntities(...)` placeholders), and this lexer is otherwise byte-based.
        if b.is_ascii() {
            self.pos += 1;
        } else {
            // If we're already in the middle of a codepoint (continuation byte), resync by
            // skipping continuation bytes.
            if (b & 0b1100_0000) == 0b1000_0000 {
                self.pos += 1;
                while self.pos < bytes.len() && (bytes[self.pos] & 0b1100_0000) == 0b1000_0000 {
                    self.pos += 1;
                }
            } else {
                let len = if (b & 0b1110_0000) == 0b1100_0000 {
                    2
                } else if (b & 0b1111_0000) == 0b1110_0000 {
                    3
                } else if (b & 0b1111_1000) == 0b1111_0000 {
                    4
                } else {
                    1
                };
                self.pos = (self.pos + len).min(bytes.len());
                while self.pos < bytes.len() && (bytes[self.pos] & 0b1100_0000) == 0b1000_0000 {
                    self.pos += 1;
                }
            }
        }
        Some(b)
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

    fn starts_with_ci(&self, kw: &str) -> bool {
        let rest = self.input.as_bytes().get(self.pos..).unwrap_or_default();
        let kwb = kw.as_bytes();
        if rest.len() < kwb.len() {
            return false;
        }
        for i in 0..kwb.len() {
            let a = rest[i];
            let b = kwb[i];
            if !a.eq_ignore_ascii_case(&b) {
                return false;
            }
        }
        true
    }

    fn starts_with_ci_word(&self, kw: &str) -> bool {
        if !self.starts_with_ci(kw) {
            return false;
        }
        let after = self.pos + kw.len();
        if after >= self.input.len() {
            return true;
        }
        let b = self.input.as_bytes()[after];
        !b.is_ascii_alphanumeric() && b != b'_'
    }

    fn lex_newline(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        match self.peek()? {
            b'\n' | b';' => {
                self.pos += 1;
                self.mode = Mode::Default;
                self.force_actor_id = false;
                self.after_signal_type = false;
                Some((start, Tok::Newline, self.pos))
            }
            _ => None,
        }
    }

    fn lex_comment(&mut self) -> bool {
        let Some(b) = self.peek() else {
            return false;
        };
        if b == b'#' {
            while let Some(b2) = self.peek() {
                if b2 == b'\n' {
                    break;
                }
                self.pos += 1;
            }
            return true;
        }
        let Some([b'%', b'%']) = self.peek2() else {
            return false;
        };
        // Mermaid directives are removed earlier in preprocess, so `%%` is always a comment here.
        while let Some(b2) = self.peek() {
            if b2 == b'\n' {
                break;
            }
            self.pos += 1;
        }
        true
    }

    fn lex_multiline_acc_descr(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode != Mode::AccDescrMultiline {
            return None;
        }
        let start = self.pos;
        let Some(rel_end) = self.input[self.pos..].find('}') else {
            let s = self.input[self.pos..].to_string();
            self.pos = self.input.len();
            return Some((start, Tok::AccDescrMultiline(s), self.pos));
        };
        let end = self.pos + rel_end;
        let s = self.input[self.pos..end].to_string();
        self.pos = end + 1;
        self.mode = Mode::Default;
        Some((start, Tok::AccDescrMultiline(s), self.pos))
    }

    fn lex_keyword_lines(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;

        if self.starts_with_ci_word("title:") {
            self.pos += "title:".len();
            self.skip_ws();
            let s = self.read_to_line_end();
            return Some((start, Tok::CompatTitle(s.trim().to_string()), self.pos));
        }

        if self.starts_with_ci_word("title") {
            let after = self.pos + "title".len();
            if after < self.input.len() && self.input.as_bytes()[after].is_ascii_whitespace() {
                self.pos = after;
                self.skip_ws();
                let s = self.read_to_line_end();
                return Some((start, Tok::Title(s.trim().to_string()), self.pos));
            }
        }

        if self.starts_with_ci_word("accTitle") {
            let after = self.pos + "accTitle".len();
            let rest = &self.input[after..];
            let colon_pos = rest.find(':')?;
            if rest[..colon_pos].chars().any(|c| c == '\n' || c == ';') {
                return None;
            }
            self.pos = after + colon_pos + 1;
            self.skip_ws();
            let s = self.read_to_line_end();
            return Some((start, Tok::AccTitle(s.trim().to_string()), self.pos));
        }

        if self.starts_with_ci_word("accDescr") {
            let after = self.pos + "accDescr".len();
            let rest = &self.input[after..];
            let non_ws = rest.find(|c: char| !c.is_whitespace())?;
            match rest[non_ws..].chars().next() {
                Some(':') => {
                    self.pos = after + non_ws + 1;
                    self.skip_ws();
                    let s = self.read_to_line_end();
                    return Some((start, Tok::AccDescr(s.trim().to_string()), self.pos));
                }
                Some('{') => {
                    self.pos = after + non_ws + 1;
                    self.mode = Mode::AccDescrMultiline;
                    return self.lex_multiline_acc_descr();
                }
                _ => {}
            }
        }

        None
    }

    fn read_to_line_end(&mut self) -> String {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'\n' || b == b';' {
                break;
            }
            if b == b'#' {
                break;
            }
            if let Some([b'%', b'%']) = self.peek2() {
                if b == b'%' {
                    break;
                }
            }
            self.pos += 1;
        }
        self.input[start..self.pos].to_string()
    }

    fn lex_word_keywords(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.starts_with_ci_word("sequenceDiagram") {
            self.pos += "sequenceDiagram".len();
            return Some((start, Tok::SequenceDiagram, self.pos));
        }
        if self.starts_with_ci_word("participant") {
            self.pos += "participant".len();
            return Some((start, Tok::Participant, self.pos));
        }
        if self.starts_with_ci_word("actor") {
            self.pos += "actor".len();
            return Some((start, Tok::ActorKw, self.pos));
        }
        if self.starts_with_ci_word("box") {
            self.pos += "box".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Box, self.pos));
        }
        if self.starts_with_ci_word("end") {
            self.pos += "end".len();
            return Some((start, Tok::End, self.pos));
        }
        if self.starts_with_ci_word("loop") {
            self.pos += "loop".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Loop, self.pos));
        }
        if self.starts_with_ci_word("rect") {
            self.pos += "rect".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Rect, self.pos));
        }
        if self.starts_with_ci_word("opt") {
            self.pos += "opt".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Opt, self.pos));
        }
        if self.starts_with_ci_word("alt") {
            self.pos += "alt".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Alt, self.pos));
        }
        if self.starts_with_ci_word("else") {
            self.pos += "else".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Else, self.pos));
        }
        if self.starts_with_ci_word("par_over") {
            self.pos += "par_over".len();
            self.mode = Mode::Line;
            return Some((start, Tok::ParOver, self.pos));
        }
        if self.starts_with_ci_word("par") {
            self.pos += "par".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Par, self.pos));
        }
        if self.starts_with_ci_word("and") {
            self.pos += "and".len();
            self.mode = Mode::Line;
            return Some((start, Tok::And, self.pos));
        }
        if self.starts_with_ci_word("critical") {
            self.pos += "critical".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Critical, self.pos));
        }
        if self.starts_with_ci_word("option") {
            self.pos += "option".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Option, self.pos));
        }
        if self.starts_with_ci_word("break") {
            self.pos += "break".len();
            self.mode = Mode::Line;
            return Some((start, Tok::Break, self.pos));
        }
        if self.starts_with_ci_word("create") {
            self.pos += "create".len();
            return Some((start, Tok::Create, self.pos));
        }
        if self.starts_with_ci_word("destroy") {
            self.pos += "destroy".len();
            return Some((start, Tok::Destroy, self.pos));
        }
        if self.starts_with_ci_word("as") {
            self.pos += "as".len();
            self.mode = Mode::Line;
            return Some((start, Tok::As, self.pos));
        }
        if self.starts_with_ci_word("note") {
            self.pos += "note".len();
            return Some((start, Tok::Note, self.pos));
        }

        if self.starts_with_ci_word("links") {
            self.pos += "links".len();
            return Some((start, Tok::Links, self.pos));
        }
        if self.starts_with_ci_word("link") {
            self.pos += "link".len();
            return Some((start, Tok::Link, self.pos));
        }
        if self.starts_with_ci_word("properties") {
            self.pos += "properties".len();
            return Some((start, Tok::Properties, self.pos));
        }
        if self.starts_with_ci_word("details") {
            self.pos += "details".len();
            return Some((start, Tok::Details, self.pos));
        }

        if self.starts_with_ci("left of") {
            let after = self.pos + "left of".len();
            if after >= self.input.len() || self.input.as_bytes()[after].is_ascii_whitespace() {
                self.pos = after;
                return Some((start, Tok::LeftOf, self.pos));
            }
        }
        if self.starts_with_ci("right of") {
            let after = self.pos + "right of".len();
            if after >= self.input.len() || self.input.as_bytes()[after].is_ascii_whitespace() {
                self.pos = after;
                return Some((start, Tok::RightOf, self.pos));
            }
        }
        if self.starts_with_ci_word("over") {
            self.pos += "over".len();
            return Some((start, Tok::Over, self.pos));
        }

        if self.starts_with_ci_word("autonumber") {
            self.pos += "autonumber".len();
            return Some((start, Tok::Autonumber, self.pos));
        }
        if self.starts_with_ci_word("off") {
            self.pos += "off".len();
            return Some((start, Tok::Off, self.pos));
        }
        if self.starts_with_ci_word("activate") {
            self.pos += "activate".len();
            return Some((start, Tok::Activate, self.pos));
        }
        if self.starts_with_ci_word("deactivate") {
            self.pos += "deactivate".len();
            return Some((start, Tok::Deactivate, self.pos));
        }

        None
    }

    fn lex_punct(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        match self.peek()? {
            b',' => {
                self.pos += 1;
                Some((start, Tok::Comma, self.pos))
            }
            b'+' => {
                self.pos += 1;
                Some((start, Tok::Plus, self.pos))
            }
            _ => None,
        }
    }

    fn lex_central_marker(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.input[self.pos..].starts_with("()") {
            self.pos += 2;
            return Some((start, Tok::Central, self.pos));
        }
        None
    }

    fn lex_signal_type(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let rest = &self.input[self.pos..];

        let (len, ty) = if rest.starts_with("<<-->>") {
            (6, 34)
        } else if rest.starts_with("<<->>") {
            (5, 33)
        } else if rest.starts_with("-->>") {
            (4, 1)
        } else if rest.starts_with("->>") {
            (3, 0)
        } else if rest.starts_with("-->") {
            (3, 6)
        } else if rest.starts_with("->") {
            (2, 5)
        } else if rest.starts_with("--x") {
            (3, 4)
        } else if rest.starts_with("-x") {
            (2, 3)
        } else if rest.starts_with("--)") {
            (3, 25)
        } else if rest.starts_with("-)") {
            (2, 24)
        } else {
            return None;
        };

        self.pos += len;
        Some((start, Tok::SignalType(ty), self.pos))
    }

    fn lex_minus(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.peek()? != b'-' {
            return None;
        }
        self.pos += 1;
        Some((start, Tok::Minus, self.pos))
    }

    fn lex_num(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        let mut saw_digit = false;
        while let Some(b) = self.input.as_bytes().get(end) {
            if b.is_ascii_digit() {
                end += 1;
                saw_digit = true;
                continue;
            }
            break;
        }

        if self.input.as_bytes().get(end) == Some(&b'.') {
            let decimal_start = end + 1;
            let mut decimal_end = decimal_start;
            while let Some(b) = self.input.as_bytes().get(decimal_end) {
                if b.is_ascii_digit() {
                    decimal_end += 1;
                    continue;
                }
                break;
            }
            let decimal_places = decimal_end - decimal_start;
            if decimal_places == 0 || decimal_places > 2 {
                return None;
            }
            end = decimal_end;
        } else if !saw_digit {
            return None;
        }

        let n: f64 = self.input[start..end].parse().ok()?;
        if !n.is_finite() {
            return None;
        }
        self.pos = end;
        Some((start, Tok::Num(n), self.pos))
    }

    fn lex_rest_of_line(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode != Mode::Line {
            return None;
        }
        let start = self.pos;
        let s = self.read_to_line_end();
        self.mode = Mode::Default;
        Some((start, Tok::RestOfLine(s.trim().to_string()), self.pos))
    }

    fn lex_config(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.input[self.pos..].starts_with("@{") {
            return None;
        }
        if start > 0 && self.input.as_bytes()[start - 1].is_ascii_whitespace() {
            return Some(Err(LexError {
                message: "Config objects must be attached to the actor id without whitespace"
                    .to_string(),
            }));
        }
        self.pos += 2;
        let Some(rel_end) = self.input[self.pos..].find('}') else {
            return Some(Err(LexError {
                message: "Unterminated config object; missing '}'".to_string(),
            }));
        };
        let end = self.pos + rel_end;
        let s = self.input[self.pos..end].trim().to_string();
        self.pos = end + 1;
        Some(Ok((start, Tok::Config(s), self.pos)))
    }

    fn lex_text(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.peek()? != b':' {
            return None;
        }
        self.pos += 1;
        let s = self.read_to_line_end();
        Some((start, Tok::Text(s.trim().to_string()), self.pos))
    }

    fn lex_actor(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        let bytes = self.input.as_bytes();

        while end < self.input.len() {
            let b = bytes[end];
            if b.is_ascii_whitespace()
                || b == b'\n'
                || b == b';'
                || b == b','
                || b == b':'
                || b == b'+'
            {
                break;
            }
            if b == b'@' && end + 1 < bytes.len() && bytes[end + 1] == b'{' {
                break;
            }
            if b == b'-' {
                let next = bytes.get(end + 1).copied();
                if matches!(next, Some(b'-' | b'>' | b'x' | b')')) {
                    break;
                }
            }
            if b == b'<' {
                break;
            }
            end += 1;
        }

        if end == start {
            return None;
        }
        let s = self.input[start..end].trim().to_string();
        self.pos = end;
        Some((start, Tok::Actor(s), self.pos))
    }

    fn lex_forced_actor_id(&mut self) -> Option<(usize, Tok, usize)> {
        if let Some(tok) = self.lex_num() {
            return Some(tok);
        }
        self.lex_actor()
    }

    fn lex_actor_before_signal(&mut self) -> Option<(usize, Tok, usize)> {
        let saved = self.pos;
        let tok = self.lex_actor()?;
        let after_actor = self.pos;

        if let Tok::Actor(actor) = &tok.1 {
            if let Some(stripped) = actor.strip_suffix("()") {
                let mut pos = after_actor;
                while let Some(b) = self.input.as_bytes().get(pos) {
                    if *b == b' ' || *b == b'\t' || *b == b'\r' {
                        pos += 1;
                        continue;
                    }
                    break;
                }
                if self.peek_signal_type_at(pos) {
                    self.pending
                        .push_back((after_actor - 2, Tok::Central, after_actor));
                    return Some((tok.0, Tok::Actor(stripped.to_string()), after_actor - 2));
                }
            }
        }

        while let Some(b) = self.input.as_bytes().get(self.pos) {
            if *b == b' ' || *b == b'\t' || *b == b'\r' {
                self.pos += 1;
                continue;
            }
            break;
        }

        let has_signal = if self.input[self.pos..].starts_with("()") {
            let mut pos = self.pos + 2;
            while let Some(b) = self.input.as_bytes().get(pos) {
                if *b == b' ' || *b == b'\t' || *b == b'\r' {
                    pos += 1;
                    continue;
                }
                break;
            }
            self.peek_signal_type_at(pos)
        } else {
            self.peek_signal_type_at_current()
        };
        self.pos = after_actor;
        if has_signal {
            Some(tok)
        } else {
            self.pos = saved;
            None
        }
    }

    fn peek_signal_type_at_current(&self) -> bool {
        self.peek_signal_type_at(self.pos)
    }

    fn peek_signal_type_at(&self, pos: usize) -> bool {
        let rest = &self.input[pos..];
        rest.starts_with("<<-->>")
            || rest.starts_with("<<->>")
            || rest.starts_with("-->>")
            || rest.starts_with("->>")
            || rest.starts_with("-->")
            || rest.starts_with("->")
            || rest.starts_with("--x")
            || rest.starts_with("-x")
            || rest.starts_with("--)")
            || rest.starts_with("-)")
    }

    fn note_emitted_token(&mut self, tok: &Tok) {
        self.after_signal_type = matches!(tok, Tok::SignalType(_));
        self.force_actor_id = matches!(
            tok,
            Tok::Participant
                | Tok::ActorKw
                | Tok::Destroy
                | Tok::Links
                | Tok::Link
                | Tok::Properties
                | Tok::Details
                | Tok::Activate
                | Tok::Deactivate
                | Tok::LeftOf
                | Tok::RightOf
                | Tok::Over
                | Tok::Plus
                | Tok::Minus
                | Tok::Comma
                | Tok::Central
        );
    }

    fn emit(
        &mut self,
        tok: (usize, Tok, usize),
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        self.note_emitted_token(&tok.1);
        Some(Ok(tok))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = std::result::Result<(usize, Tok, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.pending.pop_front() {
            self.note_emitted_token(&tok.1);
            return Some(Ok(tok));
        }

        loop {
            let start = self.pos;
            self.skip_ws();

            if self.pos >= self.input.len() {
                return None;
            }

            if self.lex_comment() {
                continue;
            }

            if let Some(tok) = self.lex_multiline_acc_descr() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_rest_of_line() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_newline() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_keyword_lines() {
                return self.emit(tok);
            }

            if self.force_actor_id {
                if let Some(tok) = self.lex_forced_actor_id() {
                    return self.emit(tok);
                }
            }

            if self.after_signal_type {
                if let Some(tok) = self.lex_central_marker() {
                    return self.emit(tok);
                }
                if let Some(tok) = self.lex_punct() {
                    return self.emit(tok);
                }
                if let Some(tok) = self.lex_minus() {
                    return self.emit(tok);
                }
                if let Some(tok) = self.lex_forced_actor_id() {
                    return self.emit(tok);
                }
            }

            if let Some(tok) = self.lex_central_marker() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_actor_before_signal() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_word_keywords() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_signal_type() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_config() {
                if let Ok((_, ref tok, _)) = tok {
                    self.note_emitted_token(tok);
                } else {
                    self.force_actor_id = false;
                }
                return Some(tok);
            }

            if let Some(tok) = self.lex_text() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_num() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_punct() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_central_marker() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_minus() {
                return self.emit(tok);
            }

            if let Some(tok) = self.lex_actor() {
                return self.emit(tok);
            }

            let _ = self.bump();
            return Some(Err(LexError {
                message: format!("Unexpected character at {start}"),
            }));
        }
    }
}
