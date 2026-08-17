use version_sync::assert_markdown_deps_updated;

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}
