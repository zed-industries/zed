use editor::{
    combine_syntax_and_fuzzy_match_highlights, display_map::ToDisplayPoint,
    scroll::autoscroll::Autoscroll, Anchor, AnchorRangeExt, DisplayPoint, Editor, ToPoint,
};
use fuzzy::StringMatch;
use gpui::{
    actions, elements::*, geometry::vector::Vector2F, AppContext, MouseState, Task, ViewContext,
    ViewHandle, WindowContext,
};
use language::Outline;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate, PickerEvent};
use settings::Settings;
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};
use workspace::Workspace;

actions!(outline, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle);
    OutlineView::init(cx);
}

fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    if let Some(editor) = workspace
        .active_item(cx)
        .and_then(|item| item.downcast::<Editor>())
    {
        let outline = editor
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
            .outline(Some(cx.global::<Settings>().theme.editor.syntax.as_ref()));
        if let Some(outline) = outline {
            workspace.toggle_modal(cx, |_, cx| {
                cx.add_view(|cx| {
                    OutlineView::new(OutlineViewDelegate::new(outline, editor, cx), cx)
                        .with_max_size(800., 1200.)
                })
            });
        }
    }
}

type OutlineView = Picker<OutlineViewDelegate>;

struct OutlineViewDelegate {
    active_editor: ViewHandle<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    prev_scroll_position: Option<Vector2F>,
    matches: Vec<StringMatch>,
    last_query: String,
}

impl OutlineViewDelegate {
    fn new(
        outline: Outline<Anchor>,
        editor: ViewHandle<Editor>,
        cx: &mut ViewContext<OutlineView>,
    ) -> Self {
        Self {
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
            editor.highlight_rows(None);
            if let Some(scroll_position) = self.prev_scroll_position {
                editor.set_scroll_position(scroll_position, cx);
            }
        })
    }

    fn set_selected_index(&mut self, ix: usize, navigate: bool, cx: &mut ViewContext<OutlineView>) {
        self.selected_match_index = ix;
        if navigate && !self.matches.is_empty() {
            let selected_match = &self.matches[self.selected_match_index];
            let outline_item = &self.outline.items[selected_match.candidate_id];
            self.active_editor.update(cx, |active_editor, cx| {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let buffer_snapshot = &snapshot.buffer_snapshot;
                let start = outline_item.range.start.to_point(buffer_snapshot);
                let end = outline_item.range.end.to_point(buffer_snapshot);
                let display_rows = start.to_display_point(&snapshot).row()
                    ..end.to_display_point(&snapshot).row() + 1;
                active_editor.highlight_rows(Some(display_rows));
                active_editor.request_autoscroll(Autoscroll::center(), cx);
            });
        }
    }
}

impl PickerDelegate for OutlineViewDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search buffer symbols...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<OutlineView>) {
        self.set_selected_index(ix, true, cx);
    }

    fn center_selection_after_match_updates(&self) -> bool {
        true
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<OutlineView>) -> Task<()> {
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
            self.matches = smol::block_on(self.outline.search(&query, cx.background().clone()));
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

    fn confirm(&mut self, cx: &mut ViewContext<OutlineView>) {
        self.prev_scroll_position.take();
        self.active_editor.update(cx, |active_editor, cx| {
            if let Some(rows) = active_editor.highlighted_rows() {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let position = DisplayPoint::new(rows.start, 0).to_point(&snapshot);
                active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([position..position])
                });
                active_editor.highlight_rows(None);
            }
        });
        cx.emit(PickerEvent::Dismiss);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<OutlineView>) {
        self.restore_active_editor(cx);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let settings = cx.global::<Settings>();
        let string_match = &self.matches[ix];
        let style = settings.theme.picker.item.style_for(mouse_state, selected);
        let outline_item = &self.outline.items[string_match.candidate_id];

        Text::new(outline_item.text.clone(), style.label.text.clone())
            .with_soft_wrap(false)
            .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                &outline_item.text,
                style.label.text.clone().into(),
                outline_item.highlight_ranges.iter().cloned(),
                &string_match.positions,
            ))
            .contained()
            .with_padding_left(20. * outline_item.depth as f32)
            .contained()
            .with_style(style.container)
            .into_any()
    }
}
