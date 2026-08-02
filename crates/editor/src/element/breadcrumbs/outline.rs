use super::*;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Task;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use ui::ListItemSpacing;

use super::path::MAX_BREADCRUMB_MENU_ENTRIES;

/// The parent of each item is the nearest preceding entry with a smaller depth rather than
/// `depth - 1`, because tree-sitter outlines can jump depth unevenly.
fn outline_parents(depths: &[usize]) -> Vec<Option<usize>> {
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

/// Items at `target_index`'s depth sharing its nearest shallower ancestor, itself included.
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

/// The items directly inside `target_index`, one level deeper.
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

/// Indices of the top-level items — those with no parent. The breadcrumb's leading path
/// segment stands in for the tree's implicit root, so it lists these.
pub(crate) fn top_level_outline_indices(depths: &[usize]) -> Vec<usize> {
    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| parent.is_none().then_some(index))
        .collect()
}

/// The symbols a breadcrumb segment can move to, filtered by the picker's query.
pub(crate) struct BreadcrumbSymbolDelegate {
    editor: WeakEntity<Editor>,
    items: Vec<OutlineItem<Anchor>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    /// The segment's own symbol, so the row standing for it reads as the current one.
    current_range: Option<Range<Anchor>>,
}

pub(crate) type BreadcrumbSymbolPicker = Picker<BreadcrumbSymbolDelegate>;

impl BreadcrumbSymbolDelegate {
    fn picker(
        editor: WeakEntity<Editor>,
        items: Vec<OutlineItem<Anchor>>,
        current_range: Option<Range<Anchor>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<BreadcrumbSymbolPicker> {
        cx.new(|cx| {
            let selected_index = current_range
                .as_ref()
                .and_then(|range| items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            let delegate = Self {
                editor,
                items,
                matches: Vec::new(),
                selected_index,
                current_range,
            };
            Picker::uniform_list(delegate, window, cx)
                .popover()
                .initial_width(rems(18.))
        })
    }

    /// Whether any listed symbol is the segment's own. If none is, the checkmark column is left
    /// out rather than indenting every row for a mark that never appears.
    fn shows_current_marker(&self) -> bool {
        self.current_range
            .as_ref()
            .is_some_and(|range| self.items.iter().any(|item| &item.range == range))
    }

    fn item_at(&self, index: usize) -> Option<&OutlineItem<Anchor>> {
        self.items.get(self.matches.get(index)?.candidate_id)
    }
}

impl PickerDelegate for BreadcrumbSymbolDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "breadcrumb symbol picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        self.selected_index = index;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search symbols…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Task<()> {
        let candidates = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                StringMatchCandidate::new(index, &flatten_text_for_single_line_display(&item.text))
            })
            .collect::<Vec<_>>();

        if query.is_empty() {
            self.matches = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.,
                })
                .collect();
            self.selected_index = self
                .current_range
                .as_ref()
                .and_then(|range| self.items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            cx.notify();
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ENTRIES,
                &Default::default(),
                executor,
            )
            .await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        let Some(item) = self.item_at(self.selected_index).cloned() else {
            return;
        };
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_to_outline_item(&item, window, cx);
            });
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<BreadcrumbSymbolPicker>) {}

    /// Rendered with the symbol's own syntax highlighting, the way the outline picker and panel
    /// draw it. The fuzzy match positions are left off, since the two highlight sets would
    /// fight.
    fn render_match(
        &self,
        index: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Option<Self::ListItem> {
        let item = self.item_at(index)?;
        let is_current = self.current_range.as_ref() == Some(&item.range);

        let mut text_style = window.text_style();
        text_style.color = Color::Default.color(cx);

        Some(
            ListItem::new(SharedString::from(format!(
                "breadcrumb-symbol-entry-{index}"
            )))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .when(self.shows_current_marker(), |this| {
                this.start_slot(div().flex_none().size(IconSize::Small.rems()).when(
                    is_current,
                    |this| {
                        this.child(
                            Icon::new(IconName::Check)
                                .color(Color::Accent)
                                .size(IconSize::Small),
                        )
                    },
                ))
            })
            .child(
                div().text_ui(cx).child(
                    StyledText::new(flatten_text_for_single_line_display(&item.text))
                        .with_default_highlights(&text_style, item.highlight_ranges.clone()),
                ),
            )
            .into_any_element(),
        )
    }
}

/// A segment whose dropdown drills into the outline: `target`'s children, else its siblings, else
/// the buffer's top-level symbols.
pub(crate) fn render_breadcrumb_symbol_segment(
    editor: WeakEntity<Editor>,
    buffer_id: BufferId,
    target: Option<OutlineItem<Anchor>>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    // `ButtonLike` wraps its click handler in `cx.stop_propagation()`, which is what keeps this
    // click from also reaching the outline toggle behind the popover.
    let trigger = ButtonLike::new(("breadcrumb-symbol", index))
        .style(ButtonStyle::Transparent)
        .size(ButtonSize::None)
        .height(rems_from_px(22.).into())
        .child(label);

    PopoverMenu::new(("breadcrumb-symbol-menu", index))
        .trigger(trigger)
        .menu(move |window, cx| {
            let editor_entity = editor.upgrade()?;
            let menu_items =
                editor_entity
                    .read(cx)
                    .breadcrumb_symbol_menu_items(buffer_id, target.as_ref(), cx);
            // Nothing to drill into, so fall through to the outline picker rather than flashing an
            // empty popover.
            if menu_items.is_empty() {
                if let Some(callback) = zed_actions::outline::TOGGLE_OUTLINE.get() {
                    callback(editor_entity.to_any_view(), window, cx);
                }
                return None;
            }
            Some(BreadcrumbSymbolDelegate::picker(
                editor.clone(),
                menu_items,
                target.as_ref().map(|item| item.range.clone()),
                window,
                cx,
            ))
        })
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sibling_outline_indices_top_level() {
        // struct A; struct B; struct C; — all depth 0, no parent.
        let depths = [0, 0, 0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 1), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![0, 1, 2]);
    }

    #[test]
    fn test_sibling_outline_indices_nested() {
        // `impl A { fn one; fn two }` then `impl B { fn three }`, i.e. [0, 1, 1, 0, 1].
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 4), vec![4]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
        assert_eq!(sibling_outline_indices(&depths, 3), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_uneven_depths() {
        // Tree-sitter outlines can jump straight from depth 0 to depth 2; the parent of a
        // depth-2 item is the nearest preceding shallower item, not a nonexistent depth-1 one.
        let depths = [0, 2, 2, 0];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_single_item() {
        let depths = [0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0]);
    }

    #[test]
    fn test_sibling_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(sibling_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_top_level() {
        // struct A; struct B; struct C; — all depth 0, none has children.
        let depths = [0, 0, 0];
        assert_eq!(child_outline_indices(&depths, 0), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_nested() {
        // `impl A { fn one; fn two }` then `impl B { fn three }`, i.e. [0, 1, 1, 0, 1].
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), vec![4]);
        // Leaf items have no children.
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 4), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_uneven_depths() {
        // The depth-2 fields are still direct children of the depth-0 struct even with no
        // depth-1 item between them — parenthood follows the nearest shallower item.
        let depths = [0, 2, 2, 0];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(child_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_top_level_outline_indices() {
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(top_level_outline_indices(&depths), vec![0, 3]);

        let depths_uneven = [0, 2, 2, 0];
        assert_eq!(top_level_outline_indices(&depths_uneven), vec![0, 3]);

        let depths_empty: [usize; 0] = [];
        assert_eq!(
            top_level_outline_indices(&depths_empty),
            Vec::<usize>::new()
        );
    }
}
