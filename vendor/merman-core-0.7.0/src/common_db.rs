use crate::MermaidConfig;
use crate::sanitize::sanitize_text;
use serde_json::{Map, Value};

fn strip_leading_whitespace(s: &str) -> String {
    s.trim_start_matches(char::is_whitespace).to_string()
}

fn collapse_newline_whitespace(s: &str) -> String {
    // Mermaid's commonDb.ts: `sanitizeText(txt).replace(/\n\s+/g, '\n')`
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        out.push(ch);
        if ch == '\n' {
            while chars.peek().is_some_and(|c| c.is_whitespace()) {
                chars.next();
            }
        }
    }
    out
}

fn sanitize_string_field<F>(obj: &mut Map<String, Value>, key: &'static str, transform: F)
where
    F: FnOnce(&str) -> String,
{
    let Some(v) = obj.get_mut(key) else {
        return;
    };
    let Value::String(s) = v else {
        return;
    };
    *s = transform(s);
}

pub fn apply_common_db_sanitization(model: &mut Value, config: &MermaidConfig) {
    let Value::Object(obj) = model else {
        return;
    };

    sanitize_string_field(obj, "title", |s| sanitize_text(s, config));
    sanitize_string_field(obj, "accTitle", |s| sanitize_acc_title(s, config));
    sanitize_string_field(obj, "accDescr", |s| sanitize_acc_descr(s, config));
}

pub(crate) fn sanitize_acc_title(s: &str, config: &MermaidConfig) -> String {
    strip_leading_whitespace(&sanitize_text(s, config))
}

pub(crate) fn sanitize_acc_descr(s: &str, config: &MermaidConfig) -> String {
    collapse_newline_whitespace(&sanitize_text(s, config))
}

pub(crate) fn sanitize_optional_title(value: &mut Option<String>, config: &MermaidConfig) {
    if let Some(s) = value.as_deref() {
        *value = Some(sanitize_text(s, config));
    }
}

pub(crate) fn sanitize_optional_acc_title(value: &mut Option<String>, config: &MermaidConfig) {
    if let Some(s) = value.as_deref() {
        *value = Some(sanitize_acc_title(s, config));
    }
}

pub(crate) fn sanitize_optional_acc_descr(value: &mut Option<String>, config: &MermaidConfig) {
    if let Some(s) = value.as_deref() {
        *value = Some(sanitize_acc_descr(s, config));
    }
}
