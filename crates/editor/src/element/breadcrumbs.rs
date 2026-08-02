//! Breadcrumb path and symbol navigation: turns the bar's segments into clickable dropdowns,
//! sharing the project panel's ordering and gitignore treatment rather than reimplementing them.

use super::*;

mod layout;
mod outline;
mod path;

use layout::{
    align_symbol_segments, classify_breadcrumb_segment_kinds, hard_cap_breadcrumb_middle_segments,
};
pub(crate) use layout::BreadcrumbSegmentKind;
use layout::{breadcrumb_layout_plan_width, plan_breadcrumb_layout};
pub(crate) use outline::{child_outline_indices, sibling_outline_indices, top_level_outline_indices};
use outline::render_breadcrumb_symbol_segment;
pub(crate) use path::{BreadcrumbDirectoryPicker, breadcrumb_path_segments};
use path::{
    BreadcrumbDirectoryListingSettings, breadcrumb_path_is_navigable,
    render_breadcrumb_directory_segment,
};

/// What a segment's dropdown drills into.
#[derive(Clone, Debug)]
pub(crate) enum BreadcrumbSegmentTarget {
    /// Lists document symbols: `item: None` is the file segment and lists top-level symbols,
    /// `Some` lists that item's children.
    Symbol {
        buffer_id: BufferId,
        item: Option<OutlineItem<Anchor>>,
    },
    /// Lists `path`'s contents. `active_path` is the same at every ancestor, so a listing at any
    /// depth can mark the trail towards it.
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        active_path: Option<Arc<RelPath>>,
        /// Whether this segment's own dropdown is open, which draws it as the active one.
        is_active_segment: bool,
    },
}

/// Flattens `text` to a single display line. The replacement must be the same UTF-8 length as the
/// newline, since highlight ranges are byte offsets into the unflattened text.
fn flatten_text_for_single_line_display(text: &str) -> String {
    const LINE_BREAK: char = '\n';
    const REPLACEMENT: &str = " ";
    debug_assert_eq!(
        LINE_BREAK.len_utf8(),
        REPLACEMENT.len(),
        "replacing {LINE_BREAK:?} with {REPLACEMENT:?} would shift byte-offset highlight ranges"
    );
    text.replace(LINE_BREAK, REPLACEMENT)
}

/// One segment, resolved ahead of the render pass so the element never reaches back into
/// `Editor` state.
struct PreparedBreadcrumbSegment {
    kind: BreadcrumbSegmentKind,
    label: HighlightedText,
    target: Option<BreadcrumbSegmentTarget>,
    /// Whether this is the dirty file's own segment. Precomputed because the `'static`
    /// `BreadcrumbsRow` can't hold `active_item` or `TabBarSettings`.
    dirty_filename_style: bool,
    /// Icon before the segment's name, which is what tells the file from the directories leading
    /// to it.
    icon: Option<SharedString>,
    /// Text colour: the path stays muted so the file it leads to reads as the subject.
    label_color: Color,
}

/// Per-segment slot width, measured once per render. `shape_line` is cached by text and font, so
/// this is a handful of lookups rather than a reshape. Counting one arrow per segment
/// overestimates slightly, which can only make the row collapse earlier than needed, never later.
struct BreadcrumbSegmentMetrics {
    widths: Vec<Pixels>,
    ellipsis_width: Pixels,
}

/// Runs describing how `render_segment` will actually paint `segment`'s label. The bold file name
/// `apply_dirty_filename_style` adds is wider than the plain style, so measuring everything at the
/// base weight would plan the row narrower than it gets painted and let it overflow.
fn segment_text_runs(
    segment: &PreparedBreadcrumbSegment,
    text: &str,
    text_style: &gpui::TextStyle,
) -> Vec<gpui::TextRun> {
    let Some(filename_offset) = segment
        .dirty_filename_style
        .then(|| dirty_filename_offset(&segment.label))
        .flatten()
    else {
        return vec![text_style.to_run(text.len())];
    };

    let mut bold_style = text_style.clone();
    bold_style.font_weight = FontWeight::BOLD;
    if filename_offset == 0 {
        return vec![bold_style.to_run(text.len())];
    }
    vec![
        text_style.to_run(filename_offset),
        bold_style.to_run(text.len() - filename_offset),
    ]
}

/// A custom `Element` rather than an `h_flex` because how many segments fit can only be decided
/// once GPUI hands back the row's real width: measured layout in `request_layout`, real children
/// built in `prepaint`, the same pattern `UniformList` uses.
struct BreadcrumbsRow {
    segments: Vec<PreparedBreadcrumbSegment>,
    editor: Option<WeakEntity<Editor>>,
}

/// Names the per-segment hover group, so the highlight lands on the label and not on the
/// separator after it.
const BREADCRUMB_SEGMENT_GROUP: &str = "breadcrumb-segment";

/// Horizontal padding around a segment's label, inside its hover highlight.
const BREADCRUMB_LABEL_PADDING: Pixels = px(4.);

/// Matches the project panel's own entry icons, so the two read as the same tree.
const BREADCRUMB_ICON_SIZE: IconSize = IconSize::Small;

/// Only the file's segment gets an icon. Directories get none, and symbols name code rather than
/// an entry in the tree.
fn breadcrumb_segment_icon(
    target: &Option<BreadcrumbSegmentTarget>,
    file_path: Option<&RelPath>,
    cx: &App,
) -> Option<SharedString> {
    if !BreadcrumbDirectoryListingSettings::get_global(cx).file_icons {
        return None;
    }
    match target {
        Some(BreadcrumbSegmentTarget::Symbol { item: None, .. }) => {
            file_icons::FileIcons::get_icon(file_path?.as_std_path(), cx)
        }
        _ => None,
    }
}

fn breadcrumb_separator_width(window: &Window) -> Pixels {
    IconSize::XSmall.rems().to_pixels(window.rem_size())
}

impl BreadcrumbsRow {
    /// The UI font rather than the buffer font: the bar reads as chrome, not as code.
    fn effective_text_style(&self, window: &Window) -> gpui::TextStyle {
        window.text_style()
    }

    fn measure(&self, window: &mut Window) -> BreadcrumbSegmentMetrics {
        let text_style = self.effective_text_style(window);
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let gap = window.rem_size() * 0.25;

        let arrow_width = breadcrumb_separator_width(window);

        let ellipsis_run = text_style.to_run("⋯".len());
        let ellipsis_label_width = window
            .text_system()
            .shape_line("⋯".into(), font_size, &[ellipsis_run], None)
            .width();
        let ellipsis_width =
            ellipsis_label_width + BREADCRUMB_LABEL_PADDING * 2. + arrow_width + gap * 2.;

        let widths = self
            .segments
            .iter()
            .map(|segment| {
                let text = flatten_text_for_single_line_display(&segment.label.text);
                let runs = segment_text_runs(segment, &text, &text_style);
                let label_width = window
                    .text_system()
                    .shape_line(text.into(), font_size, &runs, None)
                    .width();
                let icon_width = if segment.icon.is_some() {
                    BREADCRUMB_ICON_SIZE.rems().to_pixels(window.rem_size()) + gap
                } else {
                    Pixels::ZERO
                };
                icon_width + label_width + BREADCRUMB_LABEL_PADDING * 2. + arrow_width + gap * 2.
            })
            .collect();

        BreadcrumbSegmentMetrics {
            widths,
            ellipsis_width,
        }
    }

    /// Positions are in the final rendered sequence rather than the raw segment index, since
    /// that's the sequence whose last edge has nothing to point at.
    fn with_separator(
        &self,
        position: usize,
        last_position: usize,
        content: gpui::AnyElement,
        interactive: bool,
        cx: &App,
    ) -> gpui::AnyElement {
        // Only the label is painted on hover. The separator stays clickable, belonging to the
        // segment on its left, but isn't part of that segment's name.
        let label = div()
            .px(BREADCRUMB_LABEL_PADDING)
            .rounded_sm()
            // Multi buffer excerpt headers render the same trail as plain text, with no dropdowns
            // to open, so lighting it up on hover would advertise a click that does nothing.
            .when(interactive, |this| {
                this.group_hover(BREADCRUMB_SEGMENT_GROUP, |style| {
                    style.bg(cx.theme().colors().ghost_element_hover)
                })
            })
            .child(content);

        if position == last_position {
            return label.into_any_element();
        }
        h_flex()
            .gap_1()
            .child(label)
            .child(
                // Nudged down a pixel: breadcrumb text is mostly lowercase, whose visual centre
                // sits below the geometric one a centred chevron lands on.
                div().relative().top(px(2.)).child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::XSmall)
                        .color(Color::Placeholder),
                ),
            )
            .into_any_element()
    }

    fn wrap_segment(&self, element: gpui::AnyElement) -> gpui::AnyElement {
        div()
            .group(BREADCRUMB_SEGMENT_GROUP)
            .child(element)
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
        text_style.color = segment.label_color.color(cx);

        let text = if segment.dirty_filename_style
            && let Some(styled_element) =
                apply_dirty_filename_style(&segment.label, &text_style, cx)
        {
            styled_element
        } else {
            StyledText::new(flatten_text_for_single_line_display(&segment.label.text))
                .with_default_highlights(&text_style, segment.label.highlights.clone())
                .into_any()
        };

        let content = match &segment.icon {
            Some(icon) => h_flex()
                .gap_1()
                .child(
                    // The same optical nudge the separator chevron gets.
                    div().relative().top(px(2.)).child(
                        Icon::from_path(icon.clone())
                            .color(Color::Muted)
                            .size(BREADCRUMB_ICON_SIZE),
                    ),
                )
                .child(text)
                .into_any_element(),
            None => text,
        };
        let interactive = segment.target.is_some() && self.editor.is_some();
        let label = self.with_separator(position, last_position, content, interactive, cx);

        let element = match (segment.target.clone(), self.editor.clone()) {
            (Some(BreadcrumbSegmentTarget::Symbol { buffer_id, item }), Some(editor)) => {
                render_breadcrumb_symbol_segment(editor, buffer_id, item, label, index)
            }
            (
                Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path,
                    active_path,
                    is_active_segment,
                }),
                Some(editor),
            ) => {
                let Some(upgraded_editor) = editor.upgrade() else {
                    return label;
                };
                let Some(workspace) = upgraded_editor
                    .read(cx)
                    .workspace()
                    .map(|workspace| workspace.downgrade())
                else {
                    return label;
                };
                let shared_popover_handle = upgraded_editor.read(cx).breadcrumb_popover_handle();
                render_breadcrumb_directory_segment(
                    editor,
                    workspace,
                    worktree_id,
                    path,
                    active_path,
                    is_active_segment,
                    shared_popover_handle,
                    label,
                    index,
                )
            }
            _ => return label,
        };
        self.wrap_segment(element)
    }

    /// The inert "⋯" standing for a collapsed run: no popover of its own, since everything it
    /// hides is reachable by widening the window or through the segments beside it.
    fn render_ellipsis(&self, position: usize, last_position: usize, cx: &App) -> gpui::AnyElement {
        let content = Label::new("⋯").color(Color::Placeholder).into_any_element();
        self.with_separator(position, last_position, content, false, cx)
    }
}

struct BreadcrumbsRowPrepaintState {
    children: Vec<gpui::AnyElement>,
}

impl gpui::IntoElement for BreadcrumbsRow {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for BreadcrumbsRow {
    type RequestLayoutState = BreadcrumbSegmentMetrics;
    type PrepaintState = BreadcrumbsRowPrepaintState;

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
        _cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let metrics = self.measure(window);
        let natural_width = metrics
            .widths
            .iter()
            .fold(Pixels::ZERO, |total, width| total + *width);
        let line_height = window.text_style().line_height_in_pixels(window.rem_size());

        let widths = metrics.widths.clone();
        let ellipsis_width = metrics.ellipsis_width;
        let kinds: Vec<BreadcrumbSegmentKind> = self.segments.iter().map(|s| s.kind).collect();

        // A flex item's automatic minimum size is its min-content size, so answering `MinContent`
        // with the whole trail would stop the parent ever offering less. The row can always fall
        // back to one segment plus an ellipsis.
        let mut style = Style::default();
        style.min_size.width = px(0.).into();

        let layout_id = window.request_measured_layout(
            style,
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
                            );
                            breadcrumb_layout_plan_width(&widths, &plan, ellipsis_width)
                        }
                        AvailableSpace::MinContent => widths
                            .last()
                            .copied()
                            .unwrap_or(ellipsis_width)
                            .max(ellipsis_width),
                        AvailableSpace::MaxContent => natural_width,
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
        let plan = plan_breadcrumb_layout(
            &metrics.widths,
            &kinds,
            metrics.ellipsis_width,
            bounds.size.width,
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
        for (position, item) in sequence.into_iter().enumerate() {
            let mut element = match item {
                FinalItem::Segment(index) => {
                    self.render_segment(index, position, last_position, window, cx)
                }
                FinalItem::Ellipsis => self.render_ellipsis(position, last_position, cx),
            };
            let available_space = size(
                AvailableSpace::MaxContent,
                AvailableSpace::Definite(bounds.size.height),
            );
            let element_size = element.layout_as_root(available_space, window, cx);
            element.prepaint_at(point(x, bounds.origin.y), window, cx);
            x += element_size.width + gap;
            children.push(element);
        }

        // Every segment has registered its popover handle by now, which is what a pending
        // re-anchor waits for.
        if let Some(editor) = self.editor.as_ref().and_then(WeakEntity::upgrade)
            && editor.read(cx).breadcrumb_pending_reanchor()
        {
            editor.update(cx, |editor, cx| {
                editor.reanchor_breadcrumb_popover(window, cx);
            });
        }

        BreadcrumbsRowPrepaintState { children }
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
    }
}

pub fn render_breadcrumb_text(
    mut segments: Vec<HighlightedText>,
    prefix: Option<gpui::AnyElement>,
    active_item: &dyn ItemHandle,
    multibuffer_header: bool,
    cx: &App,
) -> gpui::AnyElement {
    // min_w_0 because a flex item's minimum size defaults to its content's, which would stop
    // `BreadcrumbsRow` from ever being told to collapse.
    let element = h_flex().flex_grow_1().min_w_0().text_ui(cx);

    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    // Aligned 1:1 with `segments` once the path splitting below runs. The buffer id comes from
    // the singleton rather than `outline_symbols_at_cursor`, so the path segment still gets a
    // menu when the cursor sits outside any symbol.
    let mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = Vec::new();
    // Stays 0 whenever the path splitting below doesn't run.
    let mut file_segment_index = 0usize;
    // Whether path splitting inserted a leading root segment, so
    // `classify_breadcrumb_segment_kinds` can tell it from an ordinary `Middle` component.
    let mut has_root_segment = false;
    // The buffer whose outline the segment dropdowns will need, so hovering the bar can start
    // fetching it before any of them is opened.
    let mut outline_buffer_id = None;
    let mut file_path_for_icon: Option<Arc<RelPath>> = None;
    let mut file_status = None;

    if !multibuffer_header
        && let Some(editor_entity) = editor.as_ref().and_then(WeakEntity::upgrade)
    {
        let editor_ref = editor_entity.read(cx);
        if let Some(buffer) = editor_ref.buffer().read(cx).as_singleton() {
            let buffer_id = buffer.read(cx).remote_id();
            outline_buffer_id = Some(buffer_id);
            let mut path_split = false;

            // The real open file's path, independent of any navigation below: it is both the
            // fallback bar and the `active_path` submenus keep highlighting towards.
            let real_project_path = active_item.project_path(cx);
            file_path_for_icon = real_project_path
                .as_ref()
                .map(|project_path| project_path.path.clone());
            file_status = editor_ref
                .project()
                .zip(real_project_path.as_ref())
                .and_then(|(project, project_path)| {
                    project.read(cx).project_path_git_status(project_path, cx)
                });
            // Set once a directory row is chosen (see `Editor::navigate_breadcrumb_to`); while
            // set, the bar shows that directory's path instead of the file's.
            let navigation = editor_ref.breadcrumb_navigation().cloned();
            let navigated = navigation
                .as_ref()
                .is_some_and(|navigation| navigation.navigated);
            let active_segment = navigation
                .as_ref()
                .map(|navigation| navigation.active_path.clone());

            let is_navigable = breadcrumb_path_is_navigable(
                real_project_path.is_some(),
                real_project_path.as_ref().and_then(|project_path| {
                    editor_ref
                        .project()
                        .and_then(|project| {
                            project
                                .read(cx)
                                .worktree_for_id(project_path.worktree_id, cx)
                        })
                        .map(|worktree| worktree.read(cx).is_single_file())
                }),
            );

            // The root segment is added unconditionally so sibling top-level directories stay
            // reachable from the root, not only from the file's own path. It can't double up with
            // the root `breadcrumbs_inner` already names, because this branch splices that
            // segment away wholesale.
            if is_navigable
                && !segments.is_empty()
                && let Some(project) = editor_ref.project()
            {
                let split = if let Some(navigation) = navigation
                    .as_ref()
                    .filter(|navigation| navigation.navigated)
                {
                    project
                        .read(cx)
                        .worktree_for_id(navigation.worktree_id, cx)
                        .map(|worktree| {
                            breadcrumb_path_segments(
                                navigation.worktree_id,
                                worktree.read(cx).root_name_str(),
                                &navigation.active_path,
                                real_project_path.as_ref().map(|path| path.path.clone()),
                                None,
                                active_segment.as_deref(),
                            )
                        })
                } else if let Some(project_path) = real_project_path.as_ref()
                    && let Some(worktree) = project
                        .read(cx)
                        .worktree_for_id(project_path.worktree_id, cx)
                {
                    Some(breadcrumb_path_segments(
                        project_path.worktree_id,
                        worktree.read(cx).root_name_str(),
                        &project_path.path,
                        Some(project_path.path.clone()),
                        Some(buffer_id),
                        active_segment.as_deref(),
                    ))
                } else {
                    None
                };

                if let Some((path_labels, path_targets)) = split {
                    file_segment_index = path_labels.len() - 1;
                    let replace_range = if navigated { 0..segments.len() } else { 0..1 };
                    segments.splice(replace_range, path_labels);
                    symbol_segments = path_targets;
                    path_split = true;
                    has_root_segment = true;
                }
            }

            if !path_split && is_navigable {
                symbol_segments.push(Some(BreadcrumbSegmentTarget::Symbol {
                    buffer_id,
                    item: None,
                }));
            } else if !path_split {
                symbol_segments.push(None);
            }

            if !navigated {
                let ancestors = editor_ref
                    .outline_symbols_at_cursor
                    .as_ref()
                    .filter(|(id, _)| *id == buffer_id)
                    .map(|(_, ancestors)| ancestors.as_slice())
                    .unwrap_or_default();
                symbol_segments.extend(ancestors.iter().cloned().map(|item| {
                    Some(BreadcrumbSegmentTarget::Symbol {
                        buffer_id,
                        item: Some(item),
                    })
                }));
            }
        }
    }

    let symbol_segments = align_symbol_segments(&segments, symbol_segments);
    let kinds =
        classify_breadcrumb_segment_kinds(segments.len(), file_segment_index, has_root_segment);
    let (segments, symbol_segments, kinds, file_segment_index) =
        hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, file_segment_index);

    let apply_dirty_filename_style =
        !workspace::TabBarSettings::get_global(cx).show && active_item.is_dirty(cx);

    let prepared_segments = segments
        .into_iter()
        .zip(symbol_segments)
        .zip(kinds)
        .enumerate()
        .map(|(index, ((label, target), kind))| {
            let icon = breadcrumb_segment_icon(&target, file_path_for_icon.as_deref(), cx);
            let label_color = if kind == BreadcrumbSegmentKind::File {
                crate::element::file_status_label_color(file_status)
            } else {
                Color::Muted
            };
            PreparedBreadcrumbSegment {
                kind,
                label,
                target,
                dirty_filename_style: apply_dirty_filename_style && index == file_segment_index,
                icon,
                label_color,
            }
        })
        .collect();

    let row = BreadcrumbsRow {
        segments: prepared_segments,
        editor: editor.clone(),
    };

    let breadcrumbs_stack = div()
        .min_w_0()
        .when(multibuffer_header, |this| {
            this.pl_2()
                .border_l_1()
                .border_color(cx.theme().colors().border.opacity(0.6))
        })
        .child(row)
        .into_any_element();

    let breadcrumbs = if let Some(prefix) = prefix {
        h_flex()
            .min_w_0()
            .gap_1p5()
            .child(prefix)
            .child(breadcrumbs_stack)
            .into_any_element()
    } else {
        breadcrumbs_stack
    };

    let has_project_path = active_item.project_path(cx).is_some();

    match editor {
        Some(editor) => element
            .id("breadcrumb_container")
            .when_some(outline_buffer_id, |this, buffer_id| {
                let editor = editor.clone();
                this.on_hover(move |hovered, _, cx| {
                    if *hovered {
                        editor
                            .update(cx, |editor, cx| {
                                editor.prefetch_breadcrumb_outline(buffer_id, cx)
                            })
                            .ok();
                    }
                })
            })
            // A plain row rather than a `ButtonLike`: `ButtonLike` renders `flex_none`, which would
            // stop the bar from ever being told to shrink.
            .child(
                h_flex()
                    .h(rems_from_px(22.))
                    .px_1()
                    .min_w_0()
                    .child(breadcrumbs)
                    .when(!multibuffer_header && has_project_path, |this| {
                        this.on_mouse_down(gpui::MouseButton::Right, {
                            let editor = editor.clone();
                            move |_, _, cx| {
                                if let Some(abs_path) = editor.upgrade().and_then(|editor| {
                                    editor.update(cx, |editor, cx| editor.target_file_abs_path(cx))
                                }) && let Some(path_str) = abs_path.to_str()
                                {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        path_str.to_string(),
                                    ));
                                }
                            }
                        })
                    }),
            )
            .into_any_element(),
        None => element
            .h(rems_from_px(22.)) // Match the height and padding of the `ButtonLike` in the other arm.
            .pl_1()
            .child(breadcrumbs)
            .into_any_element(),
    }
}

/// Byte offset where the file name starts in a path label, shared between painting and measuring
/// so the two can't drift apart.
fn dirty_filename_offset(segment: &HighlightedText) -> Option<usize> {
    let filename = std::path::Path::new(segment.text.as_ref()).file_name()?;
    segment.text.rfind(filename.to_string_lossy().as_ref())
}

fn apply_dirty_filename_style(
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
    cx: &App,
) -> Option<gpui::AnyElement> {
    let text = flatten_text_for_single_line_display(&segment.text);

    let filename_position = dirty_filename_offset(segment)?;

    let bold_weight = FontWeight::BOLD;
    let default_color = Color::Default.color(cx);

    if filename_position == 0 {
        let mut filename_style = text_style.clone();
        filename_style.font_weight = bold_weight;
        filename_style.color = default_color;

        return Some(
            StyledText::new(text)
                .with_default_highlights(&filename_style, [])
                .into_any(),
        );
    }

    let highlight_style = gpui::HighlightStyle {
        font_weight: Some(bold_weight),
        color: Some(default_color),
        ..Default::default()
    };

    let highlight = vec![(filename_position..text.len(), highlight_style)];
    Some(
        StyledText::new(text)
            .with_default_highlights(text_style, highlight)
            .into_any(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_text_for_single_line_display_preserves_byte_offsets() {
        // Byte-offset highlight ranges computed against `original` must stay valid against the
        // flattened result — verify by locating the same substring by offset in both strings.
        let original = "fn outer() {\n    inner()\n}";
        let flattened = flatten_text_for_single_line_display(original);

        assert_eq!(flattened, "fn outer() {     inner() }");
        assert_eq!(flattened.len(), original.len());

        let inner_offset = original.find("inner").unwrap();
        assert_eq!(
            &flattened[inner_offset..inner_offset + "inner".len()],
            "inner",
        );
    }
}
