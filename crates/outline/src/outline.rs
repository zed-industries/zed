use std::ops::Range;
use std::{
    cmp::{self, Reverse},
    collections::HashSet,
    sync::Arc,
};

use editor::RowHighlightOptions;
use editor::{Anchor, AnchorRangeExt, Editor, scroll::Autoscroll};
use fuzzy::StringMatch;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, HighlightStyle,
    MouseButton, ParentElement, Point, Render, Styled, StyledText, Task, TextStyle, WeakEntity,
    Window, actions, div, rems,
};
use language::{Outline, OutlineItem};
use menu::SelectPrevious;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::Settings;
use theme::{ActiveTheme, ThemeSettings};
use ui::{Disclosure, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{DismissDecision, ModalView};

actions!(outline, [ToggleExpand, ExpandSelected, CollapseSelected]);

pub fn init(cx: &mut App) {
    cx.observe_new(OutlineView::register).detach();
    zed_actions::outline::TOGGLE_OUTLINE
        .set(|view, window, cx| {
            let Ok(editor) = view.downcast::<Editor>() else {
                return;
            };

            toggle(editor, &Default::default(), window, cx);
        })
        .ok();
}

pub fn toggle(
    editor: Entity<Editor>,
    _: &zed_actions::outline::ToggleOutline,
    window: &mut Window,
    cx: &mut App,
) {
    let outline = editor
        .read(cx)
        .buffer()
        .read(cx)
        .snapshot(cx)
        .outline(Some(cx.theme().syntax()));

    if let Some((workspace, outline)) = editor.read(cx).workspace().zip(outline) {
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                OutlineView::new(outline, editor, window, cx)
            });
        })
    }
}

pub struct OutlineView {
    picker: Option<Entity<Picker<OutlineViewDelegate>>>,
    tree_view: Option<Entity<OutlineTreeView>>,
    use_tree_mode: bool,
}

impl Focusable for OutlineView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.use_tree_mode {
            if let Some(tree_view) = &self.tree_view {
                tree_view.read(cx).focus_handle.clone()
            } else {
                cx.focus_handle()
            }
        } else if let Some(picker) = &self.picker {
            picker.read(cx).focus_handle(cx)
        } else {
            cx.focus_handle()
        }
    }
}

impl EventEmitter<DismissEvent> for OutlineView {}
impl ModalView for OutlineView {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        if self.use_tree_mode {
            if let Some(tree_view) = &self.tree_view {
                tree_view.update(cx, |tree_view, cx| {
                    tree_view.restore_active_editor(window, cx)
                });
            }
        } else if let Some(picker) = &self.picker {
            picker.update(cx, |picker, cx| {
                picker.delegate.restore_active_editor(window, cx)
            });
        }
        DismissDecision::Dismiss(true)
    }
}

impl Render for OutlineView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.use_tree_mode {
            if let Some(tree_view) = &self.tree_view {
                v_flex().w(rems(34.)).child(tree_view.clone())
            } else {
                div()
            }
        } else if let Some(picker) = &self.picker {
            v_flex().w(rems(34.)).child(picker.clone())
        } else {
            div()
        }
    }
}

impl OutlineView {
    fn register(editor: &mut Editor, _: Option<&mut Window>, cx: &mut Context<Editor>) {
        if editor.mode().is_full() {
            let handle = cx.entity().downgrade();
            editor
                .register_action(move |action, window, cx| {
                    if let Some(editor) = handle.upgrade() {
                        toggle(editor, action, window, cx);
                    }
                })
                .detach();
        }
    }

    fn new(
        outline: Outline<Anchor>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> OutlineView {
        // TODO: Make this configurable via settings
        let use_tree_mode = true;

        let (picker, tree_view) = if use_tree_mode {
            let tree_view = OutlineTreeView::new(editor, outline, cx);
            (None, Some(cx.new(|_| tree_view)))
        } else {
            let delegate = OutlineViewDelegate::new(cx.entity().downgrade(), outline, editor, cx);
            let picker = cx.new(|cx| {
                Picker::uniform_list(delegate, window, cx).max_height(Some(vh(0.75, window)))
            });
            (Some(picker), None)
        };

        OutlineView {
            picker,
            tree_view,
            use_tree_mode,
        }
    }
}

struct OutlineViewDelegate {
    outline_view: WeakEntity<OutlineView>,
    active_editor: Entity<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    prev_scroll_position: Option<Point<f32>>,
    matches: Vec<StringMatch>,
    last_query: String,
}

enum OutlineRowHighlights {}

impl OutlineViewDelegate {
    fn new(
        outline_view: WeakEntity<OutlineView>,
        outline: Outline<Anchor>,
        editor: Entity<Editor>,

        cx: &mut Context<OutlineView>,
    ) -> Self {
        Self {
            outline_view,
            last_query: Default::default(),
            matches: Default::default(),
            selected_match_index: 0,
            prev_scroll_position: Some(editor.update(cx, |editor, cx| editor.scroll_position(cx))),
            active_editor: editor,
            outline,
        }
    }

    fn restore_active_editor(&mut self, window: &mut Window, cx: &mut App) {
        self.active_editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<OutlineRowHighlights>();
            if let Some(scroll_position) = self.prev_scroll_position {
                editor.set_scroll_position(scroll_position, window, cx);
            }
        })
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        navigate: bool,

        cx: &mut Context<Picker<OutlineViewDelegate>>,
    ) {
        self.selected_match_index = ix;

        if navigate && !self.matches.is_empty() {
            let selected_match = &self.matches[self.selected_match_index];
            let outline_item = &self.outline.items[selected_match.candidate_id];

            self.active_editor.update(cx, |active_editor, cx| {
                active_editor.clear_row_highlights::<OutlineRowHighlights>();
                active_editor.highlight_rows::<OutlineRowHighlights>(
                    outline_item.range.start..outline_item.range.end,
                    cx.theme().colors().editor_highlighted_line_background,
                    RowHighlightOptions {
                        autoscroll: true,
                        ..Default::default()
                    },
                    cx,
                );
                active_editor.request_autoscroll(Autoscroll::center(), cx);
            });
        }
    }
}

impl PickerDelegate for OutlineViewDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search buffer symbols...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<Picker<OutlineViewDelegate>>,
    ) {
        self.set_selected_index(ix, true, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<OutlineViewDelegate>>,
    ) -> Task<()> {
        let selected_index;
        if query.is_empty() {
            self.restore_active_editor(window, cx);
            self.matches = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(index, _)| StringMatch {
                    candidate_id: index,
                    score: Default::default(),
                    positions: Default::default(),
                    string: Default::default(),
                })
                .collect();

            let (buffer, cursor_offset) = self.active_editor.update(cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).snapshot(cx);
                let cursor_offset = editor.selections.newest::<usize>(cx).head();
                (buffer, cursor_offset)
            });
            selected_index = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let range = item.range.to_offset(&buffer);
                    let distance_to_closest_endpoint = cmp::min(
                        (range.start as isize - cursor_offset as isize).abs(),
                        (range.end as isize - cursor_offset as isize).abs(),
                    );
                    let depth = if range.contains(&cursor_offset) {
                        Some(item.depth)
                    } else {
                        None
                    };
                    (ix, depth, distance_to_closest_endpoint)
                })
                .max_by_key(|(_, depth, distance)| (*depth, Reverse(*distance)))
                .map(|(ix, _, _)| ix)
                .unwrap_or(0);
        } else {
            self.matches = smol::block_on(
                self.outline
                    .search(&query, cx.background_executor().clone()),
            );
            selected_index = self
                .matches
                .iter()
                .enumerate()
                .max_by_key(|(_, m)| OrderedFloat(m.score))
                .map(|(ix, _)| ix)
                .unwrap_or(0);
        }
        self.last_query = query;
        self.set_selected_index(selected_index, !self.last_query.is_empty(), cx);
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _: bool,
        window: &mut Window,
        cx: &mut Context<Picker<OutlineViewDelegate>>,
    ) {
        self.prev_scroll_position.take();
        self.set_selected_index(self.selected_match_index, true, cx);

        self.active_editor.update(cx, |active_editor, cx| {
            let highlight = active_editor
                .highlighted_rows::<OutlineRowHighlights>()
                .next();
            if let Some((rows, _)) = highlight {
                active_editor.change_selections(Some(Autoscroll::center()), window, cx, |s| {
                    s.select_ranges([rows.start..rows.start])
                });
                active_editor.clear_row_highlights::<OutlineRowHighlights>();
                window.focus(&active_editor.focus_handle(cx));
            }
        });

        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<OutlineViewDelegate>>) {
        self.outline_view
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
        self.restore_active_editor(window, cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let outline_item = self.outline.items.get(mat.candidate_id)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    div()
                        .text_ui(cx)
                        .pl(rems(outline_item.depth as f32))
                        .child(render_item(outline_item, mat.ranges(), cx)),
                ),
        )
    }
}

// Tree view implementation
pub struct OutlineTreeView {
    focus_handle: FocusHandle,
    active_editor: Entity<Editor>,
    outline: Option<Outline<Anchor>>,
    expanded_items: HashSet<usize>,
    selected_index: Option<usize>,
    filter_query: String,
    filtered_indices: Vec<usize>,
    prev_scroll_position: Option<Point<f32>>,
    auto_expand_depth: usize,
}

impl OutlineTreeView {
    fn new(editor: Entity<Editor>, outline: Outline<Anchor>, cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();
        let mut this = Self {
            focus_handle: focus_handle.clone(),
            active_editor: editor.clone(),
            outline: Some(outline),
            expanded_items: HashSet::new(),
            selected_index: None,
            filter_query: String::new(),
            filtered_indices: Vec::new(),
            prev_scroll_position: None,
            auto_expand_depth: 2,
        };

        this.update_filtered_indices();
        this.auto_expand_to_depth();

        this
    }

    fn update_filtered_indices(&mut self) {
        if let Some(outline) = &self.outline {
            if self.filter_query.is_empty() {
                self.filtered_indices = (0..outline.items.len()).collect();
            } else {
                // TODO: Implement proper fuzzy filtering
                self.filtered_indices = (0..outline.items.len())
                    .filter(|&i| {
                        outline.items[i]
                            .text
                            .to_lowercase()
                            .contains(&self.filter_query.to_lowercase())
                    })
                    .collect();
            }
        } else {
            self.filtered_indices.clear();
        }
    }

    fn auto_expand_to_depth(&mut self) {
        if let Some(outline) = &self.outline {
            for (index, item) in outline.items.iter().enumerate() {
                if item.depth < self.auto_expand_depth && self.has_children(index) {
                    self.expanded_items.insert(index);
                }
            }
        }
    }

    fn has_children(&self, index: usize) -> bool {
        if let Some(outline) = &self.outline {
            let parent_depth = outline.items[index].depth;
            outline
                .items
                .get(index + 1)
                .map(|next| next.depth > parent_depth)
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn toggle_expand(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.expanded_items.contains(&index) {
            self.expanded_items.remove(&index);
        } else {
            self.expanded_items.insert(index);
        }
        cx.notify();
    }

    fn is_visible(&self, index: usize) -> bool {
        if let Some(outline) = &self.outline {
            let item_depth = outline.items[index].depth;

            // Check if any parent is collapsed
            for i in (0..index).rev() {
                let prev_item = &outline.items[i];
                if prev_item.depth < item_depth {
                    if prev_item.depth == item_depth - 1 && !self.expanded_items.contains(&i) {
                        return false;
                    }
                }
            }

            // Check filter
            if !self.filter_query.is_empty() {
                return self.filtered_indices.contains(&index);
            }

            true
        } else {
            false
        }
    }

    fn visible_items(&self) -> Vec<(usize, &OutlineItem<Anchor>)> {
        if let Some(outline) = &self.outline {
            outline
                .items
                .iter()
                .enumerate()
                .filter(|(i, _)| self.is_visible(*i))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn move_up(&mut self, _: &SelectPrevious, _: &mut Window, cx: &mut Context<Self>) {
        let visible = self.visible_items();
        if visible.is_empty() {
            return;
        }

        let current_index = self.selected_index.unwrap_or(visible[0].0);

        // Find current position in visible items
        let current_pos = visible.iter().position(|(idx, _)| *idx == current_index);

        if let Some(pos) = current_pos {
            if pos > 0 {
                self.selected_index = Some(visible[pos - 1].0);
            } else {
                // Wrap to bottom
                self.selected_index = Some(visible[visible.len() - 1].0);
            }
        } else {
            // If current selection is not visible, select first visible
            self.selected_index = Some(visible[0].0);
        }

        cx.notify();
    }

    fn move_down(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let visible = self.visible_items();
        if visible.is_empty() {
            return;
        }

        let current_index = self.selected_index.unwrap_or(visible[visible.len() - 1].0);

        // Find current position in visible items
        let current_pos = visible.iter().position(|(idx, _)| *idx == current_index);

        if let Some(pos) = current_pos {
            if pos < visible.len() - 1 {
                self.selected_index = Some(visible[pos + 1].0);
            } else {
                // Wrap to top
                self.selected_index = Some(visible[0].0);
            }
        } else {
            // If current selection is not visible, select last visible
            self.selected_index = Some(visible[visible.len() - 1].0);
        }

        cx.notify();
    }

    fn expand_selected(&mut self, _: &ExpandSelected, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            if self.has_children(index) && !self.expanded_items.contains(&index) {
                self.expanded_items.insert(index);
                cx.notify();
            }
        }
    }

    fn collapse_selected(&mut self, _: &CollapseSelected, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            if self.has_children(index) && self.expanded_items.contains(&index) {
                self.expanded_items.remove(&index);
                cx.notify();
            }
        }
    }

    fn toggle_selected(&mut self, _: &ToggleExpand, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            if self.has_children(index) {
                self.toggle_expand(index, cx);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            self.select_item(index, window, cx);
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.restore_active_editor(window, cx);
        cx.emit(DismissEvent);
    }

    fn select_item(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = Some(index);

        if let Some(outline) = &self.outline {
            let item = &outline.items[index];
            self.active_editor.update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.select_ranges([item.range.start..item.range.start]);
                });
                cx.focus_self(window);
            });
        }

        cx.emit(DismissEvent);
    }

    fn restore_active_editor(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(scroll_position) = self.prev_scroll_position {
            self.active_editor.update(cx, |editor, cx| {
                editor.set_scroll_position(scroll_position, window, cx);
            });
        }
    }

    fn render_item(
        &self,
        index: usize,
        item: &OutlineItem<Anchor>,
        cx: &mut Context<OutlineTreeView>,
    ) -> impl IntoElement {
        let has_children = self.has_children(index);
        let is_expanded = self.expanded_items.contains(&index);
        let is_selected = self.selected_index == Some(index);

        h_flex()
            .gap_2()
            .pl(px(item.depth as f32 * 20.0))
            .w_full()
            .py_1()
            .px_2()
            .rounded_md()
            .when(is_selected, |this| {
                this.bg(cx.theme().colors().element_selected)
            })
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(if has_children {
                Disclosure::new(("outline-tree", index), is_expanded)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_expand(index, cx);
                    }))
                    .into_any_element()
            } else {
                div().w(px(16.0)).into_any_element()
            })
            .child(render_item(item, None, cx))
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    this.select_item(index, window, cx);
                }),
            )
    }
}

impl Focusable for OutlineTreeView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for OutlineTreeView {}

impl Render for OutlineTreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible_items = self.visible_items();

        v_flex()
            .size_full()
            .key_context("OutlineTreeView")
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .on_action(cx.listener(Self::expand_selected))
            .on_action(cx.listener(Self::collapse_selected))
            .on_action(cx.listener(Self::toggle_selected))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .child(
                div().px_2().py_1().child(
                    // Search input placeholder for now
                    div()
                        .h_7()
                        .px_2()
                        .rounded_md()
                        .bg(cx.theme().colors().element_background)
                        .child("Search outline..."),
                ),
            )
            .child(
                div()
                    .id("outline-tree-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .child({
                        let items: Vec<_> = visible_items
                            .into_iter()
                            .map(|(index, item)| div().child(self.render_item(index, item, cx)))
                            .collect();
                        v_flex().w_full().children(items)
                    }),
            )
    }
}

pub fn render_item<T>(
    outline_item: &OutlineItem<T>,
    match_ranges: impl IntoIterator<Item = Range<usize>>,
    cx: &App,
) -> StyledText {
    let highlight_style = HighlightStyle {
        background_color: Some(cx.theme().colors().text_accent.alpha(0.3)),
        ..Default::default()
    };
    let custom_highlights = match_ranges
        .into_iter()
        .map(|range| (range, highlight_style));

    let settings = ThemeSettings::get_global(cx);

    // TODO: We probably shouldn't need to build a whole new text style here
    // but I'm not sure how to get the current one and modify it.
    // Before this change TextStyle::default() was used here, which was giving us the wrong font and text color.
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };
    let highlights = gpui::combine_highlights(
        custom_highlights,
        outline_item.highlight_ranges.iter().cloned(),
    );

    StyledText::new(outline_item.text.clone()).with_default_highlights(&text_style, highlights)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{FakeFs, Project};
    use serde_json::json;
    use util::path;
    use workspace::{AppState, Workspace};

    #[gpui::test]
    async fn test_outline_view_row_highlights(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.rs": indoc!{"
                                       // display line 0
                    struct SingleLine; // display line 1
                                       // display line 2
                    struct MultiLine { // display line 3
                        field_1: i32,  // display line 4
                        field_2: i32,  // display line 5
                    }                  // display line 6
                "}
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, "a.rs"), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let ensure_outline_view_contents =
            |outline_view: &Entity<Picker<OutlineViewDelegate>>, cx: &mut VisualTestContext| {
                assert_eq!(query(outline_view, cx), "");
                assert_eq!(
                    outline_names(outline_view, cx),
                    vec![
                        "struct SingleLine",
                        "struct MultiLine",
                        "field_1",
                        "field_2"
                    ],
                );
            };

        let outline_view = open_outline_view(&workspace, cx);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "Initially opened outline view should have no highlights"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.dispatch_action(menu::Confirm);
        // Ensures that outline still goes to entry even if no queries have been made
        assert_single_caret_at_row(&editor, 1, cx);

        let outline_view = open_outline_view(&workspace, cx);

        cx.dispatch_action(menu::SelectNext);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![3, 4, 5, 6],
            "Second struct's rows should be highlighted"
        );
        assert_single_caret_at_row(&editor, 1, cx);

        cx.dispatch_action(menu::SelectPrevious);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![1],
            "First struct's row should be highlighted"
        );
        assert_single_caret_at_row(&editor, 1, cx);

        cx.dispatch_action(menu::Cancel);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "No rows should be highlighted after outline view is cancelled and closed"
        );
        assert_single_caret_at_row(&editor, 1, cx);

        let outline_view = open_outline_view(&workspace, cx);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "Reopened outline view should have no highlights"
        );
        assert_single_caret_at_row(&editor, 1, cx);

        let expected_first_highlighted_row = 3;
        cx.dispatch_action(menu::SelectNext);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![expected_first_highlighted_row, 4, 5, 6]
        );
        assert_single_caret_at_row(&editor, 1, cx);
        cx.dispatch_action(menu::Confirm);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "No rows should be highlighted after outline view is confirmed and closed"
        );
        // On confirm, should place the caret on the first row of the highlighted rows range.
        assert_single_caret_at_row(&editor, expected_first_highlighted_row, cx);
    }

    fn open_outline_view(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<OutlineViewDelegate>> {
        cx.dispatch_action(zed_actions::outline::ToggleOutline);
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<OutlineView>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        })
    }

    fn query(
        outline_view: &Entity<Picker<OutlineViewDelegate>>,
        cx: &mut VisualTestContext,
    ) -> String {
        outline_view.update(cx, |outline_view, cx| outline_view.query(cx))
    }

    fn outline_names(
        outline_view: &Entity<Picker<OutlineViewDelegate>>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        outline_view.read_with(cx, |outline_view, _| {
            let items = &outline_view.delegate.outline.items;
            outline_view
                .delegate
                .matches
                .iter()
                .map(|hit| items[hit.candidate_id].text.clone())
                .collect::<Vec<_>>()
        })
    }

    fn highlighted_display_rows(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> Vec<u32> {
        editor.update_in(cx, |editor, window, cx| {
            editor
                .highlighted_display_rows(window, cx)
                .into_keys()
                .map(|r| r.0)
                .collect()
        })
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        })
    }

    fn rust_lang() -> Arc<Language> {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query(
                r#"(struct_item
            (visibility_modifier)? @context
            "struct" @context
            name: (_) @name) @item

        (enum_item
            (visibility_modifier)? @context
            "enum" @context
            name: (_) @name) @item

        (enum_variant
            (visibility_modifier)? @context
            name: (_) @name) @item

        (impl_item
            "impl" @context
            trait: (_)? @name
            "for"? @context
            type: (_) @name) @item

        (trait_item
            (visibility_modifier)? @context
            "trait" @context
            name: (_) @name) @item

        (function_item
            (visibility_modifier)? @context
            (function_modifiers)? @context
            "fn" @context
            name: (_) @name) @item

        (function_signature_item
            (visibility_modifier)? @context
            (function_modifiers)? @context
            "fn" @context
            name: (_) @name) @item

        (macro_definition
            . "macro_rules!" @context
            name: (_) @name) @item

        (mod_item
            (visibility_modifier)? @context
            "mod" @context
            name: (_) @name) @item

        (type_item
            (visibility_modifier)? @context
            "type" @context
            name: (_) @name) @item

        (associated_type
            "type" @context
            name: (_) @name) @item

        (const_item
            (visibility_modifier)? @context
            "const" @context
            name: (_) @name) @item

        (field_declaration
            (visibility_modifier)? @context
            name: (_) @name) @item
"#,
            )
            .unwrap(),
        )
    }

    #[track_caller]
    fn assert_single_caret_at_row(
        editor: &Entity<Editor>,
        buffer_row: u32,
        cx: &mut VisualTestContext,
    ) {
        let selections = editor.update(cx, |editor, cx| {
            editor
                .selections
                .all::<rope::Point>(cx)
                .into_iter()
                .map(|s| s.start..s.end)
                .collect::<Vec<_>>()
        });
        assert!(
            selections.len() == 1,
            "Expected one caret selection but got: {selections:?}"
        );
        let selection = &selections[0];
        assert!(
            selection.start == selection.end,
            "Expected a single caret selection, but got: {selection:?}"
        );
        assert_eq!(selection.start.row, buffer_row);
    }
}
