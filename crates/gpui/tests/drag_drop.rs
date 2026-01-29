//! Integration tests for drag-drop types.

use gpui::{DragType, DropItem, ExternalDrop};
use smallvec::smallvec;
use std::path::PathBuf;

#[test]
fn test_external_drop_paths_only() {
    let drop = ExternalDrop(smallvec![
        DropItem::Path(PathBuf::from("/tmp/file1.txt")),
        DropItem::Path(PathBuf::from("/tmp/file2.txt")),
    ]);

    let paths: Vec<_> = drop.paths().collect();
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], &PathBuf::from("/tmp/file1.txt"));

    let urls: Vec<_> = drop.urls().collect();
    assert!(urls.is_empty());
}

#[test]
fn test_external_drop_urls_only() {
    let drop = ExternalDrop(smallvec![DropItem::Url(
        url::Url::parse("https://example.com/image.png").unwrap()
    ),]);

    let paths: Vec<_> = drop.paths().collect();
    assert!(paths.is_empty());

    let urls: Vec<_> = drop.urls().collect();
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0].as_str(), "https://example.com/image.png");
}

#[test]
fn test_external_drop_mixed() {
    let drop = ExternalDrop(smallvec![
        DropItem::Path(PathBuf::from("/tmp/file.txt")),
        DropItem::Url(url::Url::parse("https://example.com/").unwrap()),
    ]);

    assert_eq!(drop.items().len(), 2);
    assert_eq!(drop.paths().count(), 1);
    assert_eq!(drop.urls().count(), 1);
}

#[test]
fn test_external_drop_to_legacy() {
    let drop = ExternalDrop(smallvec![
        DropItem::Path(PathBuf::from("/tmp/file.txt")),
        DropItem::Url(url::Url::parse("https://example.com/").unwrap()),
    ]);

    #[allow(deprecated)]
    let legacy = drop.to_external_paths();
    assert_eq!(legacy.paths().len(), 1);
}

#[test]
fn test_drag_type_default() {
    let default = DragType::default();
    assert_eq!(default, DragType::Files);
}
