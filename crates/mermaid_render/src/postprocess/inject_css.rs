//! Builds a theme-aware CSS stylesheet and appends it into the SVG's `<style>`
//! element. All selectors are scoped to the SVG's `id` to prevent leaking.
//!
//! ```xml
//! <!-- before -->
//! <style>.node rect { fill: white; }</style>
//!
//! <!-- after -->
//! <style>.node rect { fill: white; }
//! #mermaid-1 .node rect { fill: #89b4fa !important; }
//! /* ... theme rules ... */
//! </style>
//! ```

use std::collections::VecDeque;
use std::fmt::Write;

use anyhow::Result;
use quick_xml::events::{BytesText, Event};

use crate::MermaidTheme;

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
    let colors: Vec<String> = theme
        .git_branch_colors
        .iter()
        .map(|c| crate::css_color(*c))
        .collect();
    let text_colors: Vec<String> = theme
        .git_branch_colors
        .iter()
        .map(|c| crate::css_color(crate::postprocess::util::text_color_for_background(*c)))
        .collect();
    let mut css = String::new();

    let emit = |css: &mut String, selector: &str, color: &str, txt: &str| {
        let section_index = selector
            .trim_start_matches(".section-root.section-")
            .trim_start_matches(".section-");
        write!(
            css,
            "{selector} rect, {selector} path, {selector} circle, {selector} polygon \
             {{ fill: {color} !important; }}\n\
             {selector} text, {selector} span, \
             text{selector}, tspan{selector} \
             {{ fill: {txt} !important; color: {txt} !important; }}\n\
             {selector} foreignObject div, {selector} foreignObject span, {selector} foreignObject p \
             {{ color: {txt} !important; }}\n\
             .section-edge{section_index} {{ stroke: {color} !important; }}\n",
        )
        .expect("write to String cannot fail");
    };

    emit(
        &mut css,
        ".section-root.section--1",
        &colors[0],
        &text_colors[0],
    );
    emit(&mut css, ".section--1", &colors[1], &text_colors[1]);
    for i in 0..=10 {
        let ci = 2 + (i % 6);
        emit(
            &mut css,
            &format!(".section-{i}"),
            &colors[ci],
            &text_colors[ci],
        );
    }
    css
}

fn git_branch_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for i in 0..8 {
        let c = crate::css_color(theme.git_branch_colors[i]);
        let lbl = crate::css_color(theme.git_branch_label_colors[i]);
        write!(
            css,
            ".commit{i} {{ stroke: {c}; fill: {c}; }}\n\
             .arrow{i} {{ stroke: {c}; }}\n\
             .label{i} {{ fill: {c}; }}\n\
             .branch-label{i} {{ fill: {lbl}; }}\n"
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

fn accent_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for (i, accent) in theme.accent_colors.iter().enumerate() {
        let stroke = crate::css_color(accent.foreground);
        let mut bg = accent.background;
        adjust_lightness(&mut bg, theme.dark_mode);
        let fill = crate::css_color(bg);
        let text = crate::css_color(crate::postprocess::util::text_color_for_background(bg));
        let class = format!(".zed-accent-{i}");
        write!(
            css,
            "{class} rect, {class} path, {class} circle, {class} polygon, {class} ellipse, \
             rect{class}, path{class}, circle{class}, polygon{class}, ellipse{class} \
             {{ fill: {fill} !important; stroke: {stroke} !important; }}\n\
             {class} text, {class} tspan, text{class}, tspan{class} \
             {{ fill: {text} !important; }}\n",
        )
        .expect("write to String cannot fail");
    }
    css
}

fn chart_color_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for i in 0..8 {
        let color = crate::css_color(theme.git_branch_colors[i]);
        let class = format!(".zed-chart-{i}");
        write!(
            css,
            "path.pieCircle{class} {{ fill: {color} !important; }}\n\
             .plot rect{class}, .legend rect{class} {{ fill: {color} !important; stroke: {color} !important; }}\n\
             .plot path{class} {{ stroke: {color} !important; }}\n"
        )
        .expect("write to String cannot fail");
    }
    css
}

fn scope_css(raw_css: &str, svg_id: &str) -> String {
    raw_css
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return format!("{line}\n");
            }
            if trimmed.starts_with('.')
                || trimmed.starts_with("foreignObject")
                || trimmed.starts_with("g.")
                || trimmed.starts_with("text.")
                || trimmed.starts_with("rect.")
                || trimmed.starts_with("path.")
                || trimmed.starts_with("defs")
                || trimmed.starts_with('#')
            {
                if let Some(brace) = trimmed.find('{') {
                    let (selectors, rest) = trimmed.split_at(brace);
                    let scoped: Vec<String> = selectors
                        .split(',')
                        .map(|s| format!("#{svg_id} {}", s.trim()))
                        .collect();
                    format!("        {}{}\n", scoped.join(", "), rest)
                } else {
                    format!("{line}\n")
                }
            } else {
                format!("{line}\n")
            }
        })
        .collect()
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
        foreignObject div, foreignObject span, foreignObject p {{ font-family: {font}; font-size: 16px; color: {text}; }}
        foreignObject p {{ margin: 0; }}
        foreignObject {{ overflow: visible; }}
        foreignObject div {{ max-width: none !important; }}
        .label-group foreignObject {{ font-weight: bold; }}
        .node rect, .node path {{ fill: {primary}; stroke: {border}; }}
        .node polygon {{ fill: {primary}; stroke: {border}; }}
        .label-container path {{ fill: {primary}; stroke: {border}; }}
        {mindmap_css}
        g.stateGroup rect {{ fill: {primary} !important; stroke: {border} !important; }}
        g.stateGroup text {{ fill: {text} !important; }}
        g.stateGroup .state-title {{ fill: {text} !important; }}
        .stateGroup .composit {{ fill: {background} !important; }}
        .stateGroup .alt-composit {{ fill: {tertiary} !important; }}
        .state-note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .state-note text {{ fill: {note_text} !important; }}
        .stateLabel .box {{ fill: {primary} !important; }}
        .stateLabel text {{ fill: {text} !important; }}
        .node circle.state-start {{ fill: {line} !important; stroke: {line} !important; }}
        .node .fork-join {{ fill: {line} !important; stroke: {line} !important; }}
        .node circle.state-end {{ fill: {border} !important; stroke: {background} !important; }}
        .end-state-inner {{ fill: {background} !important; }}
        .statediagram-cluster rect {{ fill: {primary} !important; stroke: {border} !important; }}
        .statediagram-cluster.statediagram-cluster .inner {{ fill: {background} !important; }}
        .statediagram-cluster.statediagram-cluster-alt .inner {{ fill: {tertiary} !important; }}
        .statediagram-state rect.divider {{ fill: {tertiary} !important; }}
        .statediagram-note rect {{ fill: {note_bg} !important; stroke: {note_border} !important; }}
        .statediagram-note text {{ fill: {note_text} !important; }}
        .statediagramTitleText {{ fill: {text} !important; }}
        .transition {{ stroke: {line} !important; }}
        .cluster-label, .nodeLabel {{ color: {text} !important; }}
        defs #statediagram-barbEnd {{ fill: {line} !important; stroke: {line} !important; }}
        #statediagram-barbEnd {{ fill: {line} !important; }}
        .edgeLabel .label rect {{ fill: {primary} !important; }}
        .edgeLabel rect {{ fill: {primary} !important; background-color: {primary} !important; }}
        .edgeLabel .label text {{ fill: {text} !important; }}
        .edgeLabel p {{ background-color: {primary} !important; }}
        .edgeLabel {{ background-color: {primary} !important; }}
        .actor {{ stroke: {actor_border}; fill: {actor_bg}; }}
        text.actor {{ text-anchor: middle; }}
        text.actor>tspan {{ fill: {actor_text} !important; stroke: none; }}
        .labelText, .labelText>tspan {{ fill: {actor_text} !important; }}
        .actor-line {{ stroke: {actor_border} !important; }}
        .messageLine0 {{ stroke: {text} !important; }}
        .messageLine1 {{ stroke: {text} !important; }}
        #arrowhead path {{ fill: {text} !important; stroke: {text} !important; }}
        #crosshead path {{ fill: {text} !important; stroke: {text} !important; }}
        .messageText {{ fill: {text} !important; }}
        .loopText, .loopText>tspan {{ fill: {text} !important; }}
        .loopLine {{ stroke: {actor_border} !important; fill: {actor_border} !important; }}
        .note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .noteText, .noteText>tspan {{ fill: {note_text} !important; }}
        .activation0, .activation1, .activation2 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .labelBox {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man line {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man circle {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .pieTitleText {{ fill: {text} !important; }}
        .slice {{ fill: {text} !important; }}
        .legend text {{ fill: {text} !important; }}
        .pieOuterCircle {{ stroke: {border} !important; }}
        .pieCircle {{ stroke: {border} !important; }}
        rect.task-type-0, rect.section-type-0 {{ fill: {primary} !important; }}
        rect.task-type-1, rect.section-type-1 {{ fill: {secondary} !important; }}
        rect.task-type-2, rect.section-type-2 {{ fill: {tertiary} !important; }}
        rect.task-type-3, rect.section-type-3 {{ fill: {primary} !important; }}
        rect.task-type-4, rect.section-type-4 {{ fill: {secondary} !important; }}
        rect.task-type-5, rect.section-type-5 {{ fill: {tertiary} !important; }}
        rect.task-type-6, rect.section-type-6 {{ fill: {primary} !important; }}
        rect.task-type-7, rect.section-type-7 {{ fill: {secondary} !important; }}
        text.journey-section, text.task {{ fill: {text} !important; }}
        .relationshipLabelBox {{ fill: {tertiary} !important; opacity: 0.7; background-color: {tertiary} !important; }}
        .labelBkg {{ background-color: {tertiary} !important; }}
        .edgeLabel .label {{ fill: {border} !important; }}
        .label {{ color: {text} !important; }}
        .relationshipLine {{ stroke: {line} !important; fill: none !important; }}
        .entityBox {{ fill: {primary}; stroke: {border}; }}
        .node .row-rect-odd path {{ fill: {er_odd} !important; }}
        .node .row-rect-even path {{ fill: {er_even} !important; }}
        .edge-thickness-normal {{ stroke-width: 1px; }}
        .relation {{ stroke: {line}; stroke-width: 1; fill: none; }}
        .edgePaths path {{ fill: none; }}
        .marker {{ fill: {line} !important; stroke: {line} !important; }}
        .marker.er {{ fill: none !important; stroke: {line} !important; }}
        .composition {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .extension {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .aggregation {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .dependency {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .lollipop {{ fill: {primary} !important; stroke: {line} !important; stroke-width: 1; }}
        .section0 {{ fill: {tertiary} !important; }}
        .section2 {{ fill: {primary} !important; }}
        .section1, .section3 {{ fill: {secondary} !important; opacity: 0.2; }}
        .sectionTitle0, .sectionTitle1, .sectionTitle2, .sectionTitle3 {{ fill: {text} !important; }}
        .sectionTitle {{ font-family: {font} !important; }}
        .task0, .task1, .task2, .task3 {{ fill: {primary} !important; stroke: {border} !important; }}
        .taskText0, .taskText1, .taskText2, .taskText3 {{ fill: {text} !important; }}
        .taskTextOutside0, .taskTextOutside1, .taskTextOutside2, .taskTextOutside3 {{ fill: {text} !important; }}
        .taskTextOutsideRight {{ fill: {text} !important; font-family: {font} !important; }}
        .taskTextOutsideLeft {{ fill: {text} !important; }}
        .active0, .active1, .active2, .active3 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .activeText0, .activeText1, .activeText2, .activeText3 {{ fill: {text} !important; }}
        .done0, .done1, .done2, .done3 {{ stroke: {border} !important; fill: {secondary} !important; stroke-width: 2; }}
        .doneText0, .doneText1, .doneText2, .doneText3 {{ fill: {text} !important; }}
        .crit0, .crit1, .crit2, .crit3 {{ fill: {error} !important; stroke: {error} !important; }}
        .critText0, .critText1, .critText2, .critText3 {{ fill: {error_text} !important; }}
        .activeCrit0, .activeCrit1, .activeCrit2, .activeCrit3 {{ fill: {warning} !important; stroke: {warning} !important; }}
        .activeCritText0, .activeCritText1, .activeCritText2, .activeCritText3 {{ fill: {warning_text} !important; }}
        .doneCrit0, .doneCrit1, .doneCrit2, .doneCrit3 {{ fill: {error} !important; stroke: {border} !important; stroke-width: 2; }}
        .doneCritText0, .doneCritText1, .doneCritText2, .doneCritText3 {{ fill: {error_text} !important; }}
        .titleText {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick text {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick {{ stroke: {border} !important; }}
        {git_branch_css}
        .commit-merge {{ stroke: {primary}; fill: {primary}; }}
        .commit-reverse {{ stroke: {primary}; fill: {primary}; stroke-width: 3; }}
        .commit-highlight-inner {{ stroke: {primary}; fill: {primary}; }}
        .tag-label {{ font-size: 10px; }}
        .tag-label-bkg {{ fill: {primary}; stroke: {border}; }}
        .tag-hole {{ fill: {line}; }}
        .commit-label {{ fill: {text}; }}
        .commit-label-bkg {{ fill: {edge_label_bg}; }}
        .commit-id, .commit-msg, .branch-label {{ fill: {text}; color: {text}; font-family: {font}; }}
        {accent_css}
        {chart_color_css}
        "#,
        mindmap_css = mindmap_section_css(theme),
        git_branch_css = git_branch_css(theme),
        accent_css = accent_css(theme),
        chart_color_css = chart_color_css(theme),
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
