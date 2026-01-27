use std::{path::Path, sync::Arc};

use crate::init_test;
use fs::FakeFs;
use project::{Project, ProjectEntryId, project_search::PathInclusionMatcher, search::SearchQuery};
use serde_json::json;
use settings::Settings;
use util::{
    path,
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
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
        path: Arc::from(RelPath::unix(Path::new("src/data")).unwrap()),
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
    let files_to_include = PathMatcher::new(vec!["src/lib"], PathStyle::Posix).unwrap();
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
