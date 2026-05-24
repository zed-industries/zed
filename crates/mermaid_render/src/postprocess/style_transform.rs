use anyhow::Result;
use quick_xml::events::{BytesText, Event};

use crate::MermaidTheme;

struct StyleTransform<I> {
    inner: I,
    injected_css: String,
    in_style: bool,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for StyleTransform<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
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
                return Some(Ok(event));
            }
            Event::Text(text) if self.in_style => {
                let css_text = match std::str::from_utf8(text.as_ref()) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e.into())),
                };
                let mut processed = strip_unsupported_css(css_text);
                processed.push_str(&self.injected_css);
                return Some(Ok(Event::Text(BytesText::from_escaped(processed))));
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
    StyleTransform {
        inner: events,
        injected_css,
        in_style: false,
    }
}

fn strip_unsupported_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.char_indices().peekable();

    while let Some(&(i, _)) = chars.peek() {
        let remaining = &css[i..];

        if remaining.starts_with("@keyframes") || remaining.starts_with("@-webkit-keyframes") {
            skip_css_block(&mut chars);
            continue;
        }

        if remaining.starts_with(":root") {
            skip_css_block(&mut chars);
            continue;
        }

        if remaining.starts_with(":not(") {
            for _ in 0..5 {
                chars.next();
            }
            let mut depth = 1u32;
            while let Some((_, c)) = chars.next() {
                if c == '(' {
                    depth += 1;
                }
                if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            continue;
        }

        let (_, ch) = chars.next().expect("peeked successfully above");
        result.push(ch);
    }

    strip_css_angle_units(&mut result);
    result
}

fn skip_css_block(chars: &mut std::iter::Peekable<std::str::CharIndices>) {
    let mut found_brace = false;
    let mut depth = 0u32;
    while let Some((_, c)) = chars.next() {
        if c == '{' {
            found_brace = true;
            depth += 1;
        } else if c == '}' {
            depth = depth.saturating_sub(1);
            if depth == 0 && found_brace {
                return;
            }
        }
    }
}

fn strip_css_angle_units(css: &mut String) {
    while let Some(pos) = css.find("deg)") {
        css.replace_range(pos..pos + 3, "");
    }
}

fn text_color_for_bg(hex_color: &str) -> &'static str {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() < 6 {
        return "#000";
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64;
    let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    if luma > 150.0 {
        "#000"
    } else {
        "#fff"
    }
}

fn mindmap_section_css(theme: &MermaidTheme) -> String {
    let colors: Vec<String> = theme
        .git_branch_colors
        .iter()
        .map(|c| crate::css_color(*c))
        .collect();
    let mut css = String::new();

    let emit = |css: &mut String, selector: &str, color: &str| {
        let txt = text_color_for_bg(color);
        let section_index = selector.trim_start_matches(".section-root.section-").trim_start_matches(".section-");
        use std::fmt::Write;
        write!(
            css,
            "{selector} rect, {selector} path, {selector} circle, {selector} polygon \
             {{ fill: {color} !important; }}\n\
             {selector} text, {selector} span \
             {{ fill: {txt} !important; color: {txt} !important; }}\n\
             {selector} foreignObject div, {selector} foreignObject span, {selector} foreignObject p \
             {{ color: {txt} !important; }}\n\
             .section-edge{section_index} {{ stroke: {color} !important; }}\n",
        )
        .expect("write to String cannot fail");
    };

    emit(&mut css, ".section-root.section--1", &colors[0]);
    emit(&mut css, ".section--1", &colors[1]);
    for i in 0..=10 {
        let ci = 2 + (i % 6);
        emit(&mut css, &format!(".section-{i}"), &colors[ci]);
    }
    css
}

fn git_branch_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for i in 0..8 {
        let c = crate::css_color(theme.git_branch_colors[i]);
        let lbl = crate::css_color(theme.git_branch_label_colors[i]);
        use std::fmt::Write;
        write!(
            css,
            ".commit{i} {{ stroke: {c}; fill: {c}; }} \
             .arrow{i} {{ stroke: {c}; }} \
             .label{i} {{ fill: {c}; }} \
             .branch-label{i} {{ fill: {lbl}; }}\n"
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
    let note_bg = crate::css_color(theme.note_background);
    let note_border = crate::css_color(theme.note_border);
    let er_odd = crate::css_color(theme.er_attr_bg_odd);
    let er_even = crate::css_color(theme.er_attr_bg_even);

    let actor_text = text_color_for_bg(&actor_bg);
    let note_text = text_color_for_bg(&note_bg);

    let raw_css = format!(
        r#"
        foreignObject div, foreignObject span, foreignObject p {{ font-family: {font}; font-size: 16px; color: {text}; }}
        foreignObject p {{ margin: 0; }}
        foreignObject {{ overflow: visible; }}
        foreignObject div {{ max-width: none !important; }}
        .label-group foreignObject {{ font-weight: bold; }}
        .node rect, .node path {{ fill: {primary} !important; stroke: {border} !important; }}
        .node polygon {{ fill: {primary} !important; stroke: {border} !important; }}
        .label-container path {{ fill: {primary} !important; stroke: {border} !important; }}
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
        .actor {{ stroke: {actor_border}; fill: {actor_bg} !important; }}
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
        .task-type-0, .section-type-0 {{ fill: {primary} !important; }}
        .task-type-1, .section-type-1 {{ fill: {secondary} !important; }}
        .task-type-2, .section-type-2 {{ fill: {tertiary} !important; }}
        .task-type-3, .section-type-3 {{ fill: {primary} !important; }}
        .task-type-4, .section-type-4 {{ fill: {secondary} !important; }}
        .task-type-5, .section-type-5 {{ fill: {tertiary} !important; }}
        .task-type-6, .section-type-6 {{ fill: {primary} !important; }}
        .task-type-7, .section-type-7 {{ fill: {secondary} !important; }}
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
        .doneCritText0, .doneCritText1, .doneCritText2, .doneCritText3 {{ fill: {text} !important; }}
        .activeCritText0, .activeCritText1, .activeCritText2, .activeCritText3 {{ fill: {text} !important; }}
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
        "#,
        mindmap_css = mindmap_section_css(theme),
        git_branch_css = git_branch_css(theme),
    );

    scope_css(&raw_css, svg_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_keyframes() {
        let input = "@keyframes bounce { 0% { transform: scale(1); } 100% { transform: scale(1.1); } } .node rect { fill: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains("@keyframes"), "got: {result}");
        assert!(result.contains(".node rect"), "got: {result}");
    }

    #[test]
    fn strips_root_blocks() {
        let input = ":root { --bg: white; } .foo { color: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains(":root"), "got: {result}");
        assert!(result.contains(".foo"), "got: {result}");
    }

    #[test]
    fn strips_not_pseudo_selectors() {
        let input = ".node:not(.mindmap-node) rect { fill: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains(":not"), "got: {result}");
        assert!(result.contains(".node rect"), "got: {result}");
    }

    #[test]
    fn strips_deg_units() {
        let input = ".foo { transform: rotate(45deg); }";
        let result = strip_unsupported_css(input);
        assert!(result.contains("rotate(45)"), "got: {result}");
        assert!(!result.contains("deg"), "got: {result}");
    }

    #[test]
    fn scope_css_prefixes_selectors() {
        let input = "        .foo { color: red; }\n";
        let result = scope_css(input, "my-svg");
        assert!(result.contains("#my-svg .foo"), "got: {result}");
    }
}
