use std::{path::Path, sync::Arc};

use crate::init_test;
use fs::{FakeFs, Fs};
use gpui::{Entity, TestAppContext, UpdateGlobal};
use project::{
    Project, ProjectEntryId, ProjectPath,
    project_search::PathInclusionMatcher,
    search::{SearchOmission, SearchOmissionReason, SearchQuery, SearchResult},
};
use serde_json::json;
use settings::{Settings, SettingsStore, WorktreeId};
use util::{
    path,
    paths::{PathMatcher, PathStyle},
    rel_path::{RelPath, rel_path},
};
use worktree::{Entry, EntryKind, WorktreeSettings};

#[gpui::test]
async fn test_path_inclusion_matcher(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            ".gitignore": "src/data/\n",
            "src": {
                "data": {
                    "main.csv": "field_1,field_2,field_3",
                },
                "lib": {
                    "main.txt": "Are you familiar with fields?",
                },
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let (worktree_settings, worktree_snapshot) = worktree.update(cx, |worktree, cx| {
        let settings_location = worktree.settings_location(cx);
        return (
            WorktreeSettings::get(Some(settings_location), cx).clone(),
            worktree.snapshot(),
        );
    });

    // Manually create a test entry for the gitignored directory since it won't
    // be loaded by the worktree
    let entry = Entry {
        id: ProjectEntryId::from_proto(1),
        kind: EntryKind::UnloadedDir,
        path: Arc::from(RelPath::from_unix_str(Path::new("src/data")).unwrap()),
        inode: 0,
        mtime: None,
        canonical_path: None,
        is_ignored: true,
        is_hidden: false,
        is_always_included: false,
        is_external: false,
        is_private: false,
        size: 0,
        char_bag: Default::default(),
        is_fifo: false,
    };

    // 1. Test searching for `field`, including ignored files without any
    // inclusion and exclusion filters.
    let include_ignored = true;
    let files_to_include = PathMatcher::default();
    let files_to_exclude = PathMatcher::default();
    let match_full_paths = false;
    let search_query = SearchQuery::text(
        "field",
        false,
        false,
        include_ignored,
        files_to_include,
        files_to_exclude,
        match_full_paths,
        None,
    )
    .unwrap();

    let path_matcher = PathInclusionMatcher::new(Arc::new(search_query));
    assert!(path_matcher.should_scan_gitignored_dir(
        &entry,
        &worktree_snapshot,
        &worktree_settings
    ));

    // 2. Test searching for `field`, including ignored files but updating
    // `files_to_include` to only include files under `src/lib`.
    let include_ignored = true;
    let files_to_include = PathMatcher::new(vec!["src/lib"], PathStyle::Unix).unwrap();
    let files_to_exclude = PathMatcher::default();
    let match_full_paths = false;
    let search_query = SearchQuery::text(
        "field",
        false,
        false,
        include_ignored,
        files_to_include,
        files_to_exclude,
        match_full_paths,
        None,
    )
    .unwrap();

    let path_matcher = PathInclusionMatcher::new(Arc::new(search_query));
    assert!(!path_matcher.should_scan_gitignored_dir(
        &entry,
        &worktree_snapshot,
        &worktree_settings
    ));
}

#[gpui::test]
async fn test_search_omissions_before_filters_and_without_scanning(cx: &mut TestAppContext) {
    init_test(cx);
    set_scan_settings(cx, 2, Vec::new());
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/one"),
        json!({
            ".gitignore": "ignored/\n*.log\n",
            "a": { "b": { "deep.txt": "needle" } },
            "ignored": { "nested": { "file.txt": "needle" } },
            "single.log": "needle",
            "visible.txt": "needle"
        }),
    )
    .await;
    fs.insert_tree(
        path!("/two"),
        json!({
            "c": { "d": { "deep.txt": "needle" } }
        }),
    )
    .await;
    let project = Project::test(
        fs.clone(),
        [path!("/one").as_ref(), path!("/two").as_ref()],
        cx,
    )
    .await;
    let worktrees = project.read_with(cx, |project, cx| {
        project.visible_worktrees(cx).collect::<Vec<_>>()
    });
    for worktree in &worktrees {
        worktree
            .read_with(cx, |worktree, _| {
                worktree.as_local().unwrap().scan_complete()
            })
            .await;
    }
    let one = worktrees[0].read_with(cx, |worktree, _| worktree.id());
    let two = worktrees[1].read_with(cx, |worktree, _| worktree.id());
    let expected = vec![
        omission(one, "a/b", SearchOmissionReason::NotIndexed),
        omission(one, "ignored", SearchOmissionReason::GitIgnored),
        omission(one, "single.log", SearchOmissionReason::GitIgnored),
        omission(two, "c/d", SearchOmissionReason::NotIndexed),
    ];
    let read_dirs = fs.read_dir_call_count();
    let (omissions, matches) = collect_search(&project, query(false), cx).await;
    assert_eq!(omissions, expected);
    assert_eq!(
        matches,
        vec![ProjectPath::from((one, rel_path("visible.txt")))]
    );
    assert_eq!(fs.read_dir_call_count(), read_dirs);

    let filtered = SearchQuery::text(
        "needle",
        false,
        false,
        false,
        PathMatcher::new(["one/visible.txt"], PathStyle::local()).unwrap(),
        PathMatcher::new(["one/visible.txt"], PathStyle::local()).unwrap(),
        true,
        None,
    )
    .unwrap();
    let (omissions, matches) = collect_search(&project, filtered, cx).await;
    assert_eq!(omissions, expected);
    assert_eq!(matches, Vec::new());
    assert_eq!(fs.read_dir_call_count(), read_dirs);
}

#[gpui::test]
async fn test_search_omissions_ignored_expansion_and_inclusions(cx: &mut TestAppContext) {
    init_test(cx);
    set_scan_settings(cx, 0, vec!["all_included/keep.rs".to_owned()]);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "ignored/\nempty/\nall_included/\n*.log\n",
            "ignored": {
                "keep.rs": "needle",
                "skipped.txt": "needle",
                "nested": { "skipped.txt": "needle" }
            },
            "all_included": { "keep.rs": "needle" },
            "empty": {},
            "single.log": "needle",
            "visible.txt": "needle"
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree = project.read_with(cx, |project, cx| {
        project.visible_worktrees(cx).next().unwrap()
    });
    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    let (omissions, matches) = collect_search(&project, query(false), cx).await;
    assert_eq!(
        omissions,
        vec![
            omission(worktree_id, "empty", SearchOmissionReason::GitIgnored),
            omission(worktree_id, "ignored", SearchOmissionReason::GitIgnored),
            omission(worktree_id, "single.log", SearchOmissionReason::GitIgnored),
        ]
    );
    assert_eq!(
        matches,
        vec![
            ProjectPath::from((worktree_id, rel_path("all_included/keep.rs"))),
            ProjectPath::from((worktree_id, rel_path("visible.txt"))),
        ]
    );

    for path in ["empty", "ignored", "ignored/nested"] {
        worktree
            .update(cx, |worktree, cx| {
                let entry_id = worktree.entry_for_path(rel_path(path)).unwrap().id;
                worktree.expand_entry(entry_id, cx).unwrap()
            })
            .await
            .unwrap();
    }
    let read_dirs = fs.read_dir_call_count();
    let (omissions, _) = collect_search(&project, query(false), cx).await;
    assert_eq!(
        omissions,
        vec![
            omission(worktree_id, "ignored", SearchOmissionReason::GitIgnored),
            omission(worktree_id, "single.log", SearchOmissionReason::GitIgnored),
        ]
    );
    assert_eq!(fs.read_dir_call_count(), read_dirs);

    set_scan_settings(
        cx,
        0,
        vec![
            "ignored/keep.rs".to_owned(),
            "all_included/keep.rs".to_owned(),
        ],
    );
    cx.run_until_parked();
    worktree
        .update(cx, |worktree, cx| {
            let entry_id = worktree.entry_for_path(rel_path("empty")).unwrap().id;
            worktree.expand_entry(entry_id, cx).unwrap()
        })
        .await
        .unwrap();
    let (omissions, matches) = collect_search(&project, query(false), cx).await;
    assert_eq!(
        omissions,
        vec![
            omission(worktree_id, "ignored", SearchOmissionReason::GitIgnored),
            omission(worktree_id, "single.log", SearchOmissionReason::GitIgnored),
        ]
    );
    assert_eq!(
        matches,
        vec![
            ProjectPath::from((worktree_id, rel_path("all_included/keep.rs"))),
            ProjectPath::from((worktree_id, rel_path("ignored/keep.rs"))),
            ProjectPath::from((worktree_id, rel_path("visible.txt"))),
        ]
    );

    let (omissions, matches) = collect_search(&project, query(true), cx).await;
    assert_eq!(omissions, Vec::new());
    assert_eq!(
        matches,
        vec![
            ProjectPath::from((worktree_id, rel_path("all_included/keep.rs"))),
            ProjectPath::from((worktree_id, rel_path("ignored/keep.rs"))),
            ProjectPath::from((worktree_id, rel_path("ignored/nested/skipped.txt"))),
            ProjectPath::from((worktree_id, rel_path("ignored/skipped.txt"))),
            ProjectPath::from((worktree_id, rel_path("single.log"))),
            ProjectPath::from((worktree_id, rel_path("visible.txt"))),
        ]
    );
}

#[gpui::test]
async fn test_search_omissions_include_ignored_keeps_unindexed_frontier(cx: &mut TestAppContext) {
    init_test(cx);
    set_scan_settings(cx, 1, Vec::new());
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "ignored/\n",
            "ignored": { "file.txt": "needle" },
            "unindexed": { "file.txt": "needle" },
            "visible.txt": "needle"
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.visible_worktrees(cx).next().unwrap().read(cx).id()
    });
    let filtered = SearchQuery::text(
        "needle",
        false,
        false,
        true,
        PathMatcher::new(["visible.txt"], PathStyle::local()).unwrap(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let (omissions, matches) = collect_search(&project, filtered, cx).await;
    assert_eq!(
        omissions,
        vec![
            omission(worktree_id, "ignored", SearchOmissionReason::NotIndexed),
            omission(worktree_id, "unindexed", SearchOmissionReason::NotIndexed),
        ]
    );
    assert_eq!(
        matches,
        vec![ProjectPath::from((worktree_id, rel_path("visible.txt")))]
    );

    let (omissions, matches) = collect_search(&project, query(true), cx).await;
    assert_eq!(
        omissions,
        vec![omission(
            worktree_id,
            "unindexed",
            SearchOmissionReason::NotIndexed
        )]
    );
    assert_eq!(
        matches,
        vec![
            ProjectPath::from((worktree_id, rel_path("ignored/file.txt"))),
            ProjectPath::from((worktree_id, rel_path("visible.txt"))),
        ]
    );
}

#[gpui::test]
async fn test_search_omissions_scan_inclusions_and_opened_only(cx: &mut TestAppContext) {
    init_test(cx);
    set_scan_settings(cx, 1, vec!["a/b/keep.txt".to_owned()]);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "a": { "b": { "keep.txt": "needle", "deferred": { "file.txt": "needle" } } },
            "other": { "file.txt": "needle" }
        }),
    )
    .await;
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.visible_worktrees(cx).next().unwrap().read(cx).id()
    });
    let (omissions, matches) = collect_search(&project, query(false), cx).await;
    assert_eq!(
        omissions,
        vec![omission(
            worktree_id,
            "other",
            SearchOmissionReason::NotIndexed
        )]
    );
    let keep_path = ProjectPath::from((worktree_id, rel_path("a/b/keep.txt")));
    assert_eq!(
        matches,
        vec![
            ProjectPath::from((worktree_id, rel_path("a/b/deferred/file.txt"))),
            keep_path.clone(),
        ]
    );
    let buffer = project
        .update(cx, |project, cx| project.open_buffer(keep_path.clone(), cx))
        .await
        .unwrap();
    let read_dirs = fs.read_dir_call_count();
    for buffers in [vec![buffer], Vec::new()] {
        let expected_matches = if buffers.is_empty() {
            Vec::new()
        } else {
            vec![keep_path.clone()]
        };
        let opened_only = SearchQuery::text(
            "needle",
            false,
            false,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            false,
            Some(buffers),
        )
        .unwrap();
        let (omissions, matches) = collect_search(&project, opened_only, cx).await;
        assert_eq!(omissions, Vec::new());
        assert_eq!(matches, expected_matches);
        assert_eq!(fs.read_dir_call_count(), read_dirs);
    }
    let results = project.update(cx, |project, cx| project.search(query(false), cx));
    drop(results.omissions);
    let mut matches = 0;
    while let Ok(result) = results.rx.recv().await {
        if let SearchResult::Buffer { .. } = result {
            matches += 1;
        }
    }
    assert_eq!(matches, 2);
    results.task_handle.await;
    assert_eq!(fs.read_dir_call_count(), read_dirs);
}

#[gpui::test]
async fn test_search_omissions_nested_different_reasons(cx: &mut TestAppContext) {
    init_test(cx);
    set_scan_settings(cx, 0, vec!["ignored/external/keep.txt".to_owned()]);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "ignored/\n",
            "ignored": { "skipped.txt": "needle", "nested": { "skipped.txt": "needle" } }
        }),
    )
    .await;
    fs.insert_tree(path!("/outside"), json!({ "keep.txt": "needle" }))
        .await;
    fs.create_symlink(
        path!("/root/ignored/external").as_ref(),
        path!("/outside").into(),
    )
    .await
    .unwrap();
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.visible_worktrees(cx).next().unwrap().read(cx).id()
    });
    let (omissions, matches) = collect_search(&project, query(false), cx).await;
    assert_eq!(
        omissions,
        vec![
            omission(worktree_id, "ignored", SearchOmissionReason::GitIgnored),
            omission(
                worktree_id,
                "ignored/external",
                SearchOmissionReason::NotIndexed
            ),
        ]
    );
    assert_eq!(matches, Vec::new());
}

fn set_scan_settings(cx: &mut TestAppContext, depth: u32, inclusions: Vec<String>) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_depth = Some(depth);
                settings.project.worktree.file_scan_inclusions = Some(inclusions);
            });
        });
    });
}

fn query(include_ignored: bool) -> SearchQuery {
    SearchQuery::text(
        "needle",
        false,
        false,
        include_ignored,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap()
}

fn omission(worktree_id: WorktreeId, path: &str, reason: SearchOmissionReason) -> SearchOmission {
    SearchOmission {
        path: ProjectPath {
            worktree_id,
            path: Arc::from(RelPath::from_unix_str(path).unwrap()),
        },
        reason,
    }
}

async fn collect_search(
    project: &Entity<Project>,
    query: SearchQuery,
    cx: &mut TestAppContext,
) -> (Vec<SearchOmission>, Vec<ProjectPath>) {
    let results = project.update(cx, |project, cx| project.search(query, cx));
    let mut matches = Vec::new();
    while let Ok(result) = results.rx.recv().await {
        if let SearchResult::Buffer { buffer, .. } = result {
            matches.push(buffer.read_with(cx, |buffer, cx| {
                let file = buffer.file().unwrap();
                ProjectPath::from((file.worktree_id(cx), file.path().clone()))
            }));
        }
    }
    let mut omissions = Vec::new();
    while let Ok(batch) = results.omissions.recv().await {
        omissions.extend(batch);
    }
    assert!(results.omissions_status.is_complete());
    results.task_handle.await;
    (omissions, matches)
}
