use super::super::*;
use super::BreadcrumbSegmentTarget;
use super::outline::flatten_text_for_single_line_display;
use super::path::reveal_directory_in_project_panel;
use crate::actions::OpenBreadcrumbNavigation;
use gpui::{anchored, deferred};
use util::paths::PathStyle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BreadcrumbSegmentKind {
    Root,
    Middle,
    File,
    Symbol,
}

pub(super) fn classify_breadcrumb_segment_kinds(
    segment_count: usize,
    file_segment_index: Option<usize>,
    has_root_segment: bool,
) -> Vec<BreadcrumbSegmentKind> {
    (0..segment_count)
        .map(|index| {
            let Some(file_segment_index) = file_segment_index else {
                return BreadcrumbSegmentKind::Middle;
            };
            match index.cmp(&file_segment_index) {
                Ordering::Greater => BreadcrumbSegmentKind::Symbol,
                Ordering::Equal => BreadcrumbSegmentKind::File,
                Ordering::Less if has_root_segment && index == 0 => BreadcrumbSegmentKind::Root,
                Ordering::Less => BreadcrumbSegmentKind::Middle,
            }
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

pub(super) const MAX_BREADCRUMB_SEGMENTS_HARD_CAP: usize = 64;

pub(super) const ELLIPSIS_GLYPH: &str = "⋯";
pub(super) const SEPARATOR_GLYPH: &str = "›";

fn hard_cap_ellipsis() -> HighlightedText {
    HighlightedText {
        text: ELLIPSIS_GLYPH.into(),
        highlights: vec![],
    }
}

fn remap_index_after_splice(index: &mut usize, range: &Range<usize>, removed: usize) {
    if *index >= range.end {
        *index -= removed - 1;
    } else if *index > range.start {
        *index = range.start;
    }
}

fn splice_segment_run(
    segments: &mut Vec<HighlightedText>,
    symbol_segments: &mut Vec<Option<BreadcrumbSegmentTarget>>,
    kinds: &mut Vec<BreadcrumbSegmentKind>,
    file_segment_index: &mut usize,
    protected_index: &mut Option<usize>,
    range: Range<usize>,
    kind: BreadcrumbSegmentKind,
) {
    if range.end <= range.start {
        return;
    }
    let removed = range.end - range.start;
    // The glyph inherits the deepest target it hides, so a capped run is still a way into the
    // range rather than a dead control.
    let hidden_target = symbol_segments[range.clone()]
        .iter()
        .rev()
        .find_map(|target| target.clone());
    segments.splice(range.clone(), Some(hard_cap_ellipsis()));
    symbol_segments.splice(range.clone(), Some(hidden_target));
    kinds.splice(range.clone(), Some(kind));
    remap_index_after_splice(file_segment_index, &range, removed);
    if let Some(protected) = protected_index.as_mut() {
        remap_index_after_splice(protected, &range, removed);
    }
}

fn segment_run_bounds(
    kinds: &[BreadcrumbSegmentKind],
    kind: BreadcrumbSegmentKind,
) -> Option<Range<usize>> {
    let start = kinds.iter().position(|candidate| *candidate == kind)?;
    let end = kinds.iter().rposition(|candidate| *candidate == kind)? + 1;
    Some(start..end)
}

fn hard_cap_kind_run(
    kind: BreadcrumbSegmentKind,
    segments: &mut Vec<HighlightedText>,
    symbol_segments: &mut Vec<Option<BreadcrumbSegmentTarget>>,
    kinds: &mut Vec<BreadcrumbSegmentKind>,
    file_segment_index: &mut usize,
    protected_index: &mut Option<usize>,
) {
    let Some(run) = segment_run_bounds(kinds, kind) else {
        return;
    };
    if run.len() <= MAX_BREADCRUMB_SEGMENTS_HARD_CAP {
        return;
    }

    if let Some(protected) = protected_index.filter(|index| run.contains(index)) {
        // Either end of the window can splice, and each glyph a splice inserts occupies one of
        // the capped slots.
        let keep = MAX_BREADCRUMB_SEGMENTS_HARD_CAP - 2;
        let mut window_start = protected.saturating_sub(keep / 2).max(run.start);
        let mut window_end = window_start + keep;
        if window_end > run.end {
            window_end = run.end;
            window_start = window_end.saturating_sub(keep).max(run.start);
        }
        // Splicing the left side first would shift the indices the right range addresses.
        splice_segment_run(
            segments,
            symbol_segments,
            kinds,
            file_segment_index,
            protected_index,
            window_end..run.end,
            kind,
        );
        splice_segment_run(
            segments,
            symbol_segments,
            kinds,
            file_segment_index,
            protected_index,
            run.start..window_start,
            kind,
        );
        return;
    }

    // The splice puts a glyph back into the run it collapses, so one capped slot is already
    // spoken for; keeping half the cap on each side would leave the cap plus one behind.
    let keep = MAX_BREADCRUMB_SEGMENTS_HARD_CAP - 1;
    let head = keep - keep / 2;
    let tail = keep / 2;
    splice_segment_run(
        segments,
        symbol_segments,
        kinds,
        file_segment_index,
        protected_index,
        run.start + head..run.end - tail,
        kind,
    );
}

/// Caps both the directory run and the symbol trail, which is what `measure` shapes and what
/// `plan_breadcrumb_layout` re-sums per drop.
pub(super) fn hard_cap_segment_runs(
    mut segments: Vec<HighlightedText>,
    mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
    mut kinds: Vec<BreadcrumbSegmentKind>,
    mut file_segment_index: usize,
    mut protected_index: Option<usize>,
) -> (
    Vec<HighlightedText>,
    Vec<Option<BreadcrumbSegmentTarget>>,
    Vec<BreadcrumbSegmentKind>,
    usize,
) {
    for kind in [BreadcrumbSegmentKind::Middle, BreadcrumbSegmentKind::Symbol] {
        hard_cap_kind_run(
            kind,
            &mut segments,
            &mut symbol_segments,
            &mut kinds,
            &mut file_segment_index,
            &mut protected_index,
        );
    }
    (segments, symbol_segments, kinds, file_segment_index)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BreadcrumbLayoutPlan {
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

pub(super) fn plan_breadcrumb_layout(
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

pub(super) const SEGMENT_TRIGGER_PADDING_X: f32 = 2.;
/// `ButtonSize::None` still applies `px_px()` - one pixel each side - so the wrapper has to
/// cancel that too, or every clickable segment paints two pixels wider than `widths` says.
pub(super) const BUTTON_LIKE_EDGE_PX: f32 = 1.;
// The trigger fills the bar so its hover and hit area cover the whole row; `ButtonSize::None`
// would otherwise pin it to 16px inside a 22px bar and leave the edges dead.
pub(super) const SEGMENT_TRIGGER_HEIGHT: f32 = 22.;

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
    /// Icon, gaps and the separator `with_separator` lays out even on the last segment.
    /// Prepaint subtracts it to turn room left in the strip into a `max_w` for the label.
    chrome_widths: Vec<Pixels>,
    ellipsis_width: Pixels,
    /// The same accounting as `chrome_widths`, for the glyph a collapsed run paints.
    ellipsis_chrome_width: Pixels,
    protected_index: Option<usize>,
    /// Carried rather than rebuilt: request_layout and prepaint both plan the layout, and the
    /// bar replans on every repaint.
    kinds: Vec<BreadcrumbSegmentKind>,
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

/// What a segment's chrome needs from the editor rather than from the segment, read once a
/// frame instead of once per segment: `active_buffer` clones a multibuffer snapshot, and the
/// strip paints up to `MAX_BREADCRUMB_SEGMENTS_HARD_CAP` segments.
#[derive(Clone, Copy)]
struct SegmentEditorState {
    /// A buffer with no file has no path to copy, so neither the tooltip line nor the
    /// right-click handler may claim otherwise.
    has_file_path: bool,
    menu_open: bool,
}

impl SegmentEditorState {
    fn read(editor: Option<&WeakEntity<Editor>>, cx: &App) -> Self {
        let Some(editor) = editor.and_then(|editor| editor.upgrade()) else {
            return Self {
                has_file_path: false,
                menu_open: false,
            };
        };
        let editor = editor.read(cx);
        Self {
            has_file_path: editor
                .active_buffer(cx)
                .is_some_and(|buffer| buffer.read(cx).file().is_some()),
            menu_open: editor.breadcrumb_navigation_menu().is_some(),
        }
    }
}

impl BreadcrumbStrip {
    fn protected_segment_index(&self, cx: &App) -> Option<usize> {
        let editor = self.editor.as_ref()?.upgrade()?;
        let menu = editor.read(cx).breadcrumb_navigation_menu()?.clone();
        let menu = menu.read(cx);
        let listing = menu.listing();
        self.segments.iter().position(|segment| {
            segment
                .target
                .as_ref()
                .is_some_and(|target| target.matches_listing(listing))
        })
    }

    #[cfg(test)]
    pub fn measured_widths_for_test(&self, window: &mut Window) -> Vec<Pixels> {
        self.measure(window).widths
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

        let arrow_run = text_style.to_run(SEPARATOR_GLYPH.len());
        let arrow_width = window
            .text_system()
            .shape_line(SEPARATOR_GLYPH.into(), font_size, &[arrow_run], None)
            .width();

        let ellipsis_run = text_style.to_run(ELLIPSIS_GLYPH.len());
        let ellipsis_label_width = window
            .text_system()
            .shape_line(ELLIPSIS_GLYPH.into(), font_size, &[ellipsis_run], None)
            .width();
        let ellipsis_chrome_width = arrow_width + gap * 2.;
        let ellipsis_width = ellipsis_label_width + ellipsis_chrome_width;

        let (widths, chrome_widths) = self
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
                let chrome_width = icon_width + arrow_width + gap * 2.;
                (label_width + chrome_width, chrome_width)
            })
            .unzip();

        BreadcrumbSegmentMetrics {
            widths,
            chrome_widths,
            ellipsis_width,
            ellipsis_chrome_width,
            protected_index: None,
            kinds: self.segments.iter().map(|segment| segment.kind).collect(),
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
        let separator = self.styled_separator_glyph(SEPARATOR_GLYPH, window, cx);
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
        max_label_width: Option<Pixels>,
        editor_state: SegmentEditorState,
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
        let label = div()
            .min_w_0()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .when_some(max_label_width, |this, max_width| this.max_w(max_width))
            .child(label)
            .into_any_element();
        let label = if let Some(icon) = segment.icon.clone() {
            h_flex()
                .min_w_0()
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

        // One deep clone per segment: the tooltip, click and right-click closures each hold the
        // target for the life of the frame.
        let content = match (segment.target.clone(), self.editor.clone()) {
            (Some(target), Some(editor)) => self.render_clickable_segment(
                ("breadcrumb-segment", index).into(),
                |index| format!("breadcrumb-segment-{index}"),
                index,
                Rc::new(target),
                label,
                editor,
                editor_state,
                cx,
            ),
            _ => label,
        };
        self.with_separator(position, last_position, content, window, cx)
    }

    fn render_clickable_segment(
        &self,
        element_id: ElementId,
        debug_selector: fn(usize) -> String,
        debug_index: usize,
        target: Rc<BreadcrumbSegmentTarget>,
        label: gpui::AnyElement,
        editor: WeakEntity<Editor>,
        editor_state: SegmentEditorState,
        cx: &mut App,
    ) -> gpui::AnyElement {
        // The title is built inside the tooltip closure: formatting a path here would allocate
        // for a string the user usually never sees.
        let tooltip_target = target.clone();
        let tooltip_editor = editor.clone();
        let copyable_path = match target.as_ref() {
            BreadcrumbSegmentTarget::Directory { .. } => true,
            BreadcrumbSegmentTarget::Symbol { .. } => editor_state.has_file_path,
        };
        let menu_open = editor_state.menu_open;
        let trigger = ButtonLike::new(element_id.clone())
            .style(ButtonStyle::Subtle)
            .size(ButtonSize::None)
            .height(px(SEGMENT_TRIGGER_HEIGHT).into())
            .child(div().px(px(SEGMENT_TRIGGER_PADDING_X)).child(label))
            .when(!menu_open, |this| {
                this.tooltip(move |_, cx| {
                    // Only the file segment names the chord, because only its listing is the
                    // one the chord opens; see `segment_tooltip_shows_navigation_chord`.
                    let title: SharedString = match tooltip_target.as_ref() {
                        BreadcrumbSegmentTarget::Directory { path, .. } => {
                            if path.is_empty() {
                                "Browse project root".into()
                            } else {
                                let path_style = tooltip_editor
                                    .upgrade()
                                    .and_then(|editor| {
                                        Some(editor.read(cx).project()?.read(cx).path_style(cx))
                                    })
                                    .unwrap_or(PathStyle::local());
                                format!("Browse {}", path.display(path_style)).into()
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
                    let chord: Option<&dyn Action> =
                        segment_tooltip_shows_navigation_chord(tooltip_target.as_ref())
                            .then_some(&OpenBreadcrumbNavigation);
                    if copyable_path {
                        Tooltip::with_meta(title, chord, "Right-click to copy this path", cx)
                    } else if let Some(chord) = chord {
                        Tooltip::for_action(title, chord, cx)
                    } else {
                        Tooltip::simple(title, cx)
                    }
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
                        editor.open_or_toggle_breadcrumb_listing((*target).clone(), window, cx);
                    });
                }
            });

        let mut wrapper = div()
            .id(element_id)
            .debug_selector(move || debug_selector(debug_index))
            .mx(px(-(SEGMENT_TRIGGER_PADDING_X + BUTTON_LIKE_EDGE_PX)));

        if copyable_path {
            let editor = editor.clone();
            let target = target.clone();
            wrapper = wrapper.on_mouse_down(MouseButton::Right, move |_, _, cx| {
                let Some(editor_entity) = editor.upgrade() else {
                    return;
                };
                let abs_path = match target.as_ref() {
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
        } = target.as_ref()
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
        hidden: Range<usize>,
        position: usize,
        last_position: usize,
        max_glyph_width: Option<Pixels>,
        editor_state: SegmentEditorState,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let content = self.styled_separator_glyph(ELLIPSIS_GLYPH, window, cx);
        // `min_w_0` before `max_w`: taffy applies the automatic minimum size afterwards, which
        // would otherwise hand the glyph back the width the cap just took away.
        let content = div()
            .min_w_0()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .when_some(max_glyph_width, |this, max_width| this.max_w(max_width))
            .child(content)
            .into_any_element();
        // Standing in for a run of hidden segments, the ellipsis browses the deepest of
        // them, so collapsing the bar never puts an ancestor out of reach.
        let deepest_hidden = hidden
            .clone()
            .rev()
            .find_map(|index| self.segments.get(index)?.target.clone());
        let content = match (deepest_hidden, self.editor.clone()) {
            (Some(target), Some(editor)) => self.render_clickable_segment(
                ("breadcrumb-collapsed-run", hidden.start).into(),
                |_| "breadcrumb-collapsed-run".to_string(),
                0,
                Rc::new(target),
                content,
                editor,
                editor_state,
                cx,
            ),
            _ => content,
        };
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
        let kinds = metrics.kinds.clone();
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
        let protected_index = metrics.protected_index;
        let kinds = &metrics.kinds;
        let plan = plan_breadcrumb_layout(
            &metrics.widths,
            kinds,
            metrics.ellipsis_width,
            bounds.size.width,
            protected_index,
        );

        let segment_count = kinds.len();
        let mut sequence = Vec::with_capacity(plan.visible.len() + plan.ellipses.len());
        let mut index = 0;
        while index < segment_count {
            if let Some(range) = plan.ellipses.iter().find(|range| range.start == index) {
                sequence.push(FinalItem::Ellipsis(range.clone()));
                index = range.end;
            } else {
                sequence.push(FinalItem::Segment(index));
                index += 1;
            }
        }

        let gap = window.rem_size() * 0.25;
        // The planner keeps the protected segment, but it is usually also the last one, and
        // dropping it here would leave an open menu with nothing to anchor to.
        let tail_is_protected = matches!(
            sequence.last(),
            Some(FinalItem::Segment(index)) if protected_index == Some(*index)
        );
        // A tail with less than an ellipsis of room paints as a separator pointing at nothing,
        // but removing it claims the trail ends where it does not. Collapsing it into an
        // ellipsis keeps `render_ellipsis` resolving to the target it hides.
        if sequence.len() > 1 && !tail_is_protected {
            let tail_start = folded_tail_start(bounds.origin.x, &sequence, metrics);
            let tail_index = match sequence.last() {
                Some(FinalItem::Segment(index)) => Some(*index),
                _ => None,
            };
            // Only a run that already occupies an earlier slot can take the tail over. A fresh
            // glyph would start where the tail did and inherit the room just found too short,
            // and dropping the tail outright would claim the trail ends where it does not - the
            // tail's own budget below already clamps it to what is left.
            if bounds.origin.x + bounds.size.width - tail_start < metrics.ellipsis_width
                && let Some(tail_index) = tail_index
                && matches!(
                    sequence.iter().rev().nth(1),
                    Some(FinalItem::Ellipsis(range)) if range.end == tail_index
                )
            {
                sequence.pop();
                if let Some(FinalItem::Ellipsis(range)) = sequence.last_mut() {
                    range.end = tail_index + 1;
                }
            }
        }

        let last_position = sequence.len().saturating_sub(1);
        let mut x = bounds.origin.x;
        let mut children = Vec::with_capacity(sequence.len());
        let mut menu_anchor_bounds = None;
        let editor_state = SegmentEditorState::read(self.editor.as_ref(), cx);
        for (position, item) in sequence.into_iter().enumerate() {
            let segment_index = match &item {
                FinalItem::Segment(index) => Some(*index),
                FinalItem::Ellipsis(_) => None,
            };
            let remaining_width = (bounds.origin.x + bounds.size.width - x).max(Pixels::ZERO);
            let is_last = position == last_position;
            let mut element = match item {
                FinalItem::Segment(index) => {
                    // The planner drops neither the last segment nor the protected one, so
                    // those two are what can overrun the strip. A protected segment reserves an
                    // ellipsis behind it, or shrinking it just moves the overrun onto the tail.
                    let reserved = metrics.chrome_widths[index]
                        + if is_last {
                            Pixels::ZERO
                        } else {
                            metrics.ellipsis_width
                        };
                    let label_budget = (is_last || protected_index == Some(index))
                        .then(|| (remaining_width - reserved).max(Pixels::ZERO));
                    self.render_segment(
                        index,
                        position,
                        last_position,
                        label_budget,
                        editor_state,
                        window,
                        cx,
                    )
                }
                FinalItem::Ellipsis(hidden) => {
                    // The collapse below can only pick which slot a run lands in, never widen
                    // it, and a protected segment ahead of it may already have eaten the strip.
                    // Only a hard cap keeps the glyph inside the bounds we were handed.
                    let glyph_budget =
                        (remaining_width - metrics.ellipsis_chrome_width).max(Pixels::ZERO);
                    self.render_ellipsis(
                        hidden,
                        position,
                        last_position,
                        Some(glyph_budget),
                        editor_state,
                        window,
                        cx,
                    )
                }
            };
            let available_width = if is_last {
                AvailableSpace::Definite(remaining_width)
            } else {
                AvailableSpace::MaxContent
            };
            let available_space = size(
                available_width,
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
                    -(rems_from_px(5_f32 - SEGMENT_TRIGGER_PADDING_X) * window.rem_size()),
                    rems_from_px(2_f32) * window.rem_size(),
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
        // Clamped to `cursor` as well as to the text: overlapping highlights would otherwise
        // emit a run for ground already covered, and the run lengths must sum to `text.len()`.
        let start = range.start.max(cursor).min(text.len());
        let end = range.end.min(text.len());
        if start >= end {
            continue;
        }
        if start > cursor {
            runs.push(text_style.to_run(start - cursor));
        }
        let mut styled = text_style.clone();
        if let Some(weight) = highlight.font_weight {
            styled.font_weight = weight;
        }
        if let Some(style) = highlight.font_style {
            styled.font_style = style;
        }
        runs.push(styled.to_run(end - start));
        cursor = end;
    }
    if cursor < text.len() {
        runs.push(text_style.to_run(text.len() - cursor));
    }
    if runs.is_empty() {
        runs.push(text_style.to_run(text.len()));
    }
    runs
}

enum FinalItem {
    Segment(usize),
    Ellipsis(Range<usize>),
}

/// The `OpenBreadcrumbNavigation` chord opens the current file's outline, which is what the file
/// segment opens too, so only that segment names the chord. Directory segments open a directory
/// and the deeper symbol segments open a symbol's children, neither of which the chord does.
pub(super) fn segment_tooltip_shows_navigation_chord(target: &BreadcrumbSegmentTarget) -> bool {
    matches!(target, BreadcrumbSegmentTarget::Symbol { item: None, .. })
}

/// Where the tail begins: the sum of every item before it. Each measured width already
/// carries its trailing gap, so adding the inter-item gap again here overestimated the sum by
/// a gap per item and folded tails that fit.
fn folded_tail_start(
    origin: Pixels,
    sequence: &[FinalItem],
    metrics: &BreadcrumbSegmentMetrics,
) -> Pixels {
    sequence
        .iter()
        .rev()
        .skip(1)
        .fold(origin, |x, item| match item {
            FinalItem::Segment(index) => x + metrics.widths[*index],
            FinalItem::Ellipsis(_) => x + metrics.ellipsis_width,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn test_only_the_file_segment_advertises_the_navigation_chord() {
        use util::rel_path::rel_path;

        let file = BreadcrumbSegmentTarget::Symbol {
            buffer_id: language::BufferId::new(1).unwrap(),
            item: None,
        };
        assert!(
            segment_tooltip_shows_navigation_chord(&file),
            "the file segment: the chord opens the file outline it opens too"
        );

        let directory = BreadcrumbSegmentTarget::Directory {
            worktree_id: project::WorktreeId::from_usize(0),
            path: rel_path("src").into_arc(),
        };
        assert!(
            !segment_tooltip_shows_navigation_chord(&directory),
            "a directory segment opens a directory, which the chord does not"
        );
    }

    #[test]
    fn test_tail_start_counts_each_item_once() {
        let metrics = BreadcrumbSegmentMetrics {
            widths: vec![px(40.), px(60.), px(50.)],
            chrome_widths: vec![px(10.), px(10.), px(10.)],
            ellipsis_width: px(20.),
            ellipsis_chrome_width: px(12.),
            protected_index: None,
            kinds: vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
            ],
        };
        let sequence = vec![
            FinalItem::Segment(0),
            FinalItem::Ellipsis(1..2),
            FinalItem::Segment(2),
        ];
        assert_eq!(
            folded_tail_start(px(100.), &sequence, &metrics),
            px(160.),
            "each width already carries its trailing gap; adding the gap again folded tails              that fit"
        );
    }
}
