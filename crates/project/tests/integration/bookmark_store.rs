use std::{path::Path, sync::Arc};

use collections::BTreeMap;
use futures::{FutureExt, StreamExt};
use gpui::{Entity, TestAppContext};
use language::Buffer;
use project::{
    Project,
    bookmark_store::{BookmarkStore, SerializedBookmark, SerializedNumberedBookmark},
};
use serde_json::json;
use util::{path, rel_path::rel_path};

mod integration {
    use super::*;
    use fs::Fs as _;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
    }

    fn project_path(path: &str) -> Arc<Path> {
        Arc::from(Path::new(path))
    }

    fn serialized_bookmark(row: u32) -> SerializedBookmark {
        SerializedBookmark {
            row,
            label: String::new(),
        }
    }

    async fn open_buffer(
        project: &Entity<Project>,
        path: &str,
        cx: &mut TestAppContext,
    ) -> Entity<Buffer> {
        project
            .update(cx, |project, cx| {
                project.open_local_buffer(Path::new(path), cx)
            })
            .await
            .unwrap()
    }

    fn add_bookmarks(
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        rows: &[u32],
        cx: &mut TestAppContext,
    ) {
        let buffer = buffer.clone();
        project.update(cx, |project, cx| {
            let bookmark_store = project.bookmark_store();
            let snapshot = buffer.read(cx).snapshot();
            for &row in rows {
                let anchor = snapshot.anchor_after(text::Point::new(row, 0));
                bookmark_store.update(cx, |store, cx| {
                    store.toggle_bookmark(buffer.clone(), anchor, String::new(), cx);
                });
            }
        });
    }

    fn add_labeled_bookmark(
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        row: u32,
        label: &str,
        cx: &mut TestAppContext,
    ) {
        let buffer = buffer.clone();
        project.update(cx, |project, cx| {
            let bookmark_store = project.bookmark_store();
            let snapshot = buffer.read(cx).snapshot();
            let anchor = snapshot.anchor_after(text::Point::new(row, 0));
            bookmark_store.update(cx, |store, cx| {
                store.toggle_bookmark(buffer.clone(), anchor, label.to_string(), cx);
            });
        });
    }

    fn get_all_bookmarks(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        project.read_with(cx, |project, cx| {
            project
                .bookmark_store()
                .read(cx)
                .all_serialized_bookmarks(cx)
        })
    }

    fn build_serialized(
        entries: &[(&str, &[u32])],
    ) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        let mut map = BTreeMap::new();
        for &(path_str, rows) in entries {
            let path = project_path(path_str);
            map.insert(
                path,
                rows.iter().map(|&row| serialized_bookmark(row)).collect(),
            );
        }
        map
    }

    async fn restore_bookmarks(
        project: &Entity<Project>,
        serialized: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        cx: &mut TestAppContext,
    ) {
        project
            .update(cx, |project, cx| {
                project.bookmark_store().update(cx, |store, cx| {
                    store.load_serialized_bookmarks(serialized, cx)
                })
            })
            .await
            .expect("with_serialized_bookmarks should succeed");
    }

    fn clear_bookmarks(project: &Entity<Project>, cx: &mut TestAppContext) {
        project.update(cx, |project, cx| {
            project.bookmark_store().update(cx, |store, cx| {
                store.clear_bookmarks(cx);
            });
        });
    }

    fn assert_bookmark_rows(
        bookmarks: &BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        path: &str,
        expected_rows: &[u32],
    ) {
        let path = project_path(path);
        let file_bookmarks = bookmarks
            .get(&path)
            .unwrap_or_else(|| panic!("Expected bookmarks for {}", path.display()));
        let rows: Vec<u32> = file_bookmarks.iter().map(|b| b.row).collect();
        assert_eq!(rows, expected_rows, "Bookmark rows for {}", path.display());
    }

    fn assert_bookmark_labels(
        bookmarks: &BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        path: &str,
        expected: &[(u32, &str)],
    ) {
        let path = project_path(path);
        let file_bookmarks = bookmarks
            .get(&path)
            .unwrap_or_else(|| panic!("Expected bookmarks for {}", path.display()));
        let actual: Vec<_> = file_bookmarks
            .iter()
            .map(|bookmark| (bookmark.row, bookmark.label.as_str()))
            .collect();
        assert_eq!(actual, expected, "Bookmark labels for {}", path.display());
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_empty(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"file1.rs": "line1\nline2\n"}))
            .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_single_file(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\nline3\nline4\nline5\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[0, 2], cx);

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_eq!(bookmarks.len(), 1);
        assert_bookmark_rows(&bookmarks, path!("/project/file1.rs"), &[0, 2]);
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_includes_labels(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\nline3\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;

        add_labeled_bookmark(&project, &buffer, 0, "first", cx);
        add_labeled_bookmark(&project, &buffer, 2, "  keeps inner spaces  ", cx);

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_bookmark_labels(
            &bookmarks,
            path!("/project/file1.rs"),
            &[(0, "first"), (2, "  keeps inner spaces  ")],
        );
    }

    #[gpui::test]
    async fn test_bookmark_operations_ignore_other_buffers(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"sub": {"file.rs": "first\nsecond\nthird\n"}}),
        )
        .await;

        let project = Project::test(fs, [path!("/project/sub").as_ref()], cx).await;
        let stored_buffer = open_buffer(&project, path!("/project/sub/file.rs"), cx).await;

        add_labeled_bookmark(&project, &stored_buffer, 1, "original", cx);
        let original_bookmarks = get_all_bookmarks(&project, cx);

        // Overlapping worktrees can create distinct buffers for the same absolute path.
        let parent_worktree = project
            .update(cx, |project, cx| {
                project.create_worktree(path!("/project"), true, cx)
            })
            .await
            .expect("parent worktree should open");
        let other_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((parent_worktree.read(cx).id(), rel_path("sub/file.rs")), cx)
            })
            .await
            .expect("the same file should open through the parent worktree");
        assert_ne!(stored_buffer, other_buffer);
        cx.update(|cx| {
            let expected_path = Some(project_path(path!("/project/sub/file.rs")));
            assert_eq!(
                BookmarkStore::abs_path_from_buffer(&stored_buffer, cx),
                expected_path
            );
            assert_eq!(
                BookmarkStore::abs_path_from_buffer(&other_buffer, cx),
                expected_path
            );
        });

        // Bookmarks remain bound to the original buffer. Requests from another buffer
        // for the same path must be ignored to avoid mixing anchors from different buffers.
        let bookmark_store = project.read_with(cx, |project, _| project.bookmark_store());
        let mut events = cx.events(&bookmark_store);
        let other_snapshot = other_buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        for row in [1, 2] {
            let anchor = other_snapshot.anchor_after(text::Point::new(row, 0));
            bookmark_store.update(cx, |store, cx| {
                assert!(store.find_bookmark(&other_buffer, anchor, cx).is_none());
                store.edit_bookmark(&other_buffer, anchor, "ignored edit".into(), cx);
                store.toggle_bookmark(other_buffer.clone(), anchor, "ignored toggle".into(), cx);
            });
            assert_eq!(get_all_bookmarks(&project, cx), original_bookmarks);
        }
        bookmark_store.update(cx, |store, cx| {
            assert!(
                store
                    .bookmarks_for_buffer(
                        other_buffer.clone(),
                        other_snapshot.anchor_before(0)
                            ..other_snapshot.anchor_after(other_snapshot.len()),
                        &other_snapshot,
                        cx,
                    )
                    .is_empty()
            );
        });
        assert!(events.next().now_or_never().is_none());

        let bookmarks = BookmarkStore::all_bookmarks(&bookmark_store, cx)
            .await
            .expect("bookmarks should remain readable");
        let [bookmark] = bookmarks.as_slice() else {
            panic!("the original bookmark should be preserved");
        };
        assert_eq!(bookmark.buffer, stored_buffer);
        assert_eq!(bookmark.label, "original");
        let stored_anchor = bookmark.anchor;
        bookmark_store.update(cx, |store, cx| {
            assert_eq!(
                store
                    .find_bookmark(&stored_buffer, stored_anchor, cx)
                    .expect("the original buffer should still find its bookmark")
                    .label,
                "original",
            );
            store.edit_bookmark(&stored_buffer, stored_anchor, "updated".into(), cx);
        });
        assert_bookmark_labels(
            &get_all_bookmarks(&project, cx),
            path!("/project/sub/file.rs"),
            &[(1, "updated")],
        );
        bookmark_store.update(cx, |store, cx| {
            store.toggle_bookmark(stored_buffer.clone(), stored_anchor, String::new(), cx);
        });
        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_multiple_files(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "line1\nline2\nline3\n",
                "file2.rs": "lineA\nlineB\nlineC\nlineD\n",
                "file3.rs": "single line"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer1 = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        let buffer2 = open_buffer(&project, path!("/project/file2.rs"), cx).await;
        let _buffer3 = open_buffer(&project, path!("/project/file3.rs"), cx).await;

        add_bookmarks(&project, &buffer1, &[1], cx);
        add_bookmarks(&project, &buffer2, &[0, 3], cx);

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_eq!(bookmarks.len(), 2);
        assert_bookmark_rows(&bookmarks, path!("/project/file1.rs"), &[1]);
        assert_bookmark_rows(&bookmarks, path!("/project/file2.rs"), &[0, 3]);
        assert!(
            !bookmarks.contains_key(&project_path(path!("/project/file3.rs"))),
            "file3.rs should have no bookmarks"
        );
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_after_toggle_off(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\nline3\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[1], cx);
        assert_eq!(get_all_bookmarks(&project, cx).len(), 1);

        // Toggle same row again to remove it
        add_bookmarks(&project, &buffer, &[1], cx);
        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_with_clear(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "line1\nline2\nline3\n",
                "file2.rs": "lineA\nlineB\n"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer1 = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        let buffer2 = open_buffer(&project, path!("/project/file2.rs"), cx).await;

        add_bookmarks(&project, &buffer1, &[0], cx);
        add_bookmarks(&project, &buffer2, &[1], cx);
        assert_eq!(get_all_bookmarks(&project, cx).len(), 2);

        clear_bookmarks(&project, cx);
        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_returns_sorted_by_path(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"b.rs": "line1\n", "a.rs": "line1\n", "c.rs": "line1\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer_b = open_buffer(&project, path!("/project/b.rs"), cx).await;
        let buffer_a = open_buffer(&project, path!("/project/a.rs"), cx).await;
        let buffer_c = open_buffer(&project, path!("/project/c.rs"), cx).await;

        add_bookmarks(&project, &buffer_b, &[0], cx);
        add_bookmarks(&project, &buffer_a, &[0], cx);
        add_bookmarks(&project, &buffer_c, &[0], cx);

        let paths: Vec<_> = get_all_bookmarks(&project, cx).keys().cloned().collect();
        assert_eq!(
            paths,
            [
                project_path(path!("/project/a.rs")),
                project_path(path!("/project/b.rs")),
                project_path(path!("/project/c.rs")),
            ]
        );
    }

    #[gpui::test]
    async fn test_all_serialized_bookmarks_deduplicates_same_row(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\nline3\nline4\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[1, 2], cx);

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_bookmark_rows(&bookmarks, path!("/project/file1.rs"), &[1, 2]);

        // Verify no duplicates
        let rows: Vec<u32> = bookmarks
            .get(&project_path(path!("/project/file1.rs")))
            .unwrap()
            .iter()
            .map(|b| b.row)
            .collect();
        let mut deduped = rows.clone();
        deduped.dedup();
        assert_eq!(rows, deduped);
    }

    #[gpui::test]
    async fn test_with_serialized_bookmarks_restores_bookmarks(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "line1\nline2\nline3\nline4\nline5\n",
                "file2.rs": "aaa\nbbb\nccc\n"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        let serialized = build_serialized(&[
            (path!("/project/file1.rs"), &[0, 3]),
            (path!("/project/file2.rs"), &[1]),
        ]);

        restore_bookmarks(&project, serialized, cx).await;

        let restored = get_all_bookmarks(&project, cx);
        assert_eq!(restored.len(), 2);
        assert_bookmark_rows(&restored, path!("/project/file1.rs"), &[0, 3]);
        assert_bookmark_rows(&restored, path!("/project/file2.rs"), &[1]);
    }

    #[gpui::test]
    async fn test_with_serialized_bookmarks_skips_out_of_range_rows(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        // 3 lines: rows 0, 1, 2
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\nline3"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        let serialized = build_serialized(&[(path!("/project/file1.rs"), &[1, 100, 2])]);
        restore_bookmarks(&project, serialized, cx).await;

        // Before resolution, unloaded bookmarks are stored as-is
        let unresolved = get_all_bookmarks(&project, cx);
        assert_bookmark_rows(&unresolved, path!("/project/file1.rs"), &[1, 2, 100]);

        // Open the buffer to trigger lazy resolution
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        project.update(cx, |project, cx| {
            let buffer_snapshot = buffer.read(cx).snapshot();
            project.bookmark_store().update(cx, |store, cx| {
                store.bookmarks_for_buffer(
                    buffer.clone(),
                    buffer_snapshot.anchor_before(0)
                        ..buffer_snapshot.anchor_after(buffer_snapshot.len()),
                    &buffer_snapshot,
                    cx,
                );
            });
        });

        // After resolution, out-of-range rows are filtered
        let restored = get_all_bookmarks(&project, cx);
        assert_bookmark_rows(&restored, path!("/project/file1.rs"), &[1, 2]);
    }

    #[gpui::test]
    async fn test_with_serialized_bookmarks_skips_empty_entries(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "line1\nline2\n", "file2.rs": "aaa\nbbb\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        let mut serialized = build_serialized(&[(path!("/project/file1.rs"), &[0])]);
        serialized.insert(project_path(path!("/project/file2.rs")), vec![]);

        restore_bookmarks(&project, serialized, cx).await;

        let restored = get_all_bookmarks(&project, cx);
        assert_eq!(restored.len(), 1);
        assert!(restored.contains_key(&project_path(path!("/project/file1.rs"))));
        assert!(!restored.contains_key(&project_path(path!("/project/file2.rs"))));
    }

    #[gpui::test]
    async fn test_with_serialized_bookmarks_all_out_of_range_produces_no_entry(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"tiny.rs": "x"}))
            .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

        let serialized = build_serialized(&[(path!("/project/tiny.rs"), &[5, 10])]);
        restore_bookmarks(&project, serialized, cx).await;

        // Before resolution, unloaded bookmarks are stored as-is
        let unresolved = get_all_bookmarks(&project, cx);
        assert_eq!(unresolved.len(), 1);

        // Open the buffer to trigger lazy resolution
        let buffer = open_buffer(&project, path!("/project/tiny.rs"), cx).await;
        project.update(cx, |project, cx| {
            let buffer_snapshot = buffer.read(cx).snapshot();
            project.bookmark_store().update(cx, |store, cx| {
                store.bookmarks_for_buffer(
                    buffer.clone(),
                    buffer_snapshot.anchor_before(0)
                        ..buffer_snapshot.anchor_after(buffer_snapshot.len()),
                    &buffer_snapshot,
                    cx,
                );
            });
        });

        // After resolution, all out-of-range rows are filtered away
        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_with_serialized_bookmarks_replaces_existing(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file1.rs": "aaa\nbbb\nccc\nddd\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file1.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[0], cx);
        assert_bookmark_rows(
            &get_all_bookmarks(&project, cx),
            path!("/project/file1.rs"),
            &[0],
        );

        // Restoring different bookmarks should replace, not merge
        let serialized = build_serialized(&[(path!("/project/file1.rs"), &[2, 3])]);
        restore_bookmarks(&project, serialized, cx).await;

        let after = get_all_bookmarks(&project, cx);
        assert_eq!(after.len(), 1);
        assert_bookmark_rows(&after, path!("/project/file1.rs"), &[2, 3]);
    }

    #[gpui::test]
    async fn test_serialize_deserialize_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "alpha.rs": "fn main() {\n    println!(\"hello\");\n    return;\n}\n",
                "beta.rs": "use std::io;\nfn read() {}\nfn write() {}\n"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer_alpha = open_buffer(&project, path!("/project/alpha.rs"), cx).await;
        let buffer_beta = open_buffer(&project, path!("/project/beta.rs"), cx).await;

        add_bookmarks(&project, &buffer_alpha, &[0, 2, 3], cx);
        add_bookmarks(&project, &buffer_beta, &[1], cx);

        // Serialize
        let serialized = get_all_bookmarks(&project, cx);
        assert_eq!(serialized.len(), 2);
        assert_bookmark_rows(&serialized, path!("/project/alpha.rs"), &[0, 2, 3]);
        assert_bookmark_rows(&serialized, path!("/project/beta.rs"), &[1]);

        // Clear and restore
        clear_bookmarks(&project, cx);
        assert!(get_all_bookmarks(&project, cx).is_empty());

        restore_bookmarks(&project, serialized, cx).await;

        let restored = get_all_bookmarks(&project, cx);
        assert_eq!(restored.len(), 2);
        assert_bookmark_rows(&restored, path!("/project/alpha.rs"), &[0, 2, 3]);
        assert_bookmark_rows(&restored, path!("/project/beta.rs"), &[1]);
    }

    #[gpui::test]
    async fn test_round_trip_preserves_bookmarks_after_file_edit(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({"file.rs": "aaa\nbbb\nccc\nddd\neee\n"}),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[1, 3], cx);

        // Insert a line at the beginning, shifting bookmarks down by 1
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "new_first_line\n")], None, cx);
        });

        let serialized = get_all_bookmarks(&project, cx);
        assert_bookmark_rows(&serialized, path!("/project/file.rs"), &[2, 4]);

        // Clear and restore
        clear_bookmarks(&project, cx);
        restore_bookmarks(&project, serialized, cx).await;

        let restored = get_all_bookmarks(&project, cx);
        assert_bookmark_rows(&restored, path!("/project/file.rs"), &[2, 4]);
    }

    #[gpui::test]
    async fn test_file_deletion_removes_bookmarks(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "aaa\nbbb\nccc\n",
                "file2.rs": "ddd\neee\nfff\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer1 = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        let buffer2 = open_buffer(&project, path!("/project/file2.rs"), cx).await;

        add_bookmarks(&project, &buffer1, &[0, 2], cx);
        add_bookmarks(&project, &buffer2, &[1], cx);
        assert_eq!(get_all_bookmarks(&project, cx).len(), 2);

        // Delete file1.rs
        fs.remove_file(path!("/project/file1.rs").as_ref(), Default::default())
            .await
            .unwrap();
        cx.executor().run_until_parked();

        // file1.rs bookmarks should be gone, file2.rs bookmarks preserved
        let bookmarks = get_all_bookmarks(&project, cx);
        assert_eq!(bookmarks.len(), 1);
        assert!(!bookmarks.contains_key(&project_path(path!("/project/file1.rs"))));
        assert_bookmark_rows(&bookmarks, path!("/project/file2.rs"), &[1]);
    }

    #[gpui::test]
    async fn test_deleting_all_bookmarked_files_clears_store(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "aaa\nbbb\n",
                "file2.rs": "ccc\nddd\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer1 = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        let buffer2 = open_buffer(&project, path!("/project/file2.rs"), cx).await;

        add_bookmarks(&project, &buffer1, &[0], cx);
        add_bookmarks(&project, &buffer2, &[1], cx);
        assert_eq!(get_all_bookmarks(&project, cx).len(), 2);

        // Delete both files
        fs.remove_file(path!("/project/file1.rs").as_ref(), Default::default())
            .await
            .unwrap();
        fs.remove_file(path!("/project/file2.rs").as_ref(), Default::default())
            .await
            .unwrap();
        cx.executor().run_until_parked();

        assert!(get_all_bookmarks(&project, cx).is_empty());
    }

    #[gpui::test]
    async fn test_file_rename_re_keys_bookmarks(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"old_name.rs": "aaa\nbbb\nccc\n"}))
            .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/old_name.rs"), cx).await;

        add_bookmarks(&project, &buffer, &[0, 2], cx);
        assert_bookmark_rows(
            &get_all_bookmarks(&project, cx),
            path!("/project/old_name.rs"),
            &[0, 2],
        );

        // Rename the file
        fs.rename(
            path!("/project/old_name.rs").as_ref(),
            path!("/project/new_name.rs").as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
        cx.executor().run_until_parked();

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_eq!(bookmarks.len(), 1);
        assert!(!bookmarks.contains_key(&project_path(path!("/project/old_name.rs"))));
        assert_bookmark_rows(&bookmarks, path!("/project/new_name.rs"), &[0, 2]);
    }

    #[gpui::test]
    async fn test_file_rename_preserves_other_bookmarks(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "rename_me.rs": "aaa\nbbb\n",
                "untouched.rs": "ccc\nddd\neee\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer_rename = open_buffer(&project, path!("/project/rename_me.rs"), cx).await;
        let buffer_other = open_buffer(&project, path!("/project/untouched.rs"), cx).await;

        add_bookmarks(&project, &buffer_rename, &[1], cx);
        add_bookmarks(&project, &buffer_other, &[0, 2], cx);

        fs.rename(
            path!("/project/rename_me.rs").as_ref(),
            path!("/project/renamed.rs").as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
        cx.executor().run_until_parked();

        let bookmarks = get_all_bookmarks(&project, cx);
        assert_eq!(bookmarks.len(), 2);
        assert_bookmark_rows(&bookmarks, path!("/project/renamed.rs"), &[1]);
        assert_bookmark_rows(&bookmarks, path!("/project/untouched.rs"), &[0, 2]);
    }

    fn toggle_numbered(
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        row: u32,
        column: u32,
        number: u8,
        unique_across_files: bool,
        cx: &mut TestAppContext,
    ) {
        let buffer = buffer.clone();
        project.update(cx, |project, cx| {
            let bookmark_store = project.bookmark_store();
            let snapshot = buffer.read(cx).snapshot();
            let anchor = snapshot.anchor_after(text::Point::new(row, column));
            bookmark_store.update(cx, |store, cx| {
                store.toggle_numbered_bookmark(
                    buffer.clone(),
                    anchor,
                    number,
                    unique_across_files,
                    cx,
                );
            });
        });
    }

    fn numbered_locations(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> Vec<(std::sync::Arc<std::path::Path>, SerializedNumberedBookmark)> {
        project.read_with(cx, |project, cx| {
            project
                .bookmark_store()
                .read(cx)
                .numbered_bookmark_locations(cx)
        })
    }

    #[gpui::test]
    async fn test_toggle_numbered_bookmark_replaces_other_number_on_same_line(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"file.rs": "aaa\nbbb\nccc\n"}))
            .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = open_buffer(&project, path!("/project/file.rs"), cx).await;

        toggle_numbered(&project, &buffer, 1, 0, 1, false, cx);
        toggle_numbered(&project, &buffer, 1, 2, 2, false, cx);

        let locations = numbered_locations(&project, cx);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations[0].1.number, 2);
        assert_eq!(locations[0].1.row, 1);
        assert_eq!(locations[0].1.column, 2);
    }

    #[gpui::test]
    async fn test_toggle_numbered_bookmark_unique_across_files(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file1.rs": "aaa\nbbb\n",
                "file2.rs": "ccc\nddd\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer1 = open_buffer(&project, path!("/project/file1.rs"), cx).await;
        let buffer2 = open_buffer(&project, path!("/project/file2.rs"), cx).await;

        toggle_numbered(&project, &buffer1, 0, 0, 3, true, cx);
        toggle_numbered(&project, &buffer2, 1, 1, 3, true, cx);

        let locations = numbered_locations(&project, cx);
        assert_eq!(locations.len(), 1);
        assert!(locations[0].0.ends_with(std::path::Path::new("file2.rs")));
        assert_eq!(locations[0].1.number, 3);
        assert_eq!(locations[0].1.row, 1);
        assert_eq!(locations[0].1.column, 1);
    }

    #[gpui::test]
    async fn test_numbered_bookmarks_in_untitled_buffer_move_to_saved_file(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"other.rs": "xxx\n"}))
            .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let untitled = project.update(cx, |project, cx| {
            project.create_local_buffer("aaa\nbbb\nccc\n", None, true, cx)
        });
        let other = open_buffer(&project, path!("/project/other.rs"), cx).await;

        toggle_numbered(&project, &untitled, 1, 2, 1, false, cx);
        toggle_numbered(&project, &other, 0, 0, 1, false, cx);
        let untitled_bookmark_row = |cx: &mut TestAppContext| {
            project.update(cx, |project, cx| {
                let snapshot = untitled.read(cx).text_snapshot();
                project.bookmark_store().update(cx, |store, cx| {
                    store
                        .numbered_bookmark_in_buffer(&untitled, 1, cx)
                        .map(|bookmark| bookmark.anchor.summary::<text::Point>(&snapshot))
                })
            })
        };
        assert_eq!(untitled_bookmark_row(cx), Some(text::Point::new(1, 2)));
        // Untitled bookmarks are not persisted, so only the file on disk is listed.
        let locations = numbered_locations(&project, cx);
        assert_eq!(locations.len(), 1);
        assert!(locations[0].0.ends_with(Path::new("other.rs")));

        toggle_numbered(&project, &untitled, 1, 2, 1, false, cx);
        assert_eq!(untitled_bookmark_row(cx), None);
        toggle_numbered(&project, &untitled, 2, 0, 4, false, cx);

        let worktree_id = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        project
            .update(cx, |project, cx| {
                project.save_buffer_as(
                    untitled.clone(),
                    project::ProjectPath {
                        worktree_id,
                        path: util::rel_path::rel_path("saved.rs").into(),
                    },
                    cx,
                )
            })
            .await
            .expect("saving the untitled buffer should succeed");
        cx.executor().run_until_parked();

        let locations = numbered_locations(&project, cx);
        assert_eq!(locations.len(), 2);
        let (saved_path, saved_bookmark) = locations
            .iter()
            .find(|(path, _)| path.ends_with(Path::new("saved.rs")))
            .expect("bookmark should move to the saved file");
        assert_eq!(saved_path.as_ref(), Path::new(path!("/project/saved.rs")));
        assert_eq!(saved_bookmark.number, 4);
        assert_eq!(saved_bookmark.row, 2);
    }

    #[gpui::test]
    async fn test_numbered_bookmarks_vscode_project_file_roundtrip(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "a.rs": "aaa\nbbb\nccc\n",
                "b.rs": "ddd\neee\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer_a = open_buffer(&project, path!("/project/a.rs"), cx).await;
        let buffer_b = open_buffer(&project, path!("/project/b.rs"), cx).await;

        toggle_numbered(&project, &buffer_a, 2, 1, 1, false, cx);
        toggle_numbered(&project, &buffer_b, 0, 0, 9, false, cx);

        let project_file = project.read_with(cx, |project, cx| {
            project
                .bookmark_store()
                .read(cx)
                .vscode_project_file_for_worktree(std::path::Path::new(path!("/project")), cx)
        });

        assert_eq!(project_file.files.len(), 2);
        assert_eq!(project_file.files[0].path, "a.rs");
        assert_eq!(project_file.files[0].bookmarks.len(), 10);
        assert_eq!(project_file.files[0].bookmarks[1].line, 2);
        assert_eq!(project_file.files[0].bookmarks[1].column, 1);
        assert_eq!(project_file.files[0].bookmarks[0].line, -1);
        assert_eq!(project_file.files[1].path, "b.rs");
        assert_eq!(project_file.files[1].bookmarks[9].line, 0);

        project.update(cx, |project, cx| {
            project.bookmark_store().update(cx, |store, cx| {
                store.clear_all_numbered_bookmarks(cx);
            });
        });
        assert!(numbered_locations(&project, cx).is_empty());

        project.update(cx, |project, cx| {
            project.bookmark_store().update(cx, |store, cx| {
                store.load_vscode_project_file(
                    std::path::Path::new(path!("/project")),
                    project_file,
                    cx,
                );
            });
        });

        let restored = numbered_locations(&project, cx);
        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0].1.number, 1);
        assert_eq!(restored[0].1.row, 2);
        assert_eq!(restored[1].1.number, 9);
        assert_eq!(restored[1].1.row, 0);
    }
}
