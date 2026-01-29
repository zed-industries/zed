use std::cmp;

use collections::{HashMap, HashSet};
use gpui::{
    AbsoluteLength, AnyElement, App, AvailableSpace, Bounds, Context, DragMoveEvent, Element,
    Entity, GlobalElementId, Hsla, InspectorElementId, IntoElement, LayoutId, Length,
    ParentElement, Pixels, StatefulInteractiveElement, Styled, TextStyleRefinement, Window, div,
    linear_color_stop, linear_gradient, point, px, size,
};
use multi_buffer::{Anchor, ExcerptId};
use settings::Settings;
use text::BufferId;
use theme::ActiveTheme;
use ui::scrollbars::ShowScrollbar;
use ui::{h_flex, prelude::*, v_flex};

use gpui::ContentMask;

use crate::{
    DisplayRow, Editor, EditorSettings, EditorSnapshot, EditorStyle, FILE_HEADER_HEIGHT,
    MULTI_BUFFER_EXCERPT_HEADER_HEIGHT, RowExt, StickyHeaderExcerpt,
    display_map::Block,
    element::{EditorElement, SplitSide, header_jump_data, render_buffer_header},
    scroll::ScrollOffset,
    split::SplittableEditor,
};

const RESIZE_HANDLE_WIDTH: f32 = 8.0;

#[derive(Debug, Clone)]
struct DraggedSplitHandle;

pub struct SplitEditorState {
    left_ratio: f32,
    visible_left_ratio: f32,
    cached_width: Pixels,
}

impl SplitEditorState {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            left_ratio: 0.5,
            visible_left_ratio: 0.5,
            cached_width: px(0.),
        }
    }

    #[allow(clippy::misnamed_getters)]
    pub fn left_ratio(&self) -> f32 {
        self.visible_left_ratio
    }

    pub fn right_ratio(&self) -> f32 {
        1.0 - self.visible_left_ratio
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedSplitHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;
        let bounds_width = bounds.right() - bounds.left();

        if bounds_width > px(0.) {
            self.cached_width = bounds_width;
        }

        let min_ratio = 0.1;
        let max_ratio = 0.9;

        let new_ratio = (drag_position.x - bounds.left()) / bounds_width;
        self.visible_left_ratio = new_ratio.clamp(min_ratio, max_ratio);
    }

    fn commit_ratio(&mut self) {
        self.left_ratio = self.visible_left_ratio;
    }

    fn on_double_click(&mut self) {
        self.left_ratio = 0.5;
        self.visible_left_ratio = 0.5;
    }
}

#[derive(IntoElement)]
pub struct SplitEditorView {
    splittable_editor: Entity<SplittableEditor>,
    style: EditorStyle,
    split_state: Entity<SplitEditorState>,
}

impl SplitEditorView {
    pub fn new(
        splittable_editor: Entity<SplittableEditor>,
        style: EditorStyle,
        split_state: Entity<SplitEditorState>,
    ) -> Self {
        Self {
            splittable_editor,
            style,
            split_state,
        }
    }
}

fn render_resize_handle(
    state: &Entity<SplitEditorState>,
    separator_color: Hsla,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let state_for_click = state.clone();

    div()
        .id("split-resize-container")
        .relative()
        .h_full()
        .flex_shrink_0()
        .w(px(1.))
        .bg(separator_color)
        .child(
            div()
                .id("split-resize-handle")
                .absolute()
                .left(px(-RESIZE_HANDLE_WIDTH / 2.0))
                .w(px(RESIZE_HANDLE_WIDTH))
                .h_full()
                .cursor_col_resize()
                .block_mouse_except_scroll()
                .on_click(move |event, _, cx| {
                    if event.click_count() >= 2 {
                        state_for_click.update(cx, |state, _| {
                            state.on_double_click();
                        });
                    }
                    cx.stop_propagation();
                })
                .on_drag(DraggedSplitHandle, |_, _, _, cx| cx.new(|_| gpui::Empty)),
        )
        .into_any_element()
}

impl RenderOnce for SplitEditorView {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let splittable_editor = self.splittable_editor.read(cx);

        assert!(
            splittable_editor.lhs_editor().is_some(),
            "`SplitEditorView` requires `SplittableEditor` to be in split mode"
        );

        let lhs_editor = splittable_editor.lhs_editor().unwrap().clone();
        let rhs_editor = splittable_editor.rhs_editor().clone();

        let mut lhs = EditorElement::new(&lhs_editor, self.style.clone());
        let mut rhs = EditorElement::new(&rhs_editor, self.style.clone());

        lhs.set_split_side(SplitSide::Left);
        rhs.set_split_side(SplitSide::Right);

        let left_ratio = self.split_state.read(cx).left_ratio();
        let right_ratio = self.split_state.read(cx).right_ratio();

        let separator_color = cx.theme().colors().border_variant;

        let resize_handle = render_resize_handle(&self.split_state, separator_color, window, cx);

        let state_for_drag = self.split_state.downgrade();
        let state_for_drop = self.split_state.downgrade();

        let buffer_headers = SplitBufferHeadersElement::new(rhs_editor, self.style.clone());

        div()
            .id("split-editor-view-container")
            .size_full()
            .relative()
            .child(
                h_flex()
                    .id("split-editor-view")
                    .size_full()
                    .on_drag_move::<DraggedSplitHandle>(move |event, window, cx| {
                        state_for_drag
                            .update(cx, |state, cx| {
                                state.on_drag_move(event, window, cx);
                            })
                            .ok();
                    })
                    .on_drop::<DraggedSplitHandle>(move |_, _, cx| {
                        state_for_drop
                            .update(cx, |state, _| {
                                state.commit_ratio();
                            })
                            .ok();
                    })
                    .child(
                        div()
                            .id("split-editor-left")
                            .flex_shrink()
                            .min_w_0()
                            .h_full()
                            .flex_basis(DefiniteLength::Fraction(left_ratio))
                            .overflow_hidden()
                            .child(lhs),
                    )
                    .child(resize_handle)
                    .child(
                        div()
                            .id("split-editor-right")
                            .flex_shrink()
                            .min_w_0()
                            .h_full()
                            .flex_basis(DefiniteLength::Fraction(right_ratio))
                            .overflow_hidden()
                            .child(rhs),
                    ),
            )
            .child(buffer_headers)
    }
}

struct SplitBufferHeadersElement {
    editor: Entity<Editor>,
    style: EditorStyle,
}

impl SplitBufferHeadersElement {
    fn new(editor: Entity<Editor>, style: EditorStyle) -> Self {
        Self { editor, style }
    }
}

struct BufferHeaderLayout {
    element: AnyElement,
}

struct SplitBufferHeadersPrepaintState {
    sticky_header: Option<AnyElement>,
    non_sticky_headers: Vec<BufferHeaderLayout>,
}

impl IntoElement for SplitBufferHeadersElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SplitBufferHeadersElement {
    type RequestLayoutState = ();
    type PrepaintState = SplitBufferHeadersPrepaintState;

    fn id(&self) -> Option<gpui::ElementId> {
        Some("split-buffer-headers".into())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = gpui::Style::default();
        style.position = gpui::Position::Absolute;
        style.inset.top = DefiniteLength::Fraction(0.0).into();
        style.inset.left = DefiniteLength::Fraction(0.0).into();
        style.size.width = Length::Definite(DefiniteLength::Fraction(1.0));
        style.size.height = Length::Definite(DefiniteLength::Fraction(1.0));
        let layout_id = window.request_layout(style, [], _cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if bounds.size.width <= px(0.) || bounds.size.height <= px(0.) {
            return SplitBufferHeadersPrepaintState {
                sticky_header: None,
                non_sticky_headers: Vec::new(),
            };
        }

        let rem_size = self.rem_size();
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };

        window.with_rem_size(rem_size, |window| {
            window.with_text_style(Some(text_style), |window| {
                Self::prepaint_inner(self, bounds, window, cx)
            })
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let rem_size = self.rem_size();
        let text_style = TextStyleRefinement {
            font_size: Some(self.style.text.font_size),
            line_height: Some(self.style.text.line_height),
            ..Default::default()
        };

        window.with_rem_size(rem_size, |window| {
            window.with_text_style(Some(text_style), |window| {
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    for header_layout in &mut prepaint.non_sticky_headers {
                        header_layout.element.paint(window, cx);
                    }

                    if let Some(mut sticky_header) = prepaint.sticky_header.take() {
                        sticky_header.paint(window, cx);
                    }
                });
            });
        });
    }
}

impl SplitBufferHeadersElement {
    fn rem_size(&self) -> Option<Pixels> {
        match self.style.text.font_size {
            AbsoluteLength::Pixels(pixels) => {
                let rem_size_scale = {
                    let default_font_size_scale = 14. / ui::BASE_REM_SIZE_IN_PX;
                    let default_font_size_delta = 1. - default_font_size_scale;
                    1. + default_font_size_delta
                };

                Some(pixels * rem_size_scale)
            }
            AbsoluteLength::Rems(rems) => Some(rems.to_pixels(ui::BASE_REM_SIZE_IN_PX.into())),
        }
    }

    fn prepaint_inner(
        &mut self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> SplitBufferHeadersPrepaintState {
        let line_height = window.line_height();

        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let scroll_position = snapshot.scroll_position();

        // Compute right margin to avoid overlapping the scrollbar
        let settings = EditorSettings::get_global(cx);
        let scrollbars_shown = settings.scrollbar.show != ShowScrollbar::Never;
        let vertical_scrollbar_width = (scrollbars_shown
            && settings.scrollbar.axes.vertical
            && self.editor.read(cx).show_scrollbars.vertical)
            .then_some(EditorElement::SCROLLBAR_WIDTH)
            .unwrap_or_default();
        let available_width = bounds.size.width - vertical_scrollbar_width;

        let visible_height_in_lines = bounds.size.height / line_height;
        let max_row = snapshot.max_point().row();
        let start_row = cmp::min(DisplayRow(scroll_position.y.floor() as u32), max_row);
        let end_row = cmp::min(
            (scroll_position.y + visible_height_in_lines as f64).ceil() as u32,
            max_row.next_row().0,
        );
        let end_row = DisplayRow(end_row);

        let (selected_buffer_ids, latest_selection_anchors) =
            self.compute_selection_info(&snapshot, cx);

        let sticky_header = if snapshot.buffer_snapshot().show_headers() {
            snapshot
                .sticky_header_excerpt(scroll_position.y)
                .map(|sticky_excerpt| {
                    self.build_sticky_header(
                        sticky_excerpt,
                        &snapshot,
                        scroll_position,
                        bounds,
                        available_width,
                        line_height,
                        &selected_buffer_ids,
                        &latest_selection_anchors,
                        start_row,
                        end_row,
                        window,
                        cx,
                    )
                })
        } else {
            None
        };

        let sticky_header_excerpt_id = snapshot
            .sticky_header_excerpt(scroll_position.y)
            .map(|e| e.excerpt.id);

        let non_sticky_headers = self.build_non_sticky_headers(
            &snapshot,
            scroll_position,
            bounds,
            available_width,
            line_height,
            start_row,
            end_row,
            &selected_buffer_ids,
            &latest_selection_anchors,
            sticky_header_excerpt_id,
            window,
            cx,
        );

        SplitBufferHeadersPrepaintState {
            sticky_header,
            non_sticky_headers,
        }
    }

    fn compute_selection_info(
        &self,
        snapshot: &EditorSnapshot,
        cx: &App,
    ) -> (HashSet<BufferId>, HashMap<BufferId, Anchor>) {
        let editor = self.editor.read(cx);
        let all_selections = editor
            .selections
            .all::<crate::Point>(&snapshot.display_snapshot);
        let all_anchor_selections = editor.selections.all_anchors(&snapshot.display_snapshot);

        let mut selected_buffer_ids = HashSet::default();
        for selection in &all_selections {
            for buffer_id in snapshot
                .buffer_snapshot()
                .buffer_ids_for_range(selection.range())
            {
                selected_buffer_ids.insert(buffer_id);
            }
        }

        let mut anchors_by_buffer: HashMap<BufferId, (usize, Anchor)> = HashMap::default();
        for selection in all_anchor_selections.iter() {
            let head = selection.head();
            if let Some(buffer_id) = head.text_anchor.buffer_id {
                anchors_by_buffer
                    .entry(buffer_id)
                    .and_modify(|(latest_id, latest_anchor)| {
                        if selection.id > *latest_id {
                            *latest_id = selection.id;
                            *latest_anchor = head;
                        }
                    })
                    .or_insert((selection.id, head));
            }
        }
        let latest_selection_anchors = anchors_by_buffer
            .into_iter()
            .map(|(buffer_id, (_, anchor))| (buffer_id, anchor))
            .collect();

        (selected_buffer_ids, latest_selection_anchors)
    }

    fn build_sticky_header(
        &self,
        StickyHeaderExcerpt { excerpt }: StickyHeaderExcerpt<'_>,
        snapshot: &EditorSnapshot,
        scroll_position: gpui::Point<ScrollOffset>,
        bounds: Bounds<Pixels>,
        available_width: Pixels,
        line_height: Pixels,
        selected_buffer_ids: &HashSet<BufferId>,
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
        start_row: DisplayRow,
        end_row: DisplayRow,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let jump_data = header_jump_data(
            snapshot,
            DisplayRow(scroll_position.y as u32),
            FILE_HEADER_HEIGHT + MULTI_BUFFER_EXCERPT_HEADER_HEIGHT,
            excerpt,
            latest_selection_anchors,
        );

        let editor_bg_color = cx.theme().colors().editor_background;
        let selected = selected_buffer_ids.contains(&excerpt.buffer_id);

        let mut header = v_flex()
            .id("sticky-buffer-header")
            .w(available_width)
            .relative()
            .child(
                div()
                    .w(available_width)
                    .h(FILE_HEADER_HEIGHT as f32 * line_height)
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(editor_bg_color.opacity(0.), 0.),
                        linear_color_stop(editor_bg_color, 0.6),
                    ))
                    .absolute()
                    .top_0(),
            )
            .child(
                render_buffer_header(
                    &self.editor,
                    excerpt,
                    false,
                    selected,
                    true,
                    jump_data,
                    window,
                    cx,
                )
                .into_any_element(),
            )
            .into_any_element();

        let mut origin = bounds.origin;

        for (block_row, block) in snapshot.blocks_in_range(start_row..end_row) {
            if !block.is_buffer_header() {
                continue;
            }

            if block_row.0 <= scroll_position.y as u32 {
                continue;
            }

            let max_row = block_row.0.saturating_sub(FILE_HEADER_HEIGHT);
            let offset = scroll_position.y - max_row as f64;

            if offset > 0.0 {
                origin.y -= Pixels::from(offset * f64::from(line_height));
            }
            break;
        }

        let available_size = size(
            AvailableSpace::Definite(available_width),
            AvailableSpace::MinContent,
        );

        header.prepaint_as_root(origin, available_size, window, cx);

        header
    }

    fn build_non_sticky_headers(
        &self,
        snapshot: &EditorSnapshot,
        scroll_position: gpui::Point<ScrollOffset>,
        bounds: Bounds<Pixels>,
        available_width: Pixels,
        line_height: Pixels,
        start_row: DisplayRow,
        end_row: DisplayRow,
        selected_buffer_ids: &HashSet<BufferId>,
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
        sticky_header_excerpt_id: Option<ExcerptId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<BufferHeaderLayout> {
        let mut headers = Vec::new();

        for (block_row, block) in snapshot.blocks_in_range(start_row..end_row) {
            let (excerpt, is_folded) = match block {
                Block::BufferHeader { excerpt, .. } => {
                    if sticky_header_excerpt_id == Some(excerpt.id) {
                        continue;
                    }
                    (excerpt, false)
                }
                Block::FoldedBuffer { first_excerpt, .. } => (first_excerpt, true),
                // ExcerptBoundary is just a separator line, not a buffer header
                Block::ExcerptBoundary { .. } | Block::Custom(_) | Block::Spacer { .. } => continue,
            };

            let selected = selected_buffer_ids.contains(&excerpt.buffer_id);
            let jump_data = header_jump_data(
                snapshot,
                block_row,
                block.height(),
                excerpt,
                latest_selection_anchors,
            );

            let mut header = render_buffer_header(
                &self.editor,
                excerpt,
                is_folded,
                selected,
                false,
                jump_data,
                window,
                cx,
            )
            .into_any_element();

            let y_offset = (block_row.0 as f64 - scroll_position.y) * f64::from(line_height);
            let origin = point(bounds.origin.x, bounds.origin.y + Pixels::from(y_offset));

            let available_size = size(
                AvailableSpace::Definite(available_width),
                AvailableSpace::MinContent,
            );

            header.prepaint_as_root(origin, available_size, window, cx);

            headers.push(BufferHeaderLayout { element: header });
        }

        headers
    }
}
