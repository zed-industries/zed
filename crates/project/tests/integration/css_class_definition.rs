use std::path::Path;

use crate::init_test;
use fs::FakeFs;
use language::{Language, LanguageConfig, LanguageMatcher, PointUtf16};
use project::Project;
use serde_json::json;

fn html_language() -> Language {
    Language::new(
        LanguageConfig {
            name: "HTML".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["html".into()],
                ..Default::default()
            },
            ..Default::default()
        },
        None,
    )
}

/// Verifies that go-to-definition on a CSS class name in HTML
/// navigates to the CSS definition even when it's inline in a <style> tag
/// in the same file.
#[gpui::test]
async fn test_css_class_inline_style_same_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            "index.html": r#"<!doctype html>
<html>
<head>
<style>
.bg-grid {
  position: fixed;
  inset: 0;
}
</style>
</head>
<body>
<div class="bg-grid"></div>
</body>
</html>"#,
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [], cx).await;
    cx.background_executor.run_until_parked();

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(html_language()));

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(Path::new("/root"), true, cx)
        })
        .await
        .unwrap();

    worktree
        .read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;

    let buffer = project
        .update(cx, |p, cx| {
            p.open_local_buffer(Path::new("/root/index.html"), cx)
        })
        .await
        .unwrap();

    let lang_name = buffer.read_with(cx, |b, _| b.language().map(|l| l.name().to_string()));
    assert_eq!(
        lang_name,
        Some("HTML".to_string()),
        "File should be detected as HTML"
    );
    let position = PointUtf16::new(11, 15);

    let links = project
        .update(cx, |p, cx| p.definitions(&buffer, position, cx))
        .await
        .unwrap();

    assert!(
        links.is_some(),
        "Should find CSS class definition for bg-grid"
    );
    let links = links.unwrap();
    assert!(!links.is_empty(), "Should have at least one LocationLink");

    let link = &links[0];

    cx.update(|cx| {
        let target_snapshot = link.target.buffer.read(cx).snapshot();
        let target_text = target_snapshot
            .text_for_range(link.target.range.clone())
            .collect::<String>();
        assert_eq!(target_text, ".bg-grid", "Target should be the CSS selector");

        let origin = link.origin.as_ref().expect("Origin should be Some");
        let origin_snapshot = origin.buffer.read(cx).snapshot();
        let origin_text = origin_snapshot
            .text_for_range(origin.range.clone())
            .collect::<String>();
        assert_eq!(origin_text, "bg-grid", "Origin should be the class token");

        assert_eq!(
            origin.buffer.entity_id(),
            link.target.buffer.entity_id(),
            "Origin and target should be in the same file for inline CSS"
        );

        assert_ne!(
            origin.range, link.target.range,
            "Origin and target should have different ranges"
        );
    });
}
