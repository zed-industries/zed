use crate::Result;
use crate::entities::decode_entities_minimal;
use crate::svg::foreign_object_label_fallback_svg_text;
use std::borrow::Cow;
use std::collections::HashSet;

use super::util::{extract_quoted_attr, find_tag_end};
use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone, Copy, Default)]
pub struct ForeignObjectFallbackPostprocessor;

impl SvgPostprocessor for ForeignObjectFallbackPostprocessor {
    fn name(&self) -> &'static str {
        "foreign-object-fallback"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if !svg.contains("<foreignObject") {
            return Ok(svg);
        }
        Ok(Cow::Owned(foreign_object_fallback_svg(&svg)))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StripForeignObjectPostprocessor;

impl SvgPostprocessor for StripForeignObjectPostprocessor {
    fn name(&self) -> &'static str {
        "strip-foreign-object"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if !svg.contains("<foreignObject") {
            return Ok(svg);
        }
        Ok(Cow::Owned(strip_foreign_objects(&svg)))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DropNativeDuplicateFallbacksPostprocessor;

impl SvgPostprocessor for DropNativeDuplicateFallbacksPostprocessor {
    fn name(&self) -> &'static str {
        "drop-native-duplicate-fallbacks"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if !svg.contains(r#"data-merman-foreignobject="fallback""#) {
            return Ok(svg);
        }
        Ok(Cow::Owned(drop_native_duplicate_fallbacks(&svg)))
    }
}

pub(crate) fn foreign_object_fallback_svg(svg: &str) -> String {
    foreign_object_label_fallback_svg_text(svg)
}

pub(crate) fn strip_foreign_objects(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len());
    let mut cursor = 0;

    while let Some(rel_start) = svg[cursor..].find("<foreignObject") {
        let start = cursor + rel_start;
        out.push_str(&svg[cursor..start]);

        let Some(open_end) = find_tag_end(svg, start) else {
            out.push_str(&svg[start..]);
            return out;
        };

        if svg[start..=open_end].trim_end().ends_with("/>") {
            cursor = open_end + 1;
            continue;
        }

        let close_start = open_end + 1;
        let Some(rel_close) = svg[close_start..].find("</foreignObject>") else {
            cursor = open_end + 1;
            continue;
        };
        cursor = close_start + rel_close + "</foreignObject>".len();
    }

    out.push_str(&svg[cursor..]);
    out
}

pub fn drop_native_duplicate_fallbacks(svg: &str) -> String {
    let native_text = collect_native_text_contents(svg);
    if native_text.is_empty() {
        return svg.to_string();
    }

    let mut out = String::with_capacity(svg.len());
    let mut cursor = 0;
    while let Some(rel_start) = svg[cursor..].find(r#"data-merman-foreignobject="fallback""#) {
        let attr_start = cursor + rel_start;
        let Some(group_start) = svg[..attr_start].rfind("<g") else {
            out.push_str(&svg[cursor..attr_start]);
            cursor = attr_start;
            continue;
        };
        if group_start < cursor {
            out.push_str(&svg[cursor..attr_start]);
            cursor = attr_start;
            continue;
        }
        let Some((close_start, group_end)) = find_matching_g_end(svg, group_start) else {
            out.push_str(&svg[cursor..attr_start]);
            cursor = attr_start;
            continue;
        };
        let Some(open_end) = find_tag_end(svg, group_start) else {
            out.push_str(&svg[cursor..attr_start]);
            cursor = attr_start;
            continue;
        };

        let fallback_text = normalize_text_content(&svg[open_end + 1..close_start]);
        if native_text.contains(fallback_text.trim()) {
            out.push_str(&svg[cursor..group_start]);
        } else {
            out.push_str(&svg[cursor..group_end]);
        }
        cursor = group_end;
    }

    out.push_str(&svg[cursor..]);
    out
}

fn collect_native_text_contents(svg: &str) -> HashSet<String> {
    let mut contents = HashSet::new();
    let mut cursor = 0;
    while let Some(rel_start) = svg[cursor..].find("<text") {
        let start = cursor + rel_start;
        let Some(open_end) = find_tag_end(svg, start) else {
            break;
        };
        let tag = &svg[start..=open_end];
        if text_tag_is_fallback(tag) || tag.trim_end().ends_with("/>") {
            cursor = open_end + 1;
            continue;
        }

        let close_start = open_end + 1;
        let Some(rel_close) = svg[close_start..].find("</text>") else {
            cursor = open_end + 1;
            continue;
        };
        let close = close_start + rel_close;
        let text = normalize_text_content(&svg[close_start..close]);
        if !text.is_empty() {
            contents.insert(text);
        }
        cursor = close + "</text>".len();
    }
    contents
}

fn text_tag_is_fallback(tag: &str) -> bool {
    extract_quoted_attr(tag, "class").is_some_and(|classes| {
        classes
            .split_whitespace()
            .any(|class| class == "merman-foreignobject-fallback-text")
    })
}

fn normalize_text_content(fragment: &str) -> String {
    decode_entities_minimal(&strip_tags(fragment))
        .trim()
        .to_string()
}

fn strip_tags(fragment: &str) -> String {
    let mut out = String::with_capacity(fragment.len());
    let mut in_tag = false;
    for ch in fragment.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn find_matching_g_end(svg: &str, group_start: usize) -> Option<(usize, usize)> {
    let open_end = find_tag_end(svg, group_start)?;
    if svg[group_start..=open_end].trim_end().ends_with("/>") {
        return Some((group_start, open_end + 1));
    }

    let mut depth = 1usize;
    let mut cursor = open_end + 1;
    while let Some(rel_tag) = svg[cursor..].find('<') {
        let tag_start = cursor + rel_tag;
        let Some(tag_end) = find_tag_end(svg, tag_start) else {
            break;
        };
        let tag = &svg[tag_start..=tag_end];
        if is_start_g_tag(tag) {
            if !tag.trim_end().ends_with("/>") {
                depth += 1;
            }
        } else if is_end_g_tag(tag) {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some((tag_start, tag_end + 1));
            }
        }
        cursor = tag_end + 1;
    }
    None
}

fn is_start_g_tag(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    tag.starts_with("<g")
        && bytes
            .get(2)
            .is_some_and(|b| b.is_ascii_whitespace() || *b == b'>' || *b == b'/')
}

fn is_end_g_tag(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    tag.starts_with("</g")
        && bytes
            .get(3)
            .is_some_and(|b| b.is_ascii_whitespace() || *b == b'>')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::pipeline::SvgPipeline;

    #[test]
    fn drop_native_duplicate_fallbacks_removes_only_matching_fallback_groups() {
        let svg = r##"<svg>
<text class="task">Make tea</text>
<g data-merman-foreignobject="fallback" class="dup">
  <rect/>
  <text class="merman-foreignobject-fallback-text">Make tea</text>
</g>
<g data-merman-foreignobject="fallback" class="keep">
  <text class="merman-foreignobject-fallback-text">Only fallback</text>
</g>
</svg>"##;

        let out = drop_native_duplicate_fallbacks(svg);

        assert!(out.contains(r#"<text class="task">Make tea</text>"#));
        assert!(!out.contains(r#"class="dup""#));
        assert!(out.contains(r#"class="keep""#));
        assert!(out.contains("Only fallback"));
    }

    #[test]
    fn resvg_safe_can_optionally_drop_native_duplicate_fallbacks() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg">
<text class="task">Make tea</text>
<g transform="translate(0,0)">
  <foreignObject width="80" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>Make tea</p></div></foreignObject>
</g>
<g transform="translate(0,40)">
  <foreignObject width="80" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>Only fallback</p></div></foreignObject>
</g>
</svg>"##;

        let out = SvgPipeline::resvg_safe()
            .with_postprocessor(DropNativeDuplicateFallbacksPostprocessor)
            .process_to_string(svg)
            .unwrap();

        assert!(!out.contains("<foreignObject"));
        assert_eq!(
            out.matches(r#"data-merman-foreignobject="fallback""#)
                .count(),
            1,
            "{out}"
        );
        assert!(out.contains("Only fallback"));
        assert!(out.contains(r#"<text class="task">Make tea</text>"#));
    }
}
