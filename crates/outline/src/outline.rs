use editor::{
    display_map::ToDisplayPoint, scroll::Autoscroll, Anchor, AnchorRangeExt, DisplayPoint, Editor,
    EditorMode, ToPoint,
};
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
            editor.highlight_rows(None);
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
            if let Some(rows) = active_editor.highlighted_rows() {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let position = DisplayPoint::new(rows.start, 0).to_point(&snapshot);
                active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([position..position])
                });
                active_editor.highlight_rows(None);
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
            font_features: settings.buffer_font.features,
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.).into(),
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
                        .text_ui()
                        .pl(rems(outline_item.depth as f32))
                        .child(styled_text),
                ),
        )
    }
}
