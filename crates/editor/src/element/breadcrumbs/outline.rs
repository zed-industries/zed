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

pub(crate) fn sibling_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
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

pub(crate) fn child_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
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

pub(crate) fn top_level_outline_indices(depths: &[usize]) -> Vec<usize> {
    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| parent.is_none().then_some(index))
        .collect()
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
                    .all(|(a, b)| a.range == b.range);
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

pub(super) fn render_outline_item_menu_row(
    item: &OutlineItem<Anchor>,
    is_current: bool,
    show_current_column: bool,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let mut text_style = window.text_style();
    text_style.color = Color::Default.color(cx);

    h_flex()
        .gap_1p5()
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
                .child(
                    StyledText::new(flatten_text_for_single_line_display(&item.text))
                        .with_default_highlights(&text_style, item.highlight_ranges.clone()),
                ),
        )
        .into_any_element()
}
