use editor::{scroll::Autoscroll, Anchor, AnchorRangeExt, Editor, EditorMode};
use fuzzy::StringMatch;
use gpui::{
    actions, div, rems, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    FontStyle, FontWeight, HighlightStyle, ParentElement, Point, Render, Styled, StyledText, Task,
    TextStyle, View, ViewContext, VisualContext, WeakView, WhiteSpace, WindowContext,
};
use language::Outline;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::Settings;
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};

use theme::{color_alpha, ActiveTheme, ThemeSettings};
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{DismissDecision, ModalView};

actions!(outline, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(OutlineView::register).detach();
}

pub fn toggle(editor: View<Editor>, _: &Toggle, cx: &mut WindowContext) {
    let outline = editor
        .read(cx)
        .buffer()
        .read(cx)
        .snapshot(cx)
        .outline(Some(&cx.theme().syntax()));

    if let Some((workspace, outline)) = editor.read(cx).workspace().zip(outline) {
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(cx, |cx| OutlineView::new(outline, editor, cx));
        })
    }
}

pub struct OutlineView {
    picker: View<Picker<OutlineViewDelegate>>,
}

impl FocusableView for OutlineView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for OutlineView {}
impl ModalView for OutlineView {
    fn on_before_dismiss(&mut self, cx: &mut ViewContext<Self>) -> DismissDecision {
        self.picker
            .update(cx, |picker, cx| picker.delegate.restore_active_editor(cx));
        DismissDecision::Dismiss(true)
    }
}

impl Render for OutlineView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl OutlineView {
    fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        if editor.mode() == EditorMode::Full {
            let handle = cx.view().downgrade();
            editor.register_action(move |action, cx| {
                if let Some(editor) = handle.upgrade() {
                    toggle(editor, action, cx);
                }
            });
        }
    }

    fn new(
        outline: Outline<Anchor>,
        editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> OutlineView {
        let delegate = OutlineViewDelegate::new(cx.view().downgrade(), outline, editor, cx);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(vh(0.75, cx)));
        OutlineView { picker }
    }
}

struct OutlineViewDelegate {
    outline_view: WeakView<OutlineView>,
    active_editor: View<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    prev_scroll_position: Option<Point<f32>>,
    matches: Vec<StringMatch>,
    last_query: String,
}

enum OutlineRowHighlights {}

impl OutlineViewDelegate {
    fn new(
        outline_view: WeakView<OutlineView>,
        outline: Outline<Anchor>,
        editor: View<Editor>,
        cx: &mut ViewContext<OutlineView>,
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

    fn restore_active_editor(&mut self, cx: &mut WindowContext) {
        self.active_editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<OutlineRowHighlights>();
            if let Some(scroll_position) = self.prev_scroll_position {
                editor.set_scroll_position(scroll_position, cx);
            }
        })
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        navigate: bool,
        cx: &mut ViewContext<Picker<OutlineViewDelegate>>,
    ) {
        self.selected_match_index = ix;

        if navigate && !self.matches.is_empty() {
            let selected_match = &self.matches[self.selected_match_index];
            let outline_item = &self.outline.items[selected_match.candidate_id];

            self.active_editor.update(cx, |active_editor, cx| {
                active_editor.clear_row_highlights::<OutlineRowHighlights>();
                active_editor.highlight_rows::<OutlineRowHighlights>(
                    outline_item.range.start..=outline_item.range.end,
                    Some(cx.theme().colors().editor_highlighted_line_background),
                    cx,
                );
                active_editor.request_autoscroll(Autoscroll::center(), cx);
            });
        }
    }
}

impl PickerDelegate for OutlineViewDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search buffer symbols...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<OutlineViewDelegate>>) {
        self.set_selected_index(ix, true, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<OutlineViewDelegate>>,
    ) -> Task<()> {
        let selected_index;
        if query.is_empty() {
            self.restore_active_editor(cx);
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

            let editor = self.active_editor.read(cx);
            let cursor_offset = editor.selections.newest::<usize>(cx).head();
            let buffer = editor.buffer().read(cx).snapshot(cx);
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

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<OutlineViewDelegate>>) {
        self.prev_scroll_position.take();

        self.active_editor.update(cx, |active_editor, cx| {
            if let Some(rows) = active_editor
                .highlighted_rows::<OutlineRowHighlights>()
                .and_then(|highlights| highlights.into_iter().next().map(|(rows, _)| rows.clone()))
            {
                active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([*rows.start()..*rows.start()])
                });
                active_editor.clear_row_highlights::<OutlineRowHighlights>();
                active_editor.focus(cx);
            }
        });

        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<OutlineViewDelegate>>) {
        self.outline_view
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
        self.restore_active_editor(cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let settings = ThemeSettings::get_global(cx);

        // TODO: We probably shouldn't need to build a whole new text style here
        // but I'm not sure how to get the current one and modify it.
        // Before this change TextStyle::default() was used here, which was giving us the wrong font and text color.
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };

        let mut highlight_style = HighlightStyle::default();
        highlight_style.background_color = Some(color_alpha(cx.theme().colors().text_accent, 0.3));

        let mat = &self.matches[ix];
        let outline_item = &self.outline.items[mat.candidate_id];

        let highlights = gpui::combine_highlights(
            mat.ranges().map(|range| (range, highlight_style)),
            outline_item.highlight_ranges.iter().cloned(),
        );

        let styled_text =
            StyledText::new(outline_item.text.clone()).with_highlights(&text_style, highlights);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(
                    div()
                        .text_ui(cx)
                        .pl(rems(outline_item.depth as f32))
                        .child(styled_text),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use collections::HashSet;
    use gpui::{TestAppContext, VisualTestContext};
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{FakeFs, Project};
    use serde_json::json;
    use workspace::{AppState, Workspace};

    use super::*;

    #[gpui::test]
    async fn test_outline_view_row_highlights(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": indoc!{"
                    struct SingleLine; // display line 0
                                       // display line 1
                    struct MultiLine { // display line 2
                        field_1: i32,  // display line 3
                        field_2: i32,  // display line 4
                    }                  // display line 5
                "}
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees().next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();
        let editor = workspace
            .update(cx, |workspace, cx| {
                workspace.open_path((worktree_id, "a.rs"), None, true, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let ensure_outline_view_contents =
            |outline_view: &View<Picker<OutlineViewDelegate>>, cx: &mut VisualTestContext| {
                assert_eq!(query(&outline_view, cx), "");
                assert_eq!(
                    outline_names(&outline_view, cx),
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

        cx.dispatch_action(menu::SelectNext);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![2, 3, 4, 5],
            "Second struct's rows should be highlighted"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.dispatch_action(menu::SelectPrev);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![0],
            "First struct's row should be highlighted"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.dispatch_action(menu::Cancel);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "No rows should be highlighted after outline view is cancelled and closed"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        let outline_view = open_outline_view(&workspace, cx);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "Reopened outline view should have no highlights"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        let expected_first_highlighted_row = 2;
        cx.dispatch_action(menu::SelectNext);
        ensure_outline_view_contents(&outline_view, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![expected_first_highlighted_row, 3, 4, 5]
        );
        assert_single_caret_at_row(&editor, 0, cx);
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
        workspace: &View<Workspace>,
        cx: &mut VisualTestContext,
    ) -> View<Picker<OutlineViewDelegate>> {
        cx.dispatch_action(Toggle);
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
        outline_view: &View<Picker<OutlineViewDelegate>>,
        cx: &mut VisualTestContext,
    ) -> String {
        outline_view.update(cx, |outline_view, cx| outline_view.query(cx))
    }

    fn outline_names(
        outline_view: &View<Picker<OutlineViewDelegate>>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        outline_view.update(cx, |outline_view, _| {
            let items = &outline_view.delegate.outline.items;
            outline_view
                .delegate
                .matches
                .iter()
                .map(|hit| items[hit.candidate_id].text.clone())
                .collect::<Vec<_>>()
        })
    }

    fn highlighted_display_rows(editor: &View<Editor>, cx: &mut VisualTestContext) -> Vec<u32> {
        editor.update(cx, |editor, cx| {
            editor
                .highlighted_display_rows(HashSet::default(), cx)
                .into_keys()
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
                Some(tree_sitter_rust::language()),
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
        editor: &View<Editor>,
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
