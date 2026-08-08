use super::super::*;
use super::BreadcrumbSegmentTarget;
use super::menu::BreadcrumbListing;
use super::outline::flatten_text_for_single_line_display;
use super::path::reveal_directory_in_project_panel;
use gpui::{anchored, deferred};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreadcrumbSegmentKind {
    Root,
    Middle,
    File,
    Symbol,
}

pub(crate) fn classify_breadcrumb_segment_kinds(
    segment_count: usize,
    file_segment_index: usize,
    has_root_segment: bool,
) -> Vec<BreadcrumbSegmentKind> {
    (0..segment_count)
        .map(|index| match index.cmp(&file_segment_index) {
            Ordering::Greater => BreadcrumbSegmentKind::Symbol,
            Ordering::Equal => BreadcrumbSegmentKind::File,
            Ordering::Less if has_root_segment && index == 0 => BreadcrumbSegmentKind::Root,
            Ordering::Less => BreadcrumbSegmentKind::Middle,
        })
        .collect()
}

pub(super) fn align_symbol_segments(
    segments: &[HighlightedText],
    symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
) -> Vec<Option<BreadcrumbSegmentTarget>> {
    if symbol_segments.len() == segments.len() {
        symbol_segments
    } else {
        vec![None; segments.len()]
    }
}

pub(crate) const MAX_BREADCRUMB_SEGMENTS_HARD_CAP: usize = 64;

fn hard_cap_ellipsis() -> HighlightedText {
    HighlightedText {
        text: "⋯".into(),
        highlights: vec![],
    }
}

fn splice_middle_run(
    segments: &mut Vec<HighlightedText>,
    symbol_segments: &mut Vec<Option<BreadcrumbSegmentTarget>>,
    kinds: &mut Vec<BreadcrumbSegmentKind>,
    file_segment_index: &mut usize,
    range: std::ops::Range<usize>,
) {
    if range.end <= range.start {
        return;
    }
    let removed = range.end - range.start;
    segments.splice(range.clone(), Some(hard_cap_ellipsis()));
    symbol_segments.splice(range.clone(), Some(None));
    kinds.splice(range.clone(), Some(BreadcrumbSegmentKind::Middle));
    if *file_segment_index >= range.end {
        *file_segment_index -= removed - 1;
    } else if *file_segment_index > range.start {
        *file_segment_index = range.start;
    }
}

pub(crate) fn hard_cap_middle_segments(
    mut segments: Vec<HighlightedText>,
    mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
    mut kinds: Vec<BreadcrumbSegmentKind>,
    mut file_segment_index: usize,
    protected_index: Option<usize>,
) -> (
    Vec<HighlightedText>,
    Vec<Option<BreadcrumbSegmentTarget>>,
    Vec<BreadcrumbSegmentKind>,
    usize,
) {
    let middle_start = kinds
        .iter()
        .position(|kind| *kind == BreadcrumbSegmentKind::Middle);
    let middle_end = kinds
        .iter()
        .rposition(|kind| *kind == BreadcrumbSegmentKind::Middle)
        .map(|index| index + 1);
    let (Some(middle_start), Some(middle_end)) = (middle_start, middle_end) else {
        return (segments, symbol_segments, kinds, file_segment_index);
    };
    let middle_len = middle_end - middle_start;
    if middle_len <= MAX_BREADCRUMB_SEGMENTS_HARD_CAP {
        return (segments, symbol_segments, kinds, file_segment_index);
    }

    if let Some(protected) =
        protected_index.filter(|index| (middle_start..middle_end).contains(index))
    {
        let keep = MAX_BREADCRUMB_SEGMENTS_HARD_CAP;
        let mut window_start = protected.saturating_sub(keep / 2).max(middle_start);
        let mut window_end = window_start + keep;
        if window_end > middle_end {
            window_end = middle_end;
            window_start = window_end.saturating_sub(keep).max(middle_start);
        }
        // Right side first so left indices stay valid.
        splice_middle_run(
            &mut segments,
            &mut symbol_segments,
            &mut kinds,
            &mut file_segment_index,
            window_end..middle_end,
        );
        splice_middle_run(
            &mut segments,
            &mut symbol_segments,
            &mut kinds,
            &mut file_segment_index,
            middle_start..window_start,
        );
        return (segments, symbol_segments, kinds, file_segment_index);
    }

    let half = MAX_BREADCRUMB_SEGMENTS_HARD_CAP / 2;
    let splice_start = middle_start + half;
    let splice_end = middle_end - half;
    splice_middle_run(
        &mut segments,
        &mut symbol_segments,
        &mut kinds,
        &mut file_segment_index,
        splice_start..splice_end,
    );

    (segments, symbol_segments, kinds, file_segment_index)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BreadcrumbLayoutPlan {
    pub(crate) visible: Vec<usize>,
    pub(crate) ellipses: Vec<Range<usize>>,
}

fn total_breadcrumb_layout_width(
    widths: &[Pixels],
    dropped: &[bool],
    ellipsis_width: Pixels,
) -> Pixels {
    let mut total = Pixels::ZERO;
    let mut in_dropped_run = false;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            if !in_dropped_run {
                total += ellipsis_width;
                in_dropped_run = true;
            }
        } else {
            total += widths[index];
            in_dropped_run = false;
        }
    }
    total
}

fn breadcrumb_layout_plan_from_dropped(dropped: &[bool]) -> BreadcrumbLayoutPlan {
    let mut visible = Vec::new();
    let mut ellipses = Vec::new();
    let mut run_start = None;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            run_start.get_or_insert(index);
        } else {
            if let Some(start) = run_start.take() {
                ellipses.push(start..index);
            }
            visible.push(index);
        }
    }
    if let Some(start) = run_start {
        ellipses.push(start..dropped.len());
    }
    BreadcrumbLayoutPlan { visible, ellipses }
}

pub(crate) fn plan_breadcrumb_layout(
    widths: &[Pixels],
    kinds: &[BreadcrumbSegmentKind],
    ellipsis_width: Pixels,
    available_width: Pixels,
    protected_index: Option<usize>,
) -> BreadcrumbLayoutPlan {
    debug_assert_eq!(widths.len(), kinds.len());
    let segment_count = widths.len();
    if segment_count == 0 {
        return BreadcrumbLayoutPlan {
            visible: Vec::new(),
            ellipses: Vec::new(),
        };
    }

    let mut dropped = vec![false; segment_count];
    if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
        return breadcrumb_layout_plan_from_dropped(&dropped);
    }

    let last_index = segment_count - 1;
    let mut drop_order = Vec::with_capacity(segment_count - 1);
    for kind in [
        BreadcrumbSegmentKind::Middle,
        BreadcrumbSegmentKind::Root,
        BreadcrumbSegmentKind::File,
        BreadcrumbSegmentKind::Symbol,
    ] {
        drop_order.extend(
            kinds
                .iter()
                .enumerate()
                .filter(|(index, segment_kind)| {
                    *index != last_index
                        && protected_index != Some(*index)
                        && **segment_kind == kind
                })
                .map(|(index, _)| index),
        );
    }

    for index in drop_order {
        dropped[index] = true;
        if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
            break;
        }
    }

    breadcrumb_layout_plan_from_dropped(&dropped)
}

pub(super) fn breadcrumb_path_is_navigable(
    has_project_path: bool,
    worktree_is_single_file: Option<bool>,
) -> bool {
    has_project_path && !worktree_is_single_file.unwrap_or(false)
}

// Negative margins cancel the trigger padding out of layout; the bar stays text-sized.
pub(super) const SEGMENT_TRIGGER_PADDING_X: f32 = 2.;
pub(super) const SEGMENT_TRIGGER_PADDING_TOP: f32 = 3.;
pub(super) const SEGMENT_TRIGGER_PADDING_BOTTOM: f32 = 4.;

#[derive(Clone)]
pub(super) struct PreparedBreadcrumbSegment {
    pub(super) kind: BreadcrumbSegmentKind,
    pub(super) label: HighlightedText,
    pub(super) target: Option<BreadcrumbSegmentTarget>,
    pub(super) dirty_filename_style: bool,
    pub(super) icon: Option<SharedString>,
    pub(super) git_status_color: Option<Color>,
}

pub(super) struct BreadcrumbSegmentMetrics {
    widths: Vec<Pixels>,
    ellipsis_width: Pixels,
    protected_index: Option<usize>,
}

fn breadcrumb_layout_plan_width(
    widths: &[Pixels],
    plan: &BreadcrumbLayoutPlan,
    ellipsis_width: Pixels,
) -> Pixels {
    let mut dropped = vec![false; widths.len()];
    for range in &plan.ellipses {
        for index in range.clone() {
            dropped[index] = true;
        }
    }
    total_breadcrumb_layout_width(widths, &dropped, ellipsis_width)
}

pub(super) struct BreadcrumbStrip {
    pub(super) segments: Vec<PreparedBreadcrumbSegment>,
    pub(super) editor: Option<WeakEntity<Editor>>,
    pub(super) breadcrumb_font: Option<Font>,
}

impl BreadcrumbStrip {
    fn protected_segment_index(&self, cx: &App) -> Option<usize> {
        let editor = self.editor.as_ref()?.upgrade()?;
        let menu = editor.read(cx).breadcrumb_navigation_menu()?.clone();
        let menu = menu.read(cx);
        match menu.listing() {
            BreadcrumbListing::Directory { worktree_id, path } => {
                self.segments.iter().position(|segment| {
                    matches!(
                        &segment.target,
                        Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id: segment_worktree,
                            path: segment_path,
                            ..
                        }) if segment_worktree == worktree_id && segment_path.as_ref() == path.as_ref()
                    )
                })
            }
            BreadcrumbListing::Symbols { buffer_id, parent } => {
                self.segments.iter().position(|segment| match &segment.target {
                    Some(BreadcrumbSegmentTarget::Symbol {
                        buffer_id: segment_buffer,
                        item,
                    }) if segment_buffer == buffer_id => match (parent, item) {
                        (None, None) => true,
                        (Some(a), Some(b)) => a.range == b.range,
                        _ => false,
                    },
                    _ => false,
                })
            }
        }
    }

    fn effective_text_style(&self, window: &Window) -> gpui::TextStyle {
        let mut text_style = window.text_style();
        if let Some(font) = &self.breadcrumb_font {
            text_style.font_family = font.family.clone();
            text_style.font_features = font.features.clone();
            text_style.font_style = font.style;
            text_style.font_weight = font.weight;
        }
        text_style
    }

    fn measure(&self, window: &mut Window) -> BreadcrumbSegmentMetrics {
        let text_style = self.effective_text_style(window);
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let gap = window.rem_size() * 0.25;

        let arrow_run = text_style.to_run("›".len());
        let arrow_width = window
            .text_system()
            .shape_line("›".into(), font_size, &[arrow_run], None)
            .width();

        let ellipsis_run = text_style.to_run("⋯".len());
        let ellipsis_label_width = window
            .text_system()
            .shape_line("⋯".into(), font_size, &[ellipsis_run], None)
            .width();
        let ellipsis_width = ellipsis_label_width + arrow_width + gap * 2.;

        let widths = self
            .segments
            .iter()
            .map(|segment| {
                let text = flatten_text_for_single_line_display(&segment.label.text);
                let label_width = if segment.dirty_filename_style {
                    measure_dirty_filename_width(
                        &text,
                        &segment.label,
                        &text_style,
                        font_size,
                        window,
                    )
                } else {
                    let runs = highlighted_text_runs(&text, &segment.label, &text_style);
                    window
                        .text_system()
                        .shape_line(text.into(), font_size, &runs, None)
                        .width()
                };
                let icon_width = if segment.icon.is_some() {
                    IconSize::Small.rems() * window.rem_size() + gap
                } else {
                    Pixels::ZERO
                };
                label_width + icon_width + arrow_width + gap * 2.
            })
            .collect();

        BreadcrumbSegmentMetrics {
            widths,
            ellipsis_width,
            protected_index: None,
        }
    }

    fn styled_separator_glyph(&self, glyph: &str, window: &Window, cx: &App) -> gpui::AnyElement {
        let mut text_style = self.effective_text_style(window);
        text_style.color = Color::Placeholder.color(cx);
        StyledText::new(glyph.to_string())
            .with_default_highlights(&text_style, [])
            .into_any()
    }

    fn with_separator(
        &self,
        position: usize,
        last_position: usize,
        content: gpui::AnyElement,
        window: &Window,
        cx: &App,
    ) -> gpui::AnyElement {
        let separator = self.styled_separator_glyph("›", window, cx);
        h_flex()
            .items_center()
            .gap_1()
            .child(content)
            .child(
                div()
                    .when(position == last_position, |this| this.invisible())
                    .child(separator),
            )
            .into_any_element()
    }

    fn render_segment(
        &self,
        index: usize,
        position: usize,
        last_position: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let segment = &self.segments[index];
        let mut text_style = self.effective_text_style(window);
        text_style.color = match segment.kind {
            BreadcrumbSegmentKind::File => {
                segment.git_status_color.unwrap_or(Color::Default).color(cx)
            }
            _ => Color::Muted.color(cx),
        };

        let label = if segment.dirty_filename_style {
            apply_dirty_filename_style(&segment.label, &text_style).unwrap_or_else(|| {
                StyledText::new(flatten_text_for_single_line_display(&segment.label.text))
                    .with_default_highlights(&text_style, segment.label.highlights.clone())
                    .into_any()
            })
        } else {
            StyledText::new(flatten_text_for_single_line_display(&segment.label.text))
                .with_default_highlights(&text_style, segment.label.highlights.clone())
                .into_any()
        };
        let label = if let Some(icon) = segment.icon.clone() {
            h_flex()
                .gap_1()
                .child(
                    Icon::from_path(icon)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )
                .child(label)
                .into_any_element()
        } else {
            label
        };

        let content = match (segment.target.clone(), self.editor.clone()) {
            (Some(target), Some(editor)) => {
                self.render_clickable_segment(index, target, label, editor, cx)
            }
            _ => label,
        };
        self.with_separator(position, last_position, content, window, cx)
    }

    fn render_clickable_segment(
        &self,
        index: usize,
        target: BreadcrumbSegmentTarget,
        label: gpui::AnyElement,
        editor: WeakEntity<Editor>,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let tooltip_title: SharedString = match &target {
            BreadcrumbSegmentTarget::Directory { path, .. } => {
                if path.is_empty() {
                    "Browse project root".into()
                } else {
                    format!("Browse {}", path.as_unix_str()).into()
                }
            }
            BreadcrumbSegmentTarget::Symbol { item, .. } => {
                if item.is_some() {
                    "Navigate related symbols".into()
                } else {
                    "Navigate file symbols".into()
                }
            }
        };
        let tooltip_meta: SharedString = "Right-click to copy this path".into();

        let menu_open = editor
            .upgrade()
            .is_some_and(|editor| editor.read(cx).breadcrumb_navigation_menu().is_some());
        let trigger = ButtonLike::new(("breadcrumb-segment", index))
            .style(ButtonStyle::Subtle)
            .size(ButtonSize::None)
            .child(
                div()
                    .px(px(SEGMENT_TRIGGER_PADDING_X))
                    .pt(px(SEGMENT_TRIGGER_PADDING_TOP))
                    .pb(px(SEGMENT_TRIGGER_PADDING_BOTTOM))
                    .child(label),
            )
            .when(!menu_open, |this| {
                this.tooltip(move |_, cx| {
                    Tooltip::with_meta(tooltip_title.clone(), None, tooltip_meta.clone(), cx)
                })
            })
            .on_click({
                let editor = editor.clone();
                let target = target.clone();
                move |event, window, cx| {
                    if event.click_count() >= 2 {
                        return;
                    }
                    let Some(editor_entity) = editor.upgrade() else {
                        return;
                    };
                    editor_entity.update(cx, |editor, cx| {
                        editor.open_or_toggle_breadcrumb_listing(target.clone(), window, cx);
                    });
                }
            });

        let mut wrapper = div()
            .id(("breadcrumb-segment-hit", index))
            .debug_selector(move || format!("breadcrumb-segment-{index}"))
            .mx(px(-SEGMENT_TRIGGER_PADDING_X))
            .mt(px(-SEGMENT_TRIGGER_PADDING_TOP))
            .mb(px(-SEGMENT_TRIGGER_PADDING_BOTTOM));

        {
            let editor = editor.clone();
            let target = target.clone();
            wrapper = wrapper.on_mouse_down(MouseButton::Right, move |_, _, cx| {
                let Some(editor_entity) = editor.upgrade() else {
                    return;
                };
                let abs_path = match &target {
                    BreadcrumbSegmentTarget::Directory { worktree_id, path } => {
                        editor_entity.read(cx).project().and_then(|project| {
                            let worktree = project.read(cx).worktree_for_id(*worktree_id, cx)?;
                            Some(worktree.read(cx).absolutize(path))
                        })
                    }
                    BreadcrumbSegmentTarget::Symbol { .. } => {
                        editor_entity.update(cx, |editor, cx| editor.target_file_abs_path(cx))
                    }
                };
                if let Some(abs_path) = abs_path
                    && let Some(path_str) = abs_path.to_str()
                {
                    cx.write_to_clipboard(ClipboardItem::new_string(path_str.to_string()));
                }
                cx.stop_propagation();
            });
        }

        if let BreadcrumbSegmentTarget::Directory {
            worktree_id, path, ..
        } = &target
        {
            let worktree_id = *worktree_id;
            let path = path.clone();
            let workspace = editor
                .upgrade()
                .and_then(|editor| editor.read(cx).workspace())
                .map(|workspace| workspace.downgrade());
            if let Some(workspace) = workspace {
                wrapper = wrapper.on_mouse_down(MouseButton::Left, move |event, _, cx| {
                    if event.click_count < 2 {
                        return;
                    }
                    reveal_directory_in_project_panel(&workspace, worktree_id, &path, cx);
                });
            }
        }

        wrapper.child(trigger).into_any_element()
    }

    fn render_ellipsis(
        &self,
        position: usize,
        last_position: usize,
        window: &Window,
        cx: &App,
    ) -> gpui::AnyElement {
        let content = self.styled_separator_glyph("⋯", window, cx);
        self.with_separator(position, last_position, content, window, cx)
    }
}

pub(super) struct BreadcrumbStripPrepaintState {
    children: Vec<gpui::AnyElement>,
    menu_element: Option<gpui::AnyElement>,
}

impl gpui::IntoElement for BreadcrumbStrip {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for BreadcrumbStrip {
    type RequestLayoutState = BreadcrumbSegmentMetrics;
    type PrepaintState = BreadcrumbStripPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut metrics = self.measure(window);
        metrics.protected_index = self.protected_segment_index(cx);
        let natural_width = metrics
            .widths
            .iter()
            .fold(Pixels::ZERO, |total, width| total + *width);
        let line_height = window.text_style().line_height_in_pixels(window.rem_size());

        let widths = metrics.widths.clone();
        let ellipsis_width = metrics.ellipsis_width;
        let kinds: Vec<BreadcrumbSegmentKind> = self.segments.iter().map(|s| s.kind).collect();
        let protected_index = metrics.protected_index;

        let layout_id = window.request_measured_layout(
            Style::default(),
            move |known_dimensions, available_space, _window, _cx| {
                let width = known_dimensions
                    .width
                    .unwrap_or(match available_space.width {
                        AvailableSpace::Definite(available_width) => {
                            let plan = plan_breadcrumb_layout(
                                &widths,
                                &kinds,
                                ellipsis_width,
                                available_width,
                                protected_index,
                            );
                            breadcrumb_layout_plan_width(&widths, &plan, ellipsis_width)
                        }
                        AvailableSpace::MinContent | AvailableSpace::MaxContent => natural_width,
                    });
                let height = known_dimensions.height.unwrap_or(line_height);
                size(width, height)
            },
        );

        (layout_id, metrics)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        metrics: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let kinds: Vec<BreadcrumbSegmentKind> = self.segments.iter().map(|s| s.kind).collect();
        let protected_index = metrics.protected_index;
        let plan = plan_breadcrumb_layout(
            &metrics.widths,
            &kinds,
            metrics.ellipsis_width,
            bounds.size.width,
            protected_index,
        );

        enum FinalItem {
            Segment(usize),
            Ellipsis,
        }

        let segment_count = kinds.len();
        let mut sequence = Vec::with_capacity(plan.visible.len() + plan.ellipses.len());
        let mut index = 0;
        while index < segment_count {
            if let Some(range) = plan.ellipses.iter().find(|range| range.start == index) {
                sequence.push(FinalItem::Ellipsis);
                index = range.end;
            } else {
                sequence.push(FinalItem::Segment(index));
                index += 1;
            }
        }

        let last_position = sequence.len().saturating_sub(1);
        let gap = window.rem_size() * 0.25;
        let mut x = bounds.origin.x;
        let mut children = Vec::with_capacity(sequence.len());
        let mut menu_anchor_bounds = None;
        for (position, item) in sequence.into_iter().enumerate() {
            let segment_index = match &item {
                FinalItem::Segment(index) => Some(*index),
                FinalItem::Ellipsis => None,
            };
            let mut element = match item {
                FinalItem::Segment(index) => {
                    self.render_segment(index, position, last_position, window, cx)
                }
                FinalItem::Ellipsis => self.render_ellipsis(position, last_position, window, cx),
            };
            let available_space = size(
                AvailableSpace::MaxContent,
                AvailableSpace::Definite(bounds.size.height),
            );
            let element_size = element.layout_as_root(available_space, window, cx);
            let y_offset = (bounds.size.height - element_size.height) / 2.;
            let origin = point(x, bounds.origin.y + y_offset);
            if let Some(segment_index) = segment_index
                && protected_index == Some(segment_index)
            {
                menu_anchor_bounds = Some(Bounds {
                    origin,
                    size: element_size,
                });
            }
            element.prepaint_at(origin, window, cx);
            x += element_size.width + gap;
            children.push(element);
        }

        let menu = self
            .editor
            .as_ref()
            .and_then(|editor| editor.upgrade())
            .and_then(|editor| editor.read(cx).breadcrumb_navigation_menu().cloned());

        let menu_element = match (menu, menu_anchor_bounds) {
            (Some(menu), Some(anchor_bounds)) => {
                let offset = point(
                    -(rems_from_px(5. - SEGMENT_TRIGGER_PADDING_X) * window.rem_size()),
                    rems_from_px(2.) * window.rem_size(),
                );
                let position = point(
                    anchor_bounds.origin.x + offset.x,
                    anchor_bounds.bottom() + offset.y,
                );
                let mut element = deferred(
                    anchored()
                        .snap_to_window_with_margin(px(8.))
                        .anchor(gpui::Anchor::TopLeft)
                        .position(position)
                        .child(menu),
                )
                .with_priority(1)
                .into_any();
                let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                let _size = element.layout_as_root(available_space, window, cx);
                element.prepaint_at(position, window, cx);
                Some(element)
            }
            (Some(_), None) => {
                if let Some(editor) = self.editor.as_ref().and_then(|editor| editor.upgrade()) {
                    window.defer(cx, move |window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.dismiss_breadcrumb_navigation(window, cx);
                        });
                    });
                }
                None
            }
            (None, _) => None,
        };

        BreadcrumbStripPrepaintState {
            children,
            menu_element,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut prepaint.children {
            child.paint(window, cx);
        }
        if let Some(menu_element) = prepaint.menu_element.as_mut() {
            menu_element.paint(window, cx);
        }
    }
}
pub(super) fn dirty_filename_bold_range(segment: &HighlightedText) -> Option<Range<usize>> {
    let filename_position = std::path::Path::new(segment.text.as_ref())
        .file_name()
        .and_then(|file_name| {
            let filename_str = file_name.to_string_lossy();
            segment.text.rfind(filename_str.as_ref())
        })?;
    Some(filename_position..segment.text.len())
}

fn highlighted_text_runs(
    text: &str,
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
) -> Vec<gpui::TextRun> {
    let mut runs = Vec::new();
    let mut cursor = 0usize;
    for (range, highlight) in &segment.highlights {
        let range = range.start.min(text.len())..range.end.min(text.len());
        if range.start >= range.end {
            continue;
        }
        if range.start > cursor {
            runs.push(text_style.to_run(range.start - cursor));
        }
        let mut styled = text_style.clone();
        if let Some(weight) = highlight.font_weight {
            styled.font_weight = weight;
        }
        if let Some(style) = highlight.font_style {
            styled.font_style = style;
        }
        runs.push(styled.to_run(range.end - range.start.min(range.end)));
        cursor = range.end;
    }
    if cursor < text.len() {
        runs.push(text_style.to_run(text.len() - cursor));
    }
    if runs.is_empty() {
        runs.push(text_style.to_run(text.len()));
    }
    runs
}

fn measure_dirty_filename_width(
    text: &str,
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
    font_size: Pixels,
    window: &mut Window,
) -> Pixels {
    let Some(bold_range) = dirty_filename_bold_range(segment) else {
        let run = text_style.to_run(text.len());
        return window
            .text_system()
            .shape_line(text.to_string().into(), font_size, &[run], None)
            .width();
    };

    if bold_range.start == 0 {
        let mut bold_style = text_style.clone();
        bold_style.font_weight = FontWeight::BOLD;
        let run = bold_style.to_run(text.len());
        return window
            .text_system()
            .shape_line(text.to_string().into(), font_size, &[run], None)
            .width();
    }

    let mut runs = Vec::new();
    if bold_range.start > 0 {
        runs.push(text_style.to_run(bold_range.start));
    }
    let mut bold_style = text_style.clone();
    bold_style.font_weight = FontWeight::BOLD;
    runs.push(bold_style.to_run(bold_range.end - bold_range.start));
    window
        .text_system()
        .shape_line(text.to_string().into(), font_size, &runs, None)
        .width()
}

pub(super) fn apply_dirty_filename_style(
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
) -> Option<gpui::AnyElement> {
    let text = flatten_text_for_single_line_display(&segment.text);
    let bold_range = dirty_filename_bold_range(segment)?;
    let bold_weight = FontWeight::BOLD;

    if bold_range.start == 0 {
        let mut filename_style = text_style.clone();
        filename_style.font_weight = bold_weight;

        return Some(
            StyledText::new(text)
                .with_default_highlights(&filename_style, [])
                .into_any(),
        );
    }

    let highlight_style = gpui::HighlightStyle {
        font_weight: Some(bold_weight),
        ..Default::default()
    };

    let highlight = vec![(bold_range, highlight_style)];
    Some(
        StyledText::new(text)
            .with_default_highlights(text_style, highlight)
            .into_any(),
    )
}
