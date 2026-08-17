use crate::Result;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};

use super::util::find_tag_end;
use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone, Copy, Default)]
pub struct GitGraphBranchLabelBaselinePostprocessor;

impl SvgPostprocessor for GitGraphBranchLabelBaselinePostprocessor {
    fn name(&self) -> &'static str {
        "gitgraph-branch-label-baseline"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if ctx.diagram_type() != Some("gitGraph") || !svg.contains("branch-label") {
            return Ok(svg);
        }

        let adjusted = center_gitgraph_branch_label_text(svg.as_ref());
        if adjusted == svg.as_ref() {
            Ok(svg)
        } else {
            Ok(Cow::Owned(adjusted))
        }
    }
}

fn center_gitgraph_branch_label_text(svg: &str) -> String {
    let mut centers = collect_branch_label_centers(svg);
    if centers.is_empty() {
        return svg.to_string();
    }

    let mut out = String::with_capacity(svg.len());
    let mut scan_cursor = 0usize;
    let mut copied_until = 0usize;
    let mut changed = false;

    while let Some(rel_start) = svg[scan_cursor..].find("<g") {
        let start = scan_cursor + rel_start;
        let Some(open_end) = find_tag_end(svg, start) else {
            break;
        };
        let tag = &svg[start..=open_end];
        let Some(label_index) = branch_label_index_from_group_tag(tag) else {
            scan_cursor = open_end + 1;
            continue;
        };
        let Some(center_y) = centers.get_mut(&label_index).and_then(VecDeque::pop_front) else {
            scan_cursor = open_end + 1;
            continue;
        };

        let group_y = extract_attr(tag, "transform")
            .and_then(parse_translate_y)
            .unwrap_or(0.0);
        let text_y = center_y - group_y;
        let Some(group_close) = svg[open_end + 1..]
            .find("</g>")
            .map(|rel| open_end + 1 + rel)
        else {
            scan_cursor = open_end + 1;
            continue;
        };
        let Some((text_start, text_end)) = find_first_text_span(svg, open_end + 1) else {
            scan_cursor = open_end + 1;
            continue;
        };
        if text_start > group_close {
            scan_cursor = open_end + 1;
            continue;
        };

        let rewritten =
            rewrite_text_block_for_centered_baseline(&svg[text_start..text_end], text_y);
        if rewritten == svg[text_start..text_end] {
            scan_cursor = text_end;
            continue;
        }

        out.push_str(&svg[copied_until..text_start]);
        out.push_str(&rewritten);
        copied_until = text_end;
        scan_cursor = text_end;
        changed = true;
    }

    if !changed {
        return svg.to_string();
    }

    out.push_str(&svg[copied_until..]);
    out
}

fn collect_branch_label_centers(svg: &str) -> HashMap<usize, VecDeque<f64>> {
    let mut centers = HashMap::new();
    let mut cursor = 0usize;

    while let Some(rel_start) = svg[cursor..].find("<rect") {
        let start = cursor + rel_start;
        let Some(end) = find_tag_end(svg, start) else {
            break;
        };
        let tag = &svg[start..=end];
        if let Some(index) = branch_label_index_from_rect_tag(tag) {
            if let (Some(y), Some(height)) = (
                extract_attr(tag, "y").and_then(parse_f64),
                extract_attr(tag, "height").and_then(parse_f64),
            ) {
                let translate_y = extract_attr(tag, "transform")
                    .and_then(parse_translate_y)
                    .unwrap_or(0.0);
                centers
                    .entry(index)
                    .or_insert_with(VecDeque::new)
                    .push_back(translate_y + y + height / 2.0);
            }
        }
        cursor = end + 1;
    }

    centers
}

fn branch_label_index_from_rect_tag(tag: &str) -> Option<usize> {
    let classes = extract_attr(tag, "class")?;
    if !classes
        .split_whitespace()
        .any(|class| class == "branchLabelBkg")
    {
        return None;
    }
    label_class_index(classes)
}

fn branch_label_index_from_group_tag(tag: &str) -> Option<usize> {
    let classes = extract_attr(tag, "class")?;
    if !classes.split_whitespace().any(|class| class == "label") {
        return None;
    }
    classes
        .split_whitespace()
        .find_map(|class| class.strip_prefix("branch-label"))
        .and_then(|value| value.parse::<usize>().ok())
}

fn label_class_index(classes: &str) -> Option<usize> {
    classes
        .split_whitespace()
        .find_map(|class| class.strip_prefix("label"))
        .and_then(|value| value.parse::<usize>().ok())
}

fn find_first_text_span(svg: &str, start: usize) -> Option<(usize, usize)> {
    let rel_text = svg[start..].find("<text")?;
    let text_start = start + rel_text;
    let text_open_end = find_tag_end(svg, text_start)?;
    let content_start = text_open_end + 1;
    let rel_close = svg[content_start..].find("</text>")?;
    let close_end = content_start + rel_close + "</text>".len();
    Some((text_start, close_end))
}

fn rewrite_text_block_for_centered_baseline(text_block: &str, text_y: f64) -> String {
    let Some(open_end) = find_tag_end(text_block, 0) else {
        return text_block.to_string();
    };
    let open_tag = &text_block[..=open_end];
    let close_start = text_block.rfind("</text>").unwrap_or(text_block.len());
    let content = &text_block[open_end + 1..close_start];
    let close = &text_block[close_start..];

    let open_tag = set_or_insert_attr(open_tag, "y", &fmt_number(text_y));
    let open_tag = set_or_insert_attr(&open_tag, "dominant-baseline", "central");
    let open_tag = set_or_insert_attr(&open_tag, "alignment-baseline", "central");
    let content = rewrite_first_tspan_dy_zero(content);

    format!("{open_tag}{content}{close}")
}

fn rewrite_first_tspan_dy_zero(content: &str) -> String {
    let Some(rel_tspan) = content.find("<tspan") else {
        return content.to_string();
    };
    let Some(tspan_end) = find_tag_end(content, rel_tspan) else {
        return content.to_string();
    };
    let tag = &content[rel_tspan..=tspan_end];
    let rewritten_tag = set_or_insert_attr(tag, "dy", "0");
    if rewritten_tag == tag {
        return content.to_string();
    }

    let mut out = String::with_capacity(content.len() + rewritten_tag.len() - tag.len());
    out.push_str(&content[..rel_tspan]);
    out.push_str(&rewritten_tag);
    out.push_str(&content[tspan_end + 1..]);
    out
}

fn set_or_insert_attr(tag: &str, name: &str, value: &str) -> String {
    if let Some((value_start, value_end)) = find_attr_value_span(tag, name) {
        let mut out = String::with_capacity(tag.len() + value.len());
        out.push_str(&tag[..value_start]);
        out.push_str(value);
        out.push_str(&tag[value_end..]);
        return out;
    }

    let insert_at = tag
        .trim_end()
        .strip_suffix("/>")
        .map(|prefix| prefix.len())
        .unwrap_or_else(|| tag.rfind('>').unwrap_or(tag.len()));
    let mut out = String::with_capacity(tag.len() + name.len() + value.len() + 4);
    out.push_str(&tag[..insert_at]);
    out.push(' ');
    out.push_str(name);
    out.push_str(r#"=""#);
    out.push_str(value);
    out.push('"');
    out.push_str(&tag[insert_at..]);
    out
}

fn extract_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let (start, end) = find_attr_value_span(tag, name)?;
    Some(&tag[start..end])
}

fn find_attr_value_span(tag: &str, name: &str) -> Option<(usize, usize)> {
    let needle = format!(r#" {name}=""#);
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')?;
    Some((start, start + end))
}

fn parse_translate_y(transform: &str) -> Option<f64> {
    let lower = transform.to_ascii_lowercase();
    let start = lower.find("translate(")? + "translate(".len();
    let end = transform[start..].find(')')? + start;
    let args = &transform[start..end];
    let mut parts = args
        .split(|ch: char| ch == ',' || ch.is_ascii_whitespace())
        .filter(|part| !part.trim().is_empty());
    let _x = parts.next()?;
    parts.next().and_then(parse_f64).or(Some(0.0))
}

fn parse_f64(raw: &str) -> Option<f64> {
    raw.trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

fn fmt_number(value: f64) -> String {
    let value = if value.abs() < 1e-9 { 0.0 } else { value };
    if (value.fract()).abs() < 1e-9 {
        return format!("{}", value as i64);
    }

    let mut out = format!("{value:.6}");
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::pipeline::{SvgPipeline, SvgPostprocessMetadata};

    #[test]
    fn gitgraph_branch_label_text_uses_rect_center_baseline() {
        let svg = r#"<svg id="g"><g><rect class="branchLabelBkg label0" rx="4" ry="4" x="-69" y="-1.5" width="53" height="21" transform="translate(-19, -8.5)"/><g class="branchLabel"><g class="label branch-label0" transform="translate(-79, -9.5)"><text><tspan xml:space="preserve" dy="1em" x="0" class="row">main</tspan></text></g></g></g></svg>"#;
        let metadata = SvgPostprocessMetadata::from_svg(svg).with_diagram_type("gitGraph");

        let out = SvgPipeline::parity()
            .with_postprocessor(GitGraphBranchLabelBaselinePostprocessor)
            .process_to_string_with_metadata(svg, &metadata)
            .unwrap();

        assert!(out.contains(r#"<text"#), "{out}");
        assert!(out.contains(r#"y="10""#), "{out}");
        assert!(out.contains(r#"dominant-baseline="central""#), "{out}");
        assert!(out.contains(r#"alignment-baseline="central""#), "{out}");
        assert!(
            out.contains(r#"<tspan xml:space="preserve" dy="0" x="0" class="row">main</tspan>"#)
        );
    }

    #[test]
    fn gitgraph_branch_label_postprocessor_ignores_other_diagrams() {
        let svg = r#"<svg id="g"><g><rect class="branchLabelBkg label0" y="-1.5" height="21"/><g class="label branch-label0"><text><tspan dy="1em">main</tspan></text></g></g></svg>"#;
        let metadata = SvgPostprocessMetadata::from_svg(svg).with_diagram_type("flowchart");

        let out = SvgPipeline::parity()
            .with_postprocessor(GitGraphBranchLabelBaselinePostprocessor)
            .process_to_string_with_metadata(svg, &metadata)
            .unwrap();

        assert_eq!(out, svg);
    }
}
