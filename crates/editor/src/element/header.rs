use std::path::Path;
use std::rc::Rc;

use collections::HashMap;
use file_icons::FileIcons;
use git::status::FileStatus;
use gpui::{
    Action, AnyElement, App, AvailableSpace, Bounds, ClickEvent, ClipboardItem, ContentMask,
    CursorStyle, DefiniteLength, Entity, Focusable as _, Hitbox, HitboxBehavior, Hsla, IntoElement,
    Length, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, ParentElement, Pixels,
    ShapedLine, SharedString, Styled, TextAlign, Window, WindowBackgroundAppearance, div, fill,
    linear_color_stop, linear_gradient, point, px, size,
};
use language::language_settings::ShowWhitespaceSetting;
use multi_buffer::{Anchor, ExcerptBoundaryInfo, MultiBuffer};
use project::Entry;
use settings::{RelativeLineNumbers, Settings};
use smallvec::SmallVec;
use sum_tree::Bias;
use text::BufferId;
use theme::ActiveTheme;
use ui::{
    ButtonLike, ContextMenu, DiffStat, Indicator, KeyBinding, Tooltip, prelude::*,
    right_click_menu, text_for_keystroke, utils::WithRemSize,
};
use util::ResultExt;
use workspace::{ItemHandle, ItemSettings, OpenInTerminal, OpenTerminal, RevealInProjectPanel};

use super::{
    BlockLayout, EditorElement, EditorLayout, LineWithInvisibles, layout_line,
    render_breadcrumb_text,
};
use crate::{
    BUFFER_HEADER_PADDING, DisplayRow, Editor, EditorSettings, EditorSnapshot, FILE_HEADER_HEIGHT,
    GutterDimensions, JumpData, MULTI_BUFFER_EXCERPT_HEADER_HEIGHT, OpenExcerpts, Point, RowExt,
    SelectionEffects, StickyHeaderExcerpt, ToPoint, ToggleFold, ToggleFoldAll,
    display_map::ToDisplayPoint,
    scroll::{Autoscroll, ScrollOffset, ScrollPixelOffset},
};

pub(crate) struct StickyHeader {
    sticky_row: DisplayRow,
    pub(crate) start_point: Point,
    pub(crate) offset: ScrollOffset,
}

pub(super) struct StickyHeaders {
    pub(super) lines: Vec<StickyHeaderLine>,
    gutter_background: Hsla,
    content_background: Hsla,
    gutter_right_padding: Pixels,
}

pub(super) struct StickyHeaderLine {
    row: DisplayRow,
    pub(super) offset: Pixels,
    line: Rc<LineWithInvisibles>,
    line_number: Option<ShapedLine>,
    elements: SmallVec<[AnyElement; 1]>,
    available_text_width: Pixels,
    hitbox: Hitbox,
}

impl EditorElement {
    pub(crate) fn sticky_headers(editor: &Editor, snapshot: &EditorSnapshot) -> Vec<StickyHeader> {
        let scroll_top = snapshot.scroll_position().y;

        let mut end_rows = Vec::<DisplayRow>::new();
        let mut rows = Vec::<StickyHeader>::new();

        for item in editor.sticky_headers.iter().flatten() {
            let selection_start = item
                .selection_range
                .start
                .to_point(snapshot.buffer_snapshot());
            let source_text_start = item
                .source_range_for_text
                .start
                .to_point(snapshot.buffer_snapshot());
            let start_column = if source_text_start.row == selection_start.row {
                source_text_start.column
            } else {
                0
            };
            let start_point = Point::new(selection_start.row, start_column);
            let end_point = item.range.end.to_point(snapshot.buffer_snapshot());

            let sticky_row = snapshot
                .display_snapshot
                .point_to_display_point(start_point, Bias::Left)
                .row();
            if rows
                .last()
                .is_some_and(|last| last.sticky_row == sticky_row)
            {
                continue;
            }

            let end_row = snapshot
                .display_snapshot
                .point_to_display_point(end_point, Bias::Left)
                .row();
            let max_sticky_row = end_row.previous_row();
            if max_sticky_row <= sticky_row {
                continue;
            }

            while end_rows
                .last()
                .is_some_and(|&last_end| last_end <= sticky_row)
            {
                end_rows.pop();
            }
            let depth = end_rows.len();
            let adjusted_scroll_top = scroll_top + depth as f64;

            if sticky_row.as_f64() >= adjusted_scroll_top || end_row.as_f64() <= adjusted_scroll_top
            {
                continue;
            }

            let max_scroll_offset = max_sticky_row.as_f64() - scroll_top;
            let offset = (depth as f64).min(max_scroll_offset);

            end_rows.push(end_row);
            rows.push(StickyHeader {
                sticky_row,
                start_point,
                offset,
            });
        }

        rows
    }

    pub(super) fn should_show_buffer_headers(&self) -> bool {
        self.split_side.is_none()
    }

    pub(super) fn layout_sticky_buffer_header(
        &self,
        StickyHeaderExcerpt { excerpt }: StickyHeaderExcerpt<'_>,
        scroll_position: gpui::Point<ScrollOffset>,
        line_height: Pixels,
        right_margin: Pixels,
        snapshot: &EditorSnapshot,
        hitbox: &Hitbox,
        selected_buffer_ids: &Vec<BufferId>,
        blocks: &[BlockLayout],
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
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

        let selected = selected_buffer_ids.contains(&excerpt.buffer_id());

        let available_width = hitbox.bounds.size.width - right_margin;

        let mut header = v_flex()
            .w_full()
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

        let mut origin = hitbox.origin;
        // Move floating header up to avoid colliding with the next buffer header.
        for block in blocks.iter() {
            if !block.is_buffer_header {
                continue;
            }

            let Some(display_row) = block.row.filter(|row| row.0 > scroll_position.y as u32) else {
                continue;
            };

            let max_row = display_row.0.saturating_sub(FILE_HEADER_HEIGHT);
            let offset = scroll_position.y - max_row as f64;

            if offset > 0.0 {
                origin.y -= Pixels::from(offset * ScrollPixelOffset::from(line_height));
            }
            break;
        }

        let size = size(
            AvailableSpace::Definite(available_width),
            AvailableSpace::MinContent,
        );

        header.prepaint_as_root(origin, size, window, cx);

        header
    }

    pub(super) fn layout_sticky_headers(
        &self,
        snapshot: &EditorSnapshot,
        editor_width: Pixels,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        content_origin: gpui::Point<Pixels>,
        gutter_dimensions: &GutterDimensions,
        gutter_hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        relative_line_numbers: RelativeLineNumbers,
        relative_to: Option<DisplayRow>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<StickyHeaders> {
        let show_line_numbers = snapshot
            .show_line_numbers
            .unwrap_or_else(|| EditorSettings::get_global(cx).gutter.line_numbers);

        let rows = Self::sticky_headers(self.editor.read(cx), snapshot);

        let mut lines = Vec::<StickyHeaderLine>::new();

        for StickyHeader {
            sticky_row,
            start_point,
            offset,
        } in rows.into_iter().rev()
        {
            let line = layout_line(
                sticky_row,
                snapshot,
                &self.style,
                editor_width,
                is_row_soft_wrapped,
                window,
                cx,
            );

            let line_number = show_line_numbers.then(|| {
                let start_display_row = start_point.to_display_point(snapshot).row();
                let relative_number = relative_to
                    .filter(|_| relative_line_numbers != RelativeLineNumbers::Disabled)
                    .map(|base| {
                        snapshot.relative_line_delta(
                            base,
                            start_display_row,
                            relative_line_numbers == RelativeLineNumbers::Wrapped,
                        )
                    });
                let number = relative_number
                    .filter(|&delta| delta != 0)
                    .map(|delta| delta.unsigned_abs() as u32)
                    .unwrap_or(start_point.row + 1);
                let color = cx.theme().colors().editor_line_number;
                self.shape_line_number(SharedString::from(number.to_string()), color, window)
            });

            lines.push(StickyHeaderLine::new(
                sticky_row,
                line_height * offset as f32,
                line,
                line_number,
                line_height,
                scroll_pixel_position,
                content_origin,
                gutter_hitbox,
                text_hitbox,
                window,
                cx,
            ));
        }

        lines.reverse();
        if lines.is_empty() {
            return None;
        }

        Some(StickyHeaders {
            lines,
            gutter_background: cx.theme().colors().editor_gutter_background,
            content_background: self.style.background,
            gutter_right_padding: gutter_dimensions.right_padding,
        })
    }

    pub(super) fn paint_sticky_headers(
        &mut self,
        layout: &mut EditorLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(mut sticky_headers) = layout.sticky_headers.take() else {
            return;
        };

        let Some(last_line_offset) = sticky_headers.lines.last().map(|line| line.offset) else {
            layout.sticky_headers = Some(sticky_headers);
            return;
        };

        let whitespace_setting = self
            .editor
            .read(cx)
            .buffer
            .read(cx)
            .language_settings(cx)
            .show_whitespaces;
        sticky_headers.paint(layout, whitespace_setting, window, cx);

        let sticky_header_hitboxes: Vec<Hitbox> = sticky_headers
            .lines
            .iter()
            .map(|line| line.hitbox.clone())
            .collect();
        let hovered_hitbox = sticky_header_hitboxes
            .iter()
            .find_map(|hitbox| hitbox.is_hovered(window).then_some(hitbox.id));

        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, _cx| {
            if !phase.bubble() {
                return;
            }

            let current_hover = sticky_header_hitboxes
                .iter()
                .find_map(|hitbox| hitbox.is_hovered(window).then_some(hitbox.id));
            if hovered_hitbox != current_hover {
                window.refresh();
            }
        });

        let position_map = layout.position_map.clone();

        for (line_index, line) in sticky_headers.lines.iter().enumerate() {
            let editor = self.editor.clone();
            let hitbox = line.hitbox.clone();
            let row = line.row;
            let line_layout = line.line.clone();
            let position_map = position_map.clone();
            window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
                if !phase.bubble() {
                    return;
                }

                if event.button == MouseButton::Left && hitbox.is_hovered(window) {
                    let point_for_position =
                        position_map.point_for_position_on_line(event.position, row, &line_layout);

                    editor.update(cx, |editor, cx| {
                        let snapshot = editor.snapshot(window, cx);
                        let anchor = snapshot
                            .display_snapshot
                            .display_point_to_anchor(point_for_position.nearest_valid, Bias::Left);
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::top_relative(
                                line_index as ScrollOffset,
                            )),
                            window,
                            cx,
                            |selections| {
                                selections.clear_disjoint();
                                selections.set_pending_anchor_range(
                                    anchor..anchor,
                                    crate::SelectMode::Character,
                                );
                            },
                        );
                        cx.stop_propagation();
                    });
                }
            });
        }

        let text_bounds = layout.position_map.text_hitbox.bounds;
        let border_top = text_bounds.top() + last_line_offset + layout.position_map.line_height;
        let separator_height = px(1.);
        let border_bounds = window.pixel_snap_bounds(Bounds::from_corners(
            point(layout.gutter_hitbox.bounds.left(), border_top),
            point(text_bounds.right(), border_top + separator_height),
        ));
        window.paint_quad(fill(border_bounds, cx.theme().colors().border_variant));

        layout.sticky_headers = Some(sticky_headers);
    }
}

impl StickyHeaders {
    fn paint(
        &mut self,
        layout: &mut EditorLayout,
        whitespace_setting: ShowWhitespaceSetting,
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_height = layout.position_map.line_height;

        for line in self.lines.iter_mut().rev() {
            window.paint_layer(
                Bounds::new(
                    layout.gutter_hitbox.origin + point(Pixels::ZERO, line.offset),
                    size(line.hitbox.size.width, line_height),
                ),
                |window| {
                    let gutter_bounds = Bounds::new(
                        layout.gutter_hitbox.origin + point(Pixels::ZERO, line.offset),
                        size(layout.gutter_hitbox.size.width, line_height),
                    );
                    window.paint_quad(fill(gutter_bounds, self.gutter_background));

                    let text_bounds = Bounds::new(
                        layout.position_map.text_hitbox.origin + point(Pixels::ZERO, line.offset),
                        size(line.available_text_width, line_height),
                    );
                    window.paint_quad(fill(text_bounds, self.content_background));

                    if line.hitbox.is_hovered(window) {
                        let hover_overlay = cx.theme().colors().panel_overlay_hover;
                        window.paint_quad(fill(gutter_bounds, hover_overlay));
                        window.paint_quad(fill(text_bounds, hover_overlay));
                    }

                    line.paint(
                        layout,
                        self.gutter_right_padding,
                        line.available_text_width,
                        layout.content_origin,
                        line_height,
                        whitespace_setting,
                        window,
                        cx,
                    );
                },
            );

            window.set_cursor_style(CursorStyle::IBeam, &line.hitbox);
        }
    }
}

impl StickyHeaderLine {
    fn new(
        row: DisplayRow,
        offset: Pixels,
        mut line: LineWithInvisibles,
        line_number: Option<ShapedLine>,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        content_origin: gpui::Point<Pixels>,
        gutter_hitbox: &Hitbox,
        text_hitbox: &Hitbox,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut elements = SmallVec::<[AnyElement; 1]>::new();
        line.prepaint_with_custom_offset(
            line_height,
            scroll_pixel_position,
            content_origin,
            offset,
            &mut elements,
            window,
            cx,
        );

        let hitbox_bounds = Bounds::new(
            gutter_hitbox.origin + point(Pixels::ZERO, offset),
            size(text_hitbox.right() - gutter_hitbox.left(), line_height),
        );
        let available_text_width =
            (hitbox_bounds.size.width - gutter_hitbox.size.width).max(Pixels::ZERO);

        Self {
            row,
            offset,
            line: Rc::new(line),
            line_number,
            elements,
            available_text_width,
            hitbox: window.insert_hitbox(hitbox_bounds, HitboxBehavior::BlockMouseExceptScroll),
        }
    }

    fn paint(
        &mut self,
        layout: &EditorLayout,
        gutter_right_padding: Pixels,
        available_text_width: Pixels,
        content_origin: gpui::Point<Pixels>,
        line_height: Pixels,
        whitespace_setting: ShowWhitespaceSetting,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_content_mask(
            Some(ContentMask {
                bounds: Bounds::new(
                    layout.position_map.text_hitbox.bounds.origin
                        + point(Pixels::ZERO, self.offset),
                    size(available_text_width, line_height),
                ),
            }),
            |window| {
                self.line.draw_with_custom_offset(
                    layout,
                    self.row,
                    content_origin,
                    self.offset,
                    whitespace_setting,
                    &[],
                    window,
                    cx,
                );
                for element in &mut self.elements {
                    element.paint(window, cx);
                }
            },
        );

        if let Some(line_number) = &self.line_number {
            let gutter_origin = layout.gutter_hitbox.origin + point(Pixels::ZERO, self.offset);
            let gutter_width = layout.gutter_hitbox.size.width;
            let origin = point(
                gutter_origin.x + gutter_width - gutter_right_padding - line_number.width,
                gutter_origin.y,
            );
            line_number
                .paint(origin, line_height, TextAlign::Left, None, window, cx)
                .log_err();
        }
    }
}

pub(crate) fn header_jump_data(
    editor_snapshot: &EditorSnapshot,
    block_row_start: DisplayRow,
    height: u32,
    first_excerpt: &ExcerptBoundaryInfo,
    latest_selection_anchors: &HashMap<BufferId, Anchor>,
) -> JumpData {
    let multibuffer_snapshot = editor_snapshot.buffer_snapshot();
    let buffer = first_excerpt.buffer(multibuffer_snapshot);
    let (jump_anchor, jump_buffer, excerpt_start) = if let Some(anchor) =
        latest_selection_anchors.get(&first_excerpt.buffer_id())
        && let Some((jump_anchor, selection_buffer)) =
            multibuffer_snapshot.anchor_to_buffer_anchor(*anchor)
    {
        let jump_offset = text::ToOffset::to_offset(&jump_anchor, selection_buffer);
        let selection_excerpt_start = multibuffer_snapshot
            .excerpts_for_buffer(jump_anchor.buffer_id)
            .find(|excerpt| {
                let start = text::ToOffset::to_offset(&excerpt.context.start, selection_buffer);
                let end = text::ToOffset::to_offset(&excerpt.context.end, selection_buffer);
                start <= jump_offset && jump_offset <= end
            })
            .map(|excerpt| excerpt.context.start)
            .unwrap_or(first_excerpt.range.context.start);
        (jump_anchor, selection_buffer, selection_excerpt_start)
    } else {
        (
            first_excerpt.range.primary.start,
            buffer,
            first_excerpt.range.context.start,
        )
    };
    let jump_position = language::ToPoint::to_point(&jump_anchor, jump_buffer);
    let rows_from_excerpt_start = if jump_anchor == excerpt_start {
        0
    } else {
        let excerpt_start_point = language::ToPoint::to_point(&excerpt_start, jump_buffer);
        jump_position.row.saturating_sub(excerpt_start_point.row)
    };

    let line_offset_from_top = (block_row_start.0 + height + rows_from_excerpt_start)
        .saturating_sub(
            editor_snapshot
                .scroll_anchor
                .scroll_position(&editor_snapshot.display_snapshot)
                .y as u32,
        );

    JumpData::MultiBufferPoint {
        anchor: jump_anchor,
        position: jump_position,
        line_offset_from_top,
    }
}

pub(crate) fn render_buffer_header(
    editor: &Entity<Editor>,
    for_excerpt: &ExcerptBoundaryInfo,
    is_folded: bool,
    is_selected: bool,
    is_sticky: bool,
    jump_data: JumpData,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let buffer_id = for_excerpt.buffer_id();
    let header_hovered_state = window.use_keyed_state(
        ("buffer-header-hovered", buffer_id.to_proto()),
        cx,
        |_, _| false,
    );
    let header_hovered = *header_hovered_state.read(cx);
    let editor_read = editor.read(cx);
    let multi_buffer = editor_read.buffer.read(cx);
    let is_read_only = editor_read.read_only(cx);
    let editor_handle: &dyn ItemHandle = editor;
    let multibuffer_snapshot = multi_buffer.snapshot(cx);
    let buffer = for_excerpt.buffer(&multibuffer_snapshot);

    let breadcrumbs = if is_selected {
        editor_read.breadcrumbs_inner(cx)
    } else {
        None
    };

    let file_status = multi_buffer
        .all_diff_hunks_expanded()
        .then(|| editor_read.status_for_buffer_id(buffer_id, cx))
        .flatten();
    let diff_stat = multi_buffer
        .all_diff_hunks_expanded()
        .then(|| multibuffer_snapshot.diff_for_buffer_id(buffer_id))
        .flatten()
        .map(|diff| diff.changed_row_counts())
        .filter(|(added, removed)| *added > 0 || *removed > 0);
    let indicator = multi_buffer.buffer(buffer_id).and_then(|buffer| {
        let buffer = buffer.read(cx);
        let indicator_color = match (buffer.has_conflict(), buffer.is_dirty()) {
            (true, _) => Some(Color::Warning),
            (_, true) => Some(Color::Accent),
            (false, false) => None,
        };
        indicator_color.map(|indicator_color| Indicator::dot().color(indicator_color))
    });

    let include_root = editor_read
        .project
        .as_ref()
        .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
        .unwrap_or_default();
    let file = buffer.file();
    let can_open_excerpts = file.is_none_or(|file| file.can_open());
    let path_style = file.map(|file| file.path_style(cx));
    let relative_path = buffer.resolve_file_path(include_root, cx);
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
    let focus_handle = editor_read.focus_handle(cx);
    let colors = cx.theme().colors();
    // On transparent windows `editor_subheader_background` stacks over the
    // editor background into a darker bar (and the sticky shadow becomes a halo),
    // so skip both unless the window is opaque.
    let opaque_window =
        cx.theme().window_background_appearance() == WindowBackgroundAppearance::Opaque;

    let show_open_file_button =
        can_open_excerpts && relative_path.is_some() && (is_selected || header_hovered);

    let header = div()
        .id(("buffer-header", buffer_id.to_proto()))
        .on_hover(move |hovered, _window, cx| {
            header_hovered_state.update(cx, |state, cx| {
                if *state != *hovered {
                    *state = *hovered;
                    cx.notify();
                }
            });
        })
        .p(BUFFER_HEADER_PADDING)
        .w_full()
        .h(FILE_HEADER_HEIGHT as f32 * window.line_height())
        .child(
            h_flex()
                .group("buffer-header-group")
                .size_full()
                .flex_basis(Length::Definite(DefiniteLength::Fraction(0.667)))
                .pl_1()
                .pr_2()
                .rounded_sm()
                .gap_1p5()
                .border_1()
                .map(|border| {
                    let border_color =
                        if is_selected && is_folded && focus_handle.contains_focused(window, cx) {
                            colors.border_focused
                        } else {
                            colors.border
                        };
                    border.border_color(border_color)
                })
                .when(is_sticky && opaque_window, |s| s.shadow_md())
                .when(opaque_window, |s| s.bg(colors.editor_subheader_background))
                .hover(|s| s.bg(colors.element_hover))
                .map(|header| {
                    let editor = editor.clone();
                    let buffer_id = for_excerpt.buffer_id();
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
                                                editor.toggle_fold_all(&ToggleFoldAll, window, cx);
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
                    editor_read
                        .addons
                        .values()
                        .filter_map(|addon| {
                            addon.render_buffer_header_controls(for_excerpt, buffer, window, cx)
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
                        .gap_1()
                        .justify_between()
                        .overflow_hidden()
                        .child(h_flex().min_w_0().flex_1().gap_0p5().overflow_hidden().map(
                            |path_header| {
                                let filename = filename
                                    .map(SharedString::from)
                                    .unwrap_or_else(|| MultiBuffer::DEFAULT_TITLE.into());

                                let full_path = match parent_path.as_deref() {
                                    Some(parent) if !parent.is_empty() => {
                                        format!("{}{}", parent, filename.as_str())
                                    }
                                    _ => filename.as_str().to_string(),
                                };

                                path_header
                                    .child(
                                        ButtonLike::new("filename-button")
                                            .when(ItemSettings::get_global(cx).file_icons, |this| {
                                                let path = std::path::Path::new(filename.as_str());
                                                let icon = FileIcons::get_icon(path, cx)
                                                    .unwrap_or_default();

                                                this.child(
                                                    Icon::from_path(icon).color(Color::Muted),
                                                )
                                            })
                                            .child(
                                                Label::new(filename)
                                                    .single_line()
                                                    .color(file_status_label_color(file_status))
                                                    .buffer_font(cx)
                                                    .when(
                                                        file_status.is_some_and(|s| s.is_deleted()),
                                                        |label| label.strikethrough(),
                                                    ),
                                            )
                                            .tooltip(move |_, cx| {
                                                Tooltip::with_meta(
                                                    "Open File",
                                                    None,
                                                    full_path.clone(),
                                                    cx,
                                                )
                                            })
                                            .on_click(window.listener_for(editor, {
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
                                    .when_some(parent_path, |then, path| {
                                        then.child(
                                            Label::new(path)
                                                .buffer_font(cx)
                                                .truncate_start()
                                                .color(
                                                    if file_status
                                                        .is_some_and(FileStatus::is_deleted)
                                                    {
                                                        Color::Custom(colors.text_disabled)
                                                    } else {
                                                        Color::Custom(colors.text_muted)
                                                    },
                                                ),
                                        )
                                    })
                                    .when(!buffer.capability.editable(), |el| {
                                        el.child(Icon::new(IconName::FileLock).color(Color::Muted))
                                    })
                                    .when_some(breadcrumbs, |then, breadcrumbs| {
                                        let font = theme_settings::ThemeSettings::get_global(cx)
                                            .buffer_font
                                            .clone();
                                        then.child(render_breadcrumb_text(
                                            breadcrumbs,
                                            Some(font),
                                            None,
                                            editor_handle,
                                            true,
                                            window,
                                            cx,
                                        ))
                                    })
                            },
                        ))
                        .child(
                            h_flex()
                                .gap_2()
                                .when_some(diff_stat, |this, (added, removed)| {
                                    let ui_font_size =
                                        theme_settings::ThemeSettings::get_global(cx)
                                            .ui_font_size(cx);
                                    this.child(WithRemSize::new(ui_font_size).child(DiffStat::new(
                                        ("buffer-header-diff-stat", buffer_id.to_proto()),
                                        added as usize,
                                        removed as usize,
                                    )))
                                })
                                .when(show_open_file_button, |this| {
                                    this.child(
                                        Button::new("open-file-button", "Open File")
                                            .style(ButtonStyle::OutlinedCustom(
                                                cx.theme().colors().border.opacity(0.6),
                                            ))
                                            .layer(ui::ElevationIndex::ElevatedSurface)
                                            .when(is_selected, |this| {
                                                this.key_binding(KeyBinding::for_action_in(
                                                    &OpenExcerpts,
                                                    &focus_handle,
                                                    cx,
                                                ))
                                            })
                                            .on_click(window.listener_for(editor, {
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
                                }),
                        )
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_click(window.listener_for(editor, {
                            let buffer_id = for_excerpt.buffer_id();
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

    let file = buffer.file().cloned();
    let editor = editor.clone();
    let buffer_snapshot = buffer.clone();

    right_click_menu(("buffer-header-context-menu", buffer_id.to_proto()))
        .trigger(move |_, _, _| header)
        .menu(move |window, cx| {
            let menu_context = focus_handle.clone();
            let editor = editor.clone();
            let file = file.clone();
            let buffer_snapshot = buffer_snapshot.clone();
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
                        e.canonical_path
                            .as_deref()
                            .map_or_else(|| worktree.absolutize(relative_path), Path::to_path_buf)
                    });
                    let has_relative_path = worktree.root_entry().is_some_and(Entry::is_dir);

                    let parent_abs_path = abs_path
                        .as_ref()
                        .and_then(|abs_path| Some(abs_path.parent()?.to_path_buf()));
                    let relative_path = has_relative_path
                        .then_some(relative_path)
                        .map(ToOwned::to_owned);

                    let visible_in_project_panel = relative_path.is_some() && worktree.is_visible();
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
                                            cx.emit(project::Event::RevealInProjectPanel(entry_id))
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

                menu = editor.update(cx, |editor, cx| {
                    let mut menu = menu;
                    for addon in editor.addons.values() {
                        menu = addon.extend_buffer_header_context_menu(
                            menu,
                            &buffer_snapshot,
                            window,
                            cx,
                        );
                    }
                    menu
                });

                menu.context(menu_context)
            })
        })
}

pub fn file_status_label_color(file_status: Option<FileStatus>) -> Color {
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
