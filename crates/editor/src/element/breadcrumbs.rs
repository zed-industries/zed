#[cfg(test)]
mod breadcrumbs_tests;
mod layout;
mod menu;
mod outline;
mod path;

use super::*;
use layout::{
    BreadcrumbStrip, PreparedBreadcrumbSegment, align_symbol_segments,
    breadcrumb_path_is_navigable, classify_breadcrumb_segment_kinds, hard_cap_segment_runs,
};
pub(crate) use menu::{BreadcrumbListing, BreadcrumbNavigationMenu};
use path::{breadcrumb_file_icon, breadcrumb_path_segments};

#[derive(Clone, Debug)]
pub(crate) enum BreadcrumbSegmentTarget {
    Symbol {
        buffer_id: BufferId,
        item: Option<OutlineItem<Anchor>>,
    },
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
    },
}

impl BreadcrumbSegmentTarget {
    /// Whether a segment points at the currently open listing. The hard cap centres its window
    /// on this segment and prepaint anchors the menu to it, so both passes call the one method.
    fn matches_listing(&self, listing: &BreadcrumbListing) -> bool {
        match (self, listing) {
            (
                BreadcrumbSegmentTarget::Directory { worktree_id, path },
                BreadcrumbListing::Directory {
                    worktree_id: listing_worktree,
                    path: listing_path,
                },
            ) => worktree_id == listing_worktree && path.as_ref() == listing_path.as_ref(),
            (
                BreadcrumbSegmentTarget::Symbol { buffer_id, item },
                BreadcrumbListing::Symbols {
                    buffer_id: listing_buffer,
                    parent,
                },
            ) => {
                buffer_id == listing_buffer
                    && match (item, parent) {
                        (None, None) => true,
                        (Some(item), Some(parent)) => outline::same_symbol_item(item, parent),
                        _ => false,
                    }
            }
            _ => false,
        }
    }
}

pub fn render_breadcrumb_text(
    segments: Vec<HighlightedText>,
    breadcrumb_font: Option<Font>,
    prefix: Option<gpui::AnyElement>,
    active_item: &dyn ItemHandle,
    multibuffer_header: bool,
    cx: &App,
) -> gpui::AnyElement {
    let element = h_flex().flex_grow_1().min_w_0().text_ui(cx);

    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    let row = prepare_breadcrumb_strip(
        segments,
        breadcrumb_font,
        prefix.is_some(),
        active_item,
        multibuffer_header,
        cx,
    );

    let breadcrumbs_stack = div()
        .min_w_0()
        .flex_1()
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
            .flex_1()
            .gap_1p5()
            .child(prefix)
            .child(breadcrumbs_stack)
            .into_any_element()
    } else {
        breadcrumbs_stack
    };

    match editor {
        Some(_editor) => element
            .id("breadcrumb_container")
            .min_w_0()
            .h(rems_from_px(22_f32))
            .px(DynamicSpacing::Base04.rems(cx))
            .when(!multibuffer_header, |this| this.overflow_hidden())
            .child(breadcrumbs)
            .into_any_element(),
        None => element
            .h(rems_from_px(22_f32))
            .pl_1()
            .child(breadcrumbs)
            .into_any_element(),
    }
}

/// Split out of the render so a test can hold what the bar is about to paint: what each segment
/// names, what it opens, and which kind - and so which colour - it is painted as.
fn prepare_breadcrumb_strip(
    mut segments: Vec<HighlightedText>,
    breadcrumb_font: Option<Font>,
    has_prefix: bool,
    active_item: &dyn ItemHandle,
    multibuffer_header: bool,
    cx: &App,
) -> BreadcrumbStrip {
    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    let mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = Vec::new();
    let mut menu_listing: Option<BreadcrumbListing> = None;
    let mut file_segment_index = 0usize;
    // Only a singleton editor has one. A terminal, an image, a diff view or a multibuffer hands
    // the bar a name and symbols with no file among them, and classifying their first segment as
    // the file would colour it like one.
    let mut has_file_segment = false;
    let mut has_root_segment = false;
    let mut file_icon: Option<SharedString> = None;
    let mut file_git_status_color: Option<Color> = None;

    if !multibuffer_header
        && let Some(editor_entity) = editor.as_ref().and_then(WeakEntity::upgrade)
    {
        let editor_ref = editor_entity.read(cx);
        if let Some(buffer) = editor_ref.buffer().read(cx).as_singleton() {
            let buffer_id = buffer.read(cx).remote_id();
            let mut path_split = false;
            has_file_segment = true;

            let real_project_path = active_item.project_path(cx);
            // Only the file segment paints it, and only when no prefix already occupies that
            // slot. Resolving it otherwise costs an icon lookup on every repaint.
            if !has_prefix {
                file_icon = breadcrumb_file_icon(
                    real_project_path
                        .as_ref()
                        .map(|project_path| project_path.path.as_ref()),
                    cx,
                );
            }
            file_git_status_color = breadcrumb_file_git_status_color(
                || {
                    editor_ref
                        .project()
                        .zip(real_project_path.as_ref())
                        .and_then(|(project, project_path)| {
                            project.read(cx).project_path_git_status(project_path, cx)
                        })
                },
                cx,
            );
            let menu = editor_ref.breadcrumb_navigation_menu().cloned();
            let navigated_path = menu
                .as_ref()
                .and_then(|menu| menu.read(cx).navigated_path());
            // Browsing an ancestor of the open file keeps the file's whole trail on the bar: that
            // directory is one of the segments already painted, and the menu anchors to it. Only
            // a listing off the file's path replaces the trail with where the user has gone.
            let navigated_off_path = navigated_path.as_ref().is_some_and(|(worktree_id, path)| {
                !real_project_path.as_ref().is_some_and(|project_path| {
                    project_path.worktree_id == *worktree_id && project_path.path.starts_with(path)
                })
            });
            let menu_symbol_trail = menu
                .as_ref()
                .map(|menu| menu.read(cx).symbol_trail().to_vec());
            menu_listing = menu.as_ref().map(|menu| menu.read(cx).listing().clone());

            // Shared by the navigability test and the path split below.
            let real_worktree = real_project_path.as_ref().and_then(|project_path| {
                editor_ref
                    .project()?
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
            });
            let is_navigable = breadcrumb_path_is_navigable(
                real_project_path.is_some(),
                real_worktree
                    .as_ref()
                    .map(|worktree| worktree.read(cx).is_single_file()),
            );

            if is_navigable
                && !segments.is_empty()
                && let Some(project) = editor_ref.project()
            {
                let split = if navigated_off_path
                    && let Some((worktree_id, path)) = navigated_path.as_ref()
                {
                    project
                        .read(cx)
                        .worktree_for_id(*worktree_id, cx)
                        .map(|worktree| {
                            breadcrumb_path_segments(
                                *worktree_id,
                                worktree.read(cx).root_name_str(),
                                path,
                                None,
                            )
                        })
                } else if let Some(project_path) = real_project_path.as_ref()
                    && let Some(worktree) = real_worktree.as_ref()
                {
                    Some(breadcrumb_path_segments(
                        project_path.worktree_id,
                        worktree.read(cx).root_name_str(),
                        &project_path.path,
                        Some(buffer_id),
                    ))
                } else {
                    None
                };

                if let Some((path_labels, path_targets)) = split {
                    file_segment_index = path_labels.len() - 1;
                    let replace_range = if navigated_off_path {
                        0..segments.len()
                    } else {
                        0..1
                    };
                    segments.splice(replace_range, path_labels);
                    symbol_segments = path_targets;
                    path_split = true;
                    has_root_segment = true;
                }
            }

            if !path_split {
                symbol_segments.push(Some(BreadcrumbSegmentTarget::Symbol {
                    buffer_id,
                    item: None,
                }));
            }

            if !navigated_off_path {
                let cursor_chain = editor_ref
                    .outline_symbols_at_cursor
                    .as_ref()
                    .filter(|(id, _)| *id == buffer_id)
                    .map(|(_, ancestors)| ancestors.clone())
                    .unwrap_or_default();
                let trail = outline::resolve_bar_symbol_trail(cursor_chain, menu_symbol_trail);
                // Labels and targets must change together or alignment blanks every target.
                let trail_labels: Vec<HighlightedText> = trail
                    .iter()
                    .map(|item| HighlightedText {
                        text: item.text.clone(),
                        highlights: item.highlight_ranges.clone(),
                    })
                    .collect();
                segments.splice(file_segment_index + 1.., trail_labels);
                symbol_segments.extend(trail.into_iter().map(|item| {
                    Some(BreadcrumbSegmentTarget::Symbol {
                        buffer_id,
                        item: Some(item),
                    })
                }));
            }
        }
    }

    let symbol_segments = align_symbol_segments(&segments, symbol_segments);
    // A multibuffer header lists symbols only; without this the first symbol would be
    // classified - and coloured - as the file segment.
    let kinds = classify_breadcrumb_segment_kinds(
        segments.len(),
        (!multibuffer_header && has_file_segment).then_some(file_segment_index),
        has_root_segment,
    );
    let protected_index = menu_listing.as_ref().and_then(|listing| {
        symbol_segments.iter().position(|segment_target| {
            segment_target
                .as_ref()
                .is_some_and(|segment_target| segment_target.matches_listing(listing))
        })
    });
    let (segments, symbol_segments, kinds, file_segment_index) = hard_cap_segment_runs(
        segments,
        symbol_segments,
        kinds,
        file_segment_index,
        protected_index,
    );

    let apply_dirty_filename_style =
        !workspace::TabBarSettings::get_global(cx).show && active_item.is_dirty(cx);
    let show_file_segment_icon = !has_prefix;

    let prepared_segments = segments
        .into_iter()
        .zip(symbol_segments)
        .zip(kinds)
        .enumerate()
        .map(|(index, ((label, target), kind))| {
            let is_file_segment = index == file_segment_index
                && matches!(
                    target,
                    Some(BreadcrumbSegmentTarget::Symbol { item: None, .. })
                );
            // The file segment carries the unsaved-changes style, and never a directory the
            // menu has browsed to. An item with no file segment keeps it on the first one,
            // which is the name it puts there and where the bar styled it before.
            let dirty_filename_style = apply_dirty_filename_style
                && if has_file_segment {
                    is_file_segment
                } else {
                    index == 0
                };
            PreparedBreadcrumbSegment {
                kind,
                label,
                target,
                dirty_filename_style,
                icon: (is_file_segment && show_file_segment_icon)
                    .then(|| file_icon.clone())
                    .flatten(),
                git_status_color: is_file_segment.then_some(file_git_status_color).flatten(),
            }
        })
        .collect();

    BreadcrumbStrip {
        segments: prepared_segments,
        editor,
        breadcrumb_font,
    }
}

/// Gated on the tab family, like the file the bar describes; the menu rows are a directory
/// listing and follow the panel family instead.
///
/// Takes the status lazily: resolving one walks every repository, so it must not run when the
/// setting is off.
pub(super) fn breadcrumb_file_git_status_color(
    status: impl FnOnce() -> Option<git::status::FileStatus>,
    cx: &App,
) -> Option<Color> {
    if !workspace::ItemSettings::get_global(cx).git_status {
        return None;
    }
    status().map(|status| file_status_label_color(Some(status)))
}
