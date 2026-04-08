use std::{path::Path, sync::Arc};

use collections::BTreeMap;
use gpui::{Entity, TestAppContext};
use language::Buffer;
use project::{Project, bookmark_store::SerializedBookmark};
use serde_json::json;
use util::path;

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
                    store.toggle_bookmark(buffer.clone(), anchor, cx);
                });
            }
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
                path.clone(),
                rows.iter().map(|&row| SerializedBookmark(row)).collect(),
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
        let rows: Vec<u32> = file_bookmarks.iter().map(|b| b.0).collect();
        assert_eq!(rows, expected_rows, "Bookmark rows for {}", path.display());
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
            .map(|b| b.0)
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
}
