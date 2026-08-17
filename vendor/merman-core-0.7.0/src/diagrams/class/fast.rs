use crate::{Error, ParseMetadata, Result};

use super::ast::{Action, Relation, RelationData};
use super::db::ClassDb;
use super::{
    LINE_DOTTED, LINE_SOLID, MERMAID_DOM_ID_PREFIX, REL_AGGREGATION, REL_COMPOSITION,
    REL_DEPENDENCY, REL_EXTENSION, REL_LOLLIPOP, REL_NONE,
};

pub(super) fn parse_class_fast_db<'a>(
    code: &str,
    meta: &'a ParseMetadata,
) -> Result<Option<ClassDb<'a>>> {
    fn parse_quoted_str(rest: &str) -> Option<(String, &str)> {
        let rest = rest.trim_start();
        if !rest.starts_with('"') {
            return None;
        }
        let inner = &rest[1..];
        let end = inner.find('"')?;
        let s = inner[..end].to_string();
        Some((s, &inner[end + 1..]))
    }

    fn parse_name(rest: &str) -> Option<(String, &str)> {
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }

        if rest.as_bytes()[0] == b'`' {
            let inner = &rest[1..];
            let (name, after) = if let Some(end) = inner.find('`') {
                (&inner[..end], &inner[end + 1..])
            } else {
                (inner, "")
            };
            let name = if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                format!("{MERMAID_DOM_ID_PREFIX}{name}")
            } else {
                name.to_string()
            };
            return Some((name, after));
        }

        let bytes = rest.as_bytes();
        let mut end = 0usize;
        while end < rest.len() {
            let b = bytes[end];
            if b.is_ascii_whitespace()
                || b == b'\n'
                || b == b'{'
                || b == b'}'
                || b == b'['
                || b == b']'
                || b == b'"'
                || b == b','
                || b == b':'
                || b == b'<'
                || b == b'>'
            {
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
        if end == 0 {
            return None;
        }
        let mut name = rest[..end].to_string();
        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            name = format!("{MERMAID_DOM_ID_PREFIX}{name}");
        }
        Some((name, &rest[end..]))
    }

    fn parse_relation_tokens(rest: &str) -> Option<(Relation, &str)> {
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }

        fn parse_relation_type(rest: &str) -> (i32, &str) {
            let rest = rest.trim_start();
            if let Some(stripped) = rest.strip_prefix("<|") {
                return (REL_EXTENSION, stripped);
            }
            if let Some(stripped) = rest.strip_prefix("|>") {
                return (REL_EXTENSION, stripped);
            }
            if let Some(stripped) = rest.strip_prefix("()") {
                return (REL_LOLLIPOP, stripped);
            }
            if let Some(stripped) = rest.strip_prefix('*') {
                return (REL_COMPOSITION, stripped);
            }
            if let Some(stripped) = rest.strip_prefix('o') {
                return (REL_AGGREGATION, stripped);
            }
            if rest.starts_with('<') || rest.starts_with('>') {
                return (REL_DEPENDENCY, &rest[1..]);
            }
            (REL_NONE, rest)
        }

        let (type1, after_t1) = parse_relation_type(rest);
        let after_t1 = after_t1.trim_start();

        let (line_type, after_line) = if let Some(stripped) = after_t1.strip_prefix("--") {
            (LINE_SOLID, stripped)
        } else if let Some(stripped) = after_t1.strip_prefix("..") {
            (LINE_DOTTED, stripped)
        } else {
            return None;
        };

        let (type2, after_t2) = parse_relation_type(after_line);
        Some((
            Relation {
                type1,
                type2,
                line_type,
            },
            after_t2,
        ))
    }

    let mut db = ClassDb::new(&meta.effective_config);
    let mut saw_header = false;
    let mut current_class: Option<String> = None;

    for raw in code.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("%%") {
            continue;
        }

        if !saw_header {
            if line.starts_with("classDiagram") {
                saw_header = true;
                continue;
            }
            return Ok(None);
        }

        if let Some(class_id) = current_class.as_deref() {
            if line == "}" {
                current_class = None;
                continue;
            }
            db.add_member(class_id, line);
            continue;
        }

        if let Some(stripped) = line.strip_prefix("direction") {
            let rest = stripped.trim_start();
            let dir = rest.split_whitespace().next().unwrap_or_default().trim();
            if matches!(dir, "TB" | "BT" | "LR" | "RL") {
                db.set_direction(dir);
                continue;
            }
            return Ok(None);
        }

        if line.starts_with("class ") || line == "class" || line.starts_with("class\t") {
            let mut rest = &line["class".len()..];
            let Some((class_id, after_id)) = parse_name(rest) else {
                return Ok(None);
            };
            rest = after_id.trim_start();

            // Optional label: ["..."]
            let mut label: Option<String> = None;
            if rest.starts_with('[') {
                let after = rest[1..].trim_start();
                let Some((lab, after_lab)) = parse_quoted_str(after) else {
                    return Ok(None);
                };
                let after_lab = after_lab.trim_start();
                if !after_lab.starts_with(']') {
                    return Ok(None);
                }
                label = Some(lab);
                rest = after_lab[1..].trim_start();
            }

            // Optional css shorthand: :::name
            let mut css: Option<String> = None;
            if rest.starts_with(":::") {
                let after = &rest[3..];
                let Some((css_name, after_css)) = parse_name(after) else {
                    return Ok(None);
                };
                css = Some(css_name);
                rest = after_css.trim_start();
            }

            let mut has_body = false;
            if rest.starts_with('{') {
                has_body = true;
                rest = rest[1..].trim_start();
                if !rest.is_empty() {
                    return Ok(None);
                }
            }
            if !rest.is_empty() {
                return Ok(None);
            }

            db.add_class(&class_id);
            if let Some(lab) = label {
                db.set_class_label(&class_id, &lab);
            }
            if let Some(css) = css {
                db.set_css_class(&class_id, &css);
            }
            if has_body {
                current_class = Some(class_id);
            }
            continue;
        }

        // Relation statement (optionally with label).
        if let Some((a, rest)) = parse_name(line) {
            let mut rest = rest.trim_start();
            let (t1, after_t1) = if let Some((t1, after)) = parse_quoted_str(rest) {
                (Some(t1), after)
            } else {
                (None, rest)
            };
            rest = after_t1.trim_start();

            let Some((relation, after_rel)) = parse_relation_tokens(rest) else {
                return Ok(None);
            };
            rest = after_rel.trim_start();

            let (t2, after_t2) = if let Some((t2, after)) = parse_quoted_str(rest) {
                (Some(t2), after)
            } else {
                (None, rest)
            };
            rest = after_t2.trim_start();

            let Some((b, after_b)) = parse_name(rest) else {
                return Ok(None);
            };
            let after_b = after_b.trim_start();

            let label = if after_b.starts_with(':') && !after_b.starts_with(":::") {
                Some(after_b.to_string())
            } else if after_b.is_empty() {
                None
            } else {
                return Ok(None);
            };

            let data = RelationData {
                id1: a,
                id2: b,
                relation,
                relation_title1: t1,
                relation_title2: t2,
                title: label,
            };
            // Mirror the grammar path (Action::AddRelation + optional Label) via `apply`.
            db.apply(Action::AddRelation { data })
                .map_err(|e| Error::DiagramParse {
                    diagram_type: meta.diagram_type.clone(),
                    message: e,
                })?;
            continue;
        }

        return Ok(None);
    }

    if !saw_header {
        return Ok(None);
    }
    if current_class.is_some() {
        return Ok(None);
    }

    Ok(Some(db))
}
