//! Clickable breadcrumb path/symbol navigation.

mod layout;
mod menu;
mod outline;
mod path;

use super::*;
use layout::{
    BreadcrumbStrip, PreparedBreadcrumbSegment, align_symbol_segments,
    breadcrumb_path_is_navigable, classify_breadcrumb_segment_kinds, hard_cap_middle_segments,
};
pub(crate) use menu::{BreadcrumbListing, BreadcrumbNavigationMenu};
#[cfg(test)]
pub(crate) use outline::{
    child_outline_indices, sibling_outline_indices, top_level_outline_indices,
};
use path::{breadcrumb_file_icon, breadcrumb_path_segments};

#[derive(Clone, Debug)]
pub enum BreadcrumbSegmentTarget {
    Symbol {
        buffer_id: BufferId,
        item: Option<OutlineItem<Anchor>>,
    },
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
    },
}

fn target_matches_listing(
    target: Option<&BreadcrumbSegmentTarget>,
    listing: &BreadcrumbListing,
) -> bool {
    match (target, listing) {
        (
            Some(BreadcrumbSegmentTarget::Directory { worktree_id, path }),
            BreadcrumbListing::Directory {
                worktree_id: listing_worktree,
                path: listing_path,
            },
        ) => worktree_id == listing_worktree && path.as_ref() == listing_path.as_ref(),
        (
            Some(BreadcrumbSegmentTarget::Symbol { buffer_id, item }),
            BreadcrumbListing::Symbols {
                buffer_id: listing_buffer,
                parent,
            },
        ) => {
            buffer_id == listing_buffer
                && match (item, parent) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a.range == b.range,
                    _ => false,
                }
        }
        _ => false,
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
    // min_w_0 so the strip can shrink inside the toolbar flex and width-collapse truncates.
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
            file_icon = breadcrumb_file_icon(
                real_project_path
                    .as_ref()
                    .map(|project_path| project_path.path.as_ref()),
                cx,
            );
            file_git_status_color = breadcrumb_file_git_status_color(
                editor_ref
                    .project()
                    .zip(real_project_path.as_ref())
                    .and_then(|(project, project_path)| {
                        project.read(cx).project_path_git_status(project_path, cx)
                    }),
                cx,
            );
            let menu = editor_ref.breadcrumb_navigation_menu().cloned();
            let (navigated, navigated_path) = menu.as_ref().map_or((false, None), |menu| {
                let menu = menu.read(cx);
                (menu.is_navigated(), menu.navigated_path())
            });
            let menu_symbol_trail = menu
                .as_ref()
                .map(|menu| menu.read(cx).symbol_trail().to_vec());
            menu_listing = menu.as_ref().map(|menu| menu.read(cx).listing().clone());

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

            if is_navigable
                && !segments.is_empty()
                && let Some(project) = editor_ref.project()
            {
                let split = if let Some((worktree_id, path)) = navigated_path.as_ref() {
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
                    && let Some(worktree) = project
                        .read(cx)
                        .worktree_for_id(project_path.worktree_id, cx)
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
                    let replace_range = if navigated { 0..segments.len() } else { 0..1 };
                    segments.splice(replace_range, path_labels);
                    symbol_segments = path_targets;
                    path_split = true;
                    has_root_segment = true;
                }
            }

            // Even without a navigable path (single-file worktree, untitled buffer) the file
            // segment stays clickable and opens the symbols listing.
            if !path_split {
                symbol_segments.push(Some(BreadcrumbSegmentTarget::Symbol {
                    buffer_id,
                    item: None,
                }));
            }

            if !navigated {
                let cursor_chain = editor_ref
                    .outline_symbols_at_cursor
                    .as_ref()
                    .filter(|(id, _)| *id == buffer_id)
                    .map(|(_, ancestors)| ancestors.clone())
                    .unwrap_or_default();
                let trail = outline::resolve_bar_symbol_trail(cursor_chain, menu_symbol_trail);
                // Labels and targets both derive from the same trail: the incoming segment
                // texts are cursor-derived, so a menu-driven trail must replace them or the
                // two lists diverge and alignment blanks every target.
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
    let kinds =
        classify_breadcrumb_segment_kinds(segments.len(), file_segment_index, has_root_segment);
    let protected_index = menu_listing.as_ref().and_then(|listing| {
        symbol_segments
            .iter()
            .position(|target| target_matches_listing(target.as_ref(), listing))
    });
    let (segments, symbol_segments, kinds, file_segment_index) = hard_cap_middle_segments(
        segments,
        symbol_segments,
        kinds,
        file_segment_index,
        protected_index,
    );

    let apply_dirty_filename_style =
        !workspace::TabBarSettings::get_global(cx).show && active_item.is_dirty(cx);
    // Tab-bar-hidden toolbar already paints a file icon via `prefix`; suppress the segment's own.
    let show_file_segment_icon = prefix.is_none();

    let prepared_segments = segments
        .into_iter()
        .zip(symbol_segments)
        .zip(kinds)
        .enumerate()
        .map(|(index, ((label, target), kind))| {
            // Only the open file's own segment carries its icon and git color; a navigated
            // directory bar has no file segment. Dirty-bold only when that segment is the real file.
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

    let has_project_path = active_item.project_path(cx).is_some();

    match editor {
        Some(editor) => element
            .id("breadcrumb_container")
            .min_w_0()
            // overflow_hidden (not scroll): width-collapse plan truncates; strip must not paint
            // under toolbar buttons on narrow windows.
            .when(!multibuffer_header, |this| this.overflow_hidden())
            .child(
                ButtonLike::new("toggle outline view")
                    .child(breadcrumbs)
                    // Transparent so nested segment hover isn't drowned out by bar Subtle tint.
                    .style(ButtonStyle::Transparent)
                    .when(!multibuffer_header && has_project_path, |this| {
                        this.on_right_click({
                            let editor = editor.clone();
                            move |_, _, cx| {
                                if let Some(abs_path) = editor.upgrade().and_then(|editor| {
                                    editor.update(cx, |editor, cx| editor.target_file_abs_path(cx))
                                }) {
                                    if let Some(path_str) = abs_path.to_str() {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            path_str.to_string(),
                                        ));
                                    }
                                }
                            }
                        })
                    }),
            )
            .into_any_element(),
        None => element
            .h(rems_from_px(22_f32)) // Match the height and padding of the `ButtonLike` in the other arm.
            .pl_1()
            .child(breadcrumbs)
            .into_any_element(),
    }
}

/// File-segment git color, gated like tabs: `ItemSettings.git_status` (includes `git.enabled`).
pub(crate) fn breadcrumb_file_git_status_color(
    status: Option<git::status::FileStatus>,
    cx: &App,
) -> Option<Color> {
    if !workspace::ItemSettings::get_global(cx).git_status {
        return None;
    }
    status.map(|status| file_status_label_color(Some(status)))
}

#[cfg(test)]
mod tests {
    use super::layout::{
        BreadcrumbSegmentKind, align_symbol_segments, breadcrumb_path_is_navigable,
        classify_breadcrumb_segment_kinds, hard_cap_middle_segments, plan_breadcrumb_layout,
    };
    use super::menu::{BreadcrumbListing, BreadcrumbNavigationMenu};
    use super::outline::flatten_text_for_single_line_display;
    use super::path::{
        DirectoryEntryIconSource, MAX_BREADCRUMB_MENU_ROWS, MAX_UNARY_DIRECTORY_SKIP_DEPTH,
        breadcrumb_directory_entries, breadcrumb_path_prefixes, directory_entry_icon_source,
        single_child_directory,
    };
    use super::*;
    use crate::MultiBuffer;
    use ::menu::{Confirm, SelectChild, SelectNext, SelectParent};
    use gpui::{TestAppContext, VisualTestContext};
    use std::rc::Rc;

    #[test]
    fn test_breadcrumb_path_is_navigable() {
        assert!(!breadcrumb_path_is_navigable(false, None));
        assert!(!breadcrumb_path_is_navigable(false, Some(false)));
        assert!(!breadcrumb_path_is_navigable(true, Some(true)));
        assert!(breadcrumb_path_is_navigable(true, Some(false)));
        assert!(breadcrumb_path_is_navigable(true, None));
    }

    #[test]
    fn test_flatten_text_for_single_line_display_preserves_byte_offsets() {
        // Newline and space are both 1-byte UTF-8, so highlight offsets stay valid.
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
        // struct A; struct B; struct C; — all depth 0, no parent.
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
        // Parent is nearest shallower, not depth-1 (tree-sitter can jump 0 → 2).
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
        // struct A; struct B; struct C; — all depth 0, none has children.
        let depths = [0, 0, 0];
        assert_eq!(child_outline_indices(&depths, 0), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_nested() {
        // impl A {         // 0
        //     fn one() {}  // 1
        //     fn two() {}  // 1
        // }
        // impl B {         // 0
        //     fn three() {}// 1
        // }
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), vec![4]);
        // Leaf items have no children.
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 4), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_uneven_depths() {
        // Depth-2 fields are still children of the depth-0 parent.
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

        // Navigated bar: every segment is a directory target (no file-level Symbol).
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

    /// The bar's symbol chain and the menu trail must never desynchronize: prefix trails keep
    /// the deeper cursor chain, divergent trails replace it (labels and targets both).
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

        // No menu: cursor chain as-is.
        let resolved = resolve_bar_symbol_trail(vec![outer.clone(), child.clone()], None);
        assert_eq!(texts(&resolved), vec!["item-0", "item-1"]);

        // Trail is a prefix of the cursor chain: keep the deeper chain.
        let resolved = resolve_bar_symbol_trail(
            vec![outer.clone(), child.clone()],
            Some(vec![outer.clone()]),
        );
        assert_eq!(texts(&resolved), vec!["item-0", "item-1"]);

        // Divergent trail (same length or not): the menu wins.
        let resolved =
            resolve_bar_symbol_trail(vec![outer.clone(), child], Some(vec![peer.clone()]));
        assert_eq!(texts(&resolved), vec!["item-2"]);
        let resolved = resolve_bar_symbol_trail(vec![outer], Some(vec![peer]));
        assert_eq!(texts(&resolved), vec!["item-2"]);
    }

    /// Segment re-click toggles closed; Escape then re-click reopens; different segment switches listing.
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
        editor.update(cx, |editor, cx| {
            editor.set_workspace_for_test(workspace.downgrade(), cx);
        });

        let root_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: RelPath::empty().into(),
        };
        let src_listing = BreadcrumbListing::Directory {
            worktree_id,
            path: rel_path("src").into_arc(),
        };

        // Open, re-click same listing → toggle closed.
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

        // Escape-style dismiss then re-click reopens immediately.
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

        // Different segment switches listing in place (menu stays open).
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

    /// set_listing under an editor.update lease must not double-lease panic.
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
        editor.update(cx, |editor, cx| {
            editor.set_workspace_for_test(workspace.downgrade(), cx);
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

        // Nested open_or_toggle (menu reuse → set_listing) while already under editor.update.
        editor.update_in(cx, |editor, window, cx| {
            editor.open_or_toggle_breadcrumb_listing_for_test(src_listing.clone(), window, cx);
            let menu = editor
                .breadcrumb_navigation_menu()
                .expect("menu remains after switch");
            assert_eq!(menu.read(cx).listing(), &src_listing);
        });
    }

    /// Toggling breadcrumbs off while a menu is open dismisses the menu.
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
        editor.update(cx, |editor, cx| {
            editor.set_workspace_for_test(workspace.downgrade(), cx);
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
    fn test_breadcrumb_file_git_status_color_respects_tabs_git_status(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use git::status::{FileStatus, StatusCode};
        use gpui::UpdateGlobal;
        use settings::SettingsStore;

        init_test(cx, |_| {});

        let status = FileStatus::worktree(StatusCode::Modified);

        // Force the enabled path, then the disabled path (mirrors menu-row gating tests).
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
        let color_on = cx.update(|cx| breadcrumb_file_git_status_color(Some(status), cx));
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
        let color_off = cx.update(|cx| breadcrumb_file_git_status_color(Some(status), cx));
        assert_eq!(
            color_off, None,
            "tabs.git_status=false must suppress file segment git color"
        );
    }

    /// The production drill must cap runaway single-child chains (symlink cycles).
    #[gpui::test]
    async fn test_directory_drill_caps_unary_chain_depth(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use util::{path, rel_path::rel_path};
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
        let entry_id = project.update(cx, |project, cx| {
            project
                .worktree_for_id(worktree_id, cx)
                .unwrap()
                .read(cx)
                .entry_for_path(rel_path("d"))
                .unwrap()
                .id
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
        menu.update_in(cx, |menu, window, cx| {
            menu.choose_directory_entry_for_test(
                rel_path("d").into_arc(),
                entry_id,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let (_, landed) = menu
            .read_with(cx, |menu, _| menu.navigated_path())
            .expect("drill lands on a navigated directory");
        // Started at depth 1 and may take at most MAX_UNARY_DIRECTORY_SKIP_DEPTH further steps.
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
        // Shorter than `segments` (e.g. navigated worktree failed to resolve).
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
        let kinds = classify_breadcrumb_segment_kinds(6, 3, true);
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

        let kinds = classify_breadcrumb_segment_kinds(3, 1, false);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        let kinds = classify_breadcrumb_segment_kinds(1, 0, false);
        assert_eq!(kinds, vec![BreadcrumbSegmentKind::File]);
    }

    /// Align before hard-cap: divergent lengths would panic on splice.
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

        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 99, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_middle_segments(segments, symbol_segments, kinds, 99, None);

        // Root + 32 + ⋯ + 32 + file = 67.
        assert_eq!(segments.len(), 67);
        {
            let segments: Vec<HighlightedText> = (0..100)
                .map(|i| HighlightedText {
                    text: format!("segment-{i}").into(),
                    highlights: vec![],
                })
                .collect();
            let symbol_segments = vec![None; segments.len()];
            let kinds = classify_breadcrumb_segment_kinds(segments.len(), 99, true);
            let (capped, _, _, _) =
                hard_cap_middle_segments(segments, symbol_segments, kinds, 99, Some(50));
            assert!(
                capped
                    .iter()
                    .any(|segment| segment.text.as_ref() == "segment-50"),
                "the protected menu-anchor segment survives the hard cap",
            );
        }
        assert_eq!(symbol_segments.len(), segments.len());
        assert_eq!(kinds.len(), segments.len());
        assert_eq!(file_segment_index, 66);
        assert_eq!(kinds[file_segment_index], BreadcrumbSegmentKind::File);
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
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 3, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_middle_segments(segments, symbol_segments, kinds, 3, None);

        assert_eq!(segments.len(), 6);
        assert_eq!(symbol_segments.len(), 6);
        assert_eq!(kinds.len(), 6);
        assert_eq!(file_segment_index, 3);
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

    /// Active directory segment (re-anchor target) must stay visible at every width, even when
    /// it is a Middle kind that would otherwise be the first to drop.
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

    /// Choosing a directory updates the listing in place on the same menu entity (no re-anchor).
    #[gpui::test]
    async fn test_choosing_breadcrumb_directory_updates_listing_in_place(cx: &mut TestAppContext) {
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

        let dir_a_id = project.update(cx, |project, cx| {
            project
                .worktree_for_id(worktree_id, cx)
                .unwrap()
                .read(cx)
                .entry_for_path(rel_path("dir_a"))
                .unwrap()
                .id
        });
        menu.update_in(cx, |menu, window, cx| {
            menu.choose_directory_entry_for_test(
                rel_path("dir_a").into_arc(),
                dir_a_id,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => {
                    panic!("listing stays on the same entity and moves to dir_a, got {other:?}")
                }
            };
            assert_eq!(path, "dir_a");
            assert!(menu.is_navigated());
            assert_eq!(
                menu.entry_names()
                    .iter()
                    .map(|n| n.as_ref())
                    .collect::<Vec<_>>(),
                vec!["child1.txt", "child2.txt"],
            );
        });
    }

    /// Directory choose skips single-child chains before landing.
    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_descends_single_child_dirs(
        cx: &mut TestAppContext,
    ) {
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
                "a": { "b": { "c.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let entry_a_id = project.update(cx, |project, cx| {
            project
                .worktree_for_id(worktree_id, cx)
                .unwrap()
                .read(cx)
                .entry_for_path(rel_path("a"))
                .unwrap()
                .id
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
        menu.update_in(cx, |menu, window, cx| {
            menu.choose_directory_entry_for_test(
                rel_path("a").into_arc(),
                entry_a_id,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "a/b");
        });
    }

    /// With auto_fold_dirs off, drill lands on the chosen directory (no unary skip).
    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_respects_auto_fold_dirs_off(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::{path, rel_path::rel_path};
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
        let entry_a_id = project.update(cx, |project, cx| {
            project
                .worktree_for_id(worktree_id, cx)
                .unwrap()
                .read(cx)
                .entry_for_path(rel_path("a"))
                .unwrap()
                .id
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
        menu.update_in(cx, |menu, window, cx| {
            menu.choose_directory_entry_for_test(
                rel_path("a").into_arc(),
                entry_a_id,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "a");
        });
    }

    /// Path menu preselects the open file (or its ancestor directory) on first listing.
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
            breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx)
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
        // Loading completion applies initial selection; no render needed.
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

    /// Choosing a gitignored directory expands it so its children appear.
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

        let root_entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        let ignored = root_entries
            .iter()
            .find(|entry| entry.name.as_ref() == "ignored_dir")
            .expect("gitignored dir is listed when hide_gitignore is false");
        assert!(ignored.is_ignored);
        let ignored_id = ignored.entry_id;
        let ignored_path = ignored.path.clone();

        let before = cx.update(|cx| {
            breadcrumb_directory_entries(&project, &worktree, rel_path("ignored_dir"), cx)
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
        menu.update_in(cx, |menu, window, cx| {
            menu.choose_directory_entry_for_test(ignored_path, ignored_id, true, window, cx);
        });
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            let path = match menu.listing() {
                BreadcrumbListing::Directory { path, .. } => path.as_unix_str().to_string(),
                other => panic!("expected directory listing, got {other:?}"),
            };
            assert_eq!(path, "ignored_dir/nested");
        });

        let after = cx.update(|_window, cx| {
            breadcrumb_directory_entries(&project, &worktree, rel_path("ignored_dir"), cx)
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

        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
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
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
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

        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
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
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
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

        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
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
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["kept.txt"],
            "hide_hidden should drop the hidden entry entirely"
        );
    }

    /// Width collapse protects the open menu's anchor segment.
    #[test]
    fn test_plan_breadcrumb_layout_protects_active_menu_segment() {
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

    /// Filter ranks the full directory set, not just the display-capped prefix.
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
        cx.simulate_keystrokes("z z");
        cx.run_until_parked();

        menu.read_with(cx, |menu, _| {
            assert_eq!(menu.filter(), "zz");
            assert_eq!(menu.filtered_entry_names(), Vec::<SharedString>::new());
            assert!(
                !menu.rank_pending(),
                "a completed zero-match rank must settle out of the searching state"
            );
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

        // Flipping the panel's sort settings must re-derive the open listing.
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
                assert!(
                    !menu
                        .filtered_entry_names()
                        .iter()
                        .any(|name| name.as_ref() == "zzz_target.txt"),
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
            let _ = window.draw(cx);
        });
        cx.simulate_keystrokes("z z z");
        cx.run_until_parked();
        cx.update(|window, cx| {
            let _ = window.draw(cx);
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

    /// Typing in the filter editor narrows rows (async fuzzy rank).
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
            let _ = window.draw(cx);
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
            let _ = window.draw(cx);
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
            .update(cx, |host, _window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.clear_filter_for_test(cx);
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

    /// Confirming a selected file row opens that file and dismisses the menu.
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
        let menu = cx.update(|window, cx| {
            BreadcrumbNavigationMenu::new(
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
            )
        });
        cx.run_until_parked();
        let dismiss_sub = cx.update(|_, cx| {
            let dismissed = dismissed.clone();
            cx.subscribe(&menu, move |_, _: &DismissEvent, _| {
                dismissed.store(true, Ordering::SeqCst);
            })
        });

        menu.update_in(cx, |menu, window, cx| {
            menu.apply_initial_selection_for_test(cx);
            for _ in 0..8 {
                if menu
                    .selected_index()
                    .and_then(|i| {
                        menu.filtered_entry_names()
                            .get(i)
                            .map(|n| n.as_ref() == "other.rs")
                    })
                    .unwrap_or(false)
                {
                    break;
                }
                menu.select_next(&SelectNext, window, cx);
            }
            assert_eq!(
                menu.filtered_entry_names()
                    .get(menu.selected_index().expect("selection after render"))
                    .map(|n| n.as_ref()),
                Some("other.rs"),
            );
            menu.confirm(&Confirm, window, cx);
        });
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

    /// breadcrumb_directory_entries carries real git summaries so labels color Modified / Created.
    #[gpui::test]
    async fn test_breadcrumb_directory_entries_git_status_colors(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::items::entry_git_aware_label_color;
        use git::status::{FileStatus, StatusCode};
        use project::{FakeFs, Project};
        use serde_json::json;
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

        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));

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
    }

    /// Symbol menu preselects the current symbol; Right enters children; Left returns.
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
                KeyBinding::new("right", SelectChild, Some("menu")),
                KeyBinding::new("left", SelectParent, Some("menu")),
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

        // Preselect on a nested listing.
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
            let _ = window.draw(cx);
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
            let _ = window.draw(cx);
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

    /// Right on the open file's row in a directory listing crosses into Symbols for that buffer.
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
            cx.bind_keys([KeyBinding::new("right", SelectChild, Some("menu"))]);
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
            // Keep the editor alive for the menu's WeakEntity.
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
            let _ = window.draw(cx);
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

        // Bound Right → SelectChild (also exercised via direct action; keystroke path is the same handler).
        host_window
            .update(cx, |host, window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.select_child(&SelectChild, window, cx);
                });
            })
            .unwrap();
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

        // Other file rows stay no-ops for Right.
        host_window
            .update(cx, |host, window, cx| {
                host.menu.update(cx, |menu, cx| {
                    menu.set_listing(
                        BreadcrumbListing::Directory {
                            worktree_id,
                            path: RelPath::empty().into_arc(),
                        },
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
                host.menu.update(cx, |menu, cx| {
                    menu.apply_initial_selection_for_test(cx);
                    for _ in 0..4 {
                        let selected = menu.selected_index().unwrap_or(0);
                        if menu
                            .entry_names()
                            .get(selected)
                            .is_some_and(|n| n.as_ref() == "lib.rs")
                        {
                            break;
                        }
                        menu.select_next(&SelectNext, window, cx);
                    }
                    // Drive SelectChild via the same bound path after selection lands on lib.rs.
                    menu.select_child(&SelectChild, window, cx);
                });
            })
            .unwrap();
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
