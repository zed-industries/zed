use std::ops::Range;
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};

use editor::scroll::ScrollOffset;
use editor::{Anchor, AnchorRangeExt, Editor, scroll::Autoscroll};
use editor::{RowHighlightOptions, SelectionEffects};
use fuzzy::StringMatch;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, HighlightStyle,
    ParentElement, Point, Render, Styled, StyledText, Task, TextStyle, WeakEntity, Window, div,
    rems,
};
use language::{Outline, OutlineItem};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::Settings;
use theme::{ActiveTheme, ThemeSettings};
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{DismissDecision, ModalView, Workspace};

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

    let workspace = window.root::<Workspace>().flatten();
    if let Some((workspace, outline)) = workspace.zip(outline) {
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                OutlineView::new(outline, editor, window, cx)
            });
        })
    }
}

pub struct OutlineView {
    picker: Entity<Picker<OutlineViewDelegate>>,
}

impl Focusable for OutlineView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for OutlineView {}
impl ModalView for OutlineView {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.restore_active_editor(window, cx)
        });
        DismissDecision::Dismiss(true)
    }
}

impl Render for OutlineView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(34.))
            .on_action(cx.listener(
                |_this: &mut OutlineView,
                 _: &zed_actions::outline::ToggleOutline,
                 _window: &mut Window,
                 cx: &mut Context<OutlineView>| {
                    // When outline::Toggle is triggered while the outline is open, dismiss it
                    cx.emit(DismissEvent);
                },
            ))
            .child(self.picker.clone())
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
        let delegate = OutlineViewDelegate::new(cx.entity().downgrade(), outline, editor, cx);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).max_height(Some(vh(0.75, window)))
        });
        OutlineView { picker }
    }
}

struct OutlineViewDelegate {
    outline_view: WeakEntity<OutlineView>,
    active_editor: Entity<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    prev_scroll_position: Option<Point<ScrollOffset>>,
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
                let cursor_offset = editor
                    .selections
                    .newest::<usize>(&editor.display_snapshot(cx))
                    .head();
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
                active_editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::center()),
                    window,
                    cx,
                    |s| s.select_ranges([rows.start..rows.start]),
                );
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
    use util::{path, rel_path::rel_path};
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
                workspace.open_path((worktree_id, rel_path("a.rs")), None, true, window, cx)
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
            crate::init(cx);
            editor::init(cx);
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
                .all::<rope::Point>(&editor.display_snapshot(cx))
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
