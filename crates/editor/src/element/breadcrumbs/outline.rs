use super::super::*;

pub(super) fn outline_parents(depths: &[usize]) -> Vec<Option<usize>> {
    let mut parents = Vec::with_capacity(depths.len());
    let mut ancestor_stack: Vec<(usize, usize)> = Vec::new();
    for (index, &depth) in depths.iter().enumerate() {
        while ancestor_stack
            .last()
            .is_some_and(|&(ancestor_depth, _)| ancestor_depth >= depth)
        {
            ancestor_stack.pop();
        }
        parents.push(ancestor_stack.last().map(|&(_, parent_index)| parent_index));
        ancestor_stack.push((depth, index));
    }
    parents
}

pub(super) fn sibling_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
    if target_index >= depths.len() {
        return Vec::new();
    }

    let parents = outline_parents(depths);
    let target_parent = parents[target_index];
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| (parent == target_parent).then_some(index))
        .collect()
}

pub(super) fn child_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
    if target_index >= depths.len() {
        return Vec::new();
    }

    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| (parent == Some(target_index)).then_some(index))
        .collect()
}

pub(super) fn top_level_outline_indices(depths: &[usize]) -> Vec<usize> {
    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| parent.is_none().then_some(index))
        .collect()
}

/// Identity for outline items across the bar and the open listing. Anchors already pin an item
/// to its position, and the two resolve items from separate snapshots, so the range is what
/// stays comparable between them. Callers pass the two items in whichever order is local to
/// them, so any clause added here has to stay symmetric.
pub(super) fn same_symbol_item(a: &OutlineItem<Anchor>, b: &OutlineItem<Anchor>) -> bool {
    a.range == b.range
}

pub(super) fn resolve_bar_symbol_trail(
    cursor_chain: Vec<OutlineItem<Anchor>>,
    menu_trail: Option<Vec<OutlineItem<Anchor>>>,
) -> Vec<OutlineItem<Anchor>> {
    match menu_trail.filter(|trail| !trail.is_empty()) {
        Some(menu_trail) => {
            let is_prefix_of_cursor_chain = menu_trail.len() <= cursor_chain.len()
                && menu_trail
                    .iter()
                    .zip(&cursor_chain)
                    .all(|(a, b)| same_symbol_item(a, b));
            if is_prefix_of_cursor_chain {
                cursor_chain
            } else {
                menu_trail
            }
        }
        None => cursor_chain,
    }
}

pub(super) fn flatten_text_for_single_line_display(text: &str) -> String {
    const LINE_BREAK: char = '\n';
    const REPLACEMENT: &str = " ";
    debug_assert_eq!(LINE_BREAK.len_utf8(), REPLACEMENT.len());
    text.replace(LINE_BREAK, REPLACEMENT)
}

/// Overlays fuzzy-match emphasis on the outline's own syntax highlights, so a filtered row
/// keeps its colours instead of falling back to one flat colour.
pub(super) fn highlights_with_match_emphasis(
    text: &str,
    syntax_highlights: &[(Range<usize>, gpui::HighlightStyle)],
    match_positions: &[usize],
    emphasis: gpui::HighlightStyle,
) -> Vec<(Range<usize>, gpui::HighlightStyle)> {
    if match_positions.is_empty() {
        return syntax_highlights.to_vec();
    }
    // A set rather than a scan per character, and without assuming the matcher returns them
    // in order.
    let match_positions: std::collections::HashSet<usize> =
        match_positions.iter().copied().collect();

    let mut merged: Vec<(Range<usize>, gpui::HighlightStyle)> = Vec::new();
    for (start, character) in text.char_indices() {
        let end = start + character.len_utf8();
        let mut style = syntax_highlights
            .iter()
            .find(|(range, _)| range.start <= start && start < range.end)
            .map(|(_, style)| *style)
            .unwrap_or_default();
        if match_positions.contains(&start) {
            style.font_weight = emphasis.font_weight.or(style.font_weight);
            style.underline = emphasis.underline.or(style.underline);
            if style.color.is_none() {
                style.color = emphasis.color;
            }
        }
        match merged.last_mut() {
            Some((range, last_style)) if *last_style == style && range.end == start => {
                range.end = end;
            }
            _ => merged.push((start..end, style)),
        }
    }
    merged
}

pub(super) fn render_outline_item_menu_row(
    item: &OutlineItem<Anchor>,
    match_positions: &[usize],
    is_current: bool,
    show_current_column: bool,
    indent: usize,
    context: Option<SharedString>,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let mut text_style = window.text_style();
    text_style.color = Color::Default.color(cx);
    let text = flatten_text_for_single_line_display(&item.text);
    let highlights = highlights_with_match_emphasis(
        &text,
        &item.highlight_ranges,
        match_positions,
        gpui::HighlightStyle {
            font_weight: Some(gpui::FontWeight::BOLD),
            color: Some(Color::Accent.color(cx)),
            ..Default::default()
        },
    );

    h_flex()
        .gap_1p5()
        .when(indent > 0, |this| {
            this.child(div().w(px(10.) * indent as f32).flex_none())
        })
        .when(is_current, |this| {
            this.child(
                Icon::new(IconName::Check)
                    .color(Color::Accent)
                    .size(IconSize::Small),
            )
        })
        .when(!is_current && show_current_column, |this| {
            this.child(div().size(IconSize::Small.rems()))
        })
        .child(
            div()
                .min_w_0()
                .overflow_x_hidden()
                .whitespace_nowrap()
                .text_ellipsis_middle()
                .child(StyledText::new(text).with_default_highlights(&text_style, highlights)),
        )
        .when_some(context, |this, context| {
            this.child(
                div().flex_none().child(
                    Label::new(context)
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            )
        })
        .into_any_element()
}
