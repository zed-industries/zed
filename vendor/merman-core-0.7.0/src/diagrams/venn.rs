use crate::{Error, ParseMetadata, Result};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VennSubsetRenderModel {
    pub sets: Vec<String>,
    pub size: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct VennTextNodeRenderModel {
    pub sets: Vec<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct VennStyleEntryRenderModel {
    pub targets: Vec<String>,
    pub styles: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VennDiagramRenderModel {
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub subsets: Vec<VennSubsetRenderModel>,
    #[serde(default, rename = "textNodes")]
    pub text_nodes: Vec<VennTextNodeRenderModel>,
    #[serde(default, rename = "styleEntries")]
    pub style_entries: Vec<VennStyleEntryRenderModel>,
}

impl VennDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone)]
struct VennParserState {
    model: VennDiagramRenderModel,
    known_sets: HashSet<String>,
    current_sets: Option<Vec<String>>,
    indent_mode: bool,
}

impl VennParserState {
    fn new() -> Self {
        Self {
            model: VennDiagramRenderModel::default(),
            known_sets: HashSet::new(),
            current_sets: None,
            indent_mode: false,
        }
    }

    fn add_subset(&mut self, identifiers: Vec<String>, label: Option<String>, size: Option<f64>) {
        let mut sets = normalize_identifier_list(identifiers);
        let resolved_size = size.unwrap_or_else(|| 10.0 / (sets.len() as f64).powi(2));
        self.current_sets = Some(sets.clone());

        if sets.len() == 1 {
            self.known_sets.insert(sets[0].clone());
        }

        self.model.subsets.push(VennSubsetRenderModel {
            sets: std::mem::take(&mut sets),
            size: resolved_size,
            label: label
                .map(|value| normalize_text(&value))
                .filter(|value| !value.is_empty()),
        });
    }

    fn validate_union_identifiers(
        &self,
        identifiers: &[String],
        meta: &ParseMetadata,
    ) -> Result<()> {
        let unknown = identifiers
            .iter()
            .map(|identifier| normalize_text(identifier))
            .filter(|identifier| !self.known_sets.contains(identifier))
            .collect::<Vec<_>>();

        if unknown.is_empty() {
            Ok(())
        } else {
            Err(parse_error(
                meta,
                format!("unknown set identifier: {}", unknown.join(", ")),
            ))
        }
    }

    fn add_text(&mut self, identifiers: Vec<String>, id: String, label: Option<String>) {
        self.model.text_nodes.push(VennTextNodeRenderModel {
            sets: normalize_identifier_list(identifiers),
            id: normalize_text(&id),
            label: label
                .map(|value| normalize_text(&value))
                .filter(|value| !value.is_empty()),
        });
    }

    fn add_style(&mut self, targets: Vec<String>, styles: BTreeMap<String, String>) {
        self.model.style_entries.push(VennStyleEntryRenderModel {
            targets: normalize_identifier_list(targets),
            styles,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextIdKind {
    IdentifierOrString,
    Numeric,
}

pub fn parse_venn(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let model = parse_venn_model_for_render(code, meta)?;
    Ok(json!({
        "type": meta.diagram_type,
        "title": model.title,
        "accTitle": model.acc_title,
        "accDescr": model.acc_descr,
        "subsets": model.subsets,
        "textNodes": model.text_nodes,
        "styleEntries": model.style_entries,
    }))
}

pub fn parse_venn_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<VennDiagramRenderModel> {
    let mut state = VennParserState::new();
    let mut lines = code.lines();

    let header_rest = loop {
        let Some(raw) = lines.next() else {
            return Err(parse_error(meta, "expected venn-beta"));
        };
        let line = strip_inline_comment_aware(raw.trim_end_matches('\r'));
        if line.trim().is_empty() {
            continue;
        }
        break parse_header(line, meta)?;
    };

    if !header_rest.trim().is_empty() {
        parse_statement(header_rest, &mut state, meta)?;
    }

    for raw in lines {
        let line = strip_inline_comment_aware(raw.trim_end_matches('\r'));
        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_indent_len(line);
        let statement = &line[indent..];
        if indent > 0 && state.indent_mode && starts_with_keyword_ci(statement, "text") {
            let tail = strip_keyword_ci(statement, "text")
                .expect("starts_with_keyword_ci and strip_keyword_ci agree");
            parse_indented_text(tail, &mut state, meta)?;
            continue;
        }

        if indent == 0 {
            state.indent_mode = false;
        }
        parse_statement(statement, &mut state, meta)?;
    }

    Ok(state.model)
}

fn parse_header<'a>(line: &'a str, meta: &ParseMetadata) -> Result<&'a str> {
    let trimmed = line.trim_start();
    let Some(rest) = trimmed.get("venn-beta".len()..) else {
        return Err(parse_error(meta, "expected venn-beta"));
    };
    if !trimmed[.."venn-beta".len()].eq_ignore_ascii_case("venn-beta") {
        return Err(parse_error(meta, "expected venn-beta"));
    }
    Ok(rest)
}

fn parse_statement(line: &str, state: &mut VennParserState, meta: &ParseMetadata) -> Result<()> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(rest) = strip_keyword_ci(trimmed, "title") {
        let title = rest.trim_start();
        if title.is_empty() {
            state.model.title = Some(String::new());
        } else {
            state.model.title = Some(title.to_string());
        }
        return Ok(());
    }

    if let Some(rest) = strip_keyword_ci(trimmed, "set") {
        parse_set_statement(rest, state, meta)?;
        state.indent_mode = true;
        return Ok(());
    }

    if let Some(rest) = strip_keyword_ci(trimmed, "union") {
        parse_union_statement(rest, state, meta)?;
        state.indent_mode = true;
        return Ok(());
    }

    if let Some(rest) = strip_keyword_ci(trimmed, "text") {
        parse_text_statement(rest, state, meta)?;
        return Ok(());
    }

    if let Some(rest) = strip_keyword_ci(trimmed, "style") {
        parse_style_statement(rest, state, meta)?;
        return Ok(());
    }

    Err(parse_error(
        meta,
        format!("unexpected venn statement: {trimmed}"),
    ))
}

fn parse_set_statement(
    input: &str,
    state: &mut VennParserState,
    meta: &ParseMetadata,
) -> Result<()> {
    let (identifier, rest) = parse_identifier(input, meta)?;
    let rest = skip_ws(rest);
    if rest.starts_with(',') {
        return Err(parse_error(meta, "set requires single identifier"));
    }

    let (label, rest) = parse_optional_bracket_label(rest, meta)?;
    let (size, rest) = parse_optional_size(rest, meta)?;
    expect_end(rest, meta)?;

    state.add_subset(vec![identifier], label, size);
    Ok(())
}

fn parse_union_statement(
    input: &str,
    state: &mut VennParserState,
    meta: &ParseMetadata,
) -> Result<()> {
    let (identifiers, rest) = parse_identifier_list(input, meta)?;
    if identifiers.len() < 2 {
        return Err(parse_error(meta, "union requires multiple identifiers"));
    }
    state.validate_union_identifiers(&identifiers, meta)?;

    let (label, rest) = parse_optional_bracket_label(rest, meta)?;
    let (size, rest) = parse_optional_size(rest, meta)?;
    expect_end(rest, meta)?;

    state.add_subset(identifiers, label, size);
    Ok(())
}

fn parse_text_statement(
    input: &str,
    state: &mut VennParserState,
    meta: &ParseMetadata,
) -> Result<()> {
    let (sets, rest) = parse_identifier_list(input, meta)?;
    let (id, kind, rest) = parse_text_id(rest, meta)?;
    let (label, rest) = parse_optional_bracket_label(rest, meta)?;
    if kind == TextIdKind::Numeric && label.is_some() {
        return Err(parse_error(meta, "unexpected label after numeric text id"));
    }
    expect_end(rest, meta)?;

    state.add_text(sets, id, label);
    Ok(())
}

fn parse_indented_text(
    input: &str,
    state: &mut VennParserState,
    meta: &ParseMetadata,
) -> Result<()> {
    let sets = state
        .current_sets
        .clone()
        .ok_or_else(|| parse_error(meta, "text requires set"))?;
    let (id, kind, rest) = parse_text_id(input, meta)?;
    let (label, rest) = parse_optional_bracket_label(rest, meta)?;
    if kind == TextIdKind::Numeric && label.is_some() {
        return Err(parse_error(meta, "unexpected label after numeric text id"));
    }
    expect_end(rest, meta)?;

    state.add_text(sets, id, label);
    Ok(())
}

fn parse_style_statement(
    input: &str,
    state: &mut VennParserState,
    meta: &ParseMetadata,
) -> Result<()> {
    let (targets, rest) = parse_identifier_list(input, meta)?;
    let styles = parse_styles(rest, meta)?;
    state.add_style(targets, styles);
    Ok(())
}

fn parse_identifier_list<'a>(
    input: &'a str,
    meta: &ParseMetadata,
) -> Result<(Vec<String>, &'a str)> {
    let (first, mut rest) = parse_identifier(input, meta)?;
    let mut identifiers = vec![first];

    loop {
        rest = skip_ws(rest);
        let Some(after_comma) = rest.strip_prefix(',') else {
            break;
        };
        let (next, after_next) = parse_identifier(after_comma, meta)?;
        identifiers.push(next);
        rest = after_next;
    }

    Ok((identifiers, rest))
}

fn parse_identifier<'a>(input: &'a str, meta: &ParseMetadata) -> Result<(String, &'a str)> {
    let input = skip_ws(input);
    if let Some((value, rest)) = parse_string_token(input) {
        return Ok((value, rest));
    }

    let bytes = input.as_bytes();
    let Some(&first) = bytes.first() else {
        return Err(parse_error(meta, "expected identifier"));
    };
    if !(first.is_ascii_alphabetic() || first == b'_') {
        return Err(parse_error(meta, "expected identifier"));
    }

    let mut end = 1usize;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_alphanumeric() || b == b'_' || b == b'-' {
            end += 1;
        } else {
            break;
        }
    }

    Ok((input[..end].to_string(), &input[end..]))
}

fn parse_text_id<'a>(
    input: &'a str,
    meta: &ParseMetadata,
) -> Result<(String, TextIdKind, &'a str)> {
    let input = skip_ws(input);
    if let Some((value, rest)) = parse_string_token(input) {
        return Ok((value, TextIdKind::IdentifierOrString, rest));
    }
    if let Some((value, rest)) = parse_numeric_token(input) {
        return Ok((value, TextIdKind::Numeric, rest));
    }
    let (identifier, rest) = parse_identifier(input, meta)?;
    Ok((identifier, TextIdKind::IdentifierOrString, rest))
}

fn parse_string_token(input: &str) -> Option<(String, &str)> {
    let rest = input.strip_prefix('"')?;
    let end = rest.find('"')?;
    let value = &input[..end + 2];
    Some((value.to_string(), &rest[end + 1..]))
}

fn parse_numeric_token(input: &str) -> Option<(String, &str)> {
    let mut end = 0usize;
    let mut chars = input.char_indices().peekable();
    if chars.peek().is_some_and(|(_, ch)| matches!(ch, '+' | '-')) {
        let (idx, ch) = chars.next()?;
        end = idx + ch.len_utf8();
    }

    let mut digits_before = 0usize;
    while chars.peek().is_some_and(|(_, ch)| ch.is_ascii_digit()) {
        let (idx, ch) = chars.next()?;
        end = idx + ch.len_utf8();
        digits_before += 1;
    }

    let mut digits_after = 0usize;
    if chars.peek().is_some_and(|(_, ch)| *ch == '.') {
        let (idx, ch) = chars.next()?;
        end = idx + ch.len_utf8();
        while chars.peek().is_some_and(|(_, ch)| ch.is_ascii_digit()) {
            let (idx, ch) = chars.next()?;
            end = idx + ch.len_utf8();
            digits_after += 1;
        }
    }

    if digits_before == 0 && digits_after == 0 {
        return None;
    }

    Some((input[..end].to_string(), &input[end..]))
}

fn parse_optional_bracket_label<'a>(
    input: &'a str,
    meta: &ParseMetadata,
) -> Result<(Option<String>, &'a str)> {
    let input = skip_ws(input);
    let Some(rest) = input.strip_prefix('[') else {
        return Ok((None, input));
    };

    if let Some(rest) = rest.strip_prefix('"') {
        let Some(end) = rest.find("\"]") else {
            return Err(parse_error(meta, "unterminated bracket label"));
        };
        let label = rest[..end].to_string();
        return Ok((Some(label), &rest[end + 2..]));
    }

    let Some(end) = rest.find(']') else {
        return Err(parse_error(meta, "unterminated bracket label"));
    };
    if rest[..end].contains('"') {
        return Err(parse_error(meta, "invalid bracket label"));
    }
    Ok((Some(rest[..end].trim().to_string()), &rest[end + 1..]))
}

fn parse_optional_size<'a>(input: &'a str, meta: &ParseMetadata) -> Result<(Option<f64>, &'a str)> {
    let input = skip_ws(input);
    let Some(rest) = input.strip_prefix(':') else {
        return Ok((None, input));
    };
    let (raw, rest) =
        parse_numeric_token(skip_ws(rest)).ok_or_else(|| parse_error(meta, "expected numeric"))?;
    let value = raw
        .parse::<f64>()
        .map_err(|_| parse_error(meta, "expected numeric"))?;
    Ok((Some(value), rest))
}

fn parse_styles(input: &str, meta: &ParseMetadata) -> Result<BTreeMap<String, String>> {
    let mut styles = BTreeMap::new();
    let mut rest = skip_ws(input);
    if rest.is_empty() {
        return Err(parse_error(meta, "expected style field"));
    }

    loop {
        let (key, after_key) = parse_identifier(rest, meta)?;
        let after_key = skip_ws(after_key);
        let Some(after_colon) = after_key.strip_prefix(':') else {
            return Err(parse_error(meta, "expected ':' after style field"));
        };
        let (value, after_value) = parse_style_value(after_colon, meta)?;
        styles.insert(key, value);

        rest = skip_ws(after_value);
        let Some(after_comma) = rest.strip_prefix(',') else {
            break;
        };
        rest = skip_ws(after_comma);
        if rest.is_empty() {
            return Err(parse_error(meta, "expected style field"));
        }
    }

    expect_end(rest, meta)?;
    Ok(styles)
}

fn parse_style_value<'a>(input: &'a str, meta: &ParseMetadata) -> Result<(String, &'a str)> {
    let input = skip_ws(input);
    if let Some((value, rest)) = parse_string_token(input) {
        return Ok((normalize_text(&value), rest));
    }

    let (raw, rest) = take_style_value_segment(input);
    let value = style_value_tokens(raw, meta)?.join(" ");
    if value.is_empty() {
        return Err(parse_error(meta, "expected style value"));
    }
    Ok((value, rest))
}

fn take_style_value_segment(input: &str) -> (&str, &str) {
    let mut in_quote = false;
    let mut paren_depth = 0usize;

    for (idx, ch) in input.char_indices() {
        match ch {
            '"' => in_quote = !in_quote,
            '(' if !in_quote => paren_depth += 1,
            ')' if !in_quote => paren_depth = paren_depth.saturating_sub(1),
            ',' if !in_quote && paren_depth == 0 => return (&input[..idx], &input[idx..]),
            _ => {}
        }
    }

    (input, "")
}

fn style_value_tokens(input: &str, meta: &ParseMetadata) -> Result<Vec<String>> {
    let mut rest = input.trim();
    let mut tokens = Vec::new();
    while !rest.is_empty() {
        if let Some((token, after)) = parse_rgb_like_token(rest) {
            tokens.push(token.to_string());
            rest = skip_ws(after);
            continue;
        }
        if let Some((token, after)) = parse_hex_color_token(rest) {
            tokens.push(token.to_string());
            rest = skip_ws(after);
            continue;
        }
        if let Some((token, after)) = parse_numeric_token(rest) {
            tokens.push(token);
            rest = skip_ws(after);
            continue;
        }
        if let Ok((identifier, after)) = parse_identifier(rest, meta) {
            tokens.push(identifier);
            rest = skip_ws(after);
            continue;
        }

        return Err(parse_error(meta, "expected style value"));
    }
    Ok(tokens)
}

fn parse_rgb_like_token(input: &str) -> Option<(&str, &str)> {
    let lower = input.to_ascii_lowercase();
    if !(lower.starts_with("rgb(") || lower.starts_with("rgba(")) {
        return None;
    }
    let end = input.find(')')?;
    Some((&input[..end + 1], &input[end + 1..]))
}

fn parse_hex_color_token(input: &str) -> Option<(&str, &str)> {
    let rest = input.strip_prefix('#')?;
    let len = rest
        .bytes()
        .take_while(|byte| byte.is_ascii_hexdigit())
        .count();
    if (3..=8).contains(&len) {
        Some((&input[..len + 1], &input[len + 1..]))
    } else {
        None
    }
}

fn normalize_identifier_list(identifiers: Vec<String>) -> Vec<String> {
    let mut out = identifiers
        .into_iter()
        .map(|identifier| normalize_text(&identifier))
        .collect::<Vec<_>>();
    out.sort();
    out
}

fn normalize_text(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn leading_indent_len(line: &str) -> usize {
    line.chars()
        .take_while(|ch| matches!(ch, ' ' | '\t'))
        .map(char::len_utf8)
        .sum()
}

fn skip_ws(input: &str) -> &str {
    input.trim_start_matches(|ch| matches!(ch, ' ' | '\t'))
}

fn strip_keyword_ci<'a>(input: &'a str, keyword: &str) -> Option<&'a str> {
    let candidate = input.get(..keyword.len())?;
    if !candidate.eq_ignore_ascii_case(keyword) {
        return None;
    }
    let rest = &input[keyword.len()..];
    if rest
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return None;
    }
    Some(rest)
}

fn starts_with_keyword_ci(input: &str, keyword: &str) -> bool {
    let Some(candidate) = input.get(..keyword.len()) else {
        return false;
    };
    if !candidate.eq_ignore_ascii_case(keyword) {
        return false;
    }
    input[keyword.len()..]
        .chars()
        .next()
        .is_none_or(|ch| ch.is_whitespace())
}

fn expect_end(input: &str, meta: &ParseMetadata) -> Result<()> {
    if input.trim().is_empty() {
        Ok(())
    } else {
        Err(parse_error(
            meta,
            format!("unexpected trailing venn tokens: {}", input.trim()),
        ))
    }
}

fn strip_inline_comment_aware(line: &str) -> &str {
    let mut in_quote = false;
    let mut bracket_depth = 0usize;
    let mut iter = line.char_indices().peekable();

    while let Some((idx, ch)) = iter.next() {
        match ch {
            '"' => in_quote = !in_quote,
            '[' if !in_quote => bracket_depth += 1,
            ']' if !in_quote => bracket_depth = bracket_depth.saturating_sub(1),
            '%' if !in_quote && bracket_depth == 0 => {
                if iter.peek().is_some_and(|(_, next)| *next == '%') {
                    return &line[..idx];
                }
            }
            _ => {}
        }
    }

    line
}

fn parse_error(meta: &ParseMetadata, message: impl Into<String>) -> Error {
    Error::DiagramParse {
        diagram_type: meta.diagram_type.clone(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, MermaidConfig, ParseMetadata, ParseOptions, RenderSemanticModel};

    fn meta() -> ParseMetadata {
        ParseMetadata {
            diagram_type: "venn".to_string(),
            config: MermaidConfig::empty_object(),
            effective_config: MermaidConfig::empty_object(),
            title: None,
        }
    }

    fn parse(input: &str) -> VennDiagramRenderModel {
        parse_venn_model_for_render(input, &meta()).unwrap()
    }

    #[test]
    fn parses_simple_sets_title_and_default_union_size() {
        let model = parse(
            r#"venn-beta
          title foo bar
          set A
          set B
          union A,B
      "#,
        );

        assert_eq!(model.title.as_deref(), Some("foo bar"));
        assert_eq!(
            model.subsets,
            vec![
                VennSubsetRenderModel {
                    sets: vec!["A".to_string()],
                    size: 10.0,
                    label: None,
                },
                VennSubsetRenderModel {
                    sets: vec!["B".to_string()],
                    size: 10.0,
                    label: None,
                },
                VennSubsetRenderModel {
                    sets: vec!["A".to_string(), "B".to_string()],
                    size: 2.5,
                    label: None,
                },
            ]
        );
    }

    #[test]
    fn parses_bracket_labels_and_size_suffixes() {
        let model = parse(
            r#"venn-beta
          title foo bar
          set A["Alpha"]:20
          set B[Beta]:12
          set C["Gamma"]:30
          union A,B["AB"]:5.3
          union C,A,B:1
      "#,
        );

        assert_eq!(model.subsets[0].label.as_deref(), Some("Alpha"));
        assert_eq!(model.subsets[0].size, 20.0);
        assert_eq!(model.subsets[1].label.as_deref(), Some("Beta"));
        assert_eq!(model.subsets[1].size, 12.0);
        assert_eq!(model.subsets[3].sets, ["A", "B"]);
        assert_eq!(model.subsets[3].label.as_deref(), Some("AB"));
        assert_eq!(model.subsets[3].size, 5.3);
        assert_eq!(model.subsets[4].sets, ["A", "B", "C"]);
    }

    #[test]
    fn parses_text_nodes_with_explicit_and_indented_forms() {
        let model = parse(
            r#"venn-beta
          set A["Frontend"]
            text A1["React"]
            text A2
          set B["Backend"]
            text B1
          union A,B["APIs"]
            text AB1["OpenAPI"]
      "#,
        );

        assert_eq!(
            model.text_nodes,
            vec![
                VennTextNodeRenderModel {
                    sets: vec!["A".to_string()],
                    id: "A1".to_string(),
                    label: Some("React".to_string()),
                },
                VennTextNodeRenderModel {
                    sets: vec!["A".to_string()],
                    id: "A2".to_string(),
                    label: None,
                },
                VennTextNodeRenderModel {
                    sets: vec!["B".to_string()],
                    id: "B1".to_string(),
                    label: None,
                },
                VennTextNodeRenderModel {
                    sets: vec!["A".to_string(), "B".to_string()],
                    id: "AB1".to_string(),
                    label: Some("OpenAPI".to_string()),
                },
            ]
        );
    }

    #[test]
    fn parses_explicit_text_statement_and_numeric_text_id() {
        let model = parse(
            r#"venn-beta
set A
set B
union A,B
text A alpha["Alpha note"]
text A,B 42
      "#,
        );

        assert_eq!(
            model.text_nodes,
            vec![
                VennTextNodeRenderModel {
                    sets: vec!["A".to_string()],
                    id: "alpha".to_string(),
                    label: Some("Alpha note".to_string()),
                },
                VennTextNodeRenderModel {
                    sets: vec!["A".to_string(), "B".to_string()],
                    id: "42".to_string(),
                    label: None,
                },
            ]
        );
    }

    #[test]
    fn parses_style_entries() {
        let model = parse(
            r#"venn-beta
          set A
          set B
          union A,B
          style A fill:#ff6b6b, color:#333
          style A,B fill:rgb(255, 0, 128)
          style B fill:rgba(255, 0, 128, 0.5)
      "#,
        );

        assert_eq!(model.style_entries[0].targets, ["A"]);
        assert_eq!(
            model.style_entries[0]
                .styles
                .get("fill")
                .map(String::as_str),
            Some("#ff6b6b")
        );
        assert_eq!(
            model.style_entries[0]
                .styles
                .get("color")
                .map(String::as_str),
            Some("#333")
        );
        assert_eq!(model.style_entries[1].targets, ["A", "B"]);
        assert_eq!(
            model.style_entries[1]
                .styles
                .get("fill")
                .map(String::as_str),
            Some("rgb(255, 0, 128)")
        );
        assert_eq!(
            model.style_entries[2]
                .styles
                .get("fill")
                .map(String::as_str),
            Some("rgba(255, 0, 128, 0.5)")
        );
    }

    #[test]
    fn rejects_invalid_set_and_union_shapes() {
        let err = parse_venn_model_for_render("venn-beta\nset A,B\n", &meta()).unwrap_err();
        assert!(err.to_string().contains("set requires single identifier"));

        let err = parse_venn_model_for_render("venn-beta\nunion A\n", &meta()).unwrap_err();
        assert!(
            err.to_string()
                .contains("union requires multiple identifiers")
        );

        let err = parse_venn_model_for_render("venn-beta\nset Foo\nunion Foo,Buz\n", &meta())
            .unwrap_err();
        assert!(err.to_string().contains("unknown set identifier"));
    }

    #[test]
    fn parses_quoted_identifiers_and_sorts_union_sets() {
        let model = parse(
            r#"venn-beta
        set "Foo Bar"
        set Buz
        union "Foo Bar",Buz
    "#,
        );

        assert_eq!(model.subsets[0].sets, ["Foo Bar"]);
        assert_eq!(model.subsets[2].sets, ["Buz", "Foo Bar"]);
    }

    #[test]
    fn render_model_entrypoint_returns_typed_venn_model() {
        let parsed = Engine::new()
            .parse_diagram_for_render_model_sync(
                "venn-beta\nset A\nset B\nunion A,B\n",
                ParseOptions::strict(),
            )
            .unwrap()
            .unwrap();

        assert_eq!(parsed.meta.diagram_type, "venn");
        let RenderSemanticModel::Venn(model) = parsed.model else {
            panic!("expected Venn render model");
        };
        assert_eq!(model.subsets.len(), 3);
        assert_eq!(model.subsets[2].sets, ["A", "B"]);
    }
}
