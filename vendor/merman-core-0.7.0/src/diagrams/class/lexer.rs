use std::collections::VecDeque;

use super::MERMAID_DOM_ID_PREFIX;

#[derive(Debug, Clone)]
pub(crate) enum Tok {
    Newline,

    ClassDiagram,

    Direction(String),

    ClassKw,
    NamespaceKw,

    Note,
    NoteFor,

    CssClass,
    StyleKw,
    ClassDefKw,
    ClickKw,
    LinkKw,
    CallbackKw,
    HrefKw,

    StructStart,
    StructStop,

    SquareStart,
    SquareStop,

    AnnotationStart,
    AnnotationStop,

    StyleSeparator,

    Ext,
    Dep,
    Comp,
    Agg,
    Lollipop,
    Line,
    DottedLine,

    Label(String),
    Str(String),
    Name(String),
    Member(String),
    RestOfLine(String),
    LinkTarget(String),
    CallbackName(String),
    CallbackArgs(String),

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
    AfterClass,
    ClassBody,
    LineNeedId,
    LineRest,
    ClickNeedId,
    ClickAfterId,
    ClickNeedCallbackName,
    ClickAfterCallbackName,
}

pub(super) struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    pending: VecDeque<(usize, Tok, usize)>,
    mode: Mode,
}

impl<'input> Lexer<'input> {
    pub(super) fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            pending: VecDeque::new(),
            mode: Mode::Default,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
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

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn starts_with_word(&self, s: &str) -> bool {
        if !self.starts_with(s) {
            return false;
        }
        let after = self.pos + s.len();
        if after >= self.input.len() {
            return true;
        }
        let b = self.input.as_bytes()[after];
        b.is_ascii_whitespace() || matches!(b, b'{' | b'}' | b'[' | b']' | b'"' | b'`' | b':')
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
        if self.mode == Mode::AfterClass {
            self.mode = Mode::Default;
        }
        Some((start, Tok::Newline, self.pos))
    }

    fn lex_comment(&mut self) -> bool {
        if self.starts_with("%%") {
            let _ = self.read_to_newline();
            return true;
        }
        false
    }

    fn lex_acc_title(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if !self.starts_with("accTitle") {
            return None;
        }
        let after = self.pos + "accTitle".len();
        let rest = &self.input[after..];
        let colon = rest.find(':')?;
        self.pos = after + colon + 1;
        let value = self.read_to_newline();
        Some((start, Tok::AccTitle(value.trim().to_string()), self.pos))
    }

    fn lex_acc_descr(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with("accDescr") {
            return None;
        }
        let after = self.pos + "accDescr".len();
        let rest = &self.input[after..];
        let rest_trim = rest.trim_start();
        if rest_trim.starts_with('{') {
            let consumed_ws = rest.len() - rest_trim.len();
            self.pos = after + consumed_ws + 1;
            let Some(end_rel) = self.input[self.pos..].find('}') else {
                return Some(Err(LexError {
                    message: "Unterminated accDescr block; missing '}'".to_string(),
                }));
            };
            let body = self.input[self.pos..self.pos + end_rel].to_string();
            self.pos = self.pos + end_rel + 1;
            return Some(Ok((
                start,
                Tok::AccDescrMultiline(body.trim().to_string()),
                self.pos,
            )));
        }
        let colon = rest.find(':')?;
        self.pos = after + colon + 1;
        let value = self.read_to_newline();
        Some(Ok((
            start,
            Tok::AccDescr(value.trim().to_string()),
            self.pos,
        )))
    }

    fn lex_keyword(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.starts_with_word("classDiagram-v2") {
            self.pos += "classDiagram-v2".len();
            return Some((start, Tok::ClassDiagram, self.pos));
        }
        if self.starts_with_word("classDiagram") {
            self.pos += "classDiagram".len();
            return Some((start, Tok::ClassDiagram, self.pos));
        }

        if self.starts_with_word("direction") {
            let after = self.pos + "direction".len();
            self.pos = after;
            self.skip_ws();
            let dir = if self.input[self.pos..].starts_with("TB") {
                self.pos += 2;
                "TB"
            } else if self.input[self.pos..].starts_with("BT") {
                self.pos += 2;
                "BT"
            } else if self.input[self.pos..].starts_with("LR") {
                self.pos += 2;
                "LR"
            } else if self.input[self.pos..].starts_with("RL") {
                self.pos += 2;
                "RL"
            } else {
                return None;
            };
            let _ = self.read_to_newline();
            return Some((start, Tok::Direction(dir.to_string()), self.pos));
        }

        if self.starts_with_word("namespace") {
            self.pos += "namespace".len();
            return Some((start, Tok::NamespaceKw, self.pos));
        }
        if self.starts_with_word("class") {
            self.pos += "class".len();
            self.mode = Mode::AfterClass;
            return Some((start, Tok::ClassKw, self.pos));
        }

        if self.starts_with("note for") {
            self.pos += "note for".len();
            return Some((start, Tok::NoteFor, self.pos));
        }
        if self.starts_with_word("note") {
            self.pos += "note".len();
            return Some((start, Tok::Note, self.pos));
        }

        if self.starts_with_word("cssClass") {
            self.pos += "cssClass".len();
            return Some((start, Tok::CssClass, self.pos));
        }
        if self.starts_with_word("style") {
            self.pos += "style".len();
            self.mode = Mode::LineNeedId;
            return Some((start, Tok::StyleKw, self.pos));
        }
        if self.starts_with_word("classDef") {
            self.pos += "classDef".len();
            self.mode = Mode::LineNeedId;
            return Some((start, Tok::ClassDefKw, self.pos));
        }
        if self.starts_with_word("click") {
            self.pos += "click".len();
            self.mode = Mode::ClickNeedId;
            return Some((start, Tok::ClickKw, self.pos));
        }
        if self.starts_with_word("link") {
            self.pos += "link".len();
            return Some((start, Tok::LinkKw, self.pos));
        }
        if self.starts_with_word("callback") {
            self.pos += "callback".len();
            return Some((start, Tok::CallbackKw, self.pos));
        }
        if self.starts_with_word("href") {
            self.pos += "href".len();
            return Some((start, Tok::HrefKw, self.pos));
        }

        None
    }

    fn lex_link_target(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        for t in ["_self", "_blank", "_parent", "_top"] {
            if self.starts_with_word(t) {
                self.pos += t.len();
                return Some((start, Tok::LinkTarget(t.to_string()), self.pos));
            }
        }
        None
    }

    fn lex_click_call(&mut self) -> bool {
        if self.mode != Mode::ClickAfterId {
            return false;
        }
        if self.starts_with_word("call") {
            self.pos += "call".len();
            self.mode = Mode::ClickNeedCallbackName;
            return true;
        }
        false
    }

    fn lex_callback_name(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode != Mode::ClickNeedCallbackName {
            return None;
        }
        let start = self.pos;
        self.skip_ws();
        let bytes = self.input.as_bytes();
        let mut end = self.pos;
        while end < self.input.len() {
            let b = bytes[end];
            if b.is_ascii_whitespace() || b == b'\n' || b == b'(' {
                break;
            }
            end += 1;
        }
        if end == self.pos {
            return None;
        }
        let s = self.input[self.pos..end].to_string();
        self.pos = end;
        self.mode = Mode::ClickAfterCallbackName;
        Some((start, Tok::CallbackName(s), self.pos))
    }

    fn lex_callback_args(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        if self.mode != Mode::ClickAfterCallbackName {
            return None;
        }
        let start = self.pos;
        if self.peek()? != b'(' {
            return None;
        }
        self.pos += 1;
        let Some(end_rel) = self.input[self.pos..].find(')') else {
            return Some(Err(LexError {
                message: "Unterminated callback arguments; missing ')'".to_string(),
            }));
        };
        let args = self.input[self.pos..self.pos + end_rel].trim().to_string();
        self.pos = self.pos + end_rel + 1;
        self.mode = Mode::ClickAfterId;
        Some(Ok((start, Tok::CallbackArgs(args), self.pos)))
    }

    fn lex_rest_of_line(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode != Mode::LineRest {
            return None;
        }
        let start = self.pos;
        let s = self.read_to_newline();
        self.mode = Mode::Default;
        Some((start, Tok::RestOfLine(s.trim().to_string()), self.pos))
    }

    fn lex_punct(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        match self.peek()? {
            b'{' => {
                self.pos += 1;
                if self.mode == Mode::AfterClass {
                    self.mode = Mode::ClassBody;
                }
                Some((start, Tok::StructStart, self.pos))
            }
            b'}' => {
                self.pos += 1;
                if self.mode == Mode::ClassBody {
                    self.mode = Mode::Default;
                }
                Some((start, Tok::StructStop, self.pos))
            }
            b'[' => {
                self.pos += 1;
                Some((start, Tok::SquareStart, self.pos))
            }
            b']' => {
                self.pos += 1;
                Some((start, Tok::SquareStop, self.pos))
            }
            b'<' => {
                if self.input[self.pos..].starts_with("<<") {
                    self.pos += 2;
                    return Some((start, Tok::AnnotationStart, self.pos));
                }
                if self.input[self.pos..].starts_with("<|") {
                    self.pos += 2;
                    return Some((start, Tok::Ext, self.pos));
                }
                self.pos += 1;
                Some((start, Tok::Dep, self.pos))
            }
            b'>' => {
                if self.input[self.pos..].starts_with(">>") {
                    self.pos += 2;
                    return Some((start, Tok::AnnotationStop, self.pos));
                }
                self.pos += 1;
                Some((start, Tok::Dep, self.pos))
            }
            b'|' => {
                if self.input[self.pos..].starts_with("|>") {
                    self.pos += 2;
                    return Some((start, Tok::Ext, self.pos));
                }
                None
            }
            b'(' => {
                if self.input[self.pos..].starts_with("()") {
                    self.pos += 2;
                    return Some((start, Tok::Lollipop, self.pos));
                }
                None
            }
            b'*' => {
                self.pos += 1;
                Some((start, Tok::Comp, self.pos))
            }
            b'o' => {
                let next = self.input.as_bytes().get(self.pos + 1).copied();
                if matches!(next, Some(b'-' | b'.' | b' ' | b'\t') | None) {
                    self.pos += 1;
                    Some((start, Tok::Agg, self.pos))
                } else {
                    None
                }
            }
            b'.' => {
                if self.input[self.pos..].starts_with("..") {
                    self.pos += 2;
                    Some((start, Tok::DottedLine, self.pos))
                } else {
                    None
                }
            }
            b'-' => {
                if self.input[self.pos..].starts_with("--") {
                    self.pos += 2;
                    Some((start, Tok::Line, self.pos))
                } else {
                    None
                }
            }
            b':' => {
                if self.input[self.pos..].starts_with(":::") {
                    self.pos += 3;
                    Some((start, Tok::StyleSeparator, self.pos))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lex_label(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.peek()? != b':' {
            return None;
        }
        if self.input[self.pos..].starts_with(":::") {
            return None;
        }
        self.pos += 1;
        let s = self.read_to_newline();
        Some((start, Tok::Label(format!(":{}", s)), self.pos))
    }

    fn lex_str(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if self.peek()? != b'"' {
            return None;
        }
        self.pos += 1;
        let Some(rel_end) = self.input[self.pos..].find('"') else {
            return Some(Err(LexError {
                message: "Unterminated string literal; missing '\"'".to_string(),
            }));
        };
        let s = self.input[self.pos..self.pos + rel_end].to_string();
        self.pos = self.pos + rel_end + 1;
        Some(Ok((start, Tok::Str(s), self.pos)))
    }

    fn lex_name(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode == Mode::ClassBody {
            return None;
        }
        let start = self.pos;
        if self.peek()? == b'`' {
            self.pos += 1;
            let Some(rel_end) = self.input[self.pos..].find('`') else {
                let s = self.input[self.pos..].to_string();
                self.pos = self.input.len();
                return Some((start, Tok::Name(s), self.pos));
            };
            let s = self.input[self.pos..self.pos + rel_end].to_string();
            self.pos = self.pos + rel_end + 1;
            if self.mode == Mode::LineNeedId {
                self.mode = Mode::LineRest;
            }
            if self.mode == Mode::ClickNeedId {
                self.mode = Mode::ClickAfterId;
            }
            let s = if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                format!("{MERMAID_DOM_ID_PREFIX}{s}")
            } else {
                s
            };
            return Some((start, Tok::Name(s), self.pos));
        }

        let bytes = self.input.as_bytes();
        let mut end = self.pos;
        while end < self.input.len() {
            let b = bytes[end];
            if b.is_ascii_whitespace()
                || b == b'\n'
                || b == b'{'
                || b == b'}'
                || b == b'['
                || b == b']'
                || b == b'"'
                || b == b','
            {
                break;
            }
            if b == b':' {
                break;
            }
            if b == b'<' || b == b'>' {
                break;
            }
            if b == b'.' && end + 1 < bytes.len() && bytes[end + 1] == b'.' {
                break;
            }
            if b == b'-' && end + 1 < bytes.len() && bytes[end + 1] == b'-' {
                break;
            }
            end += 1;
        }
        if end == start {
            return None;
        }
        let mut s = self.input[start..end].to_string();
        if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            s = format!("{MERMAID_DOM_ID_PREFIX}{s}");
        }
        self.pos = end;
        if self.mode == Mode::LineNeedId {
            self.mode = Mode::LineRest;
        }
        if self.mode == Mode::ClickNeedId {
            self.mode = Mode::ClickAfterId;
        }
        Some((start, Tok::Name(s), self.pos))
    }

    fn lex_member(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        if self.mode != Mode::ClassBody {
            return None;
        }
        self.skip_ws();
        if self.pos >= self.input.len() {
            return Some(Err(LexError {
                message: "EOF inside class body".to_string(),
            }));
        }
        if self.peek() == Some(b'}') {
            return None;
        }
        if self.peek() == Some(b'{') {
            return Some(Err(LexError {
                message: "Unexpected '{' inside class body".to_string(),
            }));
        }
        // Newlines inside a class body are ignored by Mermaid's lexer.
        while self.peek() == Some(b'\n') {
            self.pos += 1;
            self.skip_ws();
        }
        let start = self.pos;
        let s = self.read_to_newline();
        Some(Ok((start, Tok::Member(s.trim_end().to_string()), self.pos)))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = std::result::Result<(usize, Tok, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.pending.pop_front() {
            return Some(Ok(tok));
        }

        loop {
            self.skip_ws();
            if self.pos >= self.input.len() {
                if self.mode == Mode::ClassBody {
                    return Some(Err(LexError {
                        message: "EOF inside class body".to_string(),
                    }));
                }
                return None;
            }

            if self.lex_comment() {
                continue;
            }

            if let Some(tok) = self.lex_rest_of_line() {
                return Some(Ok(tok));
            }

            if self.lex_click_call() {
                continue;
            }

            if self.mode == Mode::ClassBody && self.peek() == Some(b'\n') {
                self.pos += 1;
                continue;
            }

            if let Some(tok) = self.lex_callback_name() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_link_target() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_member() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_newline() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_acc_title() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_acc_descr() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_keyword() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_callback_args() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_punct() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_label() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_str() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_name() {
                return Some(Ok(tok));
            }

            let start = self.pos;
            let _ = self.bump();
            return Some(Err(LexError {
                message: format!("Unexpected character at {start}"),
            }));
        }
    }
}
