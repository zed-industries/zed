//! Builds a theme-aware CSS stylesheet and appends it into the SVG's `<style>`
//! element. All selectors are scoped to the SVG's `id` to prevent leaking.
//!
//! ```xml
//! <!-- before -->
//! <style>.node rect { fill: white; }</style>
//!
//! <!-- after -->
//! <style>.node rect { fill: white; }
//! #mermaid-1 .node rect { fill: #89b4fa; }
//! /* ... theme rules ... */
//! </style>
//! ```

use std::collections::VecDeque;
use std::fmt::Write;

use anyhow::Result;
use quick_xml::events::{BytesText, Event};

use crate::MermaidTheme;

/// Morally equivalent to `format!(".section-{i}")`, but without allocating
const MINDMAP_SECTION_SELECTORS: [&str; 11] = [
    ".section-0",
    ".section-1",
    ".section-2",
    ".section-3",
    ".section-4",
    ".section-5",
    ".section-6",
    ".section-7",
    ".section-8",
    ".section-9",
    ".section-10",
];

struct InjectCss<'a, I> {
    inner: I,
    injected_css: String,
    in_style: bool,
    injected: bool,
    pending: VecDeque<Event<'a>>,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for InjectCss<'a, I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.pending.pop_front() {
            return Some(Ok(event));
        }

        let event = match self.inner.next()? {
            Ok(ev) => ev,
            Err(e) => return Some(Err(e)),
        };

        match &event {
            Event::Start(e) if e.name().as_ref() == b"style" => {
                self.in_style = true;
                return Some(Ok(event));
            }
            Event::End(e) if e.name().as_ref() == b"style" => {
                self.in_style = false;
                if !self.injected {
                    self.injected = true;
                    self.pending
                        .push_back(Event::Text(BytesText::from_escaped(std::mem::take(
                            &mut self.injected_css,
                        ))));
                    self.pending.push_back(event);
                    return self.pending.pop_front().map(Ok);
                }
                return Some(Ok(event));
            }
            Event::Text(text) if self.in_style => {
                self.injected = true;
                let existing = match std::str::from_utf8(text.as_ref()) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e.into())),
                };
                let mut combined = String::with_capacity(existing.len() + self.injected_css.len());
                combined.push_str(existing);
                combined.push_str(&self.injected_css);
                return Some(Ok(Event::Text(BytesText::from_escaped(combined))));
            }
            _ => {}
        }

        Some(Ok(event))
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
    theme: &MermaidTheme,
    svg_id: &str,
) -> impl Iterator<Item = Result<Event<'a>>> {
    let injected_css = build_injected_css(theme, svg_id);
    InjectCss {
        inner: events,
        injected_css,
        in_style: false,
        injected: false,
        pending: VecDeque::new(),
    }
}

fn mindmap_section_css(theme: &MermaidTheme) -> String {
    let colors: [String; 8] = theme.git_branch_colors.map(crate::css_color);
    let fills: [String; 8] = theme.git_branch_colors.map(|c| {
        crate::css_color(blend_over_background(
            c,
            theme.background,
            ACCENT_FILL_OPACITY,
        ))
    });
    let text = crate::css_color(theme.text_color);
    let mut css = String::with_capacity(5_400);

    let emit = |css: &mut String, selector: &str, color: &str, fill: &str, txt: &str| {
        let section_index = selector
            .trim_start_matches(".section-root.section-")
            .trim_start_matches(".section-");
        write!(
            css,
            "{selector} rect, {selector} path, {selector} circle, {selector} polygon \
             {{ fill: {fill}; stroke: {color}; }}\n\
             {selector} text, {selector} span, \
             text{selector}, tspan{selector} \
             {{ fill: {txt}; color: {txt}; }}\n\
             {selector} foreignObject div, {selector} foreignObject span, {selector} foreignObject p \
             {{ color: {txt}; }}\n\
             .section-edge{section_index} {{ stroke: {color}; }}\n",
        )
        .expect("write to String cannot fail");
    };

    emit(
        &mut css,
        ".section-root.section--1",
        &colors[0],
        &fills[0],
        &text,
    );
    emit(&mut css, ".section--1", &colors[1], &fills[1], &text);
    for (i, selector) in MINDMAP_SECTION_SELECTORS.iter().enumerate() {
        let ci = 2 + (i % 6);
        emit(&mut css, selector, &colors[ci], &fills[ci], &text);
    }
    css
}

fn git_branch_css(theme: &MermaidTheme) -> String {
    let text = crate::css_color(theme.text_color);
    let mut css = String::with_capacity(8 * 200);
    for i in 0..8 {
        let c = crate::css_color(theme.git_branch_colors[i]);
        let label_fill = crate::css_color(blend_over_background(
            theme.git_branch_colors[i],
            theme.background,
            ACCENT_FILL_OPACITY,
        ));
        write!(
            css,
            ".commit{i} {{ stroke: {c}; fill: {c}; }}\n\
             .arrow{i} {{ stroke: {c}; }}\n\
             .label{i} {{ fill: {label_fill}; stroke: {c}; }}\n\
             .branch-label{i} {{ fill: {text}; }}\n"
        )
        .expect("write to String cannot fail");
    }
    css
}

fn adjust_lightness(color: &mut gpui::Hsla, dark_mode: bool) {
    if dark_mode {
        color.l = (color.l * 0.7).max(0.0);
    } else {
        color.l = (color.l * 1.3).min(1.0);
    }
}

const ACCENT_FILL_OPACITY: f32 = 0.15;

fn blend_over_background(
    foreground: gpui::Hsla,
    background: gpui::Hsla,
    opacity: f32,
) -> gpui::Hsla {
    let fg = gpui::Rgba::from(foreground);
    let bg = gpui::Rgba::from(background);
    let blended = gpui::Rgba {
        r: fg.r * opacity + bg.r * (1.0 - opacity),
        g: fg.g * opacity + bg.g * (1.0 - opacity),
        b: fg.b * opacity + bg.b * (1.0 - opacity),
        a: 1.0,
    };
    gpui::Hsla::from(blended)
}

fn accent_css(theme: &MermaidTheme) -> String {
    let mut css = String::with_capacity(theme.accent_colors.len() * 420);
    let text = crate::css_color(theme.text_color);

    for (i, accent) in theme.accent_colors.iter().enumerate() {
        let stroke = crate::css_color(accent.foreground);
        let fill = crate::css_color(blend_over_background(
            accent.background,
            theme.background,
            ACCENT_FILL_OPACITY,
        ));
        let class = format!(".zed-accent-{i}");
        write!(
            css,
            "{class} rect, {class} path, {class} circle, {class} polygon, {class} ellipse, \
             rect{class}, path{class}, circle{class}, polygon{class}, ellipse{class} \
             {{ fill: {fill}; stroke: {stroke}; }}\n\
             {class} text, {class} tspan, text{class}, tspan{class} \
             {{ fill: {text}; }}\n",
        )
        .expect("write to String cannot fail");
    }
    css
}

fn chart_color_css(theme: &MermaidTheme) -> String {
    // Each block is around 230 bytes, add some headroom
    let mut css = String::with_capacity(8 * 250);
    for i in 0..8 {
        let color = crate::css_color(theme.git_branch_colors[i]);
        let class = format!(".zed-chart-{i}");
        write!(
            css,
            "path.pieCircle{class} {{ fill: {color}; }}\n\
             .plot rect{class}, .legend rect{class} {{ fill: {color}; stroke: {color}; }}\n\
             .plot path{class} {{ stroke: {color}; }}\n"
        )
        .expect("write to String cannot fail");
    }
    css
}

fn timeline_css(theme: &MermaidTheme) -> String {
    let mut css = String::with_capacity(8 * 300);
    let text = crate::css_color(theme.text_color);
    for i in 0..8 {
        let c = crate::css_color(theme.git_branch_colors[i]);
        let fill = crate::css_color(blend_over_background(
            theme.git_branch_colors[i],
            theme.background,
            ACCENT_FILL_OPACITY,
        ));
        write!(
            css,
            "rect.task-type-{i}, rect.section-type-{i} {{ fill: {fill}; stroke: {c}; }}\n"
        )
        .expect("write to String cannot fail");
    }
    for i in 0..4 {
        let c = crate::css_color(theme.git_branch_colors[i % 8]);
        let fill = crate::css_color(blend_over_background(
            theme.git_branch_colors[i % 8],
            theme.background,
            ACCENT_FILL_OPACITY,
        ));
        write!(
            css,
            ".section{i} {{ fill: {fill}; }}\n\
             .task{i} {{ fill: {fill}; stroke: {c}; }}\n\
             .taskText{i} {{ fill: {text}; }}\n\
             .taskTextOutside{i} {{ fill: {text}; }}\n"
        )
        .expect("write to String cannot fail");
    }
    css
}

fn should_scope_css_line(trimmed: &str) -> bool {
    !trimmed.is_empty()
        && (trimmed.starts_with('.')
            || trimmed.starts_with("foreignObject")
            || trimmed.starts_with("g.")
            || trimmed.starts_with("text")
            || trimmed.starts_with("tspan")
            || trimmed.starts_with("rect.")
            || trimmed.starts_with("path.")
            || trimmed.starts_with("defs")
            || trimmed.starts_with('#'))
}

fn scoped_selector_count(raw_css: &str) -> usize {
    raw_css.lines().fold(0, |count, line| {
        let trimmed = line.trim();
        if !should_scope_css_line(trimmed) {
            return count;
        }
        let Some((selectors, _)) = trimmed.split_once('{') else {
            return count;
        };
        count.saturating_add(selectors.split(',').count())
    })
}

fn scope_css(raw_css: &str, svg_id: &str) -> String {
    let scoped_selector_prefix_len = svg_id.len().saturating_add(2);
    let result_capacity = raw_css
        .len()
        .saturating_add(scoped_selector_count(raw_css).saturating_mul(scoped_selector_prefix_len));
    let mut result = String::with_capacity(result_capacity);
    for line in raw_css.lines() {
        let trimmed = line.trim();

        if should_scope_css_line(trimmed) {
            if let Some(brace) = trimmed.find('{') {
                let (selectors, rest) = trimmed.split_at(brace);
                let mut first = true;
                for selector in selectors.split(',') {
                    if !first {
                        result.push_str(", ");
                    }
                    first = false;
                    write!(result, "#{svg_id} {}", selector.trim())
                        .expect("write to String cannot fail");
                }
                writeln!(result, "{rest}").expect("write to String cannot fail");
                continue;
            }
        }
        writeln!(result, "{line}").expect("write to String cannot fail");
    }
    result
}

fn build_injected_css(theme: &MermaidTheme, svg_id: &str) -> String {
    let font = &theme.font_family;
    let text = crate::css_color(theme.text_color);
    let line = crate::css_color(theme.line_color);
    let primary = crate::css_color(theme.primary_color);
    let border = crate::css_color(theme.primary_border_color);
    let secondary = crate::css_color(theme.secondary_color);
    let tertiary = crate::css_color(theme.tertiary_color);
    let background = crate::css_color(theme.background);
    let edge_label_bg = crate::css_color(theme.edge_label_background);
    let actor_bg = crate::css_color(theme.actor_background);
    let actor_border = crate::css_color(theme.actor_border);
    let error_bg = {
        let mut c = theme.error_color;
        adjust_lightness(&mut c, theme.dark_mode);
        c
    };
    let error = crate::css_color(error_bg);
    let error_text = crate::css_color(crate::postprocess::util::text_color_for_background(
        error_bg,
    ));
    let warning_bg = {
        let mut c = theme.warning_color;
        adjust_lightness(&mut c, theme.dark_mode);
        c
    };
    let warning = crate::css_color(warning_bg);
    let warning_text = crate::css_color(crate::postprocess::util::text_color_for_background(
        warning_bg,
    ));
    let note_bg = crate::css_color(theme.note_background);
    let note_border = crate::css_color(theme.note_border);
    let er_odd = crate::css_color(theme.er_attr_bg_odd);
    let er_even = crate::css_color(theme.er_attr_bg_even);

    let actor_text = &text;
    let note_text = &text;

    let raw_css = format!(
        r#"
        text, tspan, foreignObject div, foreignObject span, foreignObject p {{ font-family: {font} !important; }}
        foreignObject div, foreignObject span, foreignObject p {{ font-size: 16px; color: {text}; }}
        .merman-foreignobject-fallback-text {{ font-size: 16px !important; }}
        foreignObject p {{ margin: 0; }}
        foreignObject {{ overflow: visible; }}
        foreignObject div {{ max-width: none; }}
        .label-group foreignObject {{ font-weight: bold; }}
        .em-swimlane text {{ fill: {text}; }}
        .node rect, .node path {{ fill: {primary}; stroke: {border}; }}
        .node polygon {{ fill: {primary}; stroke: {border}; }}
        .label-container path {{ fill: {primary}; stroke: {border}; }}
        {mindmap_css}
        .mindmap-node line, .timeline-node line {{ stroke: transparent; }}
        g.stateGroup rect {{ fill: {primary}; stroke: {border}; }}
        g.stateGroup text {{ fill: {text}; }}
        g.stateGroup .state-title {{ fill: {text}; }}
        .stateGroup .composit {{ fill: {background}; }}
        .stateGroup .alt-composit {{ fill: {tertiary}; }}
        .state-note {{ stroke: {note_border}; fill: {note_bg}; }}
        .state-note text {{ fill: {note_text}; }}
        .stateLabel .box {{ fill: {primary}; }}
        .stateLabel text {{ fill: {text}; }}
        .node circle.state-start {{ fill: {line}; stroke: {line}; }}
        .node .fork-join {{ fill: {line}; stroke: {line}; }}
        .node circle.state-end {{ fill: {border}; stroke: {background}; }}
        .end-state-inner {{ fill: {background}; }}
        .statediagram-cluster rect {{ fill: {primary}; stroke: {border}; }}
        .statediagram-cluster.statediagram-cluster .inner {{ fill: {background}; }}
        .statediagram-cluster.statediagram-cluster-alt .inner {{ fill: {tertiary}; }}
        .statediagram-state rect.divider {{ fill: {tertiary}; }}
        .statediagram-note rect {{ fill: {note_bg}; stroke: {note_border}; }}
        .statediagram-note text {{ fill: {note_text}; }}
        .statediagramTitleText {{ fill: {text}; }}
        .transition {{ stroke: {line}; }}
        .cluster-label, .nodeLabel {{ color: {text}; }}
        defs #statediagram-barbEnd {{ fill: {line}; stroke: {line}; }}
        #statediagram-barbEnd {{ fill: {line}; }}
        .edgeLabel .label rect {{ fill: {edge_label_bg}; opacity: 1; }}
        .edgeLabel rect {{ fill: {edge_label_bg}; opacity: 1; background-color: {edge_label_bg}; }}
        .edgeLabel .label text {{ fill: {text}; }}
        .edgeLabel p {{ background-color: {primary}; }}
        .edgeLabel {{ background-color: {primary}; }}
        .actor {{ stroke: {actor_border}; fill: {actor_bg}; }}
        text.actor {{ text-anchor: middle; }}
        text.actor>tspan {{ fill: {actor_text}; stroke: none; }}
        .labelText, .labelText>tspan {{ fill: {actor_text}; }}
        .actor-line {{ stroke: {actor_border}; fill: none; }}
        .messageLine0 {{ stroke: {text}; }}
        .messageLine1 {{ stroke: {text}; }}
        #arrowhead path {{ fill: {text}; stroke: {text}; }}
        #crosshead path {{ fill: {text}; stroke: {text}; }}
        .messageText {{ fill: {text}; }}
        .loopText, .loopText>tspan {{ fill: {text}; }}
        .loopLine {{ stroke: {actor_border}; fill: {actor_border}; }}
        .note {{ stroke: {note_border}; fill: {note_bg}; }}
        .noteText, .noteText>tspan {{ fill: {note_text}; }}
        .activation0, .activation1, .activation2 {{ fill: {secondary}; stroke: {border}; }}
        .labelBox {{ stroke: {actor_border}; fill: {actor_bg}; }}
        .actor-man line {{ stroke: {actor_border}; fill: none; }}
        .actor-man circle {{ stroke: {actor_border}; fill: {actor_bg}; }}
        .pieTitleText {{ fill: {text}; }}
        .slice {{ fill: {text}; }}
        .legend text {{ fill: {text}; }}
        .pieOuterCircle {{ stroke: {border}; }}
        .pieCircle {{ stroke: {border}; }}
        {timeline_css}
        text.journey-section, text.task {{ fill: {text}; }}
        .relationshipLabelBox {{ fill: {tertiary}; opacity: 0.7; background-color: {tertiary}; }}
        .labelBkg {{ background-color: {tertiary}; }}
        .edgeLabel .label {{ fill: {text}; }}
        .label {{ color: {text}; }}
        .relationshipLine {{ stroke: {line}; fill: none; }}
        .entityBox {{ fill: {primary}; stroke: {border}; }}
        .node .row-rect-odd path {{ fill: {er_odd}; }}
        .node .row-rect-even path {{ fill: {er_even}; }}
        .edge-thickness-normal {{ stroke-width: 1px; }}
        .relation {{ stroke: {line}; stroke-width: 1; fill: none; }}
        .edgePaths path {{ fill: none; }}
        .marker {{ fill: {line}; stroke: {line}; }}
        .marker.er {{ fill: none; stroke: {line}; }}
        .composition {{ fill: {line}; stroke: {line}; stroke-width: 1; }}
        .extension {{ fill: transparent; stroke: {line}; stroke-width: 1; }}
        .aggregation {{ fill: transparent; stroke: {line}; stroke-width: 1; }}
        .dependency {{ fill: {line}; stroke: {line}; stroke-width: 1; }}
        .lollipop {{ fill: {primary}; stroke: {line}; stroke-width: 1; }}
        .sectionTitle0, .sectionTitle1, .sectionTitle2, .sectionTitle3 {{ fill: {text}; }}
        .sectionTitle {{ font-family: {font} !important; }}
        .taskTextOutsideRight {{ fill: {text}; font-family: {font} !important; }}
        .taskTextOutsideLeft {{ fill: {text}; }}
        .active0, .active1, .active2, .active3 {{ fill: {secondary}; stroke: {border}; }}
        .activeText0, .activeText1, .activeText2, .activeText3 {{ fill: {text}; }}
        .done0, .done1, .done2, .done3 {{ stroke: {border}; fill: {secondary}; stroke-width: 2; }}
        .doneText0, .doneText1, .doneText2, .doneText3 {{ fill: {text}; }}
        .crit0, .crit1, .crit2, .crit3 {{ fill: {error}; stroke: {error}; }}
        .critText0, .critText1, .critText2, .critText3 {{ fill: {error_text}; }}
        .activeCrit0, .activeCrit1, .activeCrit2, .activeCrit3 {{ fill: {warning}; stroke: {warning}; }}
        .activeCritText0, .activeCritText1, .activeCritText2, .activeCritText3 {{ fill: {warning_text}; }}
        .doneCrit0, .doneCrit1, .doneCrit2, .doneCrit3 {{ fill: {error}; stroke: {border}; stroke-width: 2; }}
        .doneCritText0, .doneCritText1, .doneCritText2, .doneCritText3 {{ fill: {error_text}; }}
        .titleText {{ fill: {text}; font-family: {font} !important; }}
        .grid .tick text {{ fill: {text}; font-family: {font} !important; }}
        .grid .tick {{ stroke: {border}; }}
        {git_branch_css}
        .commit-merge {{ stroke: {primary}; fill: {primary}; }}
        .commit-reverse {{ stroke: {primary}; fill: {primary}; stroke-width: 3; }}
        .commit-highlight-inner {{ stroke: {primary}; fill: {primary}; }}
        .tag-label {{ font-size: 10px; fill: {text}; }}
        .tag-label-bkg {{ fill: {primary}; stroke: {border}; }}
        .tag-hole {{ fill: {line}; }}
        .commit-label {{ fill: {text}; }}
        .commit-label-bkg {{ fill: {edge_label_bg}; }}
        .commit-id, .commit-msg, .branch-label {{ fill: {text}; color: {text}; font-family: {font}; }}
        {accent_css}
        .data-point text {{ fill: {text}; }}
        {chart_color_css}
        "#,
        mindmap_css = mindmap_section_css(theme),
        git_branch_css = git_branch_css(theme),
        accent_css = accent_css(theme),
        chart_color_css = chart_color_css(theme),
        timeline_css = timeline_css(theme),
    );

    scope_css(&raw_css, svg_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_css_prefixes_selectors() {
        let input = "        .foo { color: red; }\n";
        let result = scope_css(input, "my-svg");
        assert!(result.contains("#my-svg .foo"), "got: {result}");
    }
}
