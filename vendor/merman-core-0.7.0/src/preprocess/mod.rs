use crate::{DetectorRegistry, Error, MermaidConfig, Result};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct PreprocessResult {
    pub code: String,
    pub title: Option<String>,
    pub config: MermaidConfig,
}

const FRONTMATTER_DIAGRAM_CONFIG_KEYS: &[&str] = &[
    "architecture",
    "block",
    "c4",
    "class",
    "er",
    "flowchart",
    "gantt",
    "gitGraph",
    "journey",
    "kanban",
    "mindmap",
    "packet",
    "pie",
    "quadrantChart",
    "radar",
    "requirement",
    "sankey",
    "sequence",
    "state",
    "timeline",
    "venn",
    "xyChart",
];

const FRONTMATTER_DIAGRAM_CONFIG_ALIASES: &[(&str, &str)] = &[
    ("classDiagram", "class"),
    ("erDiagram", "er"),
    ("stateDiagram", "state"),
    ("xychart", "xyChart"),
];

const MAX_CONFIG_NESTING_DEPTH: usize = crate::MAX_DIAGRAM_NESTING_DEPTH;

pub fn preprocess_diagram(input: &str, registry: &DetectorRegistry) -> Result<PreprocessResult> {
    preprocess_diagram_with_known_type(input, registry, None)
}

pub fn preprocess_diagram_with_known_type(
    input: &str,
    registry: &DetectorRegistry,
    diagram_type: Option<&str>,
) -> Result<PreprocessResult> {
    let cleaned = cleanup_text(input);
    let (without_frontmatter, title, mut frontmatter_config) =
        process_frontmatter(cleaned.as_ref())?;
    let (without_directives, directive_config) =
        process_directives(without_frontmatter, registry, diagram_type)?;

    frontmatter_config.deep_merge(directive_config.as_value());

    let code = crate::utils::cleanup_mermaid_comments(without_directives.as_ref());
    Ok(PreprocessResult {
        code: code.into_owned(),
        title,
        config: frontmatter_config,
    })
}

fn cleanup_text(input: &str) -> Cow<'_, str> {
    let mut s: Cow<'_, str> = if input.contains('\r') {
        Cow::Owned(normalize_crlf(input))
    } else {
        Cow::Borrowed(input)
    };

    // Mermaid encodes `#quot;`-style sequences before parsing (`encodeEntities(...)`).
    // This is required because `#` and `;` are significant in several grammars (comments and
    // statement separators), and the encoded placeholders are later decoded by the renderer.
    //
    // Source of truth: `packages/mermaid/src/utils.ts::encodeEntities` at Mermaid@11.12.2.
    if s.contains('#') {
        s = Cow::Owned(encode_mermaid_entities_like_upstream(s.as_ref()));
    }

    // Mermaid performs this HTML attribute rewrite as part of preprocessing.
    if s.contains('<') && s.contains("=\"") {
        s = Cow::Owned(normalize_html_tag_attributes_like_upstream(s.as_ref()));
    }

    s
}

fn normalize_crlf(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\r' {
            out.push('\n');
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn normalize_html_tag_attributes_like_upstream(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = text[probe..].find('<') {
        let start = probe + rel_start;
        let tag_start = start + 1;
        if tag_start >= bytes.len() || !is_mermaid_js_word_byte(bytes[tag_start]) {
            probe = tag_start;
            continue;
        }

        let mut tag_end = tag_start + 1;
        while tag_end < bytes.len() && is_mermaid_js_word_byte(bytes[tag_end]) {
            tag_end += 1;
        }

        let Some(rel_end) = text[tag_end..].find('>') else {
            probe = tag_start;
            continue;
        };
        let end = tag_end + rel_end;

        out.push_str(&text[cursor..start]);
        out.push('<');
        out.push_str(&text[tag_start..tag_end]);
        out.push_str(&normalize_html_attributes_like_upstream(
            &text[tag_end..end],
        ));
        out.push('>');

        cursor = end + 1;
        probe = end + 1;
    }

    out.push_str(&text[cursor..]);
    out
}

fn normalize_html_attributes_like_upstream(attributes: &str) -> String {
    let mut out = String::with_capacity(attributes.len());
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = attributes[probe..].find("=\"") {
        let start = probe + rel_start;
        let value_start = start + 2;
        let Some(rel_end) = attributes[value_start..].find('"') else {
            probe = value_start;
            continue;
        };
        let end = value_start + rel_end;

        out.push_str(&attributes[cursor..start]);
        out.push_str("='");
        out.push_str(&attributes[value_start..end]);
        out.push('\'');

        cursor = end + 1;
        probe = end + 1;
    }

    out.push_str(&attributes[cursor..]);
    out
}

fn encode_mermaid_entities_like_upstream(text: &str) -> String {
    if !text.contains('#') {
        return text.to_string();
    }

    // Mirrors Mermaid `encodeEntities` (Mermaid@11.12.2):
    //
    // 1) Protect `style...:#...;` and `classDef...:#...;` so color hex fragments are not mistaken
    //    as entities by the `/#\\w+;/g` pass.
    // 2) Encode `#<name>;` and `#<number>;` sequences into placeholders that do not contain `#`/`;`.
    let mut txt = text.to_string();

    if txt.contains("style") && txt.contains(';') {
        txt = strip_hex_style_semicolons_like_upstream(&txt, "style");
    }

    if txt.contains("classDef") && txt.contains(';') {
        txt = strip_hex_style_semicolons_like_upstream(&txt, "classDef");
    }

    if txt.contains(';') {
        txt = encode_entity_placeholders_like_upstream(&txt);
    }

    txt
}

fn encode_entity_placeholders_like_upstream(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut cursor = 0usize;

    while let Some(rel_hash) = text[cursor..].find('#') {
        let start = cursor + rel_hash;
        let mut end = start + 1;
        while end < bytes.len() && is_mermaid_js_word_byte(bytes[end]) {
            end += 1;
        }

        if end > start + 1 && bytes.get(end) == Some(&b';') {
            out.push_str(&text[cursor..start]);
            let inner = &text[start + 1..end];
            if inner.bytes().all(|b| b.is_ascii_digit()) {
                out.push_str("ﬂ°°");
            } else {
                out.push_str("ﬂ°");
            }
            out.push_str(inner);
            out.push_str("¶ß");
            cursor = end + 1;
        } else {
            out.push_str(&text[cursor..=start]);
            cursor = start + 1;
        }
    }

    out.push_str(&text[cursor..]);
    out
}

fn is_mermaid_js_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn strip_hex_style_semicolons_like_upstream(text: &str, keyword: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut line_start = 0usize;

    for (idx, ch) in text.char_indices() {
        if ch == '\n' {
            strip_hex_style_semicolons_from_line(&text[line_start..idx], keyword, &mut out);
            out.push('\n');
            line_start = idx + ch.len_utf8();
        }
    }

    strip_hex_style_semicolons_from_line(&text[line_start..], keyword, &mut out);
    out
}

fn strip_hex_style_semicolons_from_line(line: &str, keyword: &str, out: &mut String) {
    let mut cursor = 0usize;
    while let Some(semicolon) = find_hex_style_match(line, keyword, cursor) {
        out.push_str(&line[cursor..semicolon]);
        cursor = semicolon + 1;
    }
    out.push_str(&line[cursor..]);
}

fn find_hex_style_match(line: &str, keyword: &str, search_start: usize) -> Option<usize> {
    let mut probe = search_start;
    while let Some(rel_start) = line[probe..].find(keyword) {
        let start = probe + rel_start;
        if let Some(semicolon) = find_hex_style_match_end(line, start + keyword.len()) {
            return Some(semicolon);
        }
        probe = start + keyword.len();
    }
    None
}

fn find_hex_style_match_end(line: &str, search_start: usize) -> Option<usize> {
    let mut probe = search_start;
    while let Some(rel_colon) = line[probe..].find(':') {
        let colon = probe + rel_colon;
        let mut hash = None;
        for (rel, ch) in line[colon + 1..].char_indices() {
            if ch.is_whitespace() {
                break;
            }
            if ch == '#' {
                hash = Some(colon + 1 + rel);
                break;
            }
        }

        if let Some(hash) = hash {
            return line[hash + 1..].rfind(';').map(|rel| hash + 1 + rel);
        }

        probe = colon + 1;
    }
    None
}

fn process_frontmatter(input: &str) -> Result<(&str, Option<String>, MermaidConfig)> {
    if !input.trim_start().starts_with("---") {
        return Ok((input, None, MermaidConfig::empty_object()));
    }

    let Some((yaml_body, stripped)) = split_frontmatter(input) else {
        return Ok((input, None, MermaidConfig::empty_object()));
    };

    if config_nesting_exceeds_limit(yaml_body) {
        return Err(Error::InvalidFrontMatterYaml {
            message: format!("config nesting exceeds {MAX_CONFIG_NESTING_DEPTH} levels"),
        });
    }

    let raw_yaml: serde_yaml::Value =
        serde_yaml::from_str(yaml_body).map_err(|e| Error::InvalidFrontMatterYaml {
            message: e.to_string(),
        })?;
    let parsed = serde_json::to_value(raw_yaml).unwrap_or(Value::Null);
    let parsed_obj = match parsed {
        Value::Object(obj) => obj,
        other => {
            crate::config::drop_value_nonrecursive(other);
            Default::default()
        }
    };

    let mut title = None;
    let mut display_mode = None;

    if let Some(Value::String(t)) = parsed_obj.get("title") {
        title = Some(t.clone());
    }
    if let Some(Value::String(dm)) = parsed_obj.get("displayMode") {
        display_mode = Some(dm.clone());
    }

    let mut config = MermaidConfig::empty_object();
    merge_top_level_frontmatter_diagram_configs(&parsed_obj, &mut config);
    if let Some(v) = parsed_obj.get("config") {
        config.deep_merge(v);
    }
    crate::config::mirror_legacy_font_family_into_theme_variables(&mut config);
    if let Some(dm) = display_mode {
        config.set_value("gantt.displayMode", Value::String(dm));
    }

    crate::config::drop_value_nonrecursive(Value::Object(parsed_obj));
    Ok((stripped, title, config))
}

fn split_frontmatter(input: &str) -> Option<(&str, &str)> {
    let after_marker = input.strip_prefix("---")?;
    let open_line_end = after_marker.find('\n')?;
    if !after_marker[..open_line_end].trim().is_empty() {
        return None;
    }

    let body_start = 3 + open_line_end + 1;
    let rest = &input[body_start..];
    let mut offset = 0usize;

    for line in rest.split_inclusive('\n') {
        let without_newline = line.trim_end_matches(['\r', '\n']);
        if without_newline.trim() == "---" {
            let body = &rest[..offset];
            let stripped = &rest[offset + line.len()..];
            return Some((body, stripped));
        }
        offset += line.len();
    }

    None
}

fn merge_top_level_frontmatter_diagram_configs(
    parsed_obj: &serde_json::Map<String, Value>,
    config: &mut MermaidConfig,
) {
    // Mermaid upstream only consumes `config`, but users commonly read docs examples as allowing
    // diagram config namespaces at the YAML root. Keep this compatibility narrow and explicit.
    for &(source_key, target_key) in FRONTMATTER_DIAGRAM_CONFIG_ALIASES {
        if let Some(value) = parsed_obj.get(source_key) {
            config.set_value(target_key, crate::config::clone_value_nonrecursive(value));
        }
    }

    for &key in FRONTMATTER_DIAGRAM_CONFIG_KEYS {
        if let Some(value) = parsed_obj.get(key) {
            config.set_value(key, crate::config::clone_value_nonrecursive(value));
        }
    }
}

fn process_directives<'a>(
    input: &'a str,
    registry: &DetectorRegistry,
    diagram_type: Option<&str>,
) -> Result<(Cow<'a, str>, MermaidConfig)> {
    let directives = detect_directives(input)?;
    if directives.is_empty() {
        return Ok((Cow::Borrowed(input), MermaidConfig::empty_object()));
    }
    let init = detect_init(&directives, input, registry, diagram_type)?;
    let wrap = directives.iter().any(|d| d.ty == "wrap");

    let mut merged = init;
    if wrap {
        merged.set_value("wrap", Value::Bool(true));
    }

    Ok((Cow::Owned(remove_directives(input)), merged))
}

fn detect_init(
    directives: &[Directive],
    input: &str,
    registry: &DetectorRegistry,
    diagram_type: Option<&str>,
) -> Result<MermaidConfig> {
    let mut merged = MermaidConfig::empty_object();
    let mut config_for_detect = MermaidConfig::empty_object();

    for d in directives {
        if d.ty != "init" && d.ty != "initialize" {
            continue;
        }

        let mut args = match &d.args {
            Some(v) => crate::config::clone_value_nonrecursive(v),
            None => Value::Object(Default::default()),
        };

        sanitize_directive(&mut args);

        // Mermaid moves a top-level `config` directive field into the diagram-type-specific config.
        if let Some(diagram_specific) = args
            .get("config")
            .map(crate::config::clone_value_nonrecursive)
        {
            let detected = diagram_type.map(|t| t.to_string()).or_else(|| {
                registry
                    .detect_type(input, &mut config_for_detect)
                    .ok()
                    .map(ToString::to_string)
            });

            if let Some(mut ty) = detected {
                if ty == "flowchart-v2" {
                    ty = "flowchart".to_string();
                }
                if let Value::Object(obj) = &mut args {
                    if let Some(old) = obj.insert(ty, diagram_specific) {
                        crate::config::drop_value_nonrecursive(old);
                    }
                    if let Some(old) = obj.remove("config") {
                        crate::config::drop_value_nonrecursive(old);
                    }
                }
            }
        }
        crate::config::mirror_legacy_font_family_into_theme_variables_value(&mut args);

        merged.deep_merge(&args);
    }

    Ok(merged)
}

#[derive(Debug, Clone)]
struct Directive {
    ty: String,
    args: Option<Value>,
}

fn detect_directives(input: &str) -> Result<Vec<Directive>> {
    let mut out = Vec::new();
    let mut pos = 0;
    let trimmed = input.trim();
    if !trimmed.contains("%%{") {
        return Ok(out);
    }

    // Mermaid's directive parser effectively treats single quotes as double quotes for JSON-like
    // directive bodies. Keep this behavior, but only pay the allocation when directives exist.
    let text = trimmed.replace('\'', "\"");

    while let Some(rel) = text[pos..].find("%%{") {
        let start = pos + rel;
        let content_start = start + 3;
        let Some(rel_end) = text[content_start..].find("}%%") else {
            break;
        };
        let content_end = content_start + rel_end;
        let raw = text[content_start..content_end].trim();

        if let Some(d) = parse_directive(raw)? {
            out.push(d);
        }

        pos = content_end + 3;
    }

    Ok(out)
}

#[derive(Clone)]
enum DirectiveValuePathSegment {
    Key(String),
    Index(usize),
}

fn sanitize_directive(value: &mut Value) {
    let mut stack = vec![Vec::<DirectiveValuePathSegment>::new()];

    while let Some(path) = stack.pop() {
        let Some(current) = directive_value_at_path_mut(value, &path) else {
            continue;
        };

        match current {
            Value::Object(map) => {
                if let Some(old) = map.remove("secure") {
                    crate::config::drop_value_nonrecursive(old);
                }

                let blocked_keys = map
                    .keys()
                    .filter(|key| key.starts_with("__"))
                    .cloned()
                    .collect::<Vec<_>>();
                for key in blocked_keys {
                    if let Some(old) = map.remove(&key) {
                        crate::config::drop_value_nonrecursive(old);
                    }
                }

                let child_keys = map.keys().cloned().collect::<Vec<_>>();
                for key in child_keys.into_iter().rev() {
                    let mut child_path = path.clone();
                    child_path.push(DirectiveValuePathSegment::Key(key));
                    stack.push(child_path);
                }
            }
            Value::Array(arr) => {
                for idx in (0..arr.len()).rev() {
                    let mut child_path = path.clone();
                    child_path.push(DirectiveValuePathSegment::Index(idx));
                    stack.push(child_path);
                }
            }
            Value::String(s) => {
                let blocked = s.contains('<') || s.contains('>') || s.contains("url(data:");
                if blocked {
                    s.clear();
                }
            }
            _ => {}
        }
    }
}

fn directive_value_at_path_mut<'a>(
    mut value: &'a mut Value,
    path: &[DirectiveValuePathSegment],
) -> Option<&'a mut Value> {
    for segment in path {
        match segment {
            DirectiveValuePathSegment::Key(key) => {
                value = value.as_object_mut()?.get_mut(key)?;
            }
            DirectiveValuePathSegment::Index(idx) => {
                value = value.as_array_mut()?.get_mut(*idx)?;
            }
        }
    }
    Some(value)
}

fn remove_directives(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pos = 0;
    while let Some(rel) = text[pos..].find("%%{") {
        let start = pos + rel;
        out.push_str(&text[pos..start]);
        let after_start = start + 3;
        if let Some(rel_end) = text[after_start..].find("}%%") {
            let end = after_start + rel_end + 3;
            pos = end;
        } else {
            return out;
        }
    }
    out.push_str(&text[pos..]);
    out
}

fn parse_directive(raw: &str) -> Result<Option<Directive>> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(None);
    }

    let mut chars = raw.chars().peekable();
    let mut ty = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ty.push(c);
            chars.next();
            continue;
        }
        break;
    }
    if ty.is_empty() {
        return Ok(None);
    }

    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }

    let args = if matches!(chars.peek(), Some(':')) {
        chars.next();
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        let rest: String = chars.collect();
        let rest = rest.trim();
        if rest.is_empty() {
            None
        } else if rest.starts_with('{') || rest.starts_with('[') {
            if config_nesting_exceeds_limit(rest) {
                return Err(Error::InvalidDirectiveJson {
                    message: format!("config nesting exceeds {MAX_CONFIG_NESTING_DEPTH} levels"),
                });
            }
            Some(
                json5::from_str::<Value>(rest).map_err(|e| Error::InvalidDirectiveJson {
                    message: e.to_string(),
                })?,
            )
        } else {
            Some(Value::String(rest.to_string()))
        }
    } else {
        None
    };

    Ok(Some(Directive { ty, args }))
}

fn config_nesting_exceeds_limit(text: &str) -> bool {
    max_flow_collection_depth(text) > MAX_CONFIG_NESTING_DEPTH
        || max_yaml_indent_depth(text) > MAX_CONFIG_NESTING_DEPTH
}

fn max_flow_collection_depth(text: &str) -> usize {
    let mut max_depth = 0usize;
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for ch in text.chars() {
        if let Some(q) = quote {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == q {
                quote = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            '{' | '[' => {
                depth = depth.saturating_add(1);
                max_depth = max_depth.max(depth);
            }
            '}' | ']' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }

    max_depth
}

fn max_yaml_indent_depth(text: &str) -> usize {
    let mut indents = Vec::<usize>::new();
    let mut max_depth = 0usize;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = line.len() - line.trim_start_matches(' ').len();
        while indents.last().is_some_and(|prev| indent <= *prev) {
            indents.pop();
        }
        indents.push(indent);
        let inline_sequence_depth = yaml_inline_sequence_indicator_count(trimmed);
        max_depth = max_depth.max(indents.len() + inline_sequence_depth.saturating_sub(1));
    }

    max_depth
}

fn yaml_inline_sequence_indicator_count(mut text: &str) -> usize {
    let mut count = 0usize;
    loop {
        let Some(after_dash) = text.strip_prefix('-') else {
            return count;
        };
        if after_dash
            .chars()
            .next()
            .is_some_and(|ch| !ch.is_whitespace())
        {
            return count;
        }
        count += 1;
        text = after_dash.trim_start();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;

    #[test]
    fn normalize_crlf_matches_mermaid_line_ending_cleanup() {
        assert_eq!(
            normalize_crlf("flowchart TD\r\nA-->B\rC-->D\n"),
            "flowchart TD\nA-->B\nC-->D\n"
        );
        assert_eq!(normalize_crlf("\r\r\n\n"), "\n\n\n");
    }

    #[test]
    fn normalize_html_tag_attributes_matches_mermaid_cleanup_shape() {
        assert_eq!(
            normalize_html_tag_attributes_like_upstream(
                r#"<span title="A" data-empty="">Label</span><br disabled="yes">"#
            ),
            r#"<span title='A' data-empty=''>Label</span><br disabled='yes'>"#
        );
        assert_eq!(
            normalize_html_tag_attributes_like_upstream(r#"<é title="A"><_x value="B"><1 n="C">"#),
            r#"<é title="A"><_x value='B'><1 n='C'>"#
        );
        assert_eq!(
            normalize_html_tag_attributes_like_upstream(r#"<span a="x" title="A>B">"#),
            r#"<span a='x' title="A>B">"#
        );
        assert_eq!(
            normalize_html_tag_attributes_like_upstream(r#"<<span title="A">"#),
            r#"<<span title='A'>"#
        );
    }

    #[test]
    fn encode_entity_placeholders_matches_mermaid_ascii_word_shape() {
        assert_eq!(
            encode_mermaid_entities_like_upstream("Hello #there; #andHere;#77653;"),
            "Hello ﬂ°there¶ß ﬂ°andHere¶ßﬂ°°77653¶ß"
        );
        assert_eq!(
            encode_mermaid_entities_like_upstream(
                "style this; is ; everything :something#not-nothing; and this too;"
            ),
            "style this; is ; everything :something#not-nothing; and this too"
        );
        assert_eq!(
            encode_mermaid_entities_like_upstream(
                "classDef this; is ; everything :something#not-nothing; and this too;"
            ),
            "classDef this; is ; everything :something#not-nothing; and this too"
        );
        assert_eq!(
            encode_mermaid_entities_like_upstream("style a fill:#fff; style b fill:#000;"),
            "style a fill:ﬂ°fff¶ß style b fill:#000"
        );
        assert_eq!(
            encode_mermaid_entities_like_upstream("style a fill: #fff;"),
            "style a fill: ﬂ°fff¶ß"
        );
        assert_eq!(
            encode_mermaid_entities_like_upstream("#é; #+123; #has-dash;"),
            "#é; #+123; #has-dash;"
        );
    }

    #[test]
    fn sanitize_directive_handles_deep_values_with_small_stack() {
        const DEPTH: usize = 2_048;
        let mut value = deep_directive_value(DEPTH, Value::String("<blocked>".to_string()));

        let handle = std::thread::Builder::new()
            .name("preprocess-deep-directive-sanitize".to_string())
            .stack_size(64 * 1024)
            .spawn(move || {
                sanitize_directive(&mut value);
                assert_eq!(
                    deep_directive_leaf(&value, DEPTH).and_then(Value::as_str),
                    Some("")
                );
                crate::config::drop_value_nonrecursive(value);
            })
            .expect("spawn deep directive sanitizer test");
        handle
            .join()
            .expect("deep directive sanitizer should finish without stack overflow");
    }

    #[test]
    fn config_nesting_counts_inline_yaml_sequence_indicators() {
        let yaml = format!(
            "config:\n  {}\"leaf\"",
            "- ".repeat(MAX_CONFIG_NESTING_DEPTH + 1)
        );
        assert!(config_nesting_exceeds_limit(&yaml));
    }

    fn deep_directive_value(depth: usize, leaf: Value) -> Value {
        let mut value = leaf;
        for idx in (0..depth).rev() {
            let mut map = Map::new();
            map.insert(format!("k{idx}"), value);
            value = Value::Object(map);
        }
        value
    }

    fn deep_directive_leaf(mut value: &Value, depth: usize) -> Option<&Value> {
        for idx in 0..depth {
            value = value.as_object()?.get(&format!("k{idx}"))?;
        }
        Some(value)
    }
}
