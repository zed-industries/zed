use super::{
    LabeledText, LexError, LinkToken, NodeLabelToken, SubgraphHeader, TitleKind, Tok,
    destruct_end_link, destruct_start_link, lex, parse_edge_label_text,
};
use std::collections::VecDeque;

pub(super) struct Lexer<'input> {
    pub(super) input: &'input str,
    pub(super) pos: usize,
    pub(super) pending: VecDeque<(usize, Tok, usize)>,
    pub(super) allow_header_direction: bool,
}

impl<'input> Lexer<'input> {
    pub(super) fn normalize_direction_token(dir: &str) -> &str {
        if dir == "TD" { "TB" } else { dir }
    }

    pub(super) fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            pending: VecDeque::new(),
            allow_header_direction: false,
        }
    }

    pub(super) fn bump(&mut self) -> Option<u8> {
        if self.pos >= self.input.len() {
            return None;
        }
        let b = self.input.as_bytes()[self.pos];
        self.pos += 1;
        Some(b)
    }

    pub(super) fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    pub(super) fn peek2(&self) -> Option<[u8; 2]> {
        if self.pos + 1 >= self.input.len() {
            return None;
        }
        Some([
            self.input.as_bytes()[self.pos],
            self.input.as_bytes()[self.pos + 1],
        ])
    }

    pub(super) fn starts_with_kw(&self, kw: &str) -> bool {
        let rest = &self.input[self.pos..];
        if !rest.starts_with(kw) {
            return false;
        }
        let after = self.pos + kw.len();
        if after >= self.input.len() {
            return true;
        }
        let b = self.input.as_bytes()[after];
        !b.is_ascii_alphanumeric() && b != b'_' && b != b'-'
    }

    pub(super) fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.pos += 1;
                continue;
            }
            break;
        }
    }

    pub(super) fn lex_sep(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        match self.peek()? {
            b'\n' => {
                let bytes = self.input.as_bytes();
                let mut look = self.pos + 1;
                while look < bytes.len() {
                    match bytes[look] {
                        b' ' | b'\t' | b'\r' => look += 1,
                        _ => break,
                    }
                }
                if look < bytes.len() {
                    let is_linkish = match bytes[look] {
                        b'~' => {
                            look + 2 < bytes.len()
                                && bytes[look + 1] == b'~'
                                && bytes[look + 2] == b'~'
                        }
                        b'=' => look + 1 < bytes.len() && bytes[look + 1] == b'=',
                        b'-' => {
                            look + 1 < bytes.len()
                                && (bytes[look + 1] == b'-' || bytes[look + 1] == b'.')
                        }
                        b'o' | b'x' | b'<' => {
                            look + 2 < bytes.len()
                                && ((bytes[look + 1] == b'-'
                                    && (bytes[look + 2] == b'-' || bytes[look + 2] == b'.'))
                                    || (bytes[look + 1] == b'=' && bytes[look + 2] == b'='))
                        }
                        _ => false,
                    };
                    if is_linkish {
                        self.pos = look;
                        return None;
                    }
                }

                self.pos += 1;
                Some((start, Tok::Sep, self.pos))
            }
            b';' => {
                self.pos += 1;
                Some((start, Tok::Sep, self.pos))
            }
            _ => None,
        }
    }

    pub(super) fn lex_comment(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let Some([b'%', b'%']) = self.peek2() else {
            return None;
        };
        // Consume until newline or EOF. If newline exists, emit Sep to keep statement boundaries.
        self.pos += 2;
        while let Some(b) = self.peek() {
            if b == b'\n' {
                self.pos += 1;
                return Some((start, Tok::Sep, self.pos));
            }
            self.pos += 1;
        }
        None
    }

    pub(super) fn lex_direction(&mut self) -> Option<(usize, Tok, usize)> {
        if !self.allow_header_direction {
            return None;
        }
        let start = self.pos;
        let rest = &self.input[self.pos..];
        for d in ["TB", "TD", "BT", "LR", "RL"] {
            if rest.starts_with(d) {
                let after = self.pos + d.len();
                if after < self.input.len() {
                    let b = self.input.as_bytes()[after];
                    if b.is_ascii_alphanumeric() || b == b'_' || b == b'-' {
                        continue;
                    }
                }
                self.pos = after;
                self.allow_header_direction = false;
                let d = Self::normalize_direction_token(d);
                return Some((start, Tok::Direction(d.to_string()), self.pos));
            }
        }

        if let Some(&b) = rest.as_bytes().first() {
            let mapped = match b {
                b'>' => Some("LR"),
                b'<' => Some("RL"),
                b'^' => Some("BT"),
                b'v' => Some("TB"),
                _ => None,
            };
            if let Some(d) = mapped {
                let after = self.pos + 1;
                if after < self.input.len() {
                    let next = self.input.as_bytes()[after];
                    if next.is_ascii_alphanumeric() || next == b'_' || next == b'-' {
                        return None;
                    }
                }
                self.pos = after;
                self.allow_header_direction = false;
                return Some((start, Tok::Direction(d.to_string()), self.pos));
            }
        }

        None
    }

    pub(super) fn lex_direction_stmt(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if !self.starts_with_kw("direction") {
            return None;
        }
        self.pos += "direction".len();
        self.skip_ws();

        let rest = &self.input[self.pos..];
        let mut dir: Option<&str> = None;
        for d in ["TB", "TD", "BT", "LR", "RL"] {
            if rest.starts_with(d) {
                dir = Some(d);
                self.pos += d.len();
                break;
            }
        }
        let Some(dir) = dir else {
            return Some((start, Tok::DirectionStmt("".to_string()), self.pos));
        };
        let dir = Self::normalize_direction_token(dir);

        while let Some(b) = self.peek() {
            if b == b'\n' || b == b';' {
                break;
            }
            self.pos += 1;
        }

        Some((start, Tok::DirectionStmt(dir.to_string()), self.pos))
    }

    pub(super) fn capture_to_stmt_end(&mut self) -> (usize, String, usize) {
        let start = self.pos;
        let mut in_double_quote = false;
        let mut in_single_quote = false;
        let mut escaped = false;
        while self.pos < self.input.len() {
            let b = self.input.as_bytes()[self.pos];
            if in_double_quote {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'"' {
                    in_double_quote = false;
                }
                self.pos += 1;
                continue;
            }
            if in_single_quote {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'\'' {
                    in_single_quote = false;
                }
                self.pos += 1;
                continue;
            }

            if b == b'"' {
                in_double_quote = true;
                self.pos += 1;
                continue;
            }
            if b == b'\'' {
                in_single_quote = true;
                self.pos += 1;
                continue;
            }

            if b == b'\n' || b == b';' {
                break;
            }
            self.pos += 1;
        }
        (start, self.input[start..self.pos].to_string(), self.pos)
    }

    pub(super) fn lex_style_sep(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.input[self.pos..].starts_with(":::") {
            self.pos += 3;
            return Some((start, Tok::StyleSep, self.pos));
        }
        None
    }

    pub(super) fn lex_shape_data(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if !self.input[self.pos..].starts_with("@{") {
            return None;
        }
        self.pos += 2;

        // Mermaid's Jison lexer has dedicated states for shapeData strings:
        // - it allows `}` inside double-quoted strings
        // - it rewrites `\n\s*` inside double-quoted strings to `<br/>`
        //
        // We mimic that behavior here while returning a single `ShapeData` token.
        let bytes = self.input.as_bytes();
        let mut out = String::new();
        let mut segment_start = self.pos;
        let mut in_string = false;

        while self.pos < self.input.len() {
            let b = bytes[self.pos];
            if !in_string {
                if b == b'"' {
                    out.push_str(&self.input[segment_start..self.pos + 1]);
                    self.pos += 1;
                    segment_start = self.pos;
                    in_string = true;
                    continue;
                }
                if b == b'}' {
                    out.push_str(&self.input[segment_start..self.pos]);
                    self.pos += 1;
                    return Some((start, Tok::ShapeData(out), self.pos));
                }
                self.pos += 1;
                continue;
            }

            if b == b'"' {
                out.push_str(&self.input[segment_start..self.pos + 1]);
                self.pos += 1;
                segment_start = self.pos;
                in_string = false;
                continue;
            }

            if b == b'\n' {
                out.push_str(&self.input[segment_start..self.pos]);
                out.push_str("<br/>");
                self.pos += 1;
                while self.pos < self.input.len() {
                    match bytes[self.pos] {
                        b' ' | b'\t' | b'\r' => self.pos += 1,
                        _ => break,
                    }
                }
                segment_start = self.pos;
                continue;
            }

            self.pos += 1;
        }

        out.push_str(&self.input[segment_start..self.pos]);
        Some((start, Tok::ShapeData(out), self.pos))
    }

    pub(super) fn lex_edge_id(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let bytes = self.input.as_bytes();
        if start >= bytes.len() {
            return None;
        }
        let first = bytes[start];
        if !first.is_ascii_alphanumeric() && first != b'_' {
            return None;
        }
        let mut i = start;
        while i < bytes.len() {
            let b = bytes[i];
            if b.is_ascii_alphanumeric() || b == b'_' || b == b'-' {
                i += 1;
                continue;
            }
            break;
        }
        if i >= bytes.len() || bytes[i] != b'@' {
            return None;
        }
        let next = bytes.get(i + 1).copied();
        if matches!(next, Some(b'{') | Some(b'"')) {
            return None;
        }
        self.pos = i + 1;
        let id = self.input[start..i].to_string();
        Some((start, Tok::EdgeId(id), self.pos))
    }

    pub(super) fn lex_style_stmt(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_kw("style") {
            return None;
        }
        self.pos += "style".len();
        self.skip_ws();
        let (_rest_start, rest, end) = self.capture_to_stmt_end();
        match lex::parse_style_stmt(&rest) {
            Ok(stmt) => Some(Ok((start, Tok::StyleStmt(stmt), end))),
            Err(e) => Some(Err(e)),
        }
    }

    pub(super) fn lex_classdef_stmt(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_kw("classDef") {
            return None;
        }
        self.pos += "classDef".len();
        self.skip_ws();
        let (_rest_start, rest, end) = self.capture_to_stmt_end();
        match lex::parse_classdef_stmt(&rest) {
            Ok(stmt) => Some(Ok((start, Tok::ClassDefStmt(stmt), end))),
            Err(e) => Some(Err(e)),
        }
    }

    pub(super) fn lex_class_assign_stmt(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_kw("class") {
            return None;
        }
        self.pos += "class".len();
        self.skip_ws();
        let (_rest_start, rest, end) = self.capture_to_stmt_end();
        match lex::parse_class_assign_stmt(&rest) {
            Ok(stmt) => Some(Ok((start, Tok::ClassAssignStmt(stmt), end))),
            Err(e) => Some(Err(e)),
        }
    }

    pub(super) fn lex_click_stmt(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_kw("click") {
            return None;
        }
        self.pos += "click".len();
        self.skip_ws();
        let (_rest_start, rest, end) = self.capture_to_stmt_end();
        match lex::parse_click_stmt(&rest) {
            Ok(stmt) => Some(Ok((start, Tok::ClickStmt(stmt), end))),
            Err(e) => Some(Err(e)),
        }
    }

    pub(super) fn lex_link_style_stmt(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_kw("linkStyle") {
            return None;
        }
        self.pos += "linkStyle".len();
        self.skip_ws();
        let (_rest_start, rest, end) = self.capture_to_stmt_end();
        match lex::parse_link_style_stmt(&rest) {
            Ok(stmt) => Some(Ok((start, Tok::LinkStyleStmt(stmt), end))),
            Err(e) => Some(Err(e)),
        }
    }

    pub(super) fn lex_subgraph_header_after_keyword(&mut self) -> Option<(usize, Tok, usize)> {
        // Match Mermaid's flowchart parser behavior: it consumes a single "SPACE" token after the
        // `subgraph` keyword, while any additional whitespace becomes part of the subgraph header
        // token (`textNoTags`). This affects whether `FlowDB.addSubGraph(...)` decides to auto-generate
        // a `subGraphN` id.
        //
        // Example:
        // - `subgraph main`   -> header text has no whitespace, id stays `main`
        // - `subgraph  main`  -> header text begins with whitespace, id becomes `subGraphN`
        if let Some(b) = self.peek() {
            if b == b'\n' || b == b';' {
                return None;
            }
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.pos += 1;
            }
        }

        let start = self.pos;
        if start >= self.input.len() {
            return None;
        }
        match self.input.as_bytes()[start] {
            b'\n' | b'\r' | b';' => return None,
            _ => {}
        }

        while self.pos < self.input.len() {
            let b = self.input.as_bytes()[self.pos];
            if b == b'\n' || b == b'\r' || b == b';' || b == b'[' {
                break;
            }
            self.pos += 1;
        }

        let raw_id = self.input[start..self.pos].to_string();
        let mut raw_title = raw_id.clone();
        let mut title_kind = TitleKind::Text;
        let mut id_equals_title = true;

        if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b'[' {
            id_equals_title = false;
            self.pos += 1;
            let title_start = self.pos;
            while self.pos < self.input.len() && self.input.as_bytes()[self.pos] != b']' {
                if self.input.as_bytes()[self.pos] == b'\n'
                    || self.input.as_bytes()[self.pos] == b';'
                {
                    break;
                }
                self.pos += 1;
            }
            raw_title = self.input[title_start..self.pos].to_string();
            let trimmed = raw_title.trim();
            if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            {
                title_kind = TitleKind::String;
            }
            if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b']' {
                self.pos += 1;
            }
        }

        Some((
            start,
            Tok::SubgraphHeader(SubgraphHeader {
                raw_id,
                raw_title,
                title_kind,
                id_equals_title,
            }),
            self.pos,
        ))
    }

    pub(super) fn lex_amp(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.peek()? != b'&' {
            return None;
        }
        self.pos += 1;
        Some((start, Tok::Amp, self.pos))
    }

    pub(super) fn lex_id(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let bytes = self.input.as_bytes();
        if start >= bytes.len() {
            return None;
        }
        let first = bytes[start];
        if !first.is_ascii_alphanumeric() && first != b'_' {
            return None;
        }
        self.pos += 1;

        while self.pos < bytes.len() {
            if self.pos + 1 < bytes.len()
                && (bytes[self.pos] == b'-' && bytes[self.pos + 1] == b'-'
                    || bytes[self.pos] == b'=' && bytes[self.pos + 1] == b'=')
            {
                break;
            }
            let b = bytes[self.pos];
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.pos += 1;
                continue;
            }
            if b == b'-' {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'-' {
                    break;
                }
                // Dotted edges start with `-.` (e.g. `A-.->B`). Avoid consuming the link start as
                // part of the id while still allowing ids like `subcontainer-child`.
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'.' {
                    break;
                }
                self.pos += 1;
                continue;
            }
            if b == b'.' {
                // Allow dots inside ids (Mermaid supports nodes like `P1.5`), but avoid consuming
                // the `.` that starts a dotted link token like `.->` when it is directly adjacent
                // to an id (e.g. `A.->B`).
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'-' {
                    break;
                }
                self.pos += 1;
                continue;
            }
            break;
        }

        if self.pos <= start {
            return None;
        }

        let id = self.input[start..self.pos].to_string();
        Some((start, Tok::Id(id), self.pos))
    }

    pub(super) fn lex_arrow_and_label(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        let bytes = self.input.as_bytes();

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum LinkFamily {
            Normal,
            Thick,
            Dotted,
            Invisible,
        }

        fn is_link_ws(b: u8) -> bool {
            matches!(b, b' ' | b'\t' | b'\r' | b'\n')
        }

        fn is_space_ws(b: u8) -> bool {
            matches!(b, b' ' | b'\t' | b'\r')
        }

        let match_link_end = |mut pos: usize,
                              family: LinkFamily,
                              allow_leading_ws: bool|
         -> Option<(usize, usize, String)> {
            let len = bytes.len();
            let match_start = pos;
            if allow_leading_ws {
                while pos < len && is_link_ws(bytes[pos]) {
                    pos += 1;
                }
            }
            let token_start = pos;
            if token_start >= len {
                return None;
            }

            let mut cur = token_start;
            let start_marker = bytes[cur];
            if matches!(start_marker, b'x' | b'o' | b'<') {
                cur += 1;
                if cur >= len {
                    return None;
                }
            }

            match family {
                LinkFamily::Invisible => {
                    cur = token_start;
                    let mut tildes = 0usize;
                    while cur < len && bytes[cur] == b'~' {
                        tildes += 1;
                        cur += 1;
                    }
                    if tildes < 3 {
                        return None;
                    }
                }
                LinkFamily::Normal => {
                    let hyphen_start = cur;
                    while cur < len && bytes[cur] == b'-' {
                        cur += 1;
                    }
                    let hyphens = cur - hyphen_start;
                    if hyphens < 2 {
                        return None;
                    }
                    if cur < len {
                        match bytes[cur] {
                            b'x' | b'o' | b'>' => {
                                cur += 1;
                            }
                            _ => {
                                // Open-ended edge: `--+` + `-` requires at least 3 hyphens total.
                                if hyphens < 3 {
                                    return None;
                                }
                            }
                        }
                    } else if hyphens < 3 {
                        return None;
                    }
                }
                LinkFamily::Thick => {
                    let eq_start = cur;
                    while cur < len && bytes[cur] == b'=' {
                        cur += 1;
                    }
                    let eqs = cur - eq_start;
                    if eqs < 2 {
                        return None;
                    }
                    if cur < len {
                        match bytes[cur] {
                            b'x' | b'o' | b'>' => {
                                cur += 1;
                            }
                            _ => {
                                // Open-ended edge: `==+` + `=` requires at least 3 '=' total.
                                if eqs < 3 {
                                    return None;
                                }
                            }
                        }
                    } else if eqs < 3 {
                        return None;
                    }
                }
                LinkFamily::Dotted => {
                    if cur < len && bytes[cur] == b'-' {
                        cur += 1;
                    }
                    let mut dots = 0usize;
                    while cur < len && bytes[cur] == b'.' {
                        dots += 1;
                        cur += 1;
                    }
                    if dots == 0 {
                        return None;
                    }
                    if cur >= len || bytes[cur] != b'-' {
                        return None;
                    }
                    cur += 1;
                    if cur < len && matches!(bytes[cur], b'x' | b'o' | b'>') {
                        cur += 1;
                    }
                }
            }

            let token_end = cur;
            let token = self.input[token_start..token_end]
                .split_whitespace()
                .collect::<String>();
            Some((match_start, token_end, token))
        };

        let compute_link = |end: String,
                            start: Option<String>|
         -> std::result::Result<LinkToken, LexError> {
            let (end_type, stroke, length) = destruct_end_link(&end);
            let mut edge_type = end_type;

            if let Some(start_str) = start.as_deref() {
                let (start_type, start_stroke) = destruct_start_link(start_str);
                if start_stroke != stroke.as_str() {
                    return Err(LexError {
                        message: "Invalid link: stroke mismatch between start and end".to_string(),
                    });
                }

                if start_type == "arrow_open" {
                    edge_type = edge_type.clone();
                } else {
                    if start_type != edge_type.as_str() {
                        return Err(LexError {
                            message: "Invalid link: start/end arrowhead mismatch".to_string(),
                        });
                    }
                    edge_type = format!("double_{start_type}");
                }

                if edge_type == "double_arrow" {
                    edge_type = "double_arrow_point".to_string();
                }
            }

            Ok(LinkToken {
                end,
                edge_type,
                stroke,
                length,
            })
        };

        // 1) Prefer full LINK tokens (matches Jison longest-match behavior).
        let families = [
            LinkFamily::Invisible,
            LinkFamily::Thick,
            LinkFamily::Normal,
            LinkFamily::Dotted,
        ];
        for family in families {
            if let Some((_mstart, mend, arrow)) = match_link_end(self.pos, family, false) {
                self.pos = mend;
                let arrow_end = mend;
                let link = match compute_link(arrow, None) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };

                // Optional pipe label: `A--x|label|B` or `A --> |label| B`.
                let mut pipe_pos = self.pos;
                while pipe_pos < self.input.len() && bytes[pipe_pos].is_ascii_whitespace() {
                    pipe_pos += 1;
                }
                if pipe_pos < self.input.len() && bytes[pipe_pos] == b'|' {
                    self.pos = pipe_pos + 1;
                    let label_start = self.pos;
                    while self.pos < self.input.len() && bytes[self.pos] != b'|' {
                        self.pos += 1;
                    }
                    if self.pos < self.input.len() && bytes[self.pos] == b'|' {
                        let raw = self.input[label_start..self.pos].trim();
                        let (text, kind) = parse_edge_label_text(raw);
                        self.pos += 1;
                        self.pending.push_back((
                            pipe_pos,
                            Tok::EdgeLabel(LabeledText { text, kind }),
                            self.pos,
                        ));
                    } else {
                        return Some(Ok((start, Tok::Arrow(link), arrow_end)));
                    }
                }

                return Some(Ok((start, Tok::Arrow(link), arrow_end)));
            }
        }

        // 2) START_LINK + edgeText + LINK (new notation): A-- text -->B
        let parse_start_link = |pos: usize| -> Option<(usize, LinkFamily, String, usize)> {
            let len = bytes.len();
            let token_start = pos;
            if token_start >= len {
                return None;
            }
            let mut cur = token_start;
            if matches!(bytes[cur], b'x' | b'o' | b'<') {
                cur += 1;
                if cur >= len {
                    return None;
                }
            }

            if cur + 1 < len && bytes[cur] == b'-' && bytes[cur + 1] == b'-' {
                cur += 2;
                let token = self.input[token_start..cur]
                    .split_whitespace()
                    .collect::<String>();
                return Some((token_start, LinkFamily::Normal, token, cur));
            }
            if cur + 1 < len && bytes[cur] == b'=' && bytes[cur + 1] == b'=' {
                cur += 2;
                let token = self.input[token_start..cur]
                    .split_whitespace()
                    .collect::<String>();
                return Some((token_start, LinkFamily::Thick, token, cur));
            }
            if cur + 1 < len && bytes[cur] == b'-' && bytes[cur + 1] == b'.' {
                cur += 2;
                let token = self.input[token_start..cur]
                    .split_whitespace()
                    .collect::<String>();
                return Some((token_start, LinkFamily::Dotted, token, cur));
            }
            None
        };

        let (_sstart, family, start_link, after_start) = parse_start_link(self.pos)?;
        let edge_text_start = after_start;
        let mut scan = edge_text_start;
        while scan < self.input.len() {
            if let Some((match_start, match_end, arrow)) = match_link_end(scan, family, true) {
                let raw_text = self.input[edge_text_start..match_start].trim();
                let (text, kind) = parse_edge_label_text(raw_text);
                self.pos = match_end;

                while self.pos < self.input.len() && is_space_ws(bytes[self.pos]) {
                    self.pos += 1;
                }

                if !text.is_empty() {
                    self.pending.push_back((
                        edge_text_start,
                        Tok::EdgeLabel(LabeledText { text, kind }),
                        match_start,
                    ));
                }
                let link = match compute_link(arrow, Some(start_link)) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                return Some(Ok((start, Tok::Arrow(link), match_end)));
            }
            scan += 1;
        }

        Some(Err(LexError {
            message: "Unterminated edge label (missing link terminator)".to_string(),
        }))
    }

    pub(super) fn lex_node_label(
        &mut self,
    ) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        let rest = &self.input[self.pos..];

        if rest.starts_with("[\\") {
            let open = "[\\";
            let content_start = self.pos + open.len();
            let end_slash = lex::find_unquoted_delim(self.input, content_start, "/]");
            let end_backslash = lex::find_unquoted_delim(self.input, content_start, "\\]");

            let (end_start, close, shape) = match (end_slash, end_backslash) {
                (None, None) => {
                    return Some(Err(LexError {
                        message: "Unterminated node label (missing `/]` or `\\]`)".to_string(),
                    }));
                }
                (Some(p), None) => (p, "/]", "inv_trapezoid"),
                (None, Some(p)) => (p, "\\]", "lean_left"),
                (Some(a), Some(b)) => {
                    if a <= b {
                        (a, "/]", "inv_trapezoid")
                    } else {
                        (b, "\\]", "lean_left")
                    }
                }
            };

            let raw = self.input[content_start..end_start].trim();
            let lt = match lex::parse_node_label_text(raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + close.len();
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: shape.to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        if rest.starts_with("[/") {
            let open = "[/";
            let content_start = self.pos + open.len();
            let end_slash = lex::find_unquoted_delim(self.input, content_start, "/]");
            let end_backslash = lex::find_unquoted_delim(self.input, content_start, "\\]");

            let (end_start, close, shape) = match (end_slash, end_backslash) {
                (None, None) => {
                    return Some(Err(LexError {
                        message: "Unterminated node label (missing `/]` or `\\]`)".to_string(),
                    }));
                }
                (Some(p), None) => (p, "/]", "lean_right"),
                (None, Some(p)) => (p, "\\]", "trapezoid"),
                (Some(a), Some(b)) => {
                    if a <= b {
                        (a, "/]", "lean_right")
                    } else {
                        (b, "\\]", "trapezoid")
                    }
                }
            };

            let raw = self.input[content_start..end_start].trim();
            let lt = match lex::parse_node_label_text(raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + close.len();
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: shape.to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        let candidates: [(&str, &str, &str); 8] = [
            ("(((", ")))", "doublecircle"),
            ("{{", "}}", "hexagon"),
            ("[[", "]]", "subroutine"),
            ("(-", "-)", "ellipse"),
            ("([", "])", "stadium"),
            ("[(", ")]", "cylinder"),
            ("((", "))", "circle"),
            (">", "]", "odd"),
        ];

        for (open, close, shape) in candidates {
            if !rest.starts_with(open) {
                continue;
            }
            let content_start = self.pos + open.len();
            let Some(end_start) = lex::find_unquoted_delim(self.input, content_start, close) else {
                return Some(Err(LexError {
                    message: format!("Unterminated node label (missing `{close}`)"),
                }));
            };
            let raw = self.input[content_start..end_start].trim();
            let lt = match lex::parse_node_label_text(raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + close.len();
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: shape.to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        if rest.starts_with("[") {
            let content_start = self.pos + 1;
            let Some(end_start) = lex::find_unquoted_delim(self.input, content_start, "]") else {
                return Some(Err(LexError {
                    message: "Unterminated node label (missing `]`)".to_string(),
                }));
            };
            let raw = self.input[content_start..end_start].trim();
            let (shape, label_raw) = lex::parse_rect_border_label(raw);
            let lt = match lex::parse_node_label_text(label_raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + 1;
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: shape.to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        if rest.starts_with("{") {
            let content_start = self.pos + 1;
            let Some(end_start) = lex::find_unquoted_delim(self.input, content_start, "}") else {
                return Some(Err(LexError {
                    message: "Unterminated node label (missing `}`)".to_string(),
                }));
            };
            let raw = self.input[content_start..end_start].trim();
            let lt = match lex::parse_node_label_text(raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + 1;
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: "diamond".to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        if rest.starts_with("(") {
            let content_start = self.pos + 1;
            let Some(end_start) = lex::find_unquoted_delim(self.input, content_start, ")") else {
                return Some(Err(LexError {
                    message: "Unterminated node label (missing `)`)".to_string(),
                }));
            };
            let raw = self.input[content_start..end_start].trim();
            let lt = match lex::parse_node_label_text(raw) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.pos = end_start + 1;
            return Some(Ok((
                start,
                Tok::NodeLabel(NodeLabelToken {
                    shape: "round".to_string(),
                    text: lt,
                }),
                self.pos,
            )));
        }

        None
    }
}
