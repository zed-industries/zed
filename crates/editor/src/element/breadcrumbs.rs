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
    mut segments: Vec<HighlightedText>,
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

    let mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = Vec::new();
    let mut menu_listing: Option<BreadcrumbListing> = None;
    let mut file_segment_index = 0usize;
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

            let real_project_path = active_item.project_path(cx);
            // Only the file segment paints it, and only when no prefix already occupies that
            // slot. Resolving it otherwise costs an icon lookup on every repaint.
            if prefix.is_none() {
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
        (!multibuffer_header).then_some(file_segment_index),
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
    let show_file_segment_icon = prefix.is_none();

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
            PreparedBreadcrumbSegment {
                kind,
                label,
                target,
                dirty_filename_style: apply_dirty_filename_style && is_file_segment,
                icon: (is_file_segment && show_file_segment_icon)
                    .then(|| file_icon.clone())
                    .flatten(),
                git_status_color: is_file_segment.then_some(file_git_status_color).flatten(),
            }
        })
        .collect();

    let row = BreadcrumbStrip {
        segments: prepared_segments,
        editor: editor.clone(),
        breadcrumb_font,
    };

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

#[cfg(test)]
mod tests {
    use super::layout::{
        BreadcrumbSegmentKind, align_symbol_segments, breadcrumb_path_is_navigable,
        classify_breadcrumb_segment_kinds, hard_cap_segment_runs, plan_breadcrumb_layout,
    };
    use super::menu::{BreadcrumbListing, BreadcrumbNavigationMenu};
    use super::outline::{
        child_outline_indices, flatten_text_for_single_line_display,
        highlights_with_match_emphasis, sibling_outline_indices, top_level_outline_indices,
    };
    use super::path::{
        DirectoryEntryIconSource, MAX_BREADCRUMB_MENU_ROWS, MAX_UNARY_DIRECTORY_SKIP_DEPTH,
        breadcrumb_directory_entries, breadcrumb_directory_listing_inputs,
        breadcrumb_path_prefixes, directory_entry_icon_source, single_child_directory,
    };
    use super::*;
    use crate::MultiBuffer;
    use ::menu::{Confirm, SelectChild, SelectNext, SelectParent};
    use gpui::{KeyBinding, TestAppContext, VisualTestContext, actions};
    use std::rc::Rc;

    actions!(breadcrumb_nav_test, [ProbeAltB]);

    fn choose_directory_entry_via_keystroke(
        menu: &Entity<BreadcrumbNavigationMenu>,
        entry_name: &str,
        cx: &mut VisualTestContext,
    ) {
        cx.update(|_window, cx| {
            cx.bind_keys([KeyBinding::new(
                "enter",
                Confirm,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });
        let menu_for_draw = menu.clone();
        cx.draw(
            gpui::point(gpui::px(0.), gpui::px(0.)),
            gpui::size(gpui::px(640.), gpui::px(480.)),
            move |_, _| menu_for_draw.into_any_element(),
        );
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.simulate_input(entry_name);
        cx.run_until_parked();
        let menu_for_draw = menu.clone();
        cx.draw(
            gpui::point(gpui::px(0.), gpui::px(0.)),
            gpui::size(gpui::px(640.), gpui::px(480.)),
            move |_, _| menu_for_draw.into_any_element(),
        );
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();
    }

    #[test]
    fn test_breadcrumb_path_is_navigable() {
        assert!(!breadcrumb_path_is_navigable(false, None));
        assert!(!breadcrumb_path_is_navigable(false, Some(false)));
        assert!(!breadcrumb_path_is_navigable(true, Some(true)));
        assert!(breadcrumb_path_is_navigable(true, Some(false)));
        assert!(breadcrumb_path_is_navigable(true, None));
    }

    #[test]
    fn test_highlights_with_match_emphasis_keeps_syntax_colours() {
        use gpui::{FontWeight, HighlightStyle, Hsla};

        let syntax_colour = Hsla {
            h: 0.5,
            s: 0.5,
            l: 0.5,
            a: 1.,
        };
        let accent = Hsla {
            h: 0.1,
            s: 1.,
            l: 0.5,
            a: 1.,
        };
        let emphasis = HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            color: Some(accent),
            ..Default::default()
        };
        let coloured = HighlightStyle {
            color: Some(syntax_colour),
            ..Default::default()
        };
        let bold_coloured = HighlightStyle {
            color: Some(syntax_colour),
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        };
        let bold_accent = HighlightStyle {
            color: Some(accent),
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        };

        // "impl Widget" with the name syntax-coloured, matching "Wi".
        let text = "impl Widget";
        let syntax = vec![(5..11, coloured)];

        assert_eq!(
            highlights_with_match_emphasis(text, &syntax, &[], emphasis),
            syntax,
            "no matches leaves the syntax highlights untouched"
        );

        assert_eq!(
            highlights_with_match_emphasis(text, &syntax, &[5, 6], emphasis),
            vec![
                (0..5, HighlightStyle::default()),
                (5..7, bold_coloured),
                (7..11, coloured),
            ],
            "a match inside a coloured run keeps the colour and adds weight"
        );

        assert_eq!(
            highlights_with_match_emphasis(text, &syntax, &[0], emphasis),
            vec![
                (0..1, bold_accent),
                (1..5, HighlightStyle::default()),
                (5..11, coloured),
            ],
            "a match outside any coloured run takes the emphasis colour"
        );

        // 'ä' is two bytes: the merged ranges must stay on char boundaries.
        let multibyte = "fn äb";
        let merged = highlights_with_match_emphasis(multibyte, &[], &[3], emphasis);
        assert_eq!(
            merged,
            vec![
                (0..3, HighlightStyle::default()),
                (3..5, bold_accent),
                (5..6, HighlightStyle::default()),
            ]
        );
        for (range, _) in &merged {
            assert!(
                multibyte.is_char_boundary(range.start) && multibyte.is_char_boundary(range.end)
            );
        }
    }

    #[test]
    fn test_flatten_text_for_single_line_display_preserves_byte_offsets() {
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

    #[test]
    fn test_breadcrumb_entry_icon_source() {
        assert_eq!(
            directory_entry_icon_source(true, true, true),
            DirectoryEntryIconSource::Folder
        );
        assert_eq!(
            directory_entry_icon_source(true, false, true),
            DirectoryEntryIconSource::Folder
        );
        assert_eq!(
            directory_entry_icon_source(true, true, false),
            DirectoryEntryIconSource::Chevron
        );
        assert_eq!(
            directory_entry_icon_source(true, false, false),
            DirectoryEntryIconSource::Chevron
        );
        assert_eq!(
            directory_entry_icon_source(false, true, true),
            DirectoryEntryIconSource::File
        );
        assert_eq!(
            directory_entry_icon_source(false, true, false),
            DirectoryEntryIconSource::File
        );
        assert_eq!(
            directory_entry_icon_source(false, false, true),
            DirectoryEntryIconSource::None
        );
        assert_eq!(
            directory_entry_icon_source(false, false, false),
            DirectoryEntryIconSource::None
        );
    }

    #[test]
    fn test_sibling_outline_indices_top_level() {
        let depths = [0, 0, 0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 1), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![0, 1, 2]);
    }

    #[test]
    fn test_sibling_outline_indices_nested() {
        // impl A {         // 0
        //     fn one() {}  // 1
        //     fn two() {}  // 1
        // }
        // impl B {         // 0
        //     fn three() {}// 1
        // }
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 4), vec![4]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
        assert_eq!(sibling_outline_indices(&depths, 3), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_uneven_depths() {
        let depths = [0, 2, 2, 0];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_single_item() {
        let depths = [0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0]);
    }

    #[test]
    fn test_sibling_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(sibling_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_top_level() {
        let depths = [0, 0, 0];
        assert_eq!(child_outline_indices(&depths, 0), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_nested() {
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), vec![4]);
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 4), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_uneven_depths() {
        let depths = [0, 2, 2, 0];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(child_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_top_level_outline_indices() {
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(top_level_outline_indices(&depths), vec![0, 3]);

        let depths_uneven = [0, 2, 2, 0];
        assert_eq!(top_level_outline_indices(&depths_uneven), vec![0, 3]);

        let depths_empty: [usize; 0] = [];
        assert_eq!(
            top_level_outline_indices(&depths_empty),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_nested() {
        use util::rel_path::rel_path;

        assert_eq!(
            breadcrumb_path_prefixes(rel_path("a/b/c.rs")),
            vec![rel_path("a"), rel_path("a/b"), rel_path("a/b/c.rs")]
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_top_level_file() {
        use util::rel_path::rel_path;

        assert_eq!(
            breadcrumb_path_prefixes(rel_path("file.rs")),
            vec![rel_path("file.rs")]
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_empty() {
        assert_eq!(
            breadcrumb_path_prefixes(RelPath::empty()),
            Vec::<&RelPath>::new()
        );
    }

    #[test]
    fn test_breadcrumb_path_segments_nested() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let buffer_id = BufferId::new(1).unwrap();
        let path = rel_path("src/main/kotlin/Foo.kt").into_arc();

        let (labels, targets) =
            breadcrumb_path_segments(worktree_id, "my-project", &path, Some(buffer_id));

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "src", "main", "kotlin", "Foo.kt"]
        );
        assert_eq!(targets.len(), labels.len());

        match targets[0].as_ref().unwrap() {
            BreadcrumbSegmentTarget::Directory {
                worktree_id: id,
                path,
            } => {
                assert_eq!(*id, worktree_id);
                assert_eq!(path.as_unix_str(), "");
            }
            other => panic!("expected root directory target, got {other:?}"),
        }

        for (index, expected_dir) in ["src", "src/main", "src/main/kotlin"]
            .into_iter()
            .enumerate()
        {
            match targets[index + 1].as_ref().unwrap() {
                BreadcrumbSegmentTarget::Directory { path, .. } => {
                    assert_eq!(path.as_unix_str(), expected_dir);
                }
                other => panic!("expected directory target, got {other:?}"),
            }
        }

        match targets.last().unwrap().as_ref().unwrap() {
            BreadcrumbSegmentTarget::Symbol {
                buffer_id: id,
                item,
            } => {
                assert_eq!(*id, buffer_id);
                assert!(item.is_none());
            }
            other => panic!("expected symbol target for the file segment, got {other:?}"),
        }
    }

    #[test]
    fn test_breadcrumb_path_segments_top_level_file() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let buffer_id = BufferId::new(1).unwrap();
        let path = rel_path("Foo.kt").into_arc();

        let (labels, targets) =
            breadcrumb_path_segments(worktree_id, "my-project", &path, Some(buffer_id));

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "Foo.kt"]
        );
        match targets[0].as_ref().unwrap() {
            BreadcrumbSegmentTarget::Directory { .. } => {}
            other => panic!("root target should be Directory, got {other:?}"),
        }
        match targets[1].as_ref().unwrap() {
            BreadcrumbSegmentTarget::Symbol { item: None, .. } => {}
            other => panic!("file target should be Symbol(None), got {other:?}"),
        }
    }

    #[test]
    fn test_breadcrumb_path_segments_navigated_directory_has_no_symbol_terminal() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let path = rel_path("src/main").into_arc();

        let (labels, targets) = breadcrumb_path_segments(worktree_id, "ihavenever", &path, None);

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["ihavenever", "src", "main"]
        );

        assert!(
            targets.iter().all(|target| {
                matches!(target, Some(BreadcrumbSegmentTarget::Directory { .. }))
            })
        );
    }

    #[test]
    fn test_breadcrumb_path_segments_includes_root_and_lists_own_children() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let path = rel_path("src/main/Foo.kt").into_arc();

        let (labels, targets) = breadcrumb_path_segments(worktree_id, "my-project", &path, None);

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "src", "main", "Foo.kt"]
        );

        let list_paths: Vec<String> = targets
            .iter()
            .map(|target| match target.as_ref().unwrap() {
                BreadcrumbSegmentTarget::Directory { path, .. } => path.as_unix_str().to_string(),
                BreadcrumbSegmentTarget::Symbol { .. } => "<symbol>".to_string(),
            })
            .collect();
        assert_eq!(list_paths, vec!["", "src", "src/main", "src/main/Foo.kt"]);
    }

    #[test]
    fn test_single_child_directory_only_when_exactly_one_dir() {
        use util::rel_path::rel_path;

        assert_eq!(
            single_child_directory(&[(rel_path("a/b").into_arc(), true)]),
            Some(rel_path("a/b").into_arc())
        );
        assert_eq!(
            single_child_directory(&[(rel_path("a/f").into_arc(), false)]),
            None
        );
        assert_eq!(
            single_child_directory(&[
                (rel_path("a/b").into_arc(), true),
                (rel_path("a/c").into_arc(), true),
            ]),
            None
        );
        assert_eq!(single_child_directory(&[]), None);
    }

    fn outline_item_at(
        row: u32,
        cx: &mut TestAppContext,
        buffer: &Entity<MultiBuffer>,
    ) -> OutlineItem<Anchor> {
        buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            let point = language::Point::new(row, 0);
            let anchor = snapshot.anchor_before(point);
            OutlineItem {
                depth: 0,
                range: anchor..snapshot.anchor_after(language::Point::new(row, 5)),
                selection_range: anchor..anchor,
                source_range_for_text: anchor..anchor,
                text: format!("item-{row}").into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            }
        })
    }

    #[gpui::test]
    fn test_resolve_bar_symbol_trail(cx: &mut TestAppContext) {
        use super::outline::resolve_bar_symbol_trail;
        crate::editor_tests::init_test(cx, |_| {});

        let buffer = cx.new(|cx| language::Buffer::local("line0\nline1\nline2\nline3\n", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let outer = outline_item_at(0, cx, &buffer);
        let child = outline_item_at(1, cx, &buffer);
        let peer = outline_item_at(2, cx, &buffer);

        let texts = |items: &[OutlineItem<Anchor>]| {
            items
                .iter()
                .map(|item| item.text.to_string())
                .collect::<Vec<_>>()
        };

        let resolved = resolve_bar_symbol_trail(vec![outer.clone(), child.clone()], None);
        assert_eq!(texts(&resolved), vec!["item-0", "item-1"]);

        let resolved = resolve_bar_symbol_trail(
            vec![outer.clone(), child.clone()],
            Some(vec![outer.clone()]),
        );
        assert_eq!(texts(&resolved), vec!["item-0", "item-1"]);

        let resolved =
            resolve_bar_symbol_trail(vec![outer.clone(), child], Some(vec![peer.clone()]));
        assert_eq!(texts(&resolved), vec!["item-2"]);
        let resolved = resolve_bar_symbol_trail(vec![outer], Some(vec![peer]));
        assert_eq!(texts(&resolved), vec!["item-2"]);
    }

    #[gpui::test]
    async fn test_segment_toggle_and_switch_listing(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            serde_json::json!({
                "src": { "a.txt": "", "b.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let (editor, editor_cx) = {
            let buffer = cx.new(|cx| language::Buffer::local("", cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
            let editor = window.root(cx).unwrap();
            (editor, VisualTestContext::from_window(*window, cx))
        };
        let cx = &mut { editor_cx };
        editor.update(cx, |editor, _cx| {
            editor.set_workspace_for_test(workspace.downgrade());
        });

        let root_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: RelPath::empty().into(),
        };
        let src_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: rel_path("src").into_arc(),
        };

        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(root_listing.clone(), window, cx);
            assert!(editor.breadcrumb_navigation_menu().is_some());
            editor.open_or_toggle_breadcrumb_listing_for_test(root_listing.clone(), window, cx);
            assert!(
                editor.breadcrumb_navigation_menu().is_none(),
                "same-listing re-click toggles closed"
            );
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(root_listing.clone(), window, cx);
            assert!(editor.breadcrumb_navigation_menu().is_some());
            let menu = editor.breadcrumb_navigation_menu().unwrap().clone();
            menu.update(cx, |_, cx| cx.emit(gpui::DismissEvent));
        });
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.breadcrumb_navigation_menu().is_none());
            editor.open_or_toggle_breadcrumb_listing_for_test(root_listing.clone(), window, cx);
            assert!(
                editor.breadcrumb_navigation_menu().is_some(),
                "after Escape-style dismiss, re-click opens immediately"
            );
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            let menu_before = editor.breadcrumb_navigation_menu().unwrap().entity_id();
            editor.open_or_toggle_breadcrumb_listing_for_test(src_listing.clone(), window, cx);
            let menu = editor
                .breadcrumb_navigation_menu()
                .expect("menu stays open when switching listings");
            assert_eq!(menu.entity_id(), menu_before, "same menu entity");
            let listing = menu.read(cx).listing().clone();
            assert_eq!(
                listing, src_listing,
                "listing switches to the other segment"
            );
        });
    }

    #[gpui::test]
    async fn test_switch_listing_under_editor_update_does_not_panic(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            serde_json::json!({
                "src": { "a.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let (editor, editor_cx) = {
            let buffer = cx.new(|cx| language::Buffer::local("", cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
            let editor = window.root(cx).unwrap();
            (editor, VisualTestContext::from_window(*window, cx))
        };
        let cx = &mut { editor_cx };
        editor.update(cx, |editor, _cx| {
            editor.set_workspace_for_test(workspace.downgrade());
        });

        let root_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: RelPath::empty().into(),
        };
        let src_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: rel_path("src").into_arc(),
        };

        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(root_listing, window, cx);
        });
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(src_listing.clone(), window, cx);
            let menu = editor
                .breadcrumb_navigation_menu()
                .expect("menu remains after switch");
            assert_eq!(menu.read(cx).listing(), &src_listing);
        });
    }

    #[gpui::test]
    async fn test_toggle_breadcrumbs_off_dismisses_open_menu(cx: &mut TestAppContext) {
        use crate::ToggleBreadcrumb;
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), serde_json::json!({ "a.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let (editor, editor_cx) = {
            let buffer = cx.new(|cx| language::Buffer::local("", cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
            let editor = window.root(cx).unwrap();
            (editor, VisualTestContext::from_window(*window, cx))
        };
        let cx = &mut { editor_cx };
        editor.update(cx, |editor, _cx| {
            editor.set_workspace_for_test(workspace.downgrade());
        });

        let listing = BreadcrumbListing::Directory {
            worktree_id,
            path: RelPath::empty().into(),
        };
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(listing, window, cx);
            assert!(editor.breadcrumb_navigation_menu().is_some());
            assert!(editor.breadcrumbs_visible());
            editor.toggle_breadcrumb(&ToggleBreadcrumb, window, cx);
            assert!(!editor.breadcrumbs_visible());
            assert!(
                editor.breadcrumb_navigation_menu().is_none(),
                "menu entity cleared when breadcrumbs are toggled off"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_dismisses_on_click_outside(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::{point, px};
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint with a debug id");
        let outside = point(
            menu_bounds.right() + px(40.),
            menu_bounds.bottom() + px(40.),
        );
        cx.simulate_click(outside, gpui::Modifiers::none());
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "left-click outside the menu must dismiss"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_survives_release_on_an_occluded_part_of_itself(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::{MouseButton, point, px};
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                // Stands in for the picker's scrollbar thumb: a hitbox painted over part of
                // the popup, which the real thumb is and which a test cannot get at because
                // the thumb autohides. What matters is only that it blocks hit testing inside
                // the popup's own rectangle.
                div()
                    .size_full()
                    .child(self.menu.clone())
                    .child(div().absolute().top_0().left_0().size(px(24.)).occlude())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let occluded = point(px(12.), px(12.));
        assert!(
            menu_bounds.contains(&occluded),
            "the overlay has to sit inside the popup for this to test anything, popup at \
             {menu_bounds:?}"
        );
        let outside = point(
            menu_bounds.right() + px(40.),
            menu_bounds.bottom() + px(40.),
        );

        cx.simulate_mouse_down(outside, MouseButton::Left, gpui::Modifiers::none());
        cx.simulate_mouse_up(occluded, MouseButton::Left, gpui::Modifiers::none());
        cx.run_until_parked();

        assert!(
            !dismissed.load(Ordering::SeqCst),
            "a release inside the popup must not close it just because something is painted \
             over that spot"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_survives_drag_release_outside(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::{MouseButton, point, px};
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let mut files = serde_json::Map::new();
        for index in 0..MAX_BREADCRUMB_MENU_ROWS {
            files.insert(format!("entry_{index:03}.txt"), json!(""));
        }
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!(files)).await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let outside = point(menu_bounds.right() + px(40.), menu_bounds.top() + px(120.));

        // A press outside whose release lands back inside must not leave the menu primed
        // to dismiss on the next release.
        cx.simulate_mouse_down(outside, MouseButton::Left, gpui::Modifiers::none());
        cx.simulate_mouse_up(
            menu_bounds.center(),
            MouseButton::Left,
            gpui::Modifiers::none(),
        );
        cx.run_until_parked();

        // Stands in for the scrollbar thumb, which autohides and never becomes hit-testable
        // from a test - verified by sweeping the popup's full width and height. Arming is
        // geometric, so any point inside the popup takes the same path the thumb would.
        let inside = menu_bounds.center();
        cx.simulate_mouse_down(inside, MouseButton::Left, gpui::Modifiers::none());
        cx.simulate_mouse_move(outside, MouseButton::Left, gpui::Modifiers::none());
        cx.run_until_parked();
        cx.simulate_mouse_up(outside, MouseButton::Left, gpui::Modifiers::none());
        cx.run_until_parked();

        assert!(
            !dismissed.load(Ordering::SeqCst),
            "a drag that starts inside the menu must not dismiss it on an outside release"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_navigation_button_press_does_not_prime_dismissal(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::{MouseButton, NavigationDirection, point, px};
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let outside = point(
            menu_bounds.right() + px(40.),
            menu_bounds.bottom() + px(40.),
        );

        // A button the menu never dismisses on must not prime the dismissal either.
        let back = MouseButton::Navigate(NavigationDirection::Back);
        cx.simulate_mouse_down(outside, back, gpui::Modifiers::none());
        cx.simulate_mouse_up(outside, back, gpui::Modifiers::none());
        cx.run_until_parked();

        cx.simulate_mouse_down(
            menu_bounds.center(),
            MouseButton::Left,
            gpui::Modifiers::none(),
        );
        cx.simulate_mouse_up(outside, MouseButton::Left, gpui::Modifiers::none());
        cx.run_until_parked();

        assert!(
            !dismissed.load(Ordering::SeqCst),
            "a drag out of the menu must not dismiss it after an unrelated outside press"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_right_click_outside_dismisses(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::{MouseButton, point, px};
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let outside = point(
            menu_bounds.right() + px(40.),
            menu_bounds.bottom() + px(40.),
        );
        cx.simulate_mouse_down(outside, MouseButton::Right, gpui::Modifiers::none());
        cx.simulate_mouse_up(outside, MouseButton::Right, gpui::Modifiers::none());
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "right-click outside the menu must dismiss"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_dismissed_when_no_segment_anchors(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            strip: Option<BreadcrumbStrip>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().children(self.strip.take())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| {
                crate::test::build_editor_with_project(project.clone(), multi_buffer, window, cx)
            });
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                strip: None,
            }
        });
        let editor = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.editor.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);

        // Open a listing that is not present as a bar segment target.
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("missing/dir").into_arc(),
                },
                window,
                cx,
            );
            assert!(editor.breadcrumb_navigation_menu().is_some());
        });

        host_window
            .update(cx, |host, _window, cx| {
                host.strip = Some(BreadcrumbStrip {
                    segments: vec![PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::File,
                        label: HighlightedText {
                            text: "main.rs".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: rel_path("src").into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    }],
                    editor: Some(host.editor.downgrade()),
                    breadcrumb_font: None,
                });
                cx.notify();
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();

        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_none(),
                "menu with no matching bar segment must be deferred-dismissed from prepaint"
            );
        });
    }

    // The negative case above is the only one the suite had: `protected_segment_index` returning
    // `Some` went unexercised, so a predicate that quietly stopped matching would land green.
    #[gpui::test]
    async fn test_breadcrumb_menu_survives_when_a_segment_anchors_it(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            strip: Option<BreadcrumbStrip>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().children(self.strip.take())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| {
                crate::test::build_editor_with_project(project.clone(), multi_buffer, window, cx)
            });
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                strip: None,
            }
        });
        let editor = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.editor.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);

        // The listing matches the bar segment installed below, so prepaint must find an anchor.
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("src").into_arc(),
                },
                window,
                cx,
            );
            assert!(editor.breadcrumb_navigation_menu().is_some());
        });

        host_window
            .update(cx, |host, _window, cx| {
                host.strip = Some(BreadcrumbStrip {
                    segments: vec![PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::File,
                        label: HighlightedText {
                            text: "main.rs".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: rel_path("src").into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    }],
                    editor: Some(host.editor.downgrade()),
                    breadcrumb_font: None,
                });
                cx.notify();
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.run_until_parked();

        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_some(),
                "a segment whose target matches the open listing must anchor the menu; a \
                 predicate that stops matching dismisses it from prepaint instead"
            );
        });
    }

    #[gpui::test]
    fn test_breadcrumb_file_git_status_color_respects_tabs_git_status(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use git::status::{FileStatus, StatusCode};
        use gpui::UpdateGlobal;
        use settings::SettingsStore;

        init_test(cx, |_| {});

        let status = FileStatus::worktree(StatusCode::Modified);

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.tabs.get_or_insert_default().git_status = Some(true);
                });
            });
        });
        assert!(
            cx.update(|cx| workspace::ItemSettings::get_global(cx).git_status),
            "tabs.git_status=true should enable ItemSettings.git_status"
        );
        let color_on = cx.update(|cx| breadcrumb_file_git_status_color(|| Some(status), cx));
        assert_eq!(
            color_on,
            Some(file_status_label_color(Some(status))),
            "enabled path mirrors file_status_label_color"
        );

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.tabs.get_or_insert_default().git_status = Some(false);
                });
            });
        });
        let color_off = cx.update(|cx| breadcrumb_file_git_status_color(|| Some(status), cx));
        assert_eq!(
            color_off, None,
            "tabs.git_status=false must suppress file segment git color"
        );
    }

    #[gpui::test]
    async fn test_directory_drill_caps_unary_chain_depth(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        const CHAIN_DEPTH: usize = MAX_UNARY_DIRECTORY_SKIP_DEPTH + 6;
        let mut leaf = serde_json::json!({ "leaf.txt": "" });
        for _ in 0..CHAIN_DEPTH {
            leaf = serde_json::json!({ "d": leaf });
        }

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), serde_json::json!({ "d": leaf }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        cx.run_until_parked();
        choose_directory_entry_via_keystroke(&menu, "d", cx);

        let (_, landed) = menu
            .read_with(cx, |menu, _| menu.navigated_path())
            .expect("drill lands on a navigated directory");
        assert_eq!(
            landed.components().count(),
            1 + MAX_UNARY_DIRECTORY_SKIP_DEPTH
        );
    }

    #[test]
    fn test_align_symbol_segments_realigns_divergent_lengths() {
        let segments: Vec<HighlightedText> = (0..3)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];

        let symbol_segments = align_symbol_segments(&segments, symbol_segments);

        assert_eq!(symbol_segments.len(), 3);
        assert!(symbol_segments.iter().all(Option::is_none));
    }

    #[test]
    fn test_classify_breadcrumb_segment_kinds() {
        let kinds = classify_breadcrumb_segment_kinds(6, Some(3), true);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Root,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        let kinds = classify_breadcrumb_segment_kinds(3, Some(1), false);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        // Multibuffer headers list symbols with no file segment among them.
        let kinds = classify_breadcrumb_segment_kinds(3, None, false);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
            ]
        );

        let kinds = classify_breadcrumb_segment_kinds(1, Some(0), false);
        assert_eq!(kinds, vec![BreadcrumbSegmentKind::File]);
    }

    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_does_not_panic_on_divergent_symbol_segments() {
        let segments: Vec<HighlightedText> = (0..100)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];
        let symbol_segments = align_symbol_segments(&segments, symbol_segments);
        assert_eq!(symbol_segments.len(), segments.len());

        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(99), true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 99, None);

        assert_eq!(segments.len(), 66);
        {
            let segments: Vec<HighlightedText> = (0..100)
                .map(|i| HighlightedText {
                    text: format!("segment-{i}").into(),
                    highlights: vec![],
                })
                .collect();
            let symbol_segments = vec![None; segments.len()];
            let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(99), true);
            let (capped, _, _, _) =
                hard_cap_segment_runs(segments, symbol_segments, kinds, 99, Some(50));
            let labels: Vec<&str> = capped.iter().map(|segment| segment.text.as_ref()).collect();
            assert_eq!(
                labels.iter().find(|&&label| label == "segment-50").copied(),
                Some("segment-50"),
                "the protected menu-anchor segment survives the hard cap",
            );
        }
        assert_eq!(symbol_segments.len(), segments.len());
        assert_eq!(kinds.len(), segments.len());
        assert_eq!(file_segment_index, 65);
        assert_eq!(kinds[file_segment_index], BreadcrumbSegmentKind::File);
    }

    fn capped_symbol_run(
        symbol_count: usize,
        protected_index: Option<usize>,
    ) -> (Vec<HighlightedText>, Vec<BreadcrumbSegmentKind>) {
        let segments: Vec<HighlightedText> = (0..=symbol_count)
            .map(|index| HighlightedText {
                text: format!("symbol-{index}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(0), false);
        let (capped, _, kinds, _) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 0, protected_index);
        (capped, kinds)
    }

    #[test]
    fn test_hard_cap_counts_the_glyph_it_splices_in() {
        use super::layout::{ELLIPSIS_GLYPH, MAX_BREADCRUMB_SEGMENTS_HARD_CAP};

        let (at_cap, kinds) = capped_symbol_run(MAX_BREADCRUMB_SEGMENTS_HARD_CAP, None);
        assert_eq!(at_cap.len(), 1 + MAX_BREADCRUMB_SEGMENTS_HARD_CAP);
        assert!(
            !at_cap
                .iter()
                .any(|segment| segment.text.as_ref() == ELLIPSIS_GLYPH),
            "a run of exactly the cap is within the limit, so nothing is spliced"
        );
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == BreadcrumbSegmentKind::Symbol)
                .count(),
            MAX_BREADCRUMB_SEGMENTS_HARD_CAP
        );

        let (over_cap, kinds) = capped_symbol_run(MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1, None);
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == BreadcrumbSegmentKind::Symbol)
                .count(),
            MAX_BREADCRUMB_SEGMENTS_HARD_CAP,
            "the spliced glyph is part of the run, so it has to fit inside the cap"
        );
        assert_eq!(over_cap.len(), 1 + MAX_BREADCRUMB_SEGMENTS_HARD_CAP);
        assert_eq!(
            over_cap
                .iter()
                .filter(|segment| segment.text.as_ref() == ELLIPSIS_GLYPH)
                .count(),
            1
        );

        // Both ends of a protected window splice, so each glyph has to be paid for too.
        for (symbol_count, protected) in [
            (MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1, 1),
            (MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1, 33),
            (
                MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1,
                MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1,
            ),
            (200, 100),
            (5000, 2500),
        ] {
            let (capped, kinds) = capped_symbol_run(symbol_count, Some(protected));
            let run = kinds
                .iter()
                .filter(|kind| **kind == BreadcrumbSegmentKind::Symbol)
                .count();
            assert!(
                run <= MAX_BREADCRUMB_SEGMENTS_HARD_CAP,
                "run of {run} from {symbol_count} symbols protecting {protected} exceeds the cap"
            );
            assert_eq!(capped.len(), 1 + run);
            let protected_label = format!("symbol-{protected}");
            assert!(
                capped
                    .iter()
                    .any(|segment| segment.text.as_ref() == protected_label),
                "the protected segment still has to survive the tighter window"
            );
        }
    }

    #[test]
    fn test_hard_cap_applies_to_a_symbol_trail_with_no_middle_run() {
        use super::layout::MAX_BREADCRUMB_SEGMENTS_HARD_CAP;

        // A file at the worktree root is File + N symbols: no directory run at all. Capping
        // only the directory run left this shape uncapped, so `measure` shaped every symbol
        // and the planner re-summed the widths once per drop.
        let segments: Vec<HighlightedText> = (0..201)
            .map(|index| HighlightedText {
                text: format!("symbol-{index}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(0), false);
        assert!(
            !kinds.contains(&BreadcrumbSegmentKind::Middle),
            "the fixture has to have no middle run for this to exercise the symbol cap"
        );

        let (capped, _, kinds, file_segment_index) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 0, None);

        assert_eq!(capped.len(), 1 + MAX_BREADCRUMB_SEGMENTS_HARD_CAP);
        assert_eq!(file_segment_index, 0);
        assert_eq!(kinds[file_segment_index], BreadcrumbSegmentKind::File);
        assert!(
            kinds[1..]
                .iter()
                .all(|kind| *kind == BreadcrumbSegmentKind::Symbol),
            "the spliced glyph stands in for symbols, so it has to be classified as one"
        );
    }

    #[test]
    fn test_hard_cap_keeps_the_anchored_symbol() {
        let segments: Vec<HighlightedText> = (0..201)
            .map(|index| HighlightedText {
                text: format!("symbol-{index}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(0), false);

        let (capped, _, _, _) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 0, Some(100));

        let labels: Vec<&str> = capped.iter().map(|segment| segment.text.as_ref()).collect();
        assert!(
            labels.contains(&"symbol-100"),
            "dropping the anchored segment leaves prepaint without menu_anchor_bounds, which \
             dismisses the open menu"
        );
    }

    #[test]
    fn test_hard_cap_glyph_keeps_the_deepest_target_it_hides() {
        let worktree_id = WorktreeId::from_usize(0);
        let segments: Vec<HighlightedText> = (0..100)
            .map(|index| HighlightedText {
                text: format!("dir-{index}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = (0..100)
            .map(|index| {
                Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path: RelPath::new_test(&format!("dir-{index}")).into_arc(),
                })
            })
            .collect();
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(99), true);

        let (capped, targets, _, _) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 99, None);

        let glyph = capped
            .iter()
            .position(|segment| segment.text.as_ref() == super::layout::ELLIPSIS_GLYPH)
            .expect("the middle run is over the cap, so it collapses to a glyph");
        let Some(BreadcrumbSegmentTarget::Directory { path, .. }) = &targets[glyph] else {
            panic!("a collapsed run with no target is a dead control");
        };
        assert_eq!(
            path.as_unix_str(),
            "dir-67",
            "the glyph browses the deepest directory it hides"
        );
    }

    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_leaves_ordinary_input_untouched() {
        let segments: Vec<HighlightedText> = (0..6)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), Some(3), true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_segment_runs(segments, symbol_segments, kinds, 3, None);

        assert_eq!(segments.len(), 6);
        assert_eq!(symbol_segments.len(), 6);
        assert_eq!(kinds.len(), 6);
        assert_eq!(file_segment_index, 3);
    }

    #[gpui::test]
    async fn test_lone_segment_is_budgeted_down_to_the_strip(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::px;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "a_rather_long_file_name_that_will_not_fit.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(
                    path!("/root/src/a_rather_long_file_name_that_will_not_fit.rs"),
                    cx,
                )
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
            width: gpui::Pixels,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div()
                    .debug_selector(|| "breadcrumb-strip-viewport".to_string())
                    .w(self.width)
                    .child(BreadcrumbStrip {
                        segments: self.segments.clone(),
                        editor: Some(self.editor.downgrade()),
                        breadcrumb_font: None,
                    })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                segments: vec![PreparedBreadcrumbSegment {
                    kind: BreadcrumbSegmentKind::File,
                    label: HighlightedText {
                        text: "a_rather_long_file_name_that_will_not_fit.rs".into(),
                        highlights: vec![],
                    },
                    target: Some(BreadcrumbSegmentTarget::Directory {
                        worktree_id,
                        path: rel_path("src").into_arc(),
                    }),
                    dirty_filename_style: false,
                    icon: None,
                    git_status_color: None,
                }],
                width: px(600.),
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let wide = cx
            .debug_bounds("breadcrumb-segment-0")
            .expect("the only segment paints when the strip is wide");

        host_window
            .update(cx, |host, _window, cx| {
                host.width = px(120.);
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let viewport = cx
            .debug_bounds("breadcrumb-strip-viewport")
            .expect("the fixed-width host paints");
        let narrow = cx
            .debug_bounds("breadcrumb-segment-0")
            .expect("the only segment still paints when the strip is narrow");

        assert!(
            narrow.size.width < wide.size.width,
            "a segment the planner cannot drop must be budgeted down, got {:?} in a {:?} strip",
            narrow.size.width,
            viewport.size.width
        );
        assert!(
            narrow.right() <= viewport.right(),
            "a lone segment must not paint past the strip: {narrow:?} vs {viewport:?}"
        );
    }

    // With no run in front of it a too-narrow tail stays put: replacing it with a fresh glyph
    // would start where the tail did and inherit the room already found too short.
    #[gpui::test]
    async fn test_painted_segments_occupy_the_widths_the_plan_measured(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::px;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div()
                    .debug_selector(|| "breadcrumb-strip-viewport".to_string())
                    .w(px(600.))
                    .child(BreadcrumbStrip {
                        segments: self.segments.clone(),
                        editor: Some(self.editor.downgrade()),
                        breadcrumb_font: None,
                    })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let segment = |kind, text: &str, path: &str| PreparedBreadcrumbSegment {
                kind,
                label: HighlightedText {
                    text: text.to_string().into(),
                    highlights: vec![],
                },
                target: Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path: rel_path(path).into_arc(),
                }),
                dirty_filename_style: false,
                icon: None,
                git_status_color: None,
            };
            StripHost {
                editor,
                segments: vec![
                    segment(BreadcrumbSegmentKind::Root, "root", ""),
                    segment(BreadcrumbSegmentKind::Middle, "src", "src"),
                    segment(BreadcrumbSegmentKind::File, "main.rs", "src/main.rs"),
                ],
            }
        });
        let (editor, segments) = host_window
            .read_with(cx, |host, _| (host.editor.clone(), host.segments.clone()))
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let widths = cx.update(|window, _| {
            BreadcrumbStrip {
                segments,
                editor: Some(editor.downgrade()),
                breadcrumb_font: None,
            }
            .measured_widths_for_test(window)
        });
        let bounds: Vec<_> = [
            "breadcrumb-segment-0",
            "breadcrumb-segment-1",
            "breadcrumb-segment-2",
        ]
        .into_iter()
        .map(|selector| {
            cx.debug_bounds(selector)
                .expect("a wide strip paints every segment")
        })
        .collect();
        for index in 0..2 {
            let advance = bounds[index + 1].origin.x - bounds[index].origin.x;
            let expected = widths[index];
            // Shaped glyph widths snap to the pixel grid, so an accumulated pixel is noise;
            // unaccounted chrome shows up as two per clickable segment.
            assert!(
                (advance - expected).abs() <= px(1.5),
                "segment {index} advances {advance:?} but measure said {expected:?}: chrome the \
                 plan does not account for ellipsizes the tail early"
            );
        }
    }

    #[gpui::test]
    async fn test_a_too_narrow_tail_is_kept_when_no_run_can_absorb_it(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::px;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div()
                    .debug_selector(|| "breadcrumb-strip-viewport".to_string())
                    .w(px(24.))
                    .child(BreadcrumbStrip {
                        segments: self.segments.clone(),
                        editor: Some(self.editor.downgrade()),
                        breadcrumb_font: None,
                    })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let segment = |kind, text: &str, path: &str| PreparedBreadcrumbSegment {
                kind,
                label: HighlightedText {
                    text: text.to_string().into(),
                    highlights: vec![],
                },
                target: Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path: rel_path(path).into_arc(),
                }),
                dirty_filename_style: false,
                icon: None,
                git_status_color: None,
            };
            StripHost {
                editor,
                // Two segments only, so the sequence is [Segment, Segment] and the tail has no
                // run in front of it to fold into.
                segments: vec![
                    segment(BreadcrumbSegmentKind::Middle, "src", "src"),
                    segment(BreadcrumbSegmentKind::File, "main.rs", "src/main.rs"),
                ],
            }
        });
        let editor = host_window
            .read_with(cx, |host, _| host.editor.clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Anchoring on the first segment protects it, so the planner drops nothing and both
        // segments reach prepaint at full measured width.
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("src").into_arc(),
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let viewport = cx
            .debug_bounds("breadcrumb-strip-viewport")
            .expect("the fixed-width host paints");
        let tail = cx
            .debug_bounds("breadcrumb-segment-1")
            .expect("the tail must survive: no run in front of it can take it over");
        assert!(
            tail.right() <= viewport.right(),
            "the kept tail still has to stay inside the strip: {tail:?} vs {viewport:?}"
        );
        assert_eq!(
            cx.debug_bounds("breadcrumb-collapsed-run"),
            None,
            "a fresh glyph here would replace a real click target with one that hides nothing"
        );
    }

    #[gpui::test]
    async fn test_protected_segment_and_tail_do_not_overrun_the_strip(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::px;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "nested": { "main.rs": "fn main() {}" } } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/nested/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div()
                    .debug_selector(|| "breadcrumb-strip-viewport".to_string())
                    .w(px(24.))
                    .child(BreadcrumbStrip {
                        segments: self.segments.clone(),
                        editor: Some(self.editor.downgrade()),
                        breadcrumb_font: None,
                    })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let segment = |kind, text: &str, path: &str| PreparedBreadcrumbSegment {
                kind,
                label: HighlightedText {
                    text: text.to_string().into(),
                    highlights: vec![],
                },
                target: Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path: rel_path(path).into_arc(),
                }),
                dirty_filename_style: false,
                icon: None,
                git_status_color: None,
            };
            StripHost {
                editor,
                segments: vec![
                    segment(BreadcrumbSegmentKind::Middle, "src", "src"),
                    segment(BreadcrumbSegmentKind::Middle, "nested", "src/nested"),
                    segment(BreadcrumbSegmentKind::File, "main.rs", "src/nested/main.rs"),
                ],
            }
        });
        let editor = host_window
            .read_with(cx, |host, _| host.editor.clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Anchoring the menu to the first segment makes the planner keep it at full width, so
        // it and the tail together are what can overrun a strip this narrow.
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("src").into_arc(),
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let viewport = cx
            .debug_bounds("breadcrumb-strip-viewport")
            .expect("the fixed-width host paints");
        for selector in [
            "breadcrumb-segment-0",
            "breadcrumb-segment-1",
            "breadcrumb-segment-2",
            "breadcrumb-collapsed-run",
        ] {
            if let Some(bounds) = cx.debug_bounds(selector) {
                assert!(
                    bounds.right() <= viewport.right(),
                    "{selector} paints past the strip: {bounds:?} vs {viewport:?}"
                );
            }
        }

        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_some(),
                "the anchor has to survive the squeeze, or the menu dismisses itself"
            );
        });
    }

    // `simulate_click` hardcodes click_count 1, so two calls are two single clicks and the
    // reveal never runs; the raw events are what reach the double-click guards.
    #[gpui::test]
    async fn test_double_clicking_a_directory_segment_reveals_it_in_the_project_panel(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::{Modifiers, MouseButton, MouseDownEvent, MouseUpEvent};
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::cell::Cell;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(BreadcrumbStrip {
                    segments: self.segments.clone(),
                    editor: Some(self.editor.downgrade()),
                    breadcrumb_font: None,
                })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                segments: vec![PreparedBreadcrumbSegment {
                    kind: BreadcrumbSegmentKind::Middle,
                    label: HighlightedText {
                        text: "src".into(),
                        highlights: vec![],
                    },
                    target: Some(BreadcrumbSegmentTarget::Directory {
                        worktree_id,
                        path: rel_path("src").into_arc(),
                    }),
                    dirty_filename_style: false,
                    icon: None,
                    git_status_color: None,
                }],
            }
        });
        let editor = host_window
            .read_with(cx, |host, _| host.editor.clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let revealed = Rc::new(Cell::new(None));
        let _subscription = cx.update(|_, cx| {
            let revealed = revealed.clone();
            cx.subscribe(&project, move |_, event: &project::Event, _| {
                if let project::Event::RevealInProjectPanel(entry_id) = event {
                    revealed.set(Some(*entry_id));
                }
            })
        });

        let segment = cx
            .debug_bounds("breadcrumb-segment-0")
            .expect("the directory segment paints");
        let click = |cx: &mut VisualTestContext, count: usize| {
            cx.simulate_event(MouseDownEvent {
                position: segment.center(),
                button: MouseButton::Left,
                modifiers: Modifiers::none(),
                click_count: count,
                first_mouse: false,
            });
            cx.simulate_event(MouseUpEvent {
                position: segment.center(),
                button: MouseButton::Left,
                modifiers: Modifiers::none(),
                click_count: count,
            });
            cx.run_until_parked();
        };

        click(cx, 1);
        assert!(
            revealed.get().is_none(),
            "a single click must not reveal: that is the double-click gesture"
        );
        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_some(),
                "a single click opens the listing"
            );
        });

        click(cx, 2);
        let expected = project.read_with(cx, |project, cx| {
            project
                .entry_for_path(
                    &project::ProjectPath {
                        worktree_id,
                        path: rel_path("src").into_arc(),
                    },
                    cx,
                )
                .map(|entry| entry.id)
        });
        assert_eq!(
            revealed.get(),
            expected,
            "a double click reveals the directory in the project panel"
        );
        // The listing's own fate on the second click is decided by the outside-press dismissal
        // in the menu wrapper, not by the `click_count` guard in the segment's click handler,
        // so it is not what this test pins.
    }

    #[gpui::test]
    async fn test_breadcrumb_file_segment_right_click_copies_the_file_path(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(BreadcrumbStrip {
                    segments: self.segments.clone(),
                    editor: Some(self.editor.downgrade()),
                    breadcrumb_font: None,
                })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                segments: vec![
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::Root,
                        label: HighlightedText {
                            text: "root".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: RelPath::empty().into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::Middle,
                        label: HighlightedText {
                            text: "src".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: rel_path("src").into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::File,
                        label: HighlightedText {
                            text: "main.rs".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Symbol {
                            buffer_id,
                            item: None,
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                ],
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let src_bounds = cx
            .debug_bounds("breadcrumb-segment-2")
            .expect("file segment should paint");
        cx.simulate_mouse_down(
            src_bounds.center(),
            gpui::MouseButton::Right,
            gpui::Modifiers::none(),
        );
        cx.run_until_parked();

        let expected = project.read_with(cx, |project, cx| {
            let worktree = project.worktree_for_id(worktree_id, cx).unwrap();
            worktree
                .read(cx)
                .absolutize(rel_path("src"))
                .to_string_lossy()
                .into_owned()
        });
        let file_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktree_for_id(worktree_id, cx).unwrap();
            worktree
                .read(cx)
                .absolutize(rel_path("src/main.rs"))
                .to_string_lossy()
                .into_owned()
        });

        let clipboard = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .expect("clipboard should hold the file path");
        assert_eq!(clipboard, file_path);
        assert_ne!(
            clipboard, expected,
            "the file segment must copy the file's path, not its parent directory's"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_segment_right_click_copies_directory_path(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use language::HighlightedText;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        use super::layout::{BreadcrumbSegmentKind, BreadcrumbStrip, PreparedBreadcrumbSegment};

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "main.rs": "fn main() {}" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct StripHost {
            editor: Entity<Editor>,
            segments: Vec<PreparedBreadcrumbSegment>,
        }
        impl gpui::Render for StripHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(BreadcrumbStrip {
                    segments: self.segments.clone(),
                    editor: Some(self.editor.downgrade()),
                    breadcrumb_font: None,
                })
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            StripHost {
                editor,
                segments: vec![
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::Root,
                        label: HighlightedText {
                            text: "root".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: RelPath::empty().into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::Middle,
                        label: HighlightedText {
                            text: "src".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: rel_path("src").into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                    PreparedBreadcrumbSegment {
                        kind: BreadcrumbSegmentKind::File,
                        label: HighlightedText {
                            text: "main.rs".into(),
                            highlights: vec![],
                        },
                        target: Some(BreadcrumbSegmentTarget::Directory {
                            worktree_id,
                            path: rel_path("src/main.rs").into_arc(),
                        }),
                        dirty_filename_style: false,
                        icon: None,
                        git_status_color: None,
                    },
                ],
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let src_bounds = cx
            .debug_bounds("breadcrumb-segment-1")
            .expect("src directory segment should paint");
        cx.simulate_mouse_down(
            src_bounds.center(),
            gpui::MouseButton::Right,
            gpui::Modifiers::none(),
        );
        cx.run_until_parked();

        let expected = project.read_with(cx, |project, cx| {
            let worktree = project.worktree_for_id(worktree_id, cx).unwrap();
            worktree
                .read(cx)
                .absolutize(rel_path("src"))
                .to_string_lossy()
                .into_owned()
        });
        let file_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktree_for_id(worktree_id, cx).unwrap();
            worktree
                .read(cx)
                .absolutize(rel_path("src/main.rs"))
                .to_string_lossy()
                .into_owned()
        });

        let clipboard = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .expect("clipboard should hold the directory path");
        assert_eq!(clipboard, expected);
        assert_ne!(
            clipboard, file_path,
            "directory segment must copy its own path, not the file path"
        );
    }

    #[test]
    fn test_hard_cap_protected_at_middle_of_long_path() {
        use super::layout::MAX_BREADCRUMB_SEGMENTS_HARD_CAP;

        // Root + 5000 middle dirs + file.
        let segment_count = 5002;
        let file_segment_index = segment_count - 1;
        let protected = 2500;
        let segments: Vec<HighlightedText> = (0..segment_count)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segment_count];
        let kinds =
            classify_breadcrumb_segment_kinds(segment_count, Some(file_segment_index), true);

        let (capped, _, kinds, new_file_index) = hard_cap_segment_runs(
            segments,
            symbol_segments,
            kinds,
            file_segment_index,
            Some(protected),
        );

        // Root + dual ellipses + at most CAP middle + file.
        let max_len = 1 + MAX_BREADCRUMB_SEGMENTS_HARD_CAP + 1;
        assert!(
            capped.len() <= max_len,
            "capped length {} exceeds max {}",
            capped.len(),
            max_len
        );
        let protected_label = format!("segment-{protected}");
        let labels: Vec<&str> = capped.iter().map(|segment| segment.text.as_ref()).collect();
        assert_eq!(
            labels
                .iter()
                .find(|&&label| label == protected_label.as_str())
                .copied(),
            Some(protected_label.as_str()),
            "protected middle segment must survive"
        );
        assert_eq!(kinds[new_file_index], BreadcrumbSegmentKind::File);
        assert_eq!(
            capped.iter().filter(|s| s.text.as_ref() == "⋯").count(),
            2,
            "protected mid-run path should splice both sides with an ellipsis"
        );
    }

    fn sample_breadcrumb_widths_and_kinds() -> (Vec<Pixels>, Vec<BreadcrumbSegmentKind>) {
        use BreadcrumbSegmentKind::*;
        let widths = vec![
            px(60.),  // root
            px(30.),  // a
            px(30.),  // b
            px(30.),  // c
            px(30.),  // d
            px(80.),  // file.kt
            px(90.),  // Class
            px(120.), // fun method
        ];
        let kinds = vec![Root, Middle, Middle, Middle, Middle, File, Symbol, Symbol];
        (widths, kinds)
    }

    #[test]
    fn test_plan_breadcrumb_layout_everything_fits() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();
        let total: Pixels = widths.iter().fold(Pixels::ZERO, |sum, w| sum + *w);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), total, None);

        assert_eq!(plan.visible, (0..widths.len()).collect::<Vec<_>>());
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_drops_middle_before_root_before_file_before_outer_symbols() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(380.), None);
        assert_eq!(plan.visible, vec![0, 5, 6, 7]);
        assert_eq!(plan.ellipses, vec![1..5]);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(340.), None);
        assert_eq!(plan.visible, vec![5, 6, 7]);
        assert_eq!(plan.ellipses, vec![0..5]);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(230.), None);
        assert_eq!(plan.visible, vec![6, 7]);
        assert_eq!(plan.ellipses, vec![0..6]);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(140.), None);
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_degenerate_case_always_keeps_the_last_segment() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(1.), None);
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_single_segment_never_collapses() {
        let plan = plan_breadcrumb_layout(
            &[px(500.)],
            &[BreadcrumbSegmentKind::File],
            px(20.),
            px(1.),
            None,
        );
        assert_eq!(plan.visible, vec![0]);
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_empty_input() {
        let plan = plan_breadcrumb_layout(&[], &[], px(20.), px(500.), None);
        assert!(plan.visible.is_empty());
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_protects_active_segment_at_every_width() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();
        let active_index = 2;

        let unprotected = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(340.), None);
        assert_eq!(unprotected.visible, vec![5, 6, 7]);

        let cases = [
            (px(380.), vec![2, 5, 6, 7]),
            (px(340.), vec![2, 6, 7]),
            (px(230.), vec![2, 7]),
            (px(140.), vec![2, 7]),
            (px(1.), vec![2, 7]),
        ];
        for (available, expected_visible) in cases {
            let plan =
                plan_breadcrumb_layout(&widths, &kinds, px(20.), available, Some(active_index));
            assert_eq!(
                plan.visible, expected_visible,
                "protected layout at width {available:?}"
            );
        }
    }

    #[gpui::test]
    async fn test_choosing_breadcrumb_directory_updates_listing_in_place(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir_a": {
                    "child1.txt": "",
                    "child2.txt": "",
                },
                "file.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        cx.run_until_parked();

        choose_directory_entry_via_keystroke(&menu, "dir_a", cx);

        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => {
                    panic!("listing stays on the same entity and moves to dir_a, got {other:?}")
                }
            };
            assert_eq!(path, "dir_a");
            assert!(menu.navigated_path().is_some());
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["child1.txt", "child2.txt"],
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_descends_single_child_dirs(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a": { "b": { "c.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        choose_directory_entry_via_keystroke(&menu, "a", cx);
        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "a/b");
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_respects_auto_fold_dirs_off(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_fold_dirs = Some(false);
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a": { "b": { "c.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        choose_directory_entry_via_keystroke(&menu, "a", cx);
        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "a");
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_preselects_active_path(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a.txt": "",
                "b.txt": "",
                "subdir": { "nested.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let worktree = project.update(cx, |project, cx| {
            project.worktree_for_id(worktree_id, cx).unwrap()
        });
        let root_entries = cx.update(|_window, cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        let b_txt_index = root_entries
            .iter()
            .position(|entry| entry.name.as_ref() == "b.txt")
            .expect("b.txt listed at root");
        let subdir_index = root_entries
            .iter()
            .position(|entry| entry.name.as_ref() == "subdir")
            .expect("subdir listed at root");

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    Some(rel_path("b.txt").into_arc()),
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.selected_index(),
                Some(b_txt_index),
                "preselects open file row"
            );
        });
        drop(menu);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    Some(rel_path("subdir/nested.txt").into_arc()),
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.selected_index(),
                Some(subdir_index),
                "preselects ancestor directory of open file"
            );
        });
        drop(menu);
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_expand_nested_gitignored(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored_dir/\n",
                "kept.txt": "",
                "ignored_dir": {
                    "nested": {
                        "secret.txt": "hi",
                    },
                },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let worktree = project.update(cx, |project, cx| {
            project.worktree_for_id(worktree_id, cx).unwrap()
        });
        cx.run_until_parked();

        let root_entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        let ignored = root_entries
            .iter()
            .find(|entry| entry.name.as_ref() == "ignored_dir")
            .expect("gitignored dir is listed when hide_gitignore is false");
        assert!(ignored.is_ignored);

        let before = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                rel_path("ignored_dir"),
            )
        });
        assert!(
            before.is_empty(),
            "unexpanded ignored dir should not list nested children yet"
        );

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let menu = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbNavigationMenu::new(
                    editor.downgrade(),
                    workspace.downgrade(),
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .unwrap();
        choose_directory_entry_via_keystroke(&menu, "ignored_dir", cx);

        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "ignored_dir/nested");
        });

        let after = cx.update(|_window, cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                rel_path("ignored_dir"),
            )
        });
        assert_eq!(
            after.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["nested"],
            "after choose expands the gitignored dir, nested children are visible"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_sorts_like_project_panel(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "Apple": { "leaf.txt": "" },
                "banana.txt": "",
                "Cherry.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["Apple", "banana.txt", "Cherry.txt"],
        );

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let project_panel = settings.project_panel.get_or_insert_default();
                    project_panel.sort_mode = Some(settings::ProjectPanelSortMode::FilesFirst);
                    project_panel.sort_order = Some(settings::ProjectPanelSortOrder::Unicode);
                });
            });
        });
        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["Cherry.txt", "banana.txt", "Apple"],
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_honors_hide_gitignore_setting(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored.txt",
                "kept.txt": "",
                "ignored.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec![".gitignore", "ignored.txt", "kept.txt"],
        );
        assert!(
            entries
                .iter()
                .find(|entry| entry.name.as_ref() == "ignored.txt")
                .is_some_and(|entry| entry.is_ignored)
        );

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project_panel
                        .get_or_insert_default()
                        .hide_gitignore = Some(true);
                });
            });
        });
        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec![".gitignore", "kept.txt"],
            "hide_gitignore should drop the ignored entry entirely"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_honors_hide_hidden_setting(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".hidden": "",
                "kept.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec![".hidden", "kept.txt"],
            "hidden entry is shown by default"
        );

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project_panel.get_or_insert_default().hide_hidden = Some(true);
                });
            });
        });
        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["kept.txt"],
            "hide_hidden should drop the hidden entry entirely"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_zero_match_filter_settles(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "", "beta.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        // Without this, a listing that never loaded would satisfy the zero-match assertion too.
        menu.read_with(cx, |menu, _| {
            assert!(!menu.entry_names().is_empty(), "listing must have loaded");
        });
        cx.simulate_keystrokes("z z");
        cx.run_until_parked();

        menu.read_with(cx, |menu, cx| {
            assert_eq!(menu.filter(), "zz");
            assert_eq!(menu.filtered_entry_names(), Vec::<SharedString>::new());
            assert!(
                !menu.rank_pending(),
                "a completed zero-match rank must settle out of the searching state"
            );
            // What the picker shows, not what the menu knows: a settled rank with no matches has
            // to replace the rows the user was reading, or Enter confirms one of them.
            assert_eq!(
                menu.published_row_labels(cx),
                Vec::<SharedString>::new(),
                "the previous rows must not stay on screen after a zero-match rank settles"
            );
            assert_eq!(menu.published_empty_message(cx).as_ref(), "No matches");
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_reloads_when_sort_settings_change(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "Apple": { "leaf.txt": "" }, "banana.txt": "", "Cherry.txt": "" }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.entry_names(),
                vec![
                    SharedString::from("Apple"),
                    SharedString::from("banana.txt"),
                    SharedString::from("Cherry.txt"),
                ],
            );
        });
        let reloads_before = menu.read_with(cx, |menu, _| menu.directory_reload_count_for_test());

        cx.update(|_, cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let project_panel = settings.project_panel.get_or_insert_default();
                    project_panel.sort_mode = Some(settings::ProjectPanelSortMode::FilesFirst);
                    project_panel.sort_order = Some(settings::ProjectPanelSortOrder::Unicode);
                });
            });
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert!(
                menu.directory_reload_count_for_test() > reloads_before,
                "sort_mode change must rebuild the directory listing"
            );
            assert_eq!(
                menu.entry_names(),
                vec![
                    SharedString::from("Cherry.txt"),
                    SharedString::from("banana.txt"),
                    SharedString::from("Apple"),
                ],
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_ignores_unrelated_settings_changes(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "a.txt": "", "b.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        let reloads_before = menu.read_with(cx, |menu, _| menu.directory_reload_count_for_test());

        cx.update(|_, cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    // Unrelated to BreadcrumbListingSettings fields.
                    settings.editor.auto_signature_help = Some(true);
                });
            });
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.directory_reload_count_for_test(),
                reloads_before,
                "unrelated settings must not rebuild the directory listing"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_filter_finds_entry_past_display_cap(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let mut files = serde_json::Map::new();
        for index in 0..(MAX_BREADCRUMB_MENU_ROWS + 50) {
            files.insert(format!("entry_{index:04}.txt"), json!(""));
        }
        files.insert("zzz_target.txt".into(), json!(""));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!(files)).await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                let unfiltered = menu.entry_names();
                assert!(
                    unfiltered.len() > MAX_BREADCRUMB_MENU_ROWS,
                    "fixture must exceed the display cap; got {}",
                    unfiltered.len()
                );
                assert_eq!(
                    menu.filtered_entry_names().len(),
                    MAX_BREADCRUMB_MENU_ROWS,
                    "unfiltered view is display-capped"
                );
                let filtered = menu.filtered_entry_names();
                let filtered_names: Vec<&str> = filtered.iter().map(|name| name.as_ref()).collect();
                assert_eq!(
                    filtered_names
                        .iter()
                        .find(|&&name| name == "zzz_target.txt")
                        .copied(),
                    None,
                    "target must sort past the display cap without a filter"
                );
            })
            .unwrap();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("z z z");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.filter(), "zzz");
                let filtered = menu.filtered_entry_names();
                assert_eq!(
                    filtered.iter().map(|n| n.as_ref()).collect::<Vec<_>>(),
                    vec!["zzz_target.txt"],
                    "fuzzy over the full set must surface the past-cap entry"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_truncation_keeps_the_first_matches(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        // Identical names except the index rank with equal scores, so the truncation's own
        // tie-break is the whole outcome.
        let mut files = serde_json::Map::new();
        for index in 0..(MAX_BREADCRUMB_MENU_ROWS + 50) {
            files.insert(format!("item_{index:04}.txt"), json!(""));
        }
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!(files)).await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("i t e m");
        cx.run_until_parked();

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.filter(), "item");
                let filtered = menu.filtered_entry_names();
                assert_eq!(
                    filtered.first().map(|name| name.as_ref()),
                    Some("item_0000.txt"),
                    "a capped tie must keep the first files of the listing, not the last"
                );
                assert_eq!(
                    filtered.len(),
                    MAX_BREADCRUMB_MENU_ROWS,
                    "the cap itself still applies"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_a_nested_symbol_segment_does_not_advertise_the_chord(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});

        // The predicate's third case needs a real `OutlineItem`, which needs anchors: the file
        // segment carries `item: None`, a segment for a symbol inside the file carries `Some`.
        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "fn alpha() {}
",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let range = snapshot.anchor_before(MultiBufferOffset(0))
            ..snapshot.anchor_before(MultiBufferOffset(1));
        let nested = BreadcrumbSegmentTarget::Symbol {
            buffer_id: language::BufferId::new(1).unwrap(),
            item: Some(OutlineItem {
                depth: 1,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: "alpha".into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            }),
        };

        assert!(
            !super::layout::segment_tooltip_shows_navigation_chord(&nested),
            "a nested symbol segment opens that symbol's children, which the chord does not"
        );
    }

    #[gpui::test]
    async fn test_filter_names_the_parent_of_an_out_of_level_symbol(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "class Outer {\n  fn child_a() {}\n  fn child_b() {}\n}\nclass Peer {}\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item =
            |depth: usize, text: &str, range: std::ops::Range<multi_buffer::Anchor>| OutlineItem {
                depth,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: text.into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            };
        // Two top-level classes, `Outer` holding two methods. Opened at the top level, so the
        // methods sit outside the level being browsed.
        let all_items = vec![
            item(0, "Outer", anchor_at(0)),
            item(1, "child_a", anchor_at(20)),
            item(1, "child_b", anchor_at(40)),
            item(0, "Peer", anchor_at(60)),
        ];
        let buffer_id =
            multi_buffer.read_with(cx, |mb, cx| mb.as_singleton().unwrap().read(cx).remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items,
                // Only the two top-level symbols are the browsed level.
                vec![0, 3],
                vec![],
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_input("child_a");
        cx.run_until_parked();

        menu.read_with(cx, |menu, cx| {
            let contexts = menu.published_symbol_contexts(cx);
            let child = contexts
                .iter()
                .find(|(label, _)| label.as_ref() == "child_a")
                .expect("the filter surfaces child_a from inside Outer");
            assert_eq!(
                child.1.as_ref().map(|context| context.as_ref()),
                Some("Outer"),
                "a symbol matched outside the browsed level names its parent"
            );
        });
    }

    #[gpui::test]
    async fn test_a_drill_keeps_the_filtered_rows_until_the_target_loads(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha_dir": { "inner_one.txt": "", "inner_two.txt": "" },
                "beta.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_input("alpha");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        // Only "alpha_dir" matches, so the drill acts on the row the filter left showing.
        menu.read_with(cx, |menu, cx| {
            assert_eq!(
                menu.published_row_labels(cx)
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["alpha_dir"],
            );
        });
        menu.update(cx, |menu, _| {
            let _ = menu.take_published_row_history();
        });

        cx.update(|window, cx| {
            window.dispatch_keystroke(gpui::Keystroke::parse("right").unwrap(), cx);
        });
        cx.run_until_parked();

        // Drilling keeps the rows the user was looking at until the target loads, then swaps:
        // never blank, and never the previous listing shown unfiltered.
        let published = menu.update(cx, |menu, _| menu.take_published_row_history());
        assert!(
            !published.is_empty() && published.iter().all(|rows| !rows.is_empty()),
            "a drill must not blank the rows while the target loads; published rows were              {published:?}"
        );
        assert!(
            published
                .iter()
                .all(|rows| rows.iter().all(|row| row.as_ref() != "beta.txt")),
            "and must never repaint the listing being left, unfiltered; published rows were {published:?}"
        );
        menu.read_with(cx, |menu, cx| {
            let listing_path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                _ => panic!("drill must land in a directory listing"),
            };
            assert_eq!(listing_path, "alpha_dir");
            let rows = menu.published_row_labels(cx);
            assert!(
                rows.iter().any(|row| row.as_ref() == "inner_one.txt"),
                "the drilled listing publishes once its load lands, got {rows:?}"
            );
        });
    }

    #[gpui::test]
    async fn test_one_escape_emits_one_dismiss(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use ::menu::Cancel;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "escape",
                Cancel,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "a.txt": "", "b.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        let dismissals = Rc::new(AtomicUsize::new(0));
        let _subscription = menu.update(cx, |_, cx| {
            let dismissals = dismissals.clone();
            cx.subscribe(&menu, move |_, _, _: &DismissEvent, _| {
                dismissals.fetch_add(1, Ordering::SeqCst);
            })
        });

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.update(|window, cx| {
            window.dispatch_keystroke(gpui::Keystroke::parse("escape").unwrap(), cx);
        });
        cx.run_until_parked();

        assert_eq!(
            dismissals.load(Ordering::SeqCst),
            1,
            "one Escape reaches the delegate, the picker's event, and the blur that follows \
             the teardown - the listeners get exactly one event"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_keyboard_filter(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha.txt": "",
                "beta.txt": "",
                "gamma.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                Some(util::rel_path::rel_path("beta.txt").into_arc()),
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert!(menu.filter().is_empty());
                assert_eq!(menu.entry_names().len(), 3);
                assert!(menu.selected_index().is_some());
            })
            .unwrap();

        cx.simulate_keystrokes("b");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.filter(), "b");
                let filtered = menu.filtered_entry_names();
                assert_eq!(filtered.len(), 1, "filtered={filtered:?}");
                assert_eq!(filtered[0].as_ref(), "beta.txt");
                assert_eq!(menu.selected_index(), Some(0));
            })
            .unwrap();

        host_window
            .update(cx, |host, window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.clear_filter_for_test(window, cx);
                });
            })
            .unwrap();
        cx.run_until_parked();
        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert!(menu.filter().is_empty());
                assert_eq!(menu.filtered_entry_names().len(), 3);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_breadcrumb_filter_editor_receives_typed_input(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha.txt": "",
                "beta.txt": "",
                "gamma.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_input("be");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.filter(), "be");
                let filtered = menu.filtered_entry_names();
                assert_eq!(
                    filtered.iter().map(|n| n.as_ref()).collect::<Vec<_>>(),
                    vec!["beta.txt"],
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_breadcrumb_bar_collapses_instead_of_clipping_when_narrowed(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::{px, size};
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    // The widths this test is tuned around include the file segment's icon,
                    // which follows the tab family and is off by default.
                    settings.tabs.get_or_insert_default().file_icons = Some(true);
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "some_directory": {
                    "another_directory": {
                        "a_rather_long_file_name.rs": "fn main() {}",
                    },
                },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(
                    path!("/root/some_directory/another_directory/a_rather_long_file_name.rs"),
                    cx,
                )
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct BarHost {
            editor: Entity<Editor>,
        }
        impl gpui::Render for BarHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let placeholder = vec![HighlightedText {
                    text: "placeholder".into(),
                    highlights: vec![],
                }];
                h_flex().size_full().child(render_breadcrumb_text(
                    placeholder,
                    None,
                    None,
                    &self.editor,
                    false,
                    cx,
                ))
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            BarHost { editor }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // root / some_directory / another_directory / a_rather_long_file_name.rs
        assert!(
            cx.debug_bounds("breadcrumb-segment-0").is_some()
                && cx.debug_bounds("breadcrumb-segment-3").is_some(),
            "a wide window paints the whole path"
        );

        cx.simulate_resize(size(px(320.), px(600.)));
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        assert_eq!(
            cx.debug_bounds("breadcrumb-segment-0"),
            None,
            "a narrow window must drop leading segments rather than clip the strip"
        );
        assert!(
            cx.debug_bounds("breadcrumb-segment-3").is_some(),
            "the deepest segment must survive narrowing"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_collapsed_ellipsis_browses_the_deepest_hidden_segment(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::{px, size};
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    // The widths this test is tuned around include the file segment's icon,
                    // which follows the tab family and is off by default.
                    settings.tabs.get_or_insert_default().file_icons = Some(true);
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "some_directory": {
                    "another_directory": {
                        "a_rather_long_file_name.rs": "fn main() {}",
                    },
                },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(
                    path!("/root/some_directory/another_directory/a_rather_long_file_name.rs"),
                    cx,
                )
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct BarHost {
            editor: Entity<Editor>,
        }
        impl gpui::Render for BarHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let placeholder = vec![HighlightedText {
                    text: "placeholder".into(),
                    highlights: vec![],
                }];
                h_flex().size_full().child(render_breadcrumb_text(
                    placeholder,
                    None,
                    None,
                    &self.editor,
                    false,
                    cx,
                ))
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            BarHost { editor }
        });
        let editor = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.editor.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        cx.simulate_resize(size(px(320.), px(600.)));
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        assert_eq!(
            cx.debug_bounds("breadcrumb-segment-0"),
            None,
            "the fixture must be narrow enough to collapse the leading segments"
        );

        let ellipsis = cx
            .debug_bounds("breadcrumb-collapsed-run")
            .expect("the collapsed run paints in place of what it hides");
        cx.simulate_click(ellipsis.center(), gpui::Modifiers::none());
        cx.run_until_parked();

        editor.read_with(cx, |editor, cx| {
            let listing = editor
                .breadcrumb_navigation_menu()
                .expect("clicking the collapsed run opens a listing")
                .read(cx)
                .listing()
                .clone();
            match listing {
                BreadcrumbListing::Directory { path, .. } => assert_eq!(
                    path.as_unix_str(),
                    "some_directory/another_directory",
                    "it browses the deepest segment it hides"
                ),
                other => panic!("expected a directory listing, got {other:?}"),
            }
        });
    }

    #[gpui::test]
    async fn test_stepping_left_keeps_rows_until_the_parent_loads(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "left",
                SelectParent,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "outer": { "inner": { "file.rs": "fn main() {}" }, "sibling.rs": "" } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::new_test("outer/inner").into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["file.rs"],
            );
        });
        menu.update(cx, |menu, _| {
            let _ = menu.take_published_row_history();
        });

        cx.simulate_keystrokes("left");
        cx.run_until_parked();

        let published = menu.update(cx, |menu, _| menu.take_published_row_history());
        assert!(
            !published.is_empty() && published.iter().all(|rows| !rows.is_empty()),
            "stepping left must keep the rows on screen until the parent loads, never blank to \
             Loading; published rows were {published:?}"
        );
        menu.read_with(cx, |menu, _| {
            match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => {
                    assert_eq!(path.as_unix_str(), "outer")
                }
                other => panic!("left steps to the parent directory, got {other:?}"),
            }
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["inner", "sibling.rs"],
            );
        });
    }

    #[gpui::test]
    async fn test_a_new_query_selects_its_best_match(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "alpha.txt": "", "beta.txt": "", "gamma.txt": "" }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Park the selection on the last row, as a user who arrowed there would.
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
            menu.set_selected_row(2, cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        menu.read_with(cx, |menu, _| assert_eq!(menu.selected_index(), Some(2)));

        // Every name matches "a"; the best match is "alpha.txt", not whatever sits at the old
        // index in the new results.
        cx.simulate_input("a");
        cx.run_until_parked();

        menu.read_with(cx, |menu, cx| {
            let rows = menu.published_row_labels(cx);
            assert_eq!(rows.first().map(|r| r.as_ref()), Some("alpha.txt"));
            assert_eq!(
                menu.selected_index(),
                Some(0),
                "a new query must land on its best match, got rows {rows:?}"
            );
        });
    }

    #[gpui::test]
    async fn test_typing_a_filter_does_not_blank_the_rows(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha.txt": "",
                "alpine.txt": "",
                "beta.txt": "",
                "gamma.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        menu.read_with(cx, |menu, cx| {
            assert_eq!(
                menu.published_row_labels(cx).len(),
                4,
                "fixture starts with the whole listing on screen"
            );
        });
        menu.update(cx, |menu, _| {
            let _ = menu.take_published_row_history();
        });

        // Typing goes through the picker's query editor, which takes the keystroke a hop before
        // the menu hears about it: the rows must not blank to "Searching…" in that window.
        cx.simulate_input("alp");
        cx.run_until_parked();

        let published = menu.update(cx, |menu, _| menu.take_published_row_history());
        assert!(
            !published.is_empty() && published.iter().all(|rows| !rows.is_empty()),
            "typing a filter must never blank the rows between the listing and the results; \
             published rows were {published:?}"
        );
        menu.read_with(cx, |menu, cx| {
            assert_eq!(
                menu.published_row_labels(cx)
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["alpha.txt", "alpine.txt"],
                "the settled rank replaces them with its matches"
            );
        });
    }

    #[gpui::test]
    async fn test_a_reload_under_a_filter_does_not_flash_the_rows(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha.txt": "",
                "alpine.txt": "",
                "beta.txt": "",
                "gamma.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("alp");
        cx.run_until_parked();
        menu.read_with(cx, |menu, cx| {
            let rows: Vec<String> = menu
                .published_row_labels(cx)
                .iter()
                .map(|n| n.as_ref().to_string())
                .collect();
            assert_eq!(
                rows,
                vec!["alpha.txt", "alpine.txt"],
                "fixture filters to the alp files"
            );
        });
        menu.update(cx, |menu, _| {
            let _ = menu.take_published_row_history();
        });

        fs.insert_file(path!("/root/alps.txt"), Vec::new()).await;
        cx.run_until_parked();

        let published = menu.update(cx, |menu, _| menu.take_published_row_history());
        assert!(
            !published.is_empty() && published.iter().all(|rows| !rows.is_empty()),
            "a reload under an active filter must not flash empty rows between results; published \
             rows were {published:?}"
        );
        assert!(
            published
                .iter()
                .all(|rows| rows.iter().all(|row| row.as_ref() != "beta.txt")),
            "and must never repaint the listing being left, unfiltered; published rows were {published:?}"
        );
        menu.read_with(cx, |menu, cx| {
            let rows: Vec<String> = menu
                .published_row_labels(cx)
                .iter()
                .map(|n| n.as_ref().to_string())
                .collect();
            assert!(
                rows.contains(&"alps.txt".to_string()),
                "the reloaded, still-filtered rows include the new match, got {rows:?}"
            );
        });
    }

    #[gpui::test]
    async fn test_select_parent_walks_out_of_a_directory_listing(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "left",
                SelectParent,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "outer": { "inner": { "file.rs": "fn main() {}" } } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::new_test("outer/inner").into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // Walking out of a directory is a primary interaction and only the symbol arm was
        // covered; the directory arm reaches a different branch of `select_parent`.
        for expected in ["outer", ""] {
            cx.simulate_keystrokes("left");
            cx.run_until_parked();
            menu.read_with(cx, |menu, _| match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => {
                    assert_eq!(
                        path.as_unix_str(),
                        expected,
                        "left should step to the parent"
                    )
                }
                other => panic!("expected a directory listing, got {other:?}"),
            });
        }

        // At the worktree root there is no parent, so it must stay put rather than dismiss.
        cx.simulate_keystrokes("left");
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| match menu.listing() {
            BreadcrumbListing::Directory { path, .. } => {
                assert_eq!(path.as_unix_str(), "", "left at the root is a no-op")
            }
            other => panic!("expected a directory listing, got {other:?}"),
        });
    }

    #[gpui::test]
    async fn test_clicking_a_bar_segment_opens_that_segments_listing(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::{px, size};
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "outer": { "inner": { "file.rs": "fn main() {}" } } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/outer/inner/file.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct BarHost {
            editor: Entity<Editor>,
        }
        impl gpui::Render for BarHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let placeholder = vec![HighlightedText {
                    text: "placeholder".into(),
                    highlights: vec![],
                }];
                h_flex().size_full().child(render_breadcrumb_text(
                    placeholder,
                    None,
                    None,
                    &self.editor,
                    false,
                    cx,
                ))
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            BarHost { editor }
        });
        let editor = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.editor.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Wide enough that nothing collapses, so segment indices map to the real path.
        cx.simulate_resize(size(px(1400.), px(600.)));
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // Every other test reaches the menu through a test-only shim; this one goes through the
        // bar's own `on_click`, so a segment that stops being clickable is caught here.
        let segment = cx
            .debug_bounds("breadcrumb-segment-1")
            .expect("the 'outer' path segment should paint as a clickable trigger");
        cx.simulate_click(segment.center(), gpui::Modifiers::none());
        cx.run_until_parked();

        editor.read_with(cx, |editor, cx| {
            let listing = editor
                .breadcrumb_navigation_menu()
                .expect("clicking a path segment opens its listing")
                .read(cx)
                .listing()
                .clone();
            match listing {
                BreadcrumbListing::Directory { path, .. } => {
                    assert_eq!(path.as_unix_str(), "outer")
                }
                other => panic!("expected a directory listing, got {other:?}"),
            }
        });

        // A second click on the same segment toggles it shut rather than reopening it.
        cx.simulate_click(segment.center(), gpui::Modifiers::none());
        cx.run_until_parked();
        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_none(),
                "clicking the open segment again should dismiss the menu"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_keyboard_survives_click_on_popup_chrome(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "down",
                SelectNext,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha.txt": "",
                "beta.txt": "",
                "gamma.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let dismissed = Rc::new(AtomicBool::new(false));
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        assert_eq!(menu.read_with(cx, |menu, _| menu.selected_index()), Some(0));

        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        // Padding of the filter row: popup chrome, not a row and not the filter editor.
        let chrome = point(menu_bounds.left() + px(2.), menu_bounds.top() + px(2.));
        cx.simulate_click(chrome, gpui::Modifiers::none());
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        // Without this the test would still pass with the click landing outside the popup,
        // where nothing can steal the filter editor's focus in the first place.
        assert!(
            !dismissed.load(Ordering::SeqCst),
            "the click must land inside the popup: {chrome:?} in {menu_bounds:?}"
        );

        cx.simulate_keystrokes("down");
        cx.run_until_parked();

        assert_eq!(
            menu.read_with(cx, |menu, _| menu.selected_index()),
            Some(1),
            "clicking popup chrome must not take the keyboard away from the menu"
        );

        cx.simulate_input("bet");
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filter(),
                "bet",
                "typing must still reach the filter editor"
            );
            assert_eq!(
                menu.filtered_entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["beta.txt"],
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_filter_keeps_listing_order_for_equal_scores(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let mut files = serde_json::Map::new();
        for index in 0..12 {
            files.insert(format!("item_{index:02}.txt"), json!(""));
        }
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!(files)).await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("item_");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            let filtered: Vec<_> = menu
                .filtered_entry_names()
                .iter()
                .map(|name| name.to_string())
                .collect();
            // Pinned rather than derived from `filtered`: the defect this guards reversed the
            // run, and a self-derived expectation cannot tell reversal from order.
            let expected: Vec<String> = (0..12)
                .map(|index| format!("item_{index:02}.txt"))
                .collect();
            assert_eq!(
                filtered, expected,
                "equal-scoring matches must keep the listing's own order"
            );
        });
    }

    #[gpui::test]
    async fn test_bar_keeps_the_file_trail_while_browsing_an_ancestor(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "src": { "a": { "file.rs": "fn main() {}" }, "b": { "other.rs": "" } } }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/a/file.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct BarHost {
            editor: Entity<Editor>,
        }
        impl gpui::Render for BarHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let file_name = vec![HighlightedText {
                    text: "file.rs".into(),
                    highlights: vec![],
                }];
                h_flex().size_full().child(render_breadcrumb_text(
                    file_name,
                    None,
                    None,
                    &self.editor,
                    false,
                    cx,
                ))
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| {
                let mut editor =
                    build_editor_with_project(project.clone(), multi_buffer, window, cx);
                editor.set_workspace_for_test(workspace.downgrade());
                editor
            });
            BarHost { editor }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Open on the file's own directory, then step out to the root the way Left does: a
        // navigated listing of an ancestor.
        host_window
            .update(cx, |host, window, cx| {
                host.editor.update(cx, |editor, cx| {
                    editor.open_breadcrumb_navigation(
                        BreadcrumbListing::Directory {
                            worktree_id,
                            path: rel_path("src/a").into_arc(),
                        },
                        window,
                        cx,
                    );
                });
                let menu = host
                    .editor
                    .read(cx)
                    .breadcrumb_navigation_menu()
                    .cloned()
                    .expect("menu opened");
                menu.update(cx, |menu, cx| {
                    menu.set_listing(
                        BreadcrumbListing::Directory {
                            worktree_id,
                            path: RelPath::empty().into_arc(),
                        },
                        Some(rel_path("src/a/file.rs").into_arc()),
                        true,
                        window,
                        cx,
                    );
                });
            })
            .unwrap();
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // root(0) › src(1) › a(2) › file.rs(3): the whole trail stays while the menu lists the
        // root, so the file segment is still painted and the root segment anchors the menu.
        assert!(
            cx.debug_bounds("breadcrumb-segment-3").is_some(),
            "browsing an ancestor must not drop the file's segments from the bar"
        );
        assert!(
            cx.debug_bounds("breadcrumb-segment-0").is_some(),
            "the ancestor being browsed is one of the painted segments"
        );
    }

    #[gpui::test]
    async fn test_multibuffer_bar_has_no_navigable_segment(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;

        init_test(cx, |_| {});

        // Two excerpts, so the editor is not a singleton and no segment can carry a target.
        // The toolbar bar is hidden entirely for multibuffers (`breadcrumb_location`), so the
        // outline fallback lives in the `OpenBreadcrumbNavigation` action, not in the bar.
        let multi_buffer = cx.update(|cx| {
            MultiBuffer::build_multi(
                [
                    ("fn one() {}\n", vec![text::Point::row_range(0..1)]),
                    ("fn two() {}\n", vec![text::Point::row_range(0..1)]),
                ],
                cx,
            )
        });

        struct BarHost {
            editor: Entity<Editor>,
        }
        impl gpui::Render for BarHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let placeholder = vec![HighlightedText {
                    text: "impl Widget".into(),
                    highlights: vec![],
                }];
                h_flex().size_full().child(render_breadcrumb_text(
                    placeholder,
                    None,
                    None,
                    &self.editor,
                    false,
                    cx,
                ))
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer, window, cx));
            BarHost { editor }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        assert_eq!(
            cx.debug_bounds("breadcrumb-segment-0"),
            None,
            "a multibuffer bar paints no navigable segment"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_click_opens_the_clicked_row(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "aaa.rs": "fn a() {}", "bbb.rs": "fn b() {}", "ccc.rs": "fn c() {}" }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/aaa.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let dismissed = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        let _sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let rows = menu.read_with(cx, |menu, _| menu.entry_names());
        assert_eq!(
            rows.iter().map(|name| name.as_ref()).collect::<Vec<_>>(),
            vec!["aaa.rs", "bbb.rs", "ccc.rs"],
        );

        // The picker selects and confirms in one call, so clicking the third row must open
        // the third row - not whichever row happened to be selected before.
        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let row_height = px(24.);
        let third_row = point(
            menu_bounds.left() + px(40.),
            menu_bounds.top() + px(46.) + row_height * 2.5,
        );
        cx.simulate_click(third_row, gpui::Modifiers::none());
        cx.run_until_parked();

        let opened = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.project_path(cx))
                .map(|path| path.path.as_unix_str().to_string())
        });
        assert_eq!(
            opened.as_deref(),
            Some("ccc.rs"),
            "clicking a row must open that row"
        );
    }

    #[gpui::test]
    async fn test_reopening_breadcrumb_navigation_refreshes_the_active_file(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Fs, Project};
        use serde_json::json;
        use std::path::Path;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        // Sorted, the renamed file does not land at index 0, so a stale active path and a
        // refreshed one pick different rows.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "a_zeta.rs": "fn z() {}", "b_alpha.rs": "fn a() {}" }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/b_alpha.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let editor_window = cx.add_window(|window, cx| {
            let mut editor = build_editor_with_project(project.clone(), multi_buffer, window, cx);
            editor.set_workspace_for_test(workspace.downgrade());
            editor
        });
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);
        cx.run_until_parked();

        let open_listing = |cx: &mut VisualTestContext| {
            editor.update_in(cx, |editor, window, cx| {
                editor.open_breadcrumb_navigation(
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: RelPath::empty().into_arc(),
                    },
                    window,
                    cx,
                );
            });
        };
        let selected_label = |cx: &mut VisualTestContext| {
            let menu =
                editor.read_with(cx, |editor, _| editor.breadcrumb_navigation_menu().cloned());
            menu.and_then(|menu| {
                menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                    let rows = menu.published_row_labels(cx);
                    menu.selected_index()
                        .and_then(|index| rows.get(index).cloned())
                })
            })
        };

        open_listing(cx);
        cx.run_until_parked();
        assert_eq!(
            selected_label(cx).as_deref(),
            Some("b_alpha.rs"),
            "the first open preselects the file the editor is showing"
        );

        // Save-as, in the shape the editor actually sees it: the buffer keeps its identity and
        // its project path changes underneath the open menu.
        fs.rename(
            Path::new(path!("/root/b_alpha.rs")),
            Path::new(path!("/root/c_gamma.rs")),
            fs::RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
                create_parents: false,
            },
        )
        .await
        .unwrap();
        cx.run_until_parked();

        open_listing(cx);
        cx.run_until_parked();
        assert_eq!(
            selected_label(cx).as_deref(),
            Some("c_gamma.rs"),
            "reopening onto an existing menu must refresh the path the selection resolves against"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_navigation_closes_when_the_workspace_is_gone(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.rs": "fn a() {}" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/alpha.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let editor_window = cx.add_window(|window, cx| {
            let mut editor = build_editor_with_project(project.clone(), multi_buffer, window, cx);
            editor.set_workspace_for_test(workspace.downgrade());
            editor
        });
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);
        cx.run_until_parked();

        editor.update_in(cx, |editor, window, cx| {
            editor.open_breadcrumb_navigation(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();
        editor.read_with(cx, |editor, _| {
            assert!(editor.breadcrumb_navigation_menu().is_some());
        });

        // Without a workspace the menu cannot open anything it lists, so leaving it up would
        // leave the user clicking rows that do nothing.
        editor.update(cx, |editor, _| editor.clear_workspace_for_test());
        editor.update_in(cx, |editor, window, cx| {
            editor.open_breadcrumb_navigation(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation_menu().is_none(),
                "an editor with no workspace must not leave a dead menu open"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_gives_up_a_listing_whose_ancestor_is_gone(
        cx: &mut TestAppContext,
    ) {
        breadcrumb_menu_gives_up_dead_listing(util::path!("/root/a"), cx).await;
    }

    // The listed directory's own removal names the listing itself, which used to classify as an
    // ordinary reload and repaint the dead path as "Empty directory".
    #[gpui::test]
    async fn test_breadcrumb_menu_gives_up_a_listing_that_is_itself_gone(cx: &mut TestAppContext) {
        breadcrumb_menu_gives_up_dead_listing(util::path!("/root/a/b/c"), cx).await;
    }

    async fn breadcrumb_menu_gives_up_dead_listing(removed: &str, cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Fs, Project};
        use serde_json::json;
        use std::path::Path;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "a": { "b": { "c": { "x.txt": "", "y.txt": "" } } } }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::new_test("a/b/c").into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        let dismissed = Rc::new(AtomicBool::new(false));
        let _subscription = menu.update(cx, |_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        // Control: a change inside the listing is an ordinary reload, not a reason to close.
        fs.insert_file(path!("/root/a/b/c/z.txt"), Vec::new()).await;
        cx.run_until_parked();
        assert!(
            !dismissed.load(Ordering::SeqCst),
            "a new sibling must not close the menu"
        );
        menu.read_with(cx, |menu, _| {
            assert!(
                menu.entry_names().iter().any(|name| name == "z.txt"),
                "the listing must pick the new entry up, got {:?}",
                menu.entry_names()
            );
        });

        // The update names the removed path, never the path the listing moved to.
        fs.remove_dir(
            Path::new(removed),
            fs::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "removing {removed} leaves the listing dead; it would otherwise sit there showing \
             Empty directory"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_gives_up_a_listing_replaced_by_a_file(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Fs, Project};
        use serde_json::json;
        use std::path::Path;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "a": { "b": { "c": { "x.txt": "", "y.txt": "" } } } }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::new_test("a/b/c").into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        let dismissed = Rc::new(AtomicBool::new(false));
        let _subscription = menu.update(cx, |_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        // Both mutations land in one scan, so the update the listing sees names a path that
        // still exists - as a file. Existence alone cannot reject it.
        fs.remove_dir(
            Path::new(path!("/root/a/b/c")),
            fs::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        fs.insert_file(path!("/root/a/b/c"), b"now a file".to_vec())
            .await;
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "a path that came back as a file cannot be listed; reloading would paint it as an \
             empty directory"
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_a_drill_gives_up_a_directory_deleted_mid_flight(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Fs, Project};
        use serde_json::json;
        use std::path::Path;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "enter",
                Confirm,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "gone": { "inner.txt": "" }, "stay.txt": "" }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        let dismissed = Rc::new(AtomicBool::new(false));
        let _subscription = menu.update(cx, |_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        // Select the directory row, then start the drill without letting anything land.
        cx.simulate_input("gone");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
        });

        // The removal names the drill's target while the listing being left is still the
        // current one, where it classifies as an ordinary child change - only the drill's own
        // check can notice its target is gone.
        fs.remove_dir(
            Path::new(path!("/root/gone")),
            fs::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "the drill's target is gone; installing its listing would paint a dead path as an \
             empty directory"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_rows_reload_on_a_git_change_below_their_children(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use git::status::{FileStatus, StatusCode};
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "sub": { "deep": { "file.txt": "x\n" }, "top.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        cx.read(|cx| worktree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::new_test("sub").into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();

        let reloads_before = menu.read_with(cx, |menu, _| menu.directory_reload_count_for_test());

        // The changed file is below the listing's immediate children, so the worktree
        // subscription's path check never fires for it; only the git store names it.
        fs.set_status_for_repo(
            path!("/root/.git").as_ref(),
            &[(
                "sub/deep/file.txt",
                FileStatus::worktree(StatusCode::Modified),
            )],
        );
        cx.run_until_parked();

        let reloads_after_status =
            menu.read_with(cx, |menu, _| menu.directory_reload_count_for_test());
        assert!(
            reloads_after_status > reloads_before,
            "a git change below the immediate children must reload the rows: their summaries \
             aggregate the whole subtree"
        );

        // An index-only change touches no worktree path at all.
        fs.set_index_for_repo(
            path!("/root/.git").as_ref(),
            &[("sub/deep/file.txt", "y\n".to_string())],
        );
        cx.run_until_parked();

        let reloads_after_index =
            menu.read_with(cx, |menu, _| menu.directory_reload_count_for_test());
        assert!(
            reloads_after_index > reloads_after_status,
            "an index-only change touches no path, so only the git subscription can reload for it"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_filtered_selection_survives_a_reload(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "down",
                SelectNext,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "aa.txt": "", "ab.txt": "", "ac.txt": "" }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let menu_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = menu_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*menu_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("a");
        cx.run_until_parked();
        cx.simulate_keystrokes("down");
        cx.run_until_parked();

        let arrowed = menu.read_with(cx, |menu, cx| {
            let rows = menu.published_row_labels(cx);
            menu.selected_index()
                .and_then(|index| rows.get(index).cloned())
        });
        assert_eq!(
            arrowed.as_deref(),
            Some("ab.txt"),
            "fixture must leave the arrow off the best match"
        );

        // A filesystem event rebuilds `directory_entries`, which invalidates every ranked
        // position the selection was expressed in.
        fs.insert_file(path!("/root/aa2.txt"), Vec::new()).await;
        cx.run_until_parked();

        let after_reload = menu.read_with(cx, |menu, cx| {
            let rows = menu.published_row_labels(cx);
            menu.selected_index()
                .and_then(|index| rows.get(index).cloned())
        });
        assert_eq!(
            after_reload.as_deref(),
            Some("ab.txt"),
            "a reload must not move the highlight off the row the user arrowed to"
        );

        // A second refresh lands while the first one's rank is still in flight. The first
        // latched the arrowed row and blanked the selection, so this one derives `None` - and
        // must leave the latch alone rather than carry that `None` in.
        menu.update(cx, |menu, cx| {
            menu.apply_reloaded_selection_for_test(
                Some(RelPath::new_test("ab.txt").into_arc()),
                cx,
            );
            assert_eq!(
                menu.selected_index(),
                None,
                "fixture: the first refresh must blank the selection the second one reads"
            );
            menu.apply_reloaded_selection_for_test(None, cx);
        });
        cx.run_until_parked();

        let after_second_refresh = menu.read_with(cx, |menu, cx| {
            let rows = menu.published_row_labels(cx);
            menu.selected_index()
                .and_then(|index| rows.get(index).cloned())
        });
        assert_eq!(
            after_second_refresh.as_deref(),
            Some("ab.txt"),
            "back-to-back refreshes must not drop the user's row to the top match"
        );
    }

    #[test]
    fn test_listing_path_impact() {
        use super::menu::{ListingPathImpact, listing_path_impact};

        let listing = RelPath::new_test("a/b/c");
        assert_eq!(
            listing_path_impact(&RelPath::new_test("a/b/c"), &listing),
            ListingPathImpact::Dead,
            "the listed directory itself: its removal names it, so the existence check has to run"
        );
        assert_eq!(
            listing_path_impact(&RelPath::new_test("a/b/c/x.txt"), &listing),
            ListingPathImpact::Reload,
            "a direct child"
        );
        assert_eq!(
            listing_path_impact(&RelPath::new_test("a"), &listing),
            ListingPathImpact::Dead,
            "an ancestor: the update names the old path, never the one it moved to"
        );
        assert_eq!(
            listing_path_impact(&RelPath::new_test("a/b"), &listing),
            ListingPathImpact::Dead,
            "the immediate parent is still an ancestor"
        );
        assert_eq!(
            listing_path_impact(&RelPath::new_test("a/b/c/x/y.txt"), &listing),
            ListingPathImpact::Ignore,
            "a grandchild does not change what this level lists"
        );
        assert_eq!(
            listing_path_impact(&RelPath::new_test("z"), &listing),
            ListingPathImpact::Ignore,
            "an unrelated path"
        );
        assert_eq!(
            listing_path_impact(RelPath::empty(), RelPath::empty()),
            ListingPathImpact::Dead,
            "the worktree root goes through the same existence check, which always finds it"
        );
    }

    #[test]
    fn test_arming_the_filter_barrier_keeps_the_outstanding_one() {
        use super::menu::FilterSettled;
        use futures::FutureExt as _;
        use postage::stream::Stream as _;

        let settled = FilterSettled::default();
        settled.arm_for_test();
        let mut receiver = settled
            .receiver()
            .expect("an armed barrier hands a receiver out");

        // A superseding rank arms again while that receiver is still awaited.
        settled.arm_for_test();
        assert!(
            receiver.recv().now_or_never().is_none(),
            "re-arming must not release a receiver already handed out: no rows have been \
             published for the replacement query yet"
        );

        settled.settle();
        assert!(
            receiver.recv().now_or_never().is_some(),
            "the settle that follows the publish must release it"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_filters_multibyte_names(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        // The filter runs on chars, not bytes: every name here is longer in bytes than in
        // characters, so a byte-indexed match would slice one mid-codepoint.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "日本語ファイル.rs": "fn a() {}",
                "中文文件.rs": "fn b() {}",
                "plain.rs": "fn c() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/plain.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("日本");
        cx.run_until_parked();

        let published = menu.read_with(cx, |menu, cx| menu.published_row_labels(cx));
        assert_eq!(
            published
                .iter()
                .map(|label| label.as_ref())
                .collect::<Vec<_>>(),
            vec!["日本語ファイル.rs"],
            "a multibyte query must narrow to the multibyte name that contains it"
        );

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filter(),
                "日本",
                "the query itself must survive the round trip through the picker"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_enter_waits_for_the_query_it_typed(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "enter",
                Confirm,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        // "aaa.rs" sorts first, so an Enter that acts on the unfiltered rows opens it, while
        // one that waits for the query opens the only name the query matches.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "aaa.rs": "fn a() {}",
                "zzz_unique.rs": "fn z() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/aaa.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // Dispatched without parking in between: `simulate_input` drains the executor, which
        // settles the rank and hides the race. Enter has to land while the query is still on
        // its way to the menu, which is the state a fast typist actually produces.
        for key in ["z", "z", "z", "enter"] {
            cx.update(|window, cx| {
                window.dispatch_keystroke(gpui::Keystroke::parse(key).unwrap(), cx);
            });
        }
        cx.run_until_parked();

        let opened = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.project_path(cx))
                .map(|path| path.path.as_unix_str().to_string())
        });
        assert_eq!(
            opened.as_deref(),
            Some("zzz_unique.rs"),
            "Enter must confirm against the query that was typed, not the rows it replaced"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_click_opens_the_row_the_picker_rendered(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        // Names chosen so a filter both narrows the listing and reorders it, which is what
        // makes the ranked-match space differ from the listing's own.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "zebra_match.rs": "fn z() {}",
                "alpha.rs": "fn a() {}",
                "match_beta.rs": "fn b() {}",
                "gamma.rs": "fn g() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/alpha.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                div().size_full().child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            editor.update(cx, |editor, _cx| {
                editor.set_workspace_for_test(workspace.downgrade());
            });
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("match");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // What the picker is actually painting, which under a filter is not the listing order.
        let published = menu.read_with(cx, |menu, cx| menu.published_row_labels(cx));
        assert!(
            published.len() >= 2,
            "filter should leave at least two rows, got {published:?}"
        );
        let listing = menu.read_with(cx, |menu, _| menu.entry_names());
        assert_ne!(
            published
                .iter()
                .map(|name| name.as_ref())
                .collect::<Vec<_>>(),
            listing.iter().map(|name| name.as_ref()).collect::<Vec<_>>(),
            "fixture must make the rendered rows differ from the listing order"
        );

        let second_row_label = published[1].to_string();
        let menu_bounds = cx
            .debug_bounds("breadcrumb-navigation-menu")
            .expect("menu should paint");
        let row_height = px(24.);
        let second_row = point(
            menu_bounds.left() + px(40.),
            menu_bounds.top() + px(46.) + row_height * 1.5,
        );
        cx.simulate_click(second_row, gpui::Modifiers::none());
        cx.run_until_parked();

        let opened = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.project_path(cx))
                .map(|path| path.path.as_unix_str().to_string())
        });
        assert_eq!(
            opened.as_deref(),
            Some(second_row_label.as_str()),
            "a confirm must open the row the picker rendered at that position"
        );
    }

    #[gpui::test]
    async fn test_symbol_menu_recovers_when_an_edit_deletes_the_selected_symbol(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::{Language, LanguageConfig};

        init_test(cx, |_| {});

        let language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query("(function_item name: (_) @name) @item")
            .expect("rust outline query"),
        );
        let buffer = cx
            .new(|cx| language::Buffer::local("fn alpha() {}\nfn beta() {}\nfn gamma() {}\n", cx));
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                WeakEntity::new_invalid(),
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: None,
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        fn published(
            menu: &Entity<BreadcrumbNavigationMenu>,
            cx: &mut VisualTestContext,
        ) -> Vec<String> {
            menu.read_with(cx, |menu, cx| {
                menu.published_row_labels(cx)
                    .iter()
                    .map(|label| label.to_string())
                    .collect()
            })
        }

        // Without this, a listing that never loaded would satisfy every assertion below.
        assert_eq!(
            published(&menu, cx),
            vec!["alpha", "beta", "gamma"],
            "the symbols listing must have loaded before the edit"
        );

        menu.update(cx, |menu, cx| menu.set_selected_row(1, cx));
        cx.run_until_parked();
        assert_eq!(
            menu.read_with(cx, |menu, _| menu.selected_index()),
            Some(1),
            "fixture must leave the highlight off the first row"
        );

        // Deleting the selected symbol is the branch the sibling test cannot reach: the latched
        // range no longer names any row, so the restore has to fall back rather than leave the
        // index pointing at whichever symbol slid into that slot.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(14..27, "")], None, cx);
        });
        cx.run_until_parked();

        // Rows carry their own outline item, so a listing that did not follow the edit would
        // let Enter act on a symbol the buffer no longer has.
        assert_eq!(
            published(&menu, cx),
            vec!["alpha", "gamma"],
            "an edit while the menu is open must republish the rows"
        );
        assert_eq!(
            menu.read_with(cx, |menu, cx| menu
                .selected_index()
                .and_then(|index| menu.published_row_labels(cx).get(index).cloned())),
            Some("alpha".into()),
            "with the selected symbol gone the highlight falls back to the cursor's symbol, \
             rather than staying on the index the deletion vacated"
        );
    }

    // A reload that lands while a query is active has its selection re-derived by the rank, so
    // it must still discard the latch - holding it back carries this level's row into whatever
    // level the user drills to next, where it names nothing and blanks the highlight.
    // Which side of `if children.is_empty()` runs is what the navigation docs promise: a symbol
    // segment lists its children, and falls back to its siblings only when it has none.
    #[gpui::test]
    async fn test_opening_on_a_symbol_lists_children_then_falls_back_to_siblings(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "struct Outer {}\nfn child_a() {}\nfn child_b() {}\nstruct Peer {}\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let snapshot = multi_buffer.read_with(cx, |multi_buffer, cx| multi_buffer.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item = |depth: usize, text: &str, range: Range<multi_buffer::Anchor>| OutlineItem {
            depth,
            range: range.clone(),
            selection_range: range.clone(),
            source_range_for_text: range,
            text: text.into(),
            highlight_ranges: vec![],
            name_ranges: vec![],
            body_range: None,
            annotation_range: None,
        };
        // Outer has two children; Peer has none, and its siblings are the two depth-0 items.
        let all_items = vec![
            item(0, "Outer", anchor_at(0)),
            item(1, "child_a", anchor_at(16)),
            item(1, "child_b", anchor_at(32)),
            item(0, "Peer", anchor_at(48)),
        ];
        let outer = all_items[0].clone();
        let peer = all_items[3].clone();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items.clone(),
                // Seeded empty so the listing under test is the one the menu computes.
                Vec::new(),
                Vec::new(),
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        let names = |cx: &mut VisualTestContext| {
            menu.read_with(cx, |menu, _| {
                menu.entry_names()
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<_>>()
            })
        };

        menu.update_in(cx, |menu, window, cx| {
            menu.set_listing(
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: Some(outer),
                },
                None,
                false,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            names(cx),
            vec!["child_a", "child_b"],
            "a symbol with children lists them"
        );

        menu.update_in(cx, |menu, window, cx| {
            menu.set_listing(
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: Some(peer),
                },
                None,
                false,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            names(cx),
            vec!["Outer", "Peer"],
            "a childless symbol falls back to its siblings"
        );
    }

    #[gpui::test]
    async fn test_a_filtered_reload_does_not_strand_the_symbol_latch(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::{Language, LanguageConfig};

        init_test(cx, |_| {});

        let language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query("(function_item name: (_) @name) @item")
            .expect("rust outline query"),
        );
        let buffer = cx
            .new(|cx| language::Buffer::local("fn alpha() {}\nfn beta() {}\nfn gamma() {}\n", cx));
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                WeakEntity::new_invalid(),
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: None,
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        menu.update(cx, |menu, cx| menu.set_selected_row(1, cx));
        cx.run_until_parked();
        menu.update(cx, |menu, cx| menu.set_filter_query_for_test("gamma", cx));
        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            let end = buffer.len();
            buffer.edit([(end..end, "fn delta() {}\n")], None, cx);
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert!(
                !menu.symbol_restore_pending(),
                "a reload under an active query has to consume the latch, not park it for the \
                 next listing"
            );
        });
    }

    #[gpui::test]
    async fn test_a_symbol_reload_blanks_the_filter_with_the_outline(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::{Language, LanguageConfig};

        init_test(cx, |_| {});

        let language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query("(function_item name: (_) @name) @item")
            .expect("rust outline query"),
        );
        let buffer = cx
            .new(|cx| language::Buffer::local("fn alpha() {}\nfn beta() {}\nfn gamma() {}\n", cx));
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                WeakEntity::new_invalid(),
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: None,
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        menu.update(cx, |menu, cx| menu.set_filter_query_for_test("a", cx));
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert!(
                menu.selected_index().is_some(),
                "fixture: the rank must put a selection up before the edit"
            );
        });

        buffer.update(cx, |buffer, cx| {
            let end = buffer.len();
            buffer.edit([(end..end, "fn delta() {}\n")], None, cx);
        });
        // Deliberately not parked: the reload is still in flight, and what the menu publishes
        // inside this window is what a click or a landing rank can act on. Candidates and
        // matches are outline indices, so rows and selection must go with the items they
        // indexed.
        menu.read_with(cx, |menu, cx| {
            assert_eq!(
                menu.published_row_labels(cx),
                Vec::<SharedString>::new(),
                "blanked items cannot keep published rows"
            );
            assert_eq!(
                menu.selected_index(),
                None,
                "the selection indexes the matches; it cannot outlive them"
            );
        });

        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert!(
                menu.selected_index().is_some(),
                "the reload has to rerank the query over the fresh outline"
            );
        });
    }

    #[gpui::test]
    async fn test_symbol_menu_selection_survives_an_edit_above_it(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::{Language, LanguageConfig};

        init_test(cx, |_| {});

        let language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query("(function_item name: (_) @name) @item")
            .expect("rust outline query"),
        );
        let buffer = cx
            .new(|cx| language::Buffer::local("fn alpha() {}\nfn beta() {}\nfn gamma() {}\n", cx));
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                WeakEntity::new_invalid(),
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: None,
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        let selected_label = |cx: &mut VisualTestContext| {
            menu.read_with(cx, |menu, cx| {
                menu.selected_index()
                    .and_then(|index| menu.published_row_labels(cx).get(index).cloned())
            })
        };

        menu.update(cx, |menu, cx| menu.set_selected_row(1, cx));
        cx.run_until_parked();
        assert_eq!(
            selected_label(cx),
            Some("beta".into()),
            "fixture must leave the highlight on the middle symbol"
        );

        // An edit above the selection renumbers every row below it. The selection is a row
        // index, so without an identity restore it stays on 1 and silently means `alpha`.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "fn aardvark() {}\n")], None, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            menu.read_with(cx, |menu, cx| menu
                .published_row_labels(cx)
                .iter()
                .map(|label| label.to_string())
                .collect::<Vec<_>>()),
            vec!["aardvark", "alpha", "beta", "gamma"],
            "the reload must pick up the symbol inserted above the listing"
        );
        assert_eq!(
            selected_label(cx),
            Some("beta".into()),
            "a reload must not move the highlight off the symbol the user selected"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_right_ignores_rows_a_pending_rank_will_replace(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        // "alpha_dir" is the only entry Right can drill into and the only one the first query
        // matches, so a Right that acts on the rows that query left behind moves the listing.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha_dir": { "inner_one.txt": "", "inner_two.txt": "" },
                "beta.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("alpha");
        cx.run_until_parked();
        menu.read_with(cx, |menu, cx| {
            assert!(!menu.rank_pending(), "the first query has to have settled");
            assert_eq!(
                menu.published_row_labels(cx)
                    .iter()
                    .map(|label| label.as_ref())
                    .collect::<Vec<_>>(),
                vec!["alpha_dir"],
                "leaving a directory row for Right to aim at"
            );
        });

        // The second query reaches the menu the way the picker's update task delivers it, and
        // the rank it starts runs on the background executor. Nothing from here to the Right
        // press may park: parking settles the rank and erases the state under test, which is
        // why the keystroke is dispatched raw instead of through `simulate_keystrokes`.
        menu.update(cx, |menu, cx| menu.set_filter_query_for_test("beta", cx));
        menu.read_with(cx, |menu, cx| {
            assert!(menu.rank_pending(), "the second query's rank is in flight");
            assert_eq!(
                menu.published_row_labels(cx)
                    .iter()
                    .map(|label| label.as_ref())
                    .collect::<Vec<_>>(),
                vec!["alpha_dir"],
                "and the rows still describe the query it replaced"
            );
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.update(|window, cx| {
            window.dispatch_keystroke(gpui::Keystroke::parse("right").unwrap(), cx);
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.listing(),
                &BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                "Right must not drill into a row the pending rank was about to drop"
            );
            assert_eq!(
                menu.filter(),
                "beta",
                "and declining leaves the typed query alone"
            );
        });
    }

    // A single-file worktree paints no directory segments, so stepping out of a symbols listing
    // would leave the menu with nothing to anchor to and it would dismiss itself.
    #[gpui::test]
    async fn test_step_out_of_symbols_stays_put_in_a_single_file_worktree(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::KeyBinding;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "left",
                SelectParent,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "solo.rs": "fn main() {}\nfn other() {}\n" }),
        )
        .await;
        // The worktree is the file itself, which is what makes `is_single_file` true.
        let project = Project::test(fs, [path!("/root/solo.rs").as_ref()], cx).await;
        cx.run_until_parked();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/solo.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |multi_buffer, cx| multi_buffer.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item = |text: &str, range: Range<multi_buffer::Anchor>| OutlineItem {
            depth: 0,
            range: range.clone(),
            selection_range: range.clone(),
            source_range_for_text: range,
            text: text.into(),
            highlight_ranges: vec![],
            name_ranges: vec![],
            body_range: None,
            annotation_range: None,
        };
        let all_items = vec![item("main", anchor_at(0)), item("other", anchor_at(14))];

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| {
                build_editor_with_project(project.clone(), multi_buffer.clone(), window, cx)
            });
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items.clone(),
                vec![0, 1],
                Vec::new(),
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_keystrokes("left");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert!(
                matches!(
                    menu.listing(),
                    BreadcrumbListing::Symbols { parent: None, .. }
                ),
                "stepping out of a single-file worktree has nowhere to go, got {:?}",
                menu.listing()
            );
        });
    }

    // The panel's icon settings reach the rows through two assignments in publish_rows_now that
    // only the pure helper below them was covering.
    #[gpui::test]
    async fn test_bar_file_icon_follows_the_tab_family(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use settings::SettingsStore;

        init_test(cx, |_| {});

        fn icon_with(tabs: bool, panel: bool, cx: &mut TestAppContext) -> Option<SharedString> {
            cx.update(|cx| {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings.tabs.get_or_insert_default().file_icons = Some(tabs);
                        settings.project_panel.get_or_insert_default().file_icons = Some(panel);
                    });
                });
                super::path::breadcrumb_file_icon(Some(&RelPath::new_test("main.rs")), cx)
            })
        }

        assert!(
            icon_with(true, false, cx).is_some(),
            "tabs.file_icons on must paint the bar's file icon even with the panel's off"
        );
        assert!(
            icon_with(false, true, cx).is_none(),
            "the bar describes the open file the way a tab does; the panel family belongs to \
             the menu rows"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_rows_follow_the_panel_icon_settings(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({ "sub": { "leaf.txt": "" }, "file.txt": "" }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        assert_eq!(
            menu.read_with(cx, |menu, cx| menu.published_icon_flags(cx)),
            Some((true, true)),
            "icons are on by default"
        );

        for (file_icons, folder_icons) in [(false, true), (true, false), (false, false)] {
            cx.update(|_, cx| {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    store.update_user_settings(cx, |settings| {
                        let panel = settings.project_panel.get_or_insert_default();
                        panel.file_icons = Some(file_icons);
                        panel.folder_indicator = Some(if folder_icons {
                            settings::FolderIndicator::Icon
                        } else {
                            settings::FolderIndicator::Chevron
                        });
                    });
                });
            });
            cx.run_until_parked();
            assert_eq!(
                menu.read_with(cx, |menu, cx| menu.published_icon_flags(cx)),
                Some((file_icons, folder_icons)),
                "each panel icon setting has to reach its own row flag"
            );
        }
    }

    // The drill is gated on a pending rank; a listing change bumps the filter epoch, so the two
    // epochs have to be resynced or the second Right is swallowed with nothing failing.
    #[gpui::test]
    async fn test_breadcrumb_menu_drills_twice_in_a_row(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use util::rel_path::rel_path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        // Each level needs a second child: a lone child directory is auto-folded through, and
        // the first Right would land two levels deep instead of one.
        fs.insert_tree(
            path!("/root"),
            json!({ "outer": { "inner": { "leaf.txt": "" }, "keep.txt": "" }, "z.txt": "" }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_keystrokes("right");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.listing(),
                &BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("outer/inner").into_arc(),
                },
                "a second drill must not be swallowed by a rank that is not running"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_keeps_query_when_drill_goes_nowhere(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "", "beta.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // A query that matches nothing leaves no row selected, so Right has nowhere to go; the
        // picker must not take that as its cue to clear what was typed.
        cx.simulate_input("zzz");
        cx.run_until_parked();
        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filter(),
                "zzz",
                "a drill that goes nowhere must leave the typed query alone"
            );
            assert_eq!(menu.filtered_entry_names(), Vec::<SharedString>::new());
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_clearing_filter_keeps_the_selected_entry(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new(
                    "down",
                    SelectNext,
                    Some("BreadcrumbNavigationMenu > Editor"),
                ),
                KeyBinding::new("backspace", crate::actions::Backspace, Some("Editor")),
            ]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "aa.txt": "",
                "bb.txt": "",
                "zeta_one.txt": "",
                "zeta_two.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("zeta");
        cx.run_until_parked();
        cx.simulate_keystrokes("down");
        cx.run_until_parked();

        let selected_while_filtering = menu.read_with(cx, |menu, _| {
            let names = menu.filtered_entry_names();
            assert_eq!(
                names.iter().map(|name| name.as_ref()).collect::<Vec<_>>(),
                vec!["zeta_one.txt", "zeta_two.txt"],
            );
            names[menu.selected_index().expect("a row is selected")].clone()
        });
        assert_eq!(selected_while_filtering.as_ref(), "zeta_two.txt");

        cx.simulate_keystrokes("backspace backspace backspace backspace");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(menu.filter(), "");
            let names = menu.entry_names();
            assert_eq!(
                names.iter().map(|name| name.as_ref()).collect::<Vec<_>>(),
                vec!["aa.txt", "bb.txt", "zeta_one.txt", "zeta_two.txt"],
            );
            assert_eq!(
                names[menu.selected_index().expect("a row stays selected")].clone(),
                selected_while_filtering,
                "clearing the query must keep the same entry selected, not the same row index"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_drill_from_filtered_row_selects_first_child(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "nested": {
                    "inner_a.txt": "",
                    "inner_b.txt": "",
                },
                "zebra.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("nest");
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filtered_entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["nested"],
            );
        });

        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.listing(),
                &BreadcrumbListing::Directory {
                    worktree_id,
                    path: rel_path("nested").into_arc(),
                },
            );
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["inner_a.txt", "inner_b.txt"],
            );
            assert_eq!(
                menu.selected_index(),
                Some(0),
                "drilling out of a filtered row must still select a row in the new listing"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_filter_arrows_drive_menu_not_text_cursor(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new(
                    "right",
                    SelectChild,
                    Some("BreadcrumbNavigationMenu > Editor"),
                ),
                KeyBinding::new(
                    "left",
                    SelectParent,
                    Some("BreadcrumbNavigationMenu > Editor"),
                ),
            ]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "main.rs": "fn main() {}",
                "lib.rs": "fn lib() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                Some(rel_path("main.rs").into_arc()),
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        cx.simulate_input("mai");
        cx.run_until_parked();
        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.filter(), "mai");
                assert_eq!(
                    menu.filtered_entry_names()
                        .iter()
                        .map(|n| n.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["main.rs"],
                );
            })
            .unwrap();

        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                match menu.listing() {
                    BreadcrumbListing::Symbols { parent, .. } => assert_eq!(parent, &None),
                    listing => panic!(
                        "right must run menu::SelectChild (drill), not editor cursor motion, \
                         got {listing:?}"
                    ),
                }
                // set_listing clears the filter; editor MoveRight would have left "mai".
                assert_eq!(
                    menu.filter(),
                    "",
                    "menu drill clears filter via set_listing; editor right would leave the query"
                );
            })
            .unwrap();

        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("left");
        cx.run_until_parked();

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                match menu.listing() {
                    BreadcrumbListing::Directory { path, .. } => assert_eq!(
                        path.as_unix_str(),
                        "",
                        "left steps out of the symbols and back to the file's directory"
                    ),
                    listing => panic!(
                        "left must run menu::SelectParent, not editor cursor motion, \
                         got {listing:?}"
                    ),
                }
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_breadcrumb_filter_does_not_shadow_alt_bindings(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "alt-b",
                ProbeAltB,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha.txt": "" }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let probe_fired = Rc::new(AtomicBool::new(false));
        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            probe_fired: Rc<AtomicBool>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                let probe_fired = self.probe_fired.clone();
                div()
                    .on_action(move |_: &ProbeAltB, _, _| {
                        probe_fired.store(true, Ordering::SeqCst);
                    })
                    .child(self.menu.clone())
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                None,
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                probe_fired: probe_fired.clone(),
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("alt-b");
        cx.run_until_parked();

        assert!(
            probe_fired.load(Ordering::SeqCst),
            "alt-modified bindings must reach the action path with the filter editor focused"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_menu_enter_opens_file_and_dismisses(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "start.rs": "fn start() {}",
                "other.rs": "fn other() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/start.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let cx = &mut VisualTestContext::from_window(*workspace_window, cx);
        let editor = cx.update(|window, cx| {
            cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx))
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        });
        cx.run_until_parked();

        let dismissed = Rc::new(AtomicBool::new(false));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        // The menu has to be mounted in the window we drive, or a keystroke reaches nothing.
        let host_window = cx.add_window(|window, cx| {
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                Some(rel_path("start.rs").into_arc()),
                false,
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.update(|_, cx| {
            cx.bind_keys([KeyBinding::new(
                "enter",
                Confirm,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });
        cx.run_until_parked();
        let dismiss_sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        cx.simulate_input("other");
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filtered_entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["other.rs"],
            );
        });

        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "confirm on a file must dismiss the menu"
        );
        drop(dismiss_sub);
        drop(menu);

        let active_path = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.project_path(cx))
                .map(|path| path.path.as_unix_str().to_string())
        });
        assert_eq!(
            active_path.as_deref(),
            Some("other.rs"),
            "Enter on a file row opens that file as the active item"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_respect_project_panel_git_status(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::items::entry_git_aware_label_color;
        use git::status::{FileStatus, StatusCode};
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "modified.txt": "new\n",
                "untracked.txt": "fresh\n",
                "clean.txt": "clean\n",
            }),
        )
        .await;
        fs.set_status_for_repo(
            path!("/root/.git").as_ref(),
            &[
                ("modified.txt", FileStatus::worktree(StatusCode::Modified)),
                ("untracked.txt", FileStatus::Untracked),
            ],
        );

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.read(|cx| worktree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });

        let modified = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "modified.txt")
            .expect("modified.txt listed");
        let untracked = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "untracked.txt")
            .expect("untracked.txt listed");

        assert_ne!(
            modified.git_summary,
            git::status::GitSummary::UNCHANGED,
            "modified file must carry a non-empty git summary"
        );
        assert_ne!(
            untracked.git_summary,
            git::status::GitSummary::UNCHANGED,
            "untracked file must carry a non-empty git summary"
        );
        assert_eq!(
            entry_git_aware_label_color(modified.git_summary, modified.is_ignored, false),
            Color::Modified,
        );
        assert_eq!(
            entry_git_aware_label_color(untracked.git_summary, untracked.is_ignored, false),
            Color::Created,
        );

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project_panel.get_or_insert_default().git_status = Some(false);
                });
            });
        });
        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(
                &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                RelPath::empty(),
            )
        });

        let modified_off = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "modified.txt")
            .expect("modified.txt listed");
        let untracked_off = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "untracked.txt")
            .expect("untracked.txt listed");

        assert_eq!(
            modified_off.git_summary,
            git::status::GitSummary::UNCHANGED,
            "project_panel.git_status=false must blank the listing's git summary"
        );
        assert_eq!(
            untracked_off.git_summary,
            git::status::GitSummary::UNCHANGED,
            "project_panel.git_status=false must blank the listing's git summary"
        );
        assert_eq!(
            entry_git_aware_label_color(modified_off.git_summary, modified_off.is_ignored, false),
            Color::Muted,
            "a blanked summary must render as a plain row"
        );
        assert_eq!(
            entry_git_aware_label_color(untracked_off.git_summary, untracked_off.is_ignored, false),
            Color::Muted,
            "a blanked summary must render as a plain row"
        );

        // The gate is a conjunction, so restore the panel's half and blank it from the git side
        // instead - once through each of the two settings the docs name.
        for (disable_git, enable_status) in [(Some(true), None), (Some(false), Some(false))] {
            cx.update(|cx| {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings.project_panel.get_or_insert_default().git_status = Some(true);
                        let git = settings
                            .git
                            .get_or_insert_default()
                            .enabled
                            .get_or_insert_default();
                        git.disable_git = disable_git;
                        git.enable_status = enable_status;
                    });
                });
            });
            let entries = cx.update(|cx| {
                breadcrumb_directory_entries(
                    &breadcrumb_directory_listing_inputs(&project, &worktree, cx),
                    RelPath::empty(),
                )
            });
            let modified_git_off = entries
                .iter()
                .find(|entry| entry.name.as_ref() == "modified.txt")
                .expect("modified.txt listed");
            assert_eq!(
                modified_git_off.git_summary,
                git::status::GitSummary::UNCHANGED,
                "git status tracking off must blank the summary even with the panel's own \
                 setting on: disable_git={disable_git:?} enable_status={enable_status:?}"
            );
        }
    }

    #[gpui::test]
    async fn test_clearing_a_cross_level_symbol_match_does_not_select_an_unrelated_row(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "struct Alpha;\nstruct Beta {\n  fn nested_needle() {}\n}\nstruct Gamma;\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item =
            |depth: usize, text: &str, range: std::ops::Range<multi_buffer::Anchor>| OutlineItem {
                depth,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: text.into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            };
        let gamma_range = anchor_at(60);
        let all_items = vec![
            item(0, "Alpha", anchor_at(0)),
            item(0, "Beta", anchor_at(14)),
            item(1, "nested_needle", anchor_at(30)),
            item(0, "Gamma", gamma_range.clone()),
        ];
        let buffer_id =
            multi_buffer.read_with(cx, |mb, cx| mb.as_singleton().unwrap().read(cx).remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        // Top level only; the cursor sits on Gamma, so that is the row the menu preselects.
        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items,
                vec![0, 1, 3],
                vec![gamma_range],
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
            menu.apply_initial_selection_for_test(cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        menu.read_with(cx, |menu, _| {
            assert_eq!(menu.selected_index(), Some(2), "Gamma starts selected");
        });

        cx.simulate_input("needle");
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filtered_entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["nested_needle"],
                "the only match lives below the browsed level"
            );
            assert_eq!(
                menu.selected_index(),
                Some(0),
                "and it is the highlighted row"
            );
        });

        // Clearing the query drops a match that has no row at this level. The leftover rank
        // position must not be reused as a row index - that silently highlights a symbol the
        // user never chose, and Enter would open it.
        // Emptied through the delegate's own entry point, which keeps the ranked matches until
        // the re-rank consumes them - that is the path a user takes backspacing a query away.
        menu.update(cx, |menu, cx| menu.set_filter_query_for_test("", cx));
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            assert_eq!(menu.filter(), "", "the query is gone");
            assert_eq!(
                menu.selected_index(),
                Some(2),
                "selection falls back to the cursor's symbol, not rank position 0 (Alpha)"
            );
        });
    }

    #[gpui::test]
    async fn test_symbol_filter_reaches_symbols_below_the_browsed_level(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "class Outer {\n  fn nested_needle() {}\n}\nclass Peer {}\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item =
            |depth: usize, text: &str, range: std::ops::Range<multi_buffer::Anchor>| OutlineItem {
                depth,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: text.into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            };
        let all_items = vec![
            item(0, "Outer", anchor_at(0)),
            item(1, "nested_needle", anchor_at(20)),
            item(0, "Peer", anchor_at(40)),
        ];
        let buffer_id =
            multi_buffer.read_with(cx, |mb, cx| mb.as_singleton().unwrap().read(cx).remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        // Browsing the top level only: `nested_needle` is not one of the listed rows.
        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items,
                vec![0, 2],
                Vec::new(),
                window,
                cx,
            );
            MenuHost { menu }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["Outer", "Peer"],
                "the browsed level lists only top-level symbols"
            );
        });

        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_input("needle");
        cx.run_until_parked();

        // Typing must reach a symbol nested below the browsed level, or the filter reports
        // "No matches" for something that plainly exists.
        menu.read_with(cx, |menu, _| {
            assert_eq!(
                menu.filtered_entry_names()
                    .iter()
                    .map(|name| name.as_ref())
                    .collect::<Vec<_>>(),
                vec!["nested_needle"],
            );
        });
    }

    #[gpui::test]
    async fn test_confirming_a_symbol_row_moves_the_cursor_and_refocuses_the_editor(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "enter",
                Confirm,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "class Outer {\n  fn child_a() {}\n  fn child_b() {}\n}\nclass Peer {}\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };
        let item =
            |depth: usize, text: &str, range: std::ops::Range<multi_buffer::Anchor>| OutlineItem {
                depth,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: text.into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            };
        let child_b_range = anchor_at(40);
        let all_items = vec![
            item(0, "Outer", anchor_at(0)),
            item(1, "child_a", anchor_at(20)),
            item(1, "child_b", child_b_range.clone()),
        ];
        let buffer_id =
            multi_buffer.read_with(cx, |mb, cx| mb.as_singleton().unwrap().read(cx).remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items,
                vec![1, 2],
                Vec::new(),
                window,
                cx,
            );
            MenuHost { menu, editor }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let editor = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.editor.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
            menu.set_selected_row(1, cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        // The whole point of the feature, and nothing covered it: confirming a symbol row must
        // move the cursor to that symbol and hand focus back to the editor.
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();

        let expected = child_b_range.start.to_offset(&snapshot);
        editor.read_with(cx, |editor, _| {
            let head = editor
                .selections
                .newest_anchor()
                .head()
                .to_offset(&snapshot);
            assert_eq!(
                head, expected,
                "confirming a symbol row should move the cursor to that symbol"
            );
        });
        let editor_focused = editor.update_in(cx, |editor, window, cx| {
            editor.focus_handle(cx).is_focused(window)
        });
        assert!(
            editor_focused,
            "focus should return to the editor after navigating"
        );
    }

    #[gpui::test]
    async fn test_symbol_nav_menu_preselect_and_left_right(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use gpui::KeyBinding;
        use language::OutlineItem;
        use multi_buffer::MultiBufferOffset;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new(
                    "right",
                    SelectChild,
                    Some("BreadcrumbNavigationMenu > Editor"),
                ),
                KeyBinding::new(
                    "left",
                    SelectParent,
                    Some("BreadcrumbNavigationMenu > Editor"),
                ),
            ]);
        });

        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "class Outer {\n  fn child_a() {}\n  fn child_b() {}\n}\nclass Peer {}\n",
                cx,
            )
        });
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let snapshot = multi_buffer.read_with(cx, |mb, cx| mb.snapshot(cx));
        let anchor_at = |offset: usize| {
            snapshot.anchor_before(MultiBufferOffset(offset))
                ..snapshot.anchor_before(MultiBufferOffset(offset + 1))
        };

        let outer_range = anchor_at(0);
        let child_a_range = anchor_at(20);
        let child_b_range = anchor_at(40);
        let peer_range = anchor_at(60);

        let item =
            |depth: usize, text: &str, range: std::ops::Range<multi_buffer::Anchor>| OutlineItem {
                depth,
                range: range.clone(),
                selection_range: range.clone(),
                source_range_for_text: range,
                text: text.into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            };

        let all_items = vec![
            item(0, "Outer", outer_range),
            item(1, "child_a", child_a_range),
            item(1, "child_b", child_b_range.clone()),
            item(0, "Peer", peer_range),
        ];
        let buffer_id =
            multi_buffer.read_with(cx, |mb, cx| mb.as_singleton().unwrap().read(cx).remote_id());

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items.clone(),
                vec![1, 2],
                vec![child_b_range.clone()],
                window,
                cx,
            );
            MenuHost { menu }
        });
        let mut visual_cx = VisualTestContext::from_window(*host_window, cx);
        host_window
            .update(&mut visual_cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        host_window
            .read_with(&visual_cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(
                    menu.selected_index(),
                    Some(1),
                    "preselects the current (checkmarked) symbol"
                );
                assert_eq!(
                    menu.entry_names()
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["child_a", "child_b"],
                );
            })
            .unwrap();

        let host_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let menu = BreadcrumbNavigationMenu::new_with_symbols_for_test(
                editor.downgrade(),
                buffer_id,
                all_items,
                vec![0, 3],
                vec![child_b_range],
                window,
                cx,
            );
            MenuHost { menu }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(menu.selected_index(), Some(0));
                assert_eq!(
                    menu.entry_names()
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["Outer", "Peer"],
                );
            })
            .unwrap();

        cx.simulate_keystrokes("right");
        cx.run_until_parked();
        host_window
            .update(cx, |host, _window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(
                    menu.entry_names()
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["child_a", "child_b"],
                    "Right enters the selected row's children"
                );
                assert_eq!(
                    menu.selected_index(),
                    Some(1),
                    "preselects current symbol after drilling in"
                );
            })
            .unwrap();

        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("left");
        cx.run_until_parked();
        host_window
            .update(cx, |host, _window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                assert_eq!(
                    menu.entry_names()
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["Outer", "Peer"],
                    "Left returns to the parent symbol's sibling listing"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_select_child_on_another_file_row_opens_it(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::atomic::{AtomicBool, Ordering};
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "main.rs": "fn main() {}",
                "lib.rs": "fn lib() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/main.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                Some(rel_path("main.rs").into_arc()),
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let menu = host_window
            .root(cx)
            .unwrap()
            .read_with(cx, |host, _| host.menu.clone());
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();

        // Move the selection off the open file and onto the other one.
        menu.update_in(cx, |menu, window, cx| {
            window.focus(&menu.focus_handle(cx), cx);
            let other = menu
                .entry_names()
                .iter()
                .position(|name| name.as_ref() == "lib.rs")
                .expect("lib.rs is listed");
            menu.set_selected_row(other, cx);
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        menu.read_with(cx, |menu, _| {
            let selected = menu.selected_index().expect("selection");
            assert_eq!(
                menu.entry_names().get(selected).map(|n| n.as_ref()),
                Some("lib.rs")
            );
        });

        let dismissed = Arc::new(AtomicBool::new(false));
        let dismiss_sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        // Right goes deeper; for a file that is the file itself, not a noop.
        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        assert!(
            dismissed.load(Ordering::SeqCst),
            "Right on another file row opens it and dismisses the menu"
        );
        drop(dismiss_sub);
        drop(menu);

        let active_path = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.project_path(cx))
                .map(|path| path.path.as_unix_str().to_string())
        });
        assert_eq!(active_path.as_deref(), Some("lib.rs"));
    }

    #[gpui::test]
    async fn test_select_child_on_open_file_row_opens_symbols(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor_with_project;
        use gpui::KeyBinding;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});
        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new(
                "right",
                SelectChild,
                Some("BreadcrumbNavigationMenu > Editor"),
            )]);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "main.rs": "fn main() {}",
                "lib.rs": "fn lib() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/main.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct MenuHost {
            menu: Entity<BreadcrumbNavigationMenu>,
            _editor: Entity<Editor>,
        }
        impl gpui::Render for MenuHost {
            fn render(
                &mut self,
                _window: &mut gpui::Window,
                _cx: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                self.menu.clone()
            }
        }

        let host_window = cx.add_window(|window, cx| {
            let editor =
                cx.new(|cx| build_editor_with_project(project.clone(), multi_buffer, window, cx));
            let menu = BreadcrumbNavigationMenu::new(
                editor.downgrade(),
                workspace.downgrade(),
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: RelPath::empty().into_arc(),
                },
                Some(rel_path("main.rs").into_arc()),
                false,
                window,
                cx,
            );
            MenuHost {
                menu,
                _editor: editor,
            }
        });
        let cx = &mut VisualTestContext::from_window(*host_window, cx);
        cx.run_until_parked();
        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });

        host_window
            .read_with(cx, |host, cx| {
                let menu = host.menu.read(cx);
                let selected = menu.selected_index().expect("selection");
                assert_eq!(
                    menu.entry_names().get(selected).map(|n| n.as_ref()),
                    Some("main.rs"),
                );
            })
            .unwrap();

        cx.simulate_keystrokes("right");
        cx.run_until_parked();

        host_window
            .read_with(cx, |host, cx| {
                let listing = host.menu.read(cx).listing().clone();
                match listing {
                    BreadcrumbListing::Symbols {
                        buffer_id: id,
                        parent: None,
                    } => assert_eq!(id, buffer_id),
                    other => {
                        panic!("Right on open file row opens top-level Symbols, got {other:?}")
                    }
                }
            })
            .unwrap();

        host_window
            .update(cx, |host, window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.set_listing(
                        BreadcrumbListing::Directory {
                            worktree_id,
                            path: RelPath::empty().into_arc(),
                        },
                        None,
                        false,
                        window,
                        cx,
                    );
                });
            })
            .unwrap();
        cx.run_until_parked();
        host_window
            .update(cx, |host, window, cx| {
                window.focus(&host.menu.focus_handle(cx), cx);
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                });
            })
            .unwrap();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_input("lib");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        cx.simulate_keystrokes("right");
        cx.run_until_parked();
        host_window
            .read_with(cx, |host, cx| {
                let listing = host.menu.read(cx).listing().clone();
                match listing {
                    BreadcrumbListing::Directory { path, .. } => {
                        assert_eq!(path.as_unix_str(), "");
                    }
                    other => {
                        panic!("Right on non-open file stays directory listing, got {other:?}")
                    }
                }
            })
            .unwrap();
    }
}
