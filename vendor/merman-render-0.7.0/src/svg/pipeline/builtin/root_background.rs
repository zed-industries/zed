use crate::Result;
use std::borrow::Cow;

use super::util::{escape_xml_attr, find_tag_end};
use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootBackgroundPostprocessor {
    background_color: String,
}

impl RootBackgroundPostprocessor {
    pub fn new(background_color: impl Into<String>) -> Self {
        Self {
            background_color: background_color.into(),
        }
    }

    pub fn background_color(&self) -> &str {
        &self.background_color
    }
}

impl SvgPostprocessor for RootBackgroundPostprocessor {
    fn name(&self) -> &'static str {
        "root-background"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        let background_color = self.background_color.trim();
        if background_color.is_empty() || !svg.contains("<svg") {
            return Ok(svg);
        }

        Ok(Cow::Owned(set_root_background_color(
            svg.as_ref(),
            background_color,
        )))
    }
}

pub(crate) fn set_root_background_color(svg: &str, background_color: &str) -> String {
    let Some(svg_start) = svg.find("<svg") else {
        return svg.to_string();
    };
    let Some(svg_end) = find_tag_end(svg, svg_start) else {
        return svg.to_string();
    };

    let tag = &svg[svg_start..=svg_end];
    let escaped_color = escape_xml_attr(background_color.trim());

    if let Some((style_value_start, style_value_end)) = find_quoted_attr_value_span(tag, "style") {
        let style = &tag[style_value_start..style_value_end];
        let rewritten = set_background_in_style_attr(style, &escaped_color);
        let absolute_value_start = svg_start + style_value_start;
        let absolute_value_end = svg_start + style_value_end;

        let mut out =
            String::with_capacity(svg.len() + rewritten.len().saturating_sub(style.len()));
        out.push_str(&svg[..absolute_value_start]);
        out.push_str(&rewritten);
        out.push_str(&svg[absolute_value_end..]);
        return out;
    }

    let insert_at = if svg.as_bytes().get(svg_end.saturating_sub(1)) == Some(&b'/') {
        svg_end - 1
    } else {
        svg_end
    };

    let mut out = String::with_capacity(svg.len() + escaped_color.len() + 34);
    out.push_str(&svg[..insert_at]);
    out.push_str(r#" style="background-color: "#);
    out.push_str(&escaped_color);
    out.push_str(r#";""#);
    out.push_str(&svg[insert_at..]);
    out
}

fn set_background_in_style_attr(style: &str, background_color: &str) -> String {
    let mut declarations = Vec::new();
    let mut replaced = false;

    for declaration in style.split(';') {
        let trimmed = declaration.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some((property, _value)) = trimmed.split_once(':') else {
            declarations.push(trimmed.to_string());
            continue;
        };

        if property.trim().eq_ignore_ascii_case("background-color") {
            if !replaced {
                declarations.push(format!("background-color: {background_color}"));
                replaced = true;
            }
        } else {
            declarations.push(trimmed.to_string());
        }
    }

    if !replaced {
        declarations.push(format!("background-color: {background_color}"));
    }

    format!("{};", declarations.join("; "))
}

fn find_quoted_attr_value_span(tag: &str, name: &str) -> Option<(usize, usize)> {
    let bytes = tag.as_bytes();
    let mut cursor = 0usize;

    while cursor < bytes.len() {
        let rel = tag[cursor..].find(name)?;
        let name_start = cursor + rel;
        let name_end = name_start + name.len();

        let before_ok = name_start == 0
            || bytes[name_start - 1].is_ascii_whitespace()
            || bytes[name_start - 1] == b'<';
        let after_ok = name_end == bytes.len()
            || bytes[name_end].is_ascii_whitespace()
            || bytes[name_end] == b'=';
        if !before_ok || !after_ok {
            cursor = name_end;
            continue;
        }

        let mut i = name_end;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if bytes.get(i) != Some(&b'=') {
            cursor = name_end;
            continue;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let quote = *bytes.get(i)?;
        if quote != b'"' && quote != b'\'' {
            cursor = name_end;
            continue;
        }
        let value_start = i + 1;
        let rel_end = tag[value_start..].find(quote as char)?;
        return Some((value_start, value_start + rel_end));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::pipeline::SvgPipeline;

    #[test]
    fn root_background_rewrites_existing_background_color() {
        let svg =
            r#"<svg id="diagram" style="max-width: 400px; background-color: white;"><g/></svg>"#;

        let out = SvgPipeline::parity()
            .with_postprocessor(RootBackgroundPostprocessor::new("#111827"))
            .process_to_string(svg)
            .unwrap();

        assert_eq!(
            out,
            r#"<svg id="diagram" style="max-width: 400px; background-color: #111827;"><g/></svg>"#
        );
    }

    #[test]
    fn root_background_adds_missing_style_property() {
        let svg = r#"<svg id="diagram" width="100%"><g/></svg>"#;

        let out = SvgPipeline::parity()
            .with_postprocessor(RootBackgroundPostprocessor::new("transparent"))
            .process_to_string(svg)
            .unwrap();

        assert_eq!(
            out,
            r#"<svg id="diagram" width="100%" style="background-color: transparent;"><g/></svg>"#
        );
    }

    #[test]
    fn root_background_escapes_xml_attribute_value() {
        let svg = r#"<svg id="diagram" style="max-width: 400px;"><g/></svg>"#;

        let out = set_root_background_color(svg, "rgb(1, 2, 3)&");

        assert!(out.contains("background-color: rgb(1, 2, 3)&amp;;"));
    }
}
