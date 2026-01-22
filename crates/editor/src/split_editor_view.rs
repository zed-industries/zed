use std::{cmp, collections::HashMap, path, path::Path};

use collections::HashSet;
use file_icons::FileIcons;
use git::status::FileStatus;
use gpui::{
    AbsoluteLength, Action, AnyElement, App, AvailableSpace, Bounds, ClickEvent, ClipboardItem,
    Context, DragMoveEvent, Element, Entity, Focusable, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId, Length, Modifiers, MouseButton, ParentElement, Pixels,
    StatefulInteractiveElement, Styled, TextStyleRefinement, Window, div, linear_color_stop,
    linear_gradient, point, px, size,
};
use multi_buffer::{Anchor, ExcerptId, ExcerptInfo};
use project::Entry;
use settings::Settings;
use text::BufferId;
use theme::ActiveTheme;
use ui::scrollbars::ShowScrollbar;
use ui::{
    Button, ButtonLike, ButtonStyle, ContextMenu, Icon, IconName, Indicator, KeyBinding, Label,
    Tooltip, h_flex, prelude::*, right_click_menu, text_for_keystroke, v_flex,
};
use workspace::{ItemSettings, OpenInTerminal, OpenTerminal, RevealInProjectPanel};

use crate::{
    DisplayRow, Editor, EditorSettings, EditorSnapshot, EditorStyle, FILE_HEADER_HEIGHT, JumpData,
    MULTI_BUFFER_EXCERPT_HEADER_HEIGHT, OpenExcerpts, RowExt, StickyHeaderExcerpt, ToggleFold,
    ToggleFoldAll,
    display_map::Block,
    element::{EditorElement, SplitSide},
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
            splittable_editor.secondary_editor().is_some(),
            "`SplitEditorView` requires `SplittableEditor` to be in split mode"
        );

        let lhs_editor = splittable_editor.secondary_editor().unwrap().clone();
        let rhs_editor = splittable_editor.primary_editor().clone();

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
        _bounds: Bounds<Pixels>,
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
                for header_layout in &mut prepaint.non_sticky_headers {
                    header_layout.element.paint(window, cx);
                }

                if let Some(mut sticky_header) = prepaint.sticky_header.take() {
                    sticky_header.paint(window, cx);
                }
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
                self.render_buffer_header(excerpt, false, selected, true, jump_data, window, cx)
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

            let mut header = self
                .render_buffer_header(excerpt, is_folded, selected, false, jump_data, window, cx)
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

    fn render_buffer_header(
        &self,
        for_excerpt: &ExcerptInfo,
        is_folded: bool,
        is_selected: bool,
        is_sticky: bool,
        jump_data: JumpData,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let editor = self.editor.read(cx);
        let multi_buffer = editor.buffer.read(cx);
        let is_read_only = self.editor.read(cx).read_only(cx);

        let file_status = multi_buffer
            .all_diff_hunks_expanded()
            .then(|| editor.status_for_buffer_id(for_excerpt.buffer_id, cx))
            .flatten();
        let indicator = multi_buffer
            .buffer(for_excerpt.buffer_id)
            .and_then(|buffer| {
                let buffer = buffer.read(cx);
                let indicator_color = match (buffer.has_conflict(), buffer.is_dirty()) {
                    (true, _) => Some(Color::Warning),
                    (_, true) => Some(Color::Accent),
                    (false, false) => None,
                };
                indicator_color.map(|indicator_color| Indicator::dot().color(indicator_color))
            });

        let include_root = editor
            .project
            .as_ref()
            .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
            .unwrap_or_default();
        let file = for_excerpt.buffer.file();
        let can_open_excerpts = file.is_none_or(|file| file.can_open());
        let path_style = file.map(|file| file.path_style(cx));
        let relative_path = for_excerpt.buffer.resolve_file_path(include_root, cx);
        let (parent_path, filename) = if let Some(path) = &relative_path {
            if let Some(path_style) = path_style {
                let (dir, file_name) = path_style.split(path);
                (dir.map(|dir| dir.to_owned()), Some(file_name.to_owned()))
            } else {
                (None, Some(path.clone()))
            }
        } else {
            (None, None)
        };
        let focus_handle = self.editor.read(cx).focus_handle(cx);
        let colors = cx.theme().colors();

        let header = div()
            .p_1()
            .w_full()
            .h(FILE_HEADER_HEIGHT as f32 * window.line_height())
            .child(
                h_flex()
                    .size_full()
                    .flex_basis(Length::Definite(DefiniteLength::Fraction(0.667)))
                    .pl_1()
                    .pr_2()
                    .rounded_sm()
                    .gap_1p5()
                    .when(is_sticky, |el| el.shadow_md())
                    .border_1()
                    .map(|border| {
                        let border_color = if !is_sticky
                            && is_selected
                            && is_folded
                            && focus_handle.contains_focused(window, cx)
                        {
                            colors.border_focused
                        } else {
                            colors.border
                        };
                        border.border_color(border_color)
                    })
                    .bg(colors.editor_subheader_background)
                    .hover(|style| style.bg(colors.element_hover))
                    .map(|header| {
                        let editor = self.editor.clone();
                        let buffer_id = for_excerpt.buffer_id;
                        let toggle_chevron_icon =
                            FileIcons::get_chevron_icon(!is_folded, cx).map(Icon::from_path);
                        let button_size = rems_from_px(28.);

                        header.child(
                            div()
                                .hover(|style| style.bg(colors.element_selected))
                                .rounded_xs()
                                .child(
                                    ButtonLike::new("toggle-buffer-fold")
                                        .style(ButtonStyle::Transparent)
                                        .height(button_size.into())
                                        .width(button_size)
                                        .children(toggle_chevron_icon)
                                        .tooltip({
                                            let focus_handle = focus_handle.clone();
                                            let is_folded_for_tooltip = is_folded;
                                            move |_window, cx| {
                                                Tooltip::with_meta_in(
                                                    if is_folded_for_tooltip {
                                                        "Unfold Excerpt"
                                                    } else {
                                                        "Fold Excerpt"
                                                    },
                                                    Some(&ToggleFold),
                                                    format!(
                                                        "{} to toggle all",
                                                        text_for_keystroke(
                                                            &Modifiers::alt(),
                                                            "click",
                                                            cx
                                                        )
                                                    ),
                                                    &focus_handle,
                                                    cx,
                                                )
                                            }
                                        })
                                        .on_click(move |event, window, cx| {
                                            if event.modifiers().alt {
                                                editor.update(cx, |editor, cx| {
                                                    editor.toggle_fold_all(
                                                        &ToggleFoldAll,
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            } else {
                                                if is_folded {
                                                    editor.update(cx, |editor, cx| {
                                                        editor.unfold_buffer(buffer_id, cx);
                                                    });
                                                } else {
                                                    editor.update(cx, |editor, cx| {
                                                        editor.fold_buffer(buffer_id, cx);
                                                    });
                                                }
                                            }
                                        }),
                                ),
                        )
                    })
                    .children(
                        editor
                            .addons
                            .values()
                            .filter_map(|addon| {
                                addon.render_buffer_header_controls(for_excerpt, window, cx)
                            })
                            .take(1),
                    )
                    .when(!is_read_only, |this| {
                        this.child(
                            h_flex()
                                .size_3()
                                .justify_center()
                                .flex_shrink_0()
                                .children(indicator),
                        )
                    })
                    .child(
                        h_flex()
                            .cursor_pointer()
                            .id("path_header_block")
                            .min_w_0()
                            .size_full()
                            .justify_between()
                            .overflow_hidden()
                            .child(h_flex().min_w_0().flex_1().gap_0p5().map(|path_header| {
                                let filename = filename
                                    .map(SharedString::from)
                                    .unwrap_or_else(|| "untitled".into());

                                path_header
                                    .when(ItemSettings::get_global(cx).file_icons, |el| {
                                        let path = path::Path::new(filename.as_str());
                                        let icon =
                                            FileIcons::get_icon(path, cx).unwrap_or_default();

                                        el.child(Icon::from_path(icon).color(Color::Muted))
                                    })
                                    .child(
                                        ButtonLike::new("filename-button")
                                            .child(
                                                Label::new(filename)
                                                    .single_line()
                                                    .color(file_status_label_color(file_status))
                                                    .when(
                                                        file_status.is_some_and(|s| s.is_deleted()),
                                                        |label| label.strikethrough(),
                                                    ),
                                            )
                                            .on_click(window.listener_for(&self.editor, {
                                                let jump_data = jump_data.clone();
                                                move |editor, e: &ClickEvent, window, cx| {
                                                    editor.open_excerpts_common(
                                                        Some(jump_data.clone()),
                                                        e.modifiers().secondary(),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            })),
                                    )
                                    .when(!for_excerpt.buffer.capability.editable(), |el| {
                                        el.child(Icon::new(IconName::FileLock).color(Color::Muted))
                                    })
                                    .when_some(parent_path, |then, path| {
                                        then.child(Label::new(path).truncate().color(
                                            if file_status.is_some_and(FileStatus::is_deleted) {
                                                Color::Custom(colors.text_disabled)
                                            } else {
                                                Color::Custom(colors.text_muted)
                                            },
                                        ))
                                    })
                            }))
                            .when(
                                can_open_excerpts && is_selected && relative_path.is_some(),
                                |el| {
                                    el.child(
                                        Button::new("open-file-button", "Open File")
                                            .style(ButtonStyle::OutlinedGhost)
                                            .key_binding(KeyBinding::for_action_in(
                                                &OpenExcerpts,
                                                &focus_handle,
                                                cx,
                                            ))
                                            .on_click(window.listener_for(&self.editor, {
                                                let jump_data = jump_data.clone();
                                                move |editor, e: &ClickEvent, window, cx| {
                                                    editor.open_excerpts_common(
                                                        Some(jump_data.clone()),
                                                        e.modifiers().secondary(),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            })),
                                    )
                                },
                            )
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(window.listener_for(&self.editor, {
                                let buffer_id = for_excerpt.buffer_id;
                                move |editor, e: &ClickEvent, window, cx| {
                                    if e.modifiers().alt {
                                        editor.open_excerpts_common(
                                            Some(jump_data.clone()),
                                            e.modifiers().secondary(),
                                            window,
                                            cx,
                                        );
                                        return;
                                    }

                                    if is_folded {
                                        editor.unfold_buffer(buffer_id, cx);
                                    } else {
                                        editor.fold_buffer(buffer_id, cx);
                                    }
                                }
                            })),
                    ),
            );

        let file = for_excerpt.buffer.file().cloned();
        let editor = self.editor.clone();

        right_click_menu("buffer-header-context-menu")
            .trigger(move |_, _, _| header)
            .menu(move |window, cx| {
                let menu_context = focus_handle.clone();
                let editor = editor.clone();
                let file = file.clone();
                ContextMenu::build(window, cx, move |mut menu, window, cx| {
                    if let Some(file) = file
                        && let Some(project) = editor.read(cx).project()
                        && let Some(worktree) =
                            project.read(cx).worktree_for_id(file.worktree_id(cx), cx)
                    {
                        let path_style = file.path_style(cx);
                        let worktree = worktree.read(cx);
                        let relative_path = file.path();
                        let entry_for_path = worktree.entry_for_path(relative_path);
                        let abs_path = entry_for_path.map(|e| {
                            e.canonical_path.as_deref().map_or_else(
                                || worktree.absolutize(relative_path),
                                Path::to_path_buf,
                            )
                        });
                        let has_relative_path = worktree.root_entry().is_some_and(Entry::is_dir);

                        let parent_abs_path = abs_path
                            .as_ref()
                            .and_then(|abs_path| Some(abs_path.parent()?.to_path_buf()));
                        let relative_path = has_relative_path
                            .then_some(relative_path)
                            .map(ToOwned::to_owned);

                        let visible_in_project_panel =
                            relative_path.is_some() && worktree.is_visible();
                        let reveal_in_project_panel = entry_for_path
                            .filter(|_| visible_in_project_panel)
                            .map(|entry| entry.id);
                        menu = menu
                            .when_some(abs_path, |menu, abs_path| {
                                menu.entry(
                                    "Copy Path",
                                    Some(Box::new(zed_actions::workspace::CopyPath)),
                                    window.handler_for(&editor, move |_, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            abs_path.to_string_lossy().into_owned(),
                                        ));
                                    }),
                                )
                            })
                            .when_some(relative_path, |menu, relative_path| {
                                menu.entry(
                                    "Copy Relative Path",
                                    Some(Box::new(zed_actions::workspace::CopyRelativePath)),
                                    window.handler_for(&editor, move |_, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            relative_path.display(path_style).to_string(),
                                        ));
                                    }),
                                )
                            })
                            .when(
                                reveal_in_project_panel.is_some() || parent_abs_path.is_some(),
                                |menu| menu.separator(),
                            )
                            .when_some(reveal_in_project_panel, |menu, entry_id| {
                                menu.entry(
                                    "Reveal In Project Panel",
                                    Some(Box::new(RevealInProjectPanel::default())),
                                    window.handler_for(&editor, move |editor, _, cx| {
                                        if let Some(project) = &mut editor.project {
                                            project.update(cx, |_, cx| {
                                                cx.emit(project::Event::RevealInProjectPanel(
                                                    entry_id,
                                                ))
                                            });
                                        }
                                    }),
                                )
                            })
                            .when_some(parent_abs_path, |menu, parent_abs_path| {
                                menu.entry(
                                    "Open in Terminal",
                                    Some(Box::new(OpenInTerminal)),
                                    window.handler_for(&editor, move |_, window, cx| {
                                        window.dispatch_action(
                                            OpenTerminal {
                                                working_directory: parent_abs_path.clone(),
                                                local: false,
                                            }
                                            .boxed_clone(),
                                            cx,
                                        );
                                    }),
                                )
                            });
                    }

                    menu.context(menu_context)
                })
            })
    }
}

fn header_jump_data(
    editor_snapshot: &EditorSnapshot,
    block_row_start: DisplayRow,
    height: u32,
    first_excerpt: &ExcerptInfo,
    latest_selection_anchors: &HashMap<BufferId, Anchor>,
) -> JumpData {
    let jump_target = if let Some(anchor) = latest_selection_anchors.get(&first_excerpt.buffer_id)
        && let Some(range) = editor_snapshot.context_range_for_excerpt(anchor.excerpt_id)
        && let Some(buffer) = editor_snapshot
            .buffer_snapshot()
            .buffer_for_excerpt(anchor.excerpt_id)
    {
        JumpTargetInExcerptInput {
            id: anchor.excerpt_id,
            buffer,
            excerpt_start_anchor: range.start,
            jump_anchor: anchor.text_anchor,
        }
    } else {
        JumpTargetInExcerptInput {
            id: first_excerpt.id,
            buffer: &first_excerpt.buffer,
            excerpt_start_anchor: first_excerpt.range.context.start,
            jump_anchor: first_excerpt.range.primary.start,
        }
    };
    header_jump_data_inner(editor_snapshot, block_row_start, height, &jump_target)
}

struct JumpTargetInExcerptInput<'a> {
    id: ExcerptId,
    buffer: &'a language::BufferSnapshot,
    excerpt_start_anchor: text::Anchor,
    jump_anchor: text::Anchor,
}

fn header_jump_data_inner(
    snapshot: &EditorSnapshot,
    block_row_start: DisplayRow,
    height: u32,
    for_excerpt: &JumpTargetInExcerptInput,
) -> JumpData {
    let buffer = &for_excerpt.buffer;
    let jump_position = language::ToPoint::to_point(&for_excerpt.jump_anchor, buffer);
    let excerpt_start = for_excerpt.excerpt_start_anchor;
    let rows_from_excerpt_start = if for_excerpt.jump_anchor == excerpt_start {
        0
    } else {
        let excerpt_start_point = language::ToPoint::to_point(&excerpt_start, buffer);
        jump_position.row.saturating_sub(excerpt_start_point.row)
    };

    let line_offset_from_top = (block_row_start.0 + height + rows_from_excerpt_start)
        .saturating_sub(
            snapshot
                .scroll_anchor
                .scroll_position(&snapshot.display_snapshot)
                .y as u32,
        );

    JumpData::MultiBufferPoint {
        excerpt_id: for_excerpt.id,
        anchor: for_excerpt.jump_anchor,
        position: jump_position,
        line_offset_from_top,
    }
}

fn file_status_label_color(file_status: Option<FileStatus>) -> Color {
    file_status.map_or(Color::Default, |status| {
        if status.is_conflicted() {
            Color::Conflict
        } else if status.is_modified() {
            Color::Modified
        } else if status.is_deleted() {
            Color::Disabled
        } else if status.is_created() {
            Color::Created
        } else {
            Color::Default
        }
    })
}
