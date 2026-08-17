use crate::MermaidConfig;
use crate::generated::dompurify_defaults;
use lol_html::{RewriteStrSettings, element, rewrite_str};
use std::collections::HashSet;
use std::sync::OnceLock;

fn break_to_placeholder(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = input[probe..].find('<') {
        let start = probe + rel_start;
        let Some(end) = mermaid_line_break_tag_end(input, start) else {
            probe = start + 1;
            continue;
        };

        out.push_str(&input[cursor..start]);
        out.push_str("#br#");
        cursor = end;
        probe = end;
    }

    out.push_str(&input[cursor..]);
    out
}

fn placeholder_to_break(input: &str) -> String {
    input.replace("#br#", "<br/>")
}

fn mermaid_line_break_tag_end(input: &str, start: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.get(start) != Some(&b'<')
        || !bytes
            .get(start + 1)
            .is_some_and(|b| b.eq_ignore_ascii_case(&b'b'))
        || !bytes
            .get(start + 2)
            .is_some_and(|b| b.eq_ignore_ascii_case(&b'r'))
    {
        return None;
    }

    let mut cursor = start + 3;
    while cursor < input.len() {
        let ch = input[cursor..].chars().next()?;
        if !is_js_regex_whitespace(ch) {
            break;
        }
        cursor += ch.len_utf8();
    }

    if bytes.get(cursor) == Some(&b'/') {
        cursor += 1;
    }

    (bytes.get(cursor) == Some(&b'>')).then_some(cursor + 1)
}

fn is_js_regex_whitespace(ch: char) -> bool {
    if ('\u{2000}'..='\u{200A}').contains(&ch) {
        return true;
    }

    matches!(
        ch,
        '\u{0009}'
            | '\u{000A}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{000D}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

fn default_allowed_tags() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        dompurify_defaults::DEFAULT_ALLOWED_TAGS
            .iter()
            .copied()
            .collect()
    })
}

fn default_allowed_attr() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        dompurify_defaults::DEFAULT_ALLOWED_ATTR
            .iter()
            .copied()
            .collect()
    })
}

fn default_uri_safe_attr() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        dompurify_defaults::DEFAULT_URI_SAFE_ATTRIBUTES
            .iter()
            .copied()
            .collect()
    })
}

fn default_data_uri_tags() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        dompurify_defaults::DEFAULT_DATA_URI_TAGS
            .iter()
            .copied()
            .collect()
    })
}

fn is_dompurify_data_attr_name(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("data-") else {
        return false;
    };

    !rest.is_empty() && rest.chars().all(is_dompurify_data_attr_suffix_char)
}

fn is_dompurify_data_attr_suffix_char(ch: char) -> bool {
    // Source: DOMPurify 3.4.0 `DATA_ATTR = /^data-[\-\w.\u00B7-\uFFFF]+$/`.
    matches!(
        ch,
        '-' | '.' | '_' | '0'..='9' | 'A'..='Z' | 'a'..='z'
    ) || ('\u{00B7}'..='\u{FFFF}').contains(&ch)
}

fn is_dompurify_aria_attr_name(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("aria-") else {
        return false;
    };

    !rest.is_empty() && rest.chars().all(is_dompurify_aria_attr_suffix_char)
}

fn is_dompurify_aria_attr_suffix_char(ch: char) -> bool {
    // Source: DOMPurify 3.4.0 `ARIA_ATTR = /^aria-[\-\w]+$/`.
    matches!(ch, '-' | '_' | '0'..='9' | 'A'..='Z' | 'a'..='z')
}

fn remove_dompurify_attr_whitespace(input: &str) -> std::borrow::Cow<'_, str> {
    let Some(first) = input
        .char_indices()
        .find_map(|(idx, ch)| is_dompurify_attr_whitespace(ch).then_some(idx))
    else {
        return std::borrow::Cow::Borrowed(input);
    };

    let mut out = String::with_capacity(input.len());
    out.push_str(&input[..first]);
    out.extend(
        input[first..]
            .chars()
            .filter(|ch| !is_dompurify_attr_whitespace(*ch)),
    );
    std::borrow::Cow::Owned(out)
}

fn is_dompurify_attr_whitespace(ch: char) -> bool {
    // Source: DOMPurify 3.4.0 `ATTR_WHITESPACE`.
    matches!(
        ch,
        '\u{0000}'..='\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{180E}'
            | '\u{2000}'..='\u{2029}'
            | '\u{205F}'
            | '\u{3000}'
    )
}

fn is_dompurify_script_or_data_uri(value: &str) -> bool {
    // Source: DOMPurify 3.4.0 `IS_SCRIPT_OR_DATA = /^(?:\w+script|data):/i`.
    let Some(colon) = value.find(':') else {
        return false;
    };

    let scheme = &value[..colon];
    if scheme.eq_ignore_ascii_case("data") {
        return true;
    }

    let bytes = scheme.as_bytes();
    let script = b"script";
    if bytes.len() <= script.len()
        || !bytes[bytes.len() - script.len()..].eq_ignore_ascii_case(script)
    {
        return false;
    }

    bytes[..bytes.len() - script.len()]
        .iter()
        .all(|byte| is_js_regex_word_byte(*byte))
}

fn is_dompurify_allowed_uri(value: &str) -> bool {
    // Source: DOMPurify 3.4.0 `IS_ALLOWED_URI`.
    if value.is_empty() {
        return false;
    }

    if has_dompurify_allowed_uri_scheme(value) {
        return true;
    }

    let bytes = value.as_bytes();
    if !bytes[0].is_ascii_alphabetic() {
        return true;
    }

    let mut cursor = 0usize;
    while bytes
        .get(cursor)
        .is_some_and(|byte| is_dompurify_uri_scheme_byte(*byte))
    {
        cursor += 1;
    }

    cursor == bytes.len()
        || bytes
            .get(cursor)
            .is_some_and(|byte| !is_dompurify_uri_scheme_byte(*byte) && *byte != b':')
}

fn has_dompurify_allowed_uri_scheme(value: &str) -> bool {
    let bytes = value.as_bytes();
    const ALLOWED_URI_SCHEMES: &[&[u8]] = &[
        b"http:", b"https:", b"ftp:", b"ftps:", b"mailto:", b"tel:", b"callto:", b"sms:", b"cid:",
        b"xmpp:", b"matrix:",
    ];

    ALLOWED_URI_SCHEMES
        .iter()
        .any(|scheme| ascii_case_insensitive_starts_with(bytes, 0, scheme))
}

fn is_dompurify_uri_scheme_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'+' | b'.' | b'-')
}

fn is_js_regex_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[derive(Debug, Clone)]
struct DompurifyEffectiveConfig {
    allowed_tags: HashSet<String>,
    allowed_attr: HashSet<String>,
    uri_safe_attr: HashSet<String>,
    data_uri_tags: HashSet<String>,
    forbid_tags: HashSet<String>,
    forbid_attr: HashSet<String>,
    allow_aria_attr: bool,
    allow_data_attr: bool,
    allow_unknown_protocols: bool,
    keep_content: bool,
}

fn dompurify_config_object(
    config: &MermaidConfig,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    config
        .as_value()
        .as_object()
        .and_then(|o| o.get("dompurifyConfig"))
        .and_then(|v| v.as_object())
}

fn dompurify_extract_string_list(
    dompurify_config: Option<&serde_json::Map<String, serde_json::Value>>,
    key: &str,
) -> Vec<String> {
    dompurify_config
        .and_then(|o| o.get(key))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .map(|s| s.to_ascii_lowercase())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn dompurify_effective_config(
    config: &MermaidConfig,
    forbid_style_when_unconfigured: bool,
) -> DompurifyEffectiveConfig {
    let dompurify_cfg = dompurify_config_object(config);

    let allow_aria_attr = dompurify_cfg
        .and_then(|o| o.get("ALLOW_ARIA_ATTR"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let allow_data_attr = dompurify_cfg
        .and_then(|o| o.get("ALLOW_DATA_ATTR"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let allow_unknown_protocols = dompurify_cfg
        .and_then(|o| o.get("ALLOW_UNKNOWN_PROTOCOLS"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let keep_content = dompurify_cfg
        .and_then(|o| o.get("KEEP_CONTENT"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let mut allowed_tags: HashSet<String> = if dompurify_cfg
        .and_then(|o| o.get("ALLOWED_TAGS"))
        .and_then(|v| v.as_array())
        .is_some()
    {
        dompurify_extract_string_list(dompurify_cfg, "ALLOWED_TAGS")
            .into_iter()
            .collect()
    } else {
        default_allowed_tags()
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    for t in dompurify_extract_string_list(dompurify_cfg, "ADD_TAGS") {
        allowed_tags.insert(t);
    }

    if allowed_tags.contains("table") {
        allowed_tags.insert("tbody".to_string());
    }

    let mut allowed_attr: HashSet<String> = if dompurify_cfg
        .and_then(|o| o.get("ALLOWED_ATTR"))
        .and_then(|v| v.as_array())
        .is_some()
    {
        dompurify_extract_string_list(dompurify_cfg, "ALLOWED_ATTR")
            .into_iter()
            .collect()
    } else {
        default_allowed_attr()
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    for a in dompurify_extract_string_list(dompurify_cfg, "ADD_ATTR") {
        allowed_attr.insert(a);
    }

    let mut uri_safe_attr: HashSet<String> = default_uri_safe_attr()
        .iter()
        .map(|s| s.to_string())
        .collect();
    for a in dompurify_extract_string_list(dompurify_cfg, "ADD_URI_SAFE_ATTR") {
        uri_safe_attr.insert(a);
    }

    let mut data_uri_tags: HashSet<String> = default_data_uri_tags()
        .iter()
        .map(|s| s.to_string())
        .collect();
    for t in dompurify_extract_string_list(dompurify_cfg, "ADD_DATA_URI_TAGS") {
        data_uri_tags.insert(t);
    }

    let mut forbid_tags: HashSet<String> =
        dompurify_extract_string_list(dompurify_cfg, "FORBID_TAGS")
            .into_iter()
            .collect();

    if forbid_style_when_unconfigured && dompurify_cfg.is_none() {
        forbid_tags.insert("style".to_string());
    }

    let forbid_attr: HashSet<String> = dompurify_extract_string_list(dompurify_cfg, "FORBID_ATTR")
        .into_iter()
        .collect();

    DompurifyEffectiveConfig {
        allowed_tags,
        allowed_attr,
        uri_safe_attr,
        data_uri_tags,
        forbid_tags,
        forbid_attr,
        allow_aria_attr,
        allow_data_attr,
        allow_unknown_protocols,
        keep_content,
    }
}

fn dompurify_is_valid_attribute(
    cfg: &DompurifyEffectiveConfig,
    lc_tag: &str,
    lc_name: &str,
    value: &str,
) -> bool {
    if cfg.allow_data_attr
        && !cfg.forbid_attr.contains(lc_name)
        && is_dompurify_data_attr_name(lc_name)
    {
        return true;
    }

    if cfg.allow_aria_attr && is_dompurify_aria_attr_name(lc_name) {
        return true;
    }

    if !cfg.allowed_attr.contains(lc_name) || cfg.forbid_attr.contains(lc_name) {
        return false;
    }

    if cfg.uri_safe_attr.contains(lc_name) {
        return true;
    }

    let decoded_value = decode_attr_html_entities_minimally(value);
    let value_no_ws = remove_dompurify_attr_whitespace(&decoded_value);

    if is_dompurify_allowed_uri(value_no_ws.as_ref()) {
        return true;
    }

    if matches!(lc_name, "src" | "xlink:href" | "href")
        && lc_tag != "script"
        && decoded_value.starts_with("data:")
        && cfg.data_uri_tags.contains(lc_tag)
    {
        return true;
    }

    if cfg.allow_unknown_protocols && !is_dompurify_script_or_data_uri(value_no_ws.as_ref()) {
        return true;
    }

    value.is_empty()
}

fn decode_attr_html_entities_minimally(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut out = replace_ascii_case_insensitive_literal(input, "&colon;", ":");
    out = replace_ascii_case_insensitive_literal(&out, "&newline;", "\n");
    out = replace_ascii_case_insensitive_literal(&out, "&tab;", "\t");
    out = replace_decimal_colon_entity_like_current_regex(&out);
    out = replace_hex_colon_entity_like_current_regex(&out);
    out
}

fn replace_ascii_case_insensitive_literal(input: &str, needle: &str, replacement: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let needle = needle.as_bytes();
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = input[probe..].find('&') {
        let start = probe + rel_start;
        if ascii_case_insensitive_starts_with(bytes, start, needle) {
            out.push_str(&input[cursor..start]);
            out.push_str(replacement);
            cursor = start + needle.len();
            probe = cursor;
        } else {
            probe = start + 1;
        }
    }

    out.push_str(&input[cursor..]);
    out
}

fn ascii_case_insensitive_starts_with(haystack: &[u8], start: usize, needle: &[u8]) -> bool {
    haystack
        .get(start..start + needle.len())
        .is_some_and(|candidate| {
            candidate
                .iter()
                .zip(needle)
                .all(|(a, b)| a.eq_ignore_ascii_case(b))
        })
}

fn replace_decimal_colon_entity_like_current_regex(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = input[probe..].find("&#") {
        let start = probe + rel_start;
        let mut end = start + 2;
        while bytes.get(end) == Some(&b'0') {
            end += 1;
        }

        if bytes.get(end..end + 2) == Some(b"58") {
            end += 2;
            if bytes.get(end) == Some(&b';') {
                end += 1;
            }
            out.push_str(&input[cursor..start]);
            out.push(':');
            cursor = end;
            probe = end;
        } else {
            probe = start + 1;
        }
    }

    out.push_str(&input[cursor..]);
    out
}

fn replace_hex_colon_entity_like_current_regex(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    let mut probe = 0usize;

    while let Some(rel_start) = input[probe..].find("&#") {
        let start = probe + rel_start;
        let mut end = start + 2;
        if !bytes
            .get(end)
            .is_some_and(|b| b.eq_ignore_ascii_case(&b'x'))
        {
            probe = start + 1;
            continue;
        }

        end += 1;
        while bytes.get(end) == Some(&b'0') {
            end += 1;
        }

        let is_colon_hex = bytes.get(end) == Some(&b'3')
            && bytes
                .get(end + 1)
                .is_some_and(|b| b.eq_ignore_ascii_case(&b'a'));
        if is_colon_hex {
            end += 2;
            if bytes.get(end) == Some(&b';') {
                end += 1;
            }
            out.push_str(&input[cursor..start]);
            out.push(':');
            cursor = end;
            probe = end;
        } else {
            probe = start + 1;
        }
    }

    out.push_str(&input[cursor..]);
    out
}

fn dompurify_like_sanitize_html(text: &str, cfg: &DompurifyEffectiveConfig) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    // `lol_html::rewrite_str` is less permissive than browser parsing (and therefore DOMPurify).
    // In particular, fragments containing a "stray" `<` that does not start a valid tag (e.g.
    // `"foo < bar"` or `"<\""` in a quoted string) can fail to parse. In browsers those `<`
    // tokens are treated as text and serialized as `&lt;`.
    //
    // To stay closer to Mermaid's `sanitizeText` behavior, pre-escape such `<` tokens before
    // running the DOMPurify-like rewrite.
    fn escape_stray_lt(input: &str) -> std::borrow::Cow<'_, str> {
        let bytes = input.as_bytes();
        let mut pos = 0usize;
        while pos < bytes.len() {
            if bytes[pos] == b'<' {
                let next = bytes.get(pos + 1).copied().unwrap_or(b' ');
                let tag_start = next.is_ascii_alphabetic() || matches!(next, b'/' | b'!' | b'?');
                if !tag_start {
                    break;
                }
            }
            pos += 1;
        }
        if pos >= bytes.len() {
            return std::borrow::Cow::Borrowed(input);
        }

        let mut out = String::with_capacity(input.len() + 8);
        let mut last = 0usize;
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'<' {
                let next = bytes.get(i + 1).copied().unwrap_or(b' ');
                let tag_start = next.is_ascii_alphabetic() || matches!(next, b'/' | b'!' | b'?');
                if !tag_start {
                    out.push_str(&input[last..i]);
                    out.push_str("&lt;");
                    i += 1;
                    last = i;
                    continue;
                }
            }
            i += 1;
        }
        out.push_str(&input[last..]);
        std::borrow::Cow::Owned(out)
    }

    let text = escape_stray_lt(text);

    let mut handlers = vec![
        element!("script", |el| {
            el.remove();
            Ok(())
        }),
        element!("iframe", |el| {
            el.remove();
            Ok(())
        }),
        element!("style", |el| {
            el.remove();
            Ok(())
        }),
    ];

    handlers.push(element!("a", |el| {
        // Mirror Mermaid's DOMPurify hooks:
        // - beforeSanitizeAttributes stores the target in a temporary data-* attribute
        // - afterSanitizeAttributes restores it (only if the data-* survived sanitization)
        if let Some(target) = el.get_attribute("target") {
            let _ = el.set_attribute("data-temp-href-target", &target);
        }
        Ok(())
    }));

    handlers.push(element!("*", |el| {
        let tag_name = el.tag_name();
        let lc_tag = tag_name.to_ascii_lowercase();

        if !cfg.allowed_tags.contains(&lc_tag) || cfg.forbid_tags.contains(&lc_tag) {
            if cfg.keep_content {
                el.remove_and_keep_content();
            } else {
                el.remove();
            }
            return Ok(());
        }

        let attrs: Vec<(String, String)> = el
            .attributes()
            .iter()
            .map(|a| (a.name().to_string(), a.value().to_string()))
            .collect();

        for (name, value) in attrs {
            let lc_name = name.to_ascii_lowercase();
            if !dompurify_is_valid_attribute(cfg, &lc_tag, &lc_name, &value) {
                el.remove_attribute(&name);
                continue;
            }

            if matches!(lc_name.as_str(), "href" | "src" | "xlink:href") {
                // DOMPurify validates URI values on parsed DOM values (entities already decoded).
                // `lol_html` gives us raw values, so we decode the minimal subset Mermaid relies on.
                let decoded = decode_attr_html_entities_minimally(&value);
                if decoded != value {
                    let _ = el.set_attribute(&name, &decoded);
                }
            }
        }

        if lc_tag == "a" {
            if let Some(target) = el.get_attribute("data-temp-href-target") {
                let _ = el.set_attribute("target", &target);
                el.remove_attribute("data-temp-href-target");
                if target == "_blank" {
                    let _ = el.set_attribute("rel", "noopener");
                }
            }
        }

        Ok(())
    }));

    rewrite_str(
        text.as_ref(),
        RewriteStrSettings {
            element_content_handlers: handlers,
            ..RewriteStrSettings::new()
        },
    )
    .unwrap_or_else(|_| text.into_owned())
}

pub fn remove_script(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }
    if !text.contains('<') {
        return text.to_string();
    }
    let cfg = dompurify_effective_config(
        &MermaidConfig::from_value(serde_json::Value::Object(serde_json::Map::new())),
        false,
    );
    dompurify_like_sanitize_html(text, &cfg)
}

fn sanitize_more(text: &str, config: &MermaidConfig) -> String {
    let html_labels_enabled = config.get_bool("flowchart.htmlLabels") != Some(false);
    if !html_labels_enabled {
        return text.to_string();
    }

    let level = config.get_str("securityLevel");
    if matches!(level, Some("antiscript" | "strict" | "sandbox")) {
        return remove_script(text);
    }

    if level != Some("loose") {
        let mut t = break_to_placeholder(text);
        t = t.replace('<', "&lt;").replace('>', "&gt;");
        t = t.replace('=', "&#61;");
        return placeholder_to_break(&t);
    }

    text.to_string()
}

pub fn sanitize_text(text: &str, config: &MermaidConfig) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let t = sanitize_more(text, config);
    if !t.contains('<') {
        return t;
    }
    let cfg = dompurify_effective_config(config, true);
    dompurify_like_sanitize_html(&t, &cfg)
}

pub fn sanitize_text_or_array(
    value: &serde_json::Value,
    config: &MermaidConfig,
) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => serde_json::Value::String(sanitize_text(s, config)),
        serde_json::Value::Array(arr) => serde_json::Value::Array(
            arr.iter()
                .flat_map(|v| match v {
                    serde_json::Value::Array(inner) => inner.to_vec(),
                    _ => vec![v.clone()],
                })
                .map(|v| match v {
                    serde_json::Value::String(s) => {
                        serde_json::Value::String(sanitize_text(&s, config))
                    }
                    other => other,
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cfg_strict() -> MermaidConfig {
        MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }))
    }

    #[test]
    fn break_to_placeholder_matches_mermaid_line_break_regex_shape() {
        assert_eq!(
            break_to_placeholder("A<br>B<BR/>C<br \t/>D<br   >E"),
            "A#br#B#br#C#br#D#br#E"
        );
        assert_eq!(
            break_to_placeholder("<br / > <brx> </br> < br>"),
            "<br / > <brx> </br> < br>"
        );
        assert_eq!(
            break_to_placeholder("A<br\u{00A0}/>B<br\u{FEFF}>C"),
            "A#br#B#br#C"
        );
    }

    #[test]
    fn decode_attr_entities_matches_minimal_dompurify_url_subset_without_regex() {
        assert_eq!(
            decode_attr_html_entities_minimally("javascript&colon;alert&NEWLINE;one&TAB;two"),
            "javascript:alert\none\ttwo"
        );
        assert_eq!(
            decode_attr_html_entities_minimally("a&#58;b&#00058;c&#058d"),
            "a:b:c:d"
        );
        assert_eq!(
            decode_attr_html_entities_minimally("a&#x3a;b&#X0003A;c&#x03adef"),
            "a:b:c:def"
        );
        assert_eq!(
            decode_attr_html_entities_minimally("&colon &newline &tab &#59; &#x3b;"),
            "&colon &newline &tab &#59; &#x3b;"
        );
    }

    #[test]
    fn dompurify_attr_name_matchers_follow_source_regex_boundaries() {
        assert!(is_dompurify_data_attr_name("data-x"));
        assert!(is_dompurify_data_attr_name("data-x.y_9-"));
        assert!(is_dompurify_data_attr_name("data-\u{00B7}"));
        assert!(is_dompurify_data_attr_name("data-\u{FFFF}"));
        assert!(!is_dompurify_data_attr_name("data-"));
        assert!(!is_dompurify_data_attr_name("data-\u{00B6}"));
        assert!(!is_dompurify_data_attr_name("data-x:y"));
        assert!(!is_dompurify_data_attr_name("data-\u{10000}"));

        assert!(is_dompurify_aria_attr_name("aria-label"));
        assert!(is_dompurify_aria_attr_name("aria-foo_bar"));
        assert!(!is_dompurify_aria_attr_name("aria-"));
        assert!(!is_dompurify_aria_attr_name("aria.label"));
        assert!(!is_dompurify_aria_attr_name("aria-\u{00B7}"));
    }

    #[test]
    fn dompurify_attr_whitespace_cleanup_matches_source_regex_boundaries() {
        assert_eq!(
            remove_dompurify_attr_whitespace(
                "java\u{0000}\u{0020}\u{00A0}\u{1680}\u{180E}\u{2000}\u{2029}\u{205F}\u{3000}script:"
            ),
            "javascript:"
        );
        assert_eq!(
            remove_dompurify_attr_whitespace("java\u{0021}script:"),
            "java\u{0021}script:"
        );
        assert_eq!(
            remove_dompurify_attr_whitespace("java\u{202A}script:"),
            "java\u{202A}script:"
        );
        assert_eq!(
            remove_dompurify_attr_whitespace("java\u{FEFF}script:"),
            "java\u{FEFF}script:"
        );
    }

    #[test]
    fn dompurify_script_or_data_uri_matches_source_regex_boundaries() {
        assert!(is_dompurify_script_or_data_uri("javascript:alert(1)"));
        assert!(is_dompurify_script_or_data_uri("JavaSCRIPT:alert(1)"));
        assert!(is_dompurify_script_or_data_uri("vbscript:alert(1)"));
        assert!(is_dompurify_script_or_data_uri("_script:alert(1)"));
        assert!(is_dompurify_script_or_data_uri("1script:alert(1)"));
        assert!(is_dompurify_script_or_data_uri("data:text/html,alert(1)"));
        assert!(is_dompurify_script_or_data_uri("DATA:text/html,alert(1)"));
        assert!(!is_dompurify_script_or_data_uri("script:alert(1)"));
        assert!(!is_dompurify_script_or_data_uri("java-script:alert(1)"));
        assert!(!is_dompurify_script_or_data_uri(
            "jav\u{00E1}script:alert(1)"
        ));
        assert!(!is_dompurify_script_or_data_uri("datax:text/html,alert(1)"));
        assert!(!is_dompurify_script_or_data_uri("javascript"));
    }

    #[test]
    fn dompurify_allowed_uri_matches_source_regex_boundaries() {
        for uri in [
            "http://example.test",
            "https://example.test",
            "ftp://example.test",
            "ftps://example.test",
            "mailto:user@example.test",
            "tel:+123",
            "callto:user",
            "sms:+123",
            "cid:content-id",
            "xmpp:user@example.test",
            "matrix:r/example:example.test",
            "MATRIX:r/example:example.test",
            "/relative",
            "#fragment",
            "?query",
            "1-relative",
            ":colon-relative",
            "abc",
            "abc/path",
            "abc?query",
            "abc123:allowed-by-source-prefix",
            "abc_def:allowed-by-source-prefix",
        ] {
            assert!(is_dompurify_allowed_uri(uri), "{uri}");
        }

        for uri in [
            "",
            "javascript:alert(1)",
            "data:text/html,1",
            "foo:bar",
            "abc+def:bar",
            "abc.def:bar",
            "abc-def:bar",
        ] {
            assert!(!is_dompurify_allowed_uri(uri), "{uri}");
        }
    }

    #[test]
    fn remove_script_strips_script_blocks_and_javascript_urls_and_events() {
        let label_string = r#"1
		Act1: Hello 1<script src="http://abc.com/script1.js"></script>1
		<b>Act2</b>:
		1<script>
			alert('script run......');
		</script>1
	1"#;

        let exactly_string = r#"1
		Act1: Hello 11
		<b>Act2</b>:
		11
	1"#;

        assert_eq!(remove_script(label_string).trim(), exactly_string);

        let url_in = r#"This is a <a href="javascript:runHijackingScript();">clean link</a> + <a href="javascript:runHijackingScript();">clean link</a>
  and <a href="javascript&colon;bypassedMining();">me too</a>"#;
        let url_out = r#"This is a <a>clean link</a> + <a>clean link</a>
  and <a>me too</a>"#;
        assert_eq!(remove_script(url_in).trim(), url_out);

        assert_eq!(
            remove_script(r#"<img onerror="alert('hello');">"#).trim(),
            "<img>"
        );
    }

    #[test]
    fn remove_script_decodes_colon_entities_before_url_validation_without_regex() {
        assert_eq!(
            remove_script(
                r#"<a href="javascript&#58;alert(1)">decimal</a><a href="javascript&#x3A;alert(1)">hex</a>"#
            ),
            "<a>decimal</a><a>hex</a>"
        );
    }

    #[test]
    fn remove_script_preserves_target_and_adds_noopener_for_blank() {
        assert_eq!(
            remove_script(
                r#"<a href="https://mermaid.js.org/" target="_blank">note about mermaid</a>"#
            )
            .trim(),
            r#"<a href="https://mermaid.js.org/" target="_blank" rel="noopener">note about mermaid</a>"#
        );

        assert_eq!(
            remove_script(
                r#"<a href="https://mermaid.js.org/" target="_self">note about mermaid</a>"#
            )
            .trim(),
            r#"<a href="https://mermaid.js.org/" target="_self">note about mermaid</a>"#
        );
    }

    #[test]
    fn remove_script_removes_iframes() {
        let out = remove_script(
            r#"<iframe src="http://abc.com/script1.js"></iframe>
    <iframe src="http://example.com/iframeexample"></iframe>"#,
        );
        assert_eq!(out.trim(), "");
    }

    #[test]
    fn sanitize_text_strict_runs_remove_script_and_forbids_style() {
        let cfg = cfg_strict();
        assert_eq!(
            sanitize_text(r#"<style>.x{color:red}</style><b>ok</b>"#, &cfg),
            "<b>ok</b>"
        );
        assert!(
            !sanitize_text("javajavascript:script:alert(1)", &cfg).contains("javascript:alert(1)")
        );
    }

    #[test]
    fn sanitize_text_matches_mermaid_common_spec_minimally() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }));
        let malicious = "javajavascript:script:alert(1)";
        let out = sanitize_text(malicious, &cfg);
        assert!(!out.contains("javascript:alert(1)"));
    }

    #[test]
    fn sanitize_text_preserves_mermaid_line_break_tags_without_regex() {
        let cfg = MermaidConfig::from_value(json!({
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text("A<br \t/>B<BR>C", &cfg);
        assert!(out.contains("A<br"));
        assert!(out.contains(">B<br"));
        assert!(out.ends_with(">C"));
        assert!(!out.contains("&lt;br"));
    }

    #[test]
    fn sanitize_text_sandbox_runs_remove_script_like_mermaid() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "sandbox",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(r#"<b a=1>ok</b><br/>x"#, &cfg);
        assert!(out.contains("<b"));
        assert!(out.contains("ok"));
        assert!(out.contains("<br"));
        assert!(!out.contains("&lt;"));
        assert!(!out.contains("&equals;"));
    }

    #[test]
    fn sanitize_text_dompurify_config_add_attr_allows_onclick_like_dompurify() {
        // Mermaid supports passing `dompurifyConfig` through to DOMPurify.
        // Reference: `repo-ref/mermaid/packages/mermaid/src/diagrams/common/common.ts`.
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "ADD_ATTR": ["onclick"] }
        }));
        assert_eq!(
            sanitize_text(r#"<b onclick="alert(1)">ok</b>"#, &cfg),
            r#"<b onclick="alert(1)">ok</b>"#
        );
    }

    #[test]
    fn sanitize_text_dompurify_config_forbid_attr_removes_href_like_dompurify() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "FORBID_ATTR": ["href"] }
        }));
        assert_eq!(sanitize_text(r#"<a href="/x">y</a>"#, &cfg), "<a>y</a>");
    }

    #[test]
    fn sanitize_text_dompurify_defaults_strip_unknown_attribute_and_keep_style_attr() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true }
        }));
        assert_eq!(sanitize_text(r#"<b foo="bar">ok</b>"#, &cfg), "<b>ok</b>");
        assert_eq!(
            sanitize_text(r#"<b style="color:red">ok</b>"#, &cfg),
            r#"<b style="color:red">ok</b>"#
        );
    }

    #[test]
    fn sanitize_text_dompurify_defaults_remove_unknown_tag_keep_content() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true }
        }));
        assert_eq!(
            sanitize_text(r#"<custom-tag onclick="alert(1)">x</custom-tag>"#, &cfg),
            "x"
        );
    }

    #[test]
    fn sanitize_text_dompurify_defaults_allow_aria_and_data_attrs() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(
            r#"<b data-x="1" data-x.y_9-="2" aria-label="x" aria-foo_bar="y" data-x:y="bad" aria.foo="bad" foo="bar">ok</b>"#,
            &cfg,
        );
        assert!(!out.contains("foo="));
        assert!(!out.contains("data-x:y="));
        assert!(!out.contains("aria.foo="));
        assert!(out.contains(r#"data-x="1""#));
        assert!(out.contains(r#"data-x.y_9-="2""#));
        assert!(out.contains(r#"aria-label="x""#));
        assert!(out.contains(r#"aria-foo_bar="y""#));
        assert!(out.starts_with("<b"));
        assert!(out.ends_with(">ok</b>"));

        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "ALLOW_DATA_ATTR": false, "ALLOW_ARIA_ATTR": false }
        }));
        assert_eq!(
            sanitize_text(
                r#"<b data-x="1" data-x.y_9-="2" aria-label="x">ok</b>"#,
                &cfg
            ),
            "<b>ok</b>"
        );
    }

    #[test]
    fn sanitize_text_allows_svg_elements_inside_svg_container() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(
            r#"<svg><path fill="currentColor" d="M224 0c-17.7 0-32 14.3-32 32v19.2"/></svg>"#,
            &cfg,
        );
        assert!(out.contains("<svg"));
        assert!(out.contains("<path"));
        assert!(out.contains("fill=\"currentColor\""));
        assert!(out.contains("d=\"M224 0c-17.7"));
    }

    #[test]
    fn sanitize_text_strips_javascript_xlink_href_in_svg() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(
            r#"<svg><a xlink:href="javascript:alert(1)">x</a></svg>"#,
            &cfg,
        );
        assert!(out.contains("<svg"));
        assert!(out.contains("<a"));
        assert!(out.contains(">x</a>"));
        assert!(!out.to_ascii_lowercase().contains("javascript:"));
        assert!(!out.to_ascii_lowercase().contains("xlink:href"));
    }

    #[test]
    fn sanitize_text_strips_javascript_href_after_dompurify_attr_whitespace_cleanup() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text("<a href=\"java\u{00A0}script:alert(1)\">x</a>", &cfg);
        assert!(out.contains("<a"));
        assert!(out.contains(">x</a>"));
        assert!(!out.to_ascii_lowercase().contains("javascript:"));
        assert!(!out.to_ascii_lowercase().contains("href="));
    }

    #[test]
    fn sanitize_text_dompurify_allowed_uri_matches_pinned_source_schemes() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(
            r#"<a href="matrix:r/example:example.test">matrix</a><a href="foo:bar">foo</a>"#,
            &cfg,
        );
        assert!(out.contains(r#"href="matrix:r/example:example.test""#));
        assert!(out.contains(">matrix</a>"));
        assert!(out.contains(">foo</a>"));
        assert!(!out.contains(r#"href="foo:bar""#));
    }

    #[test]
    fn sanitize_text_allow_unknown_protocols_still_blocks_script_or_data_uri() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "ALLOW_UNKNOWN_PROTOCOLS": true }
        }));
        let out = sanitize_text(
            r#"<a href="foo:bar">ok</a><a href="javascript:alert(1)">bad</a><a href="data:text/html,1">data</a>"#,
            &cfg,
        );
        assert!(out.contains(r#"href="foo:bar""#));
        assert!(out.contains(">ok</a>"));
        assert!(out.contains(">bad</a>"));
        assert!(out.contains(">data</a>"));
        assert!(!out.to_ascii_lowercase().contains("javascript:"));
        assert!(!out.to_ascii_lowercase().contains("data:text/html"));
    }

    #[test]
    fn sanitize_text_dompurify_hook_target_depends_on_allow_data_attr() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true }
        }));
        let out = sanitize_text(
            r#"<a href="https://mermaid.js.org/" target="_blank">x</a>"#,
            &cfg,
        );
        assert!(out.contains("target=\"_blank\""));
        assert!(out.contains("rel=\"noopener\""));

        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "strict",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "ALLOW_DATA_ATTR": false }
        }));
        let out = sanitize_text(
            r#"<a href="https://mermaid.js.org/" target="_blank">x</a>"#,
            &cfg,
        );
        assert!(!out.contains("target=\"_blank\""));
        // In Mermaid strict mode, the `removeScript()` pass adds `rel=noopener` before the second
        // DOMPurify pass runs, so `rel` can remain even if the target is later removed.
        assert!(out.contains("rel=\"noopener\""));
    }

    #[test]
    fn sanitize_text_dompurify_keep_content_false_removes_custom_element_content() {
        let cfg = MermaidConfig::from_value(json!({
            "securityLevel": "loose",
            "flowchart": { "htmlLabels": true },
            "dompurifyConfig": { "KEEP_CONTENT": false }
        }));
        assert_eq!(sanitize_text("<custom-tag>x</custom-tag>", &cfg), "");
    }
}
