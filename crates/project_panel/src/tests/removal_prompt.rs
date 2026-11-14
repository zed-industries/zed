use crate::{ProjectPanel, RemovalKind};

#[test]
fn test_single_file_trash_prompt() {
    let items = [("document.txt", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &items, 0);

    assert_eq!(
        prompt.message,
        "Are you sure you want to delete `document.txt`?"
    );
    if cfg!(target_os = "windows") {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore this file from the Recycle Bin.")
        );
        assert_eq!(prompt.confirmation_label, "&Move to Recycle Bin");
    } else {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore this file from the Trash.")
        );
        assert_eq!(prompt.confirmation_label, "Move to Trash");
    }
}

#[test]
fn test_single_folder_trash_prompt() {
    let items = [("references", true)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &items, 0);

    assert_eq!(
        prompt.message,
        "Are you sure you want to delete `references` and its contents?"
    );
    if cfg!(target_os = "windows") {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore this folder from the Recycle Bin.")
        );
        assert_eq!(prompt.confirmation_label, "&Move to Recycle Bin");
    } else {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore this folder from the Trash.")
        );
        assert_eq!(prompt.confirmation_label, "Move to Trash");
    }
}

#[test]
fn test_multiple_items_trash_prompt() {
    // All files
    let files = [("a.txt", false), ("b.txt", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &files, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to delete the following 2 files?\n`a.txt`\n`b.txt`"
    );

    // All folders
    let folders = [("dir1", true), ("dir2", true)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &folders, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to delete the following 2 folders and their contents?\n`dir1`\n`dir2`"
    );

    // Mixed
    let mixed = [("dir1", true), ("a.txt", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &mixed, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to delete the following 2 items and their contents?\n`dir1`\n`a.txt`"
    );
    if cfg!(target_os = "windows") {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore these items from the Recycle Bin.")
        );
    } else {
        assert_eq!(
            prompt.detail.as_deref(),
            Some("You can restore these items from the Trash.")
        );
    }
}

#[test]
fn test_permanent_delete_prompt() {
    // Single file
    let file = [("config.json", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Delete, &file, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to permanently delete `config.json`?"
    );
    assert_eq!(
        prompt.detail.as_deref(),
        Some("This action cannot be undone.")
    );
    assert_eq!(prompt.confirmation_label, "Delete");

    // Single folder
    let folder = [("node_modules", true)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Delete, &folder, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to permanently delete `node_modules` and its contents?"
    );
    assert_eq!(
        prompt.detail.as_deref(),
        Some("This action cannot be undone.")
    );
    assert_eq!(prompt.confirmation_label, "Delete");

    // Multiple
    let multiple = [("dir1", true), ("file.rs", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Delete, &multiple, 0);
    assert_eq!(
        prompt.message,
        "Are you sure you want to permanently delete the following 2 items?\n`dir1`\n`file.rs`"
    );
}

#[test]
fn test_dirty_buffers_warning() {
    let items = [("unsaved.txt", false)];
    let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, &items, 1);
    assert!(
        prompt
            .message
            .ends_with("\n\nIt has unsaved changes, which will be lost.")
    );
}
