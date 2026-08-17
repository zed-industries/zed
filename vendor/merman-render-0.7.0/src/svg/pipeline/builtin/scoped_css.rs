use crate::Result;
use std::borrow::Cow;

use super::css_override::{CssOverridePolicy, strip_css_important};
use super::util::{escape_xml_attr, find_matching_brace, find_tag_end};
use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone)]
pub struct ScopedCssPostprocessor {
    css: String,
    override_policy: CssOverridePolicy,
}

impl ScopedCssPostprocessor {
    pub fn new(css: impl Into<String>) -> Self {
        Self {
            css: css.into(),
            override_policy: CssOverridePolicy::Preserve,
        }
    }

    pub fn with_override_policy(mut self, policy: CssOverridePolicy) -> Self {
        self.override_policy = policy;
        self
    }

    pub fn css(&self) -> &str {
        &self.css
    }

    pub fn override_policy(&self) -> CssOverridePolicy {
        self.override_policy
    }
}

impl SvgPostprocessor for ScopedCssPostprocessor {
    fn name(&self) -> &'static str {
        "scoped-css"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if self.css.trim().is_empty() {
            return Ok(svg);
        }

        let mut base = match self.override_policy {
            CssOverridePolicy::Preserve => svg.into_owned(),
            CssOverridePolicy::StripExistingImportant => strip_css_important(svg.as_ref()),
        };
        let css = decode_mermaid_css_hash_placeholders(&self.css);
        let scoped_css = scope_css(css.as_ref(), ctx.svg_id());
        inject_style(&mut base, &scoped_css);
        Ok(Cow::Owned(base))
    }
}

fn inject_style(svg: &mut String, css: &str) {
    let css = css.replace("</style", "<\\/style");
    let style = format!(
        r#"<style data-merman-postprocess="scoped-css">{}</style>"#,
        css
    );

    if let Some(start) = svg.find("<svg") {
        if let Some(end) = find_tag_end(svg, start) {
            if let Some(style_close_start) = svg.rfind("</style") {
                if let Some(style_close_end) = find_tag_end(svg, style_close_start) {
                    svg.insert_str(style_close_end + 1, &style);
                    return;
                }
            }
            svg.insert_str(end + 1, &style);
            return;
        }
    }

    svg.push_str(&style);
}

fn scope_css(css: &str, svg_id: Option<&str>) -> String {
    let Some(svg_id) = svg_id.filter(|id| !id.trim().is_empty()) else {
        return css.to_string();
    };
    let scope = format!("#{}", css_escape_id(svg_id));
    scope_css_block(css, &scope)
}

fn decode_mermaid_css_hash_placeholders(css: &str) -> Cow<'_, str> {
    if !css.contains('ﬂ') && !css.contains('¶') {
        return Cow::Borrowed(css);
    }

    Cow::Owned(
        css.replace("ﬂ°°", "#")
            .replace("ﬂ°", "#")
            .replace("¶ß", ";"),
    )
}

fn scope_css_block(css: &str, scope: &str) -> String {
    let mut out = String::with_capacity(css.len() + scope.len() * 4);
    let mut cursor = 0;

    while let Some(rel_open) = css[cursor..].find('{') {
        let open = cursor + rel_open;
        let selector_start = css[cursor..open]
            .rfind(';')
            .map(|rel| cursor + rel + 1)
            .unwrap_or(cursor);
        out.push_str(&scope_css_statement_prefix(&css[cursor..selector_start]));
        let selector = &css[selector_start..open];
        let Some(close) = find_matching_brace(css, open) else {
            out.push_str(&css[cursor..]);
            return out;
        };

        if selector.trim_start().starts_with('@') {
            push_scoped_at_rule(&mut out, selector, &css[open + 1..close], scope);
        } else {
            out.push_str(&scope_selector(selector, &scope));
            out.push(' ');
            out.push_str(&css[open..=close]);
        }
        cursor = close + 1;
    }

    out.push_str(&css[cursor..]);
    out
}

fn scope_css_statement_prefix(prefix: &str) -> String {
    let trimmed = prefix.trim_start();
    if trimmed.starts_with("@import")
        || trimmed.starts_with("@namespace")
        || trimmed.starts_with("@charset")
    {
        String::new()
    } else {
        prefix.to_string()
    }
}

fn push_scoped_at_rule(out: &mut String, selector: &str, body: &str, scope: &str) {
    let name = selector
        .trim_start()
        .split(|ch: char| ch.is_whitespace() || ch == '{')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    if is_css_keyframes_rule(&name) {
        out.push_str(selector);
        out.push('{');
        out.push_str(body);
        out.push('}');
    } else if is_css_grouping_rule(&name) {
        out.push_str(selector);
        out.push('{');
        out.push_str(&scope_css_block(body, scope));
        out.push('}');
    }
}

fn is_css_keyframes_rule(name: &str) -> bool {
    name == "@keyframes" || name == "@-webkit-keyframes"
}

fn is_css_grouping_rule(name: &str) -> bool {
    matches!(
        name,
        "@media" | "@supports" | "@layer" | "@scope" | "@container" | "@starting-style"
    )
}

fn scope_selector(selector: &str, scope: &str) -> String {
    selector
        .split(',')
        .map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                String::new()
            } else if trimmed == ":root" || trimmed == "svg" {
                scope.to_string()
            } else {
                let expanded = trimmed.replace('&', scope);
                if expanded.starts_with(scope) {
                    expanded
                } else {
                    format!("{scope} {expanded}")
                }
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn css_escape_id(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for ch in id.chars() {
        let ok = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_';
        if ok {
            out.push(ch);
        } else {
            out.push('\\');
            out.push(ch);
        }
    }
    out
}

#[allow(dead_code)]
fn scoped_attr_selector(id: &str) -> String {
    format!(r#"svg[id="{}"]"#, escape_xml_attr(id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::pipeline::SvgPipeline;

    #[test]
    fn scoped_css_injects_after_root_svg_tag_when_no_style_exists() {
        let svg = r#"<svg id="diagram"><rect class="node"/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(
                ".node rect, text.label { fill: red; }",
            ))
            .process_to_string(svg)
            .unwrap();

        assert!(out.starts_with(r#"<svg id="diagram"><style"#));
        assert!(out.contains("#diagram .node rect, #diagram text.label { fill: red; }"));
    }

    #[test]
    fn scoped_css_injects_after_existing_style_for_cascade_order() {
        let svg =
            r#"<svg id="diagram"><style>#diagram .node rect { fill: red; }</style><g/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(".node rect { fill: green; }"))
            .process_to_string(svg)
            .unwrap();

        let existing = out.find("fill: red").unwrap();
        let injected = out.find("fill: green").unwrap();
        assert!(
            existing < injected,
            "injected CSS should follow Mermaid CSS for cascade order: {out}"
        );
    }

    #[test]
    fn scoped_css_can_strip_existing_important_before_injection() {
        let svg = r#"<svg id="diagram"><style>.node{fill:red !important;}</style></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(
                ScopedCssPostprocessor::new(".node { fill: green; }")
                    .with_override_policy(CssOverridePolicy::StripExistingImportant),
            )
            .process_to_string(svg)
            .unwrap();

        assert!(!out.contains("!important"));
        assert!(out.contains("#diagram .node { fill: green; }"));
    }

    #[test]
    fn scoped_css_matches_mermaid_ampersand_selector_namespace() {
        let svg = r#"<svg id="diagram"><g/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(
                ":not(&){background:green !important}",
            ))
            .process_to_string(svg)
            .unwrap();

        assert!(out.contains("#diagram :not(#diagram) {background:green !important}"));
    }

    #[test]
    fn scoped_css_scopes_nested_grouping_at_rules_and_drops_unsupported_rules() {
        let svg = r#"<svg id="diagram"><g/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(
                "@import url('https://example.test/styles.css'); @media (max-width: 600px) { * { fill: red; } } @supports selector(h2 > p) { h2 > p { color: red; } }",
            ))
            .process_to_string(svg)
            .unwrap();

        assert!(!out.contains("@import"));
        assert!(out.contains("@media (max-width: 600px) {"));
        assert!(out.contains("#diagram * { fill: red; }"));
        assert!(out.contains("@supports selector(h2 > p) {"));
        assert!(out.contains("#diagram h2 > p { color: red; }"));
    }

    #[test]
    fn scoped_css_keeps_keyframes_unscoped_like_mermaid() {
        let svg = r#"<svg id="diagram"><g/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(
                "@keyframes dash { to { stroke-dashoffset: 1000; } } .edge { animation: dash 1s; }",
            ))
            .process_to_string(svg)
            .unwrap();

        assert!(out.contains("@keyframes dash { to { stroke-dashoffset: 1000; } }"));
        assert!(out.contains("#diagram .edge { animation: dash 1s; }"));
    }

    #[test]
    fn scoped_css_decodes_mermaid_hash_placeholders_as_css_hashes() {
        let svg = r#"<svg id="diagram"><g/></svg>"#;
        let out = SvgPipeline::parity()
            .with_postprocessor(ScopedCssPostprocessor::new(".node { fill: ﬂ°°123456¶ß }"))
            .process_to_string(svg)
            .unwrap();

        assert!(out.contains("#diagram .node { fill: #123456; }"));
    }
}
